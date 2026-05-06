use std::fs::DirEntry;
use std::path::PathBuf;
use std::env;
use std::io::{BufReader, Seek};
use regex::Regex;

use chrono::{TimeZone, Utc};
use tauri::{AppHandle, Emitter};

use crate::db::worlds::WorldQuery;

pub async fn main(app: AppHandle) -> anyhow::Result<()> {
  log::debug!("Log watcher started");
  let vrc_dir = {
    let mut path = PathBuf::from(env::home_dir().unwrap());
    path.push("AppData");
    path.push("LocalLow");
    path.push("VRChat");
    path.push("VRChat");
    path
  };
  log::debug!("Watching log files in {:?}", vrc_dir);

  loop {
    let mut updated = false;

    let mut log_files: Vec<DirEntry> = std::fs::read_dir(&vrc_dir)?
      .filter(|entry| { entry.is_ok() })
      .map(|entry| { entry.unwrap() })
      .filter(|entry| {
        if let Some(s) = entry.file_name().to_str() {
          if s.starts_with("output_log_") {
            return true;
          }
        }
        false
       })
      .collect();
    log::debug!("Found log files: {:?}", log_files);

    log_files.sort_by(|a,b| a.file_name().cmp(&b.file_name()));

    let mut process_target: Vec<(u64, DirEntry)> = vec![]; // begin_at, file
    for entry in log_files.into_iter() {
      log::debug!("Attempting to process log file: {:?}", entry.path());
      if let Some(v) = crate::db::log_files::get_log(&entry.file_name().into_string().unwrap()).await? {
        if (v.read_at as u64) < std::fs::metadata(entry.path())?.len() {
          process_target.push(((v.read_at as u64), entry));
        }
      } else {
        process_target.push((0, entry));
      }
    }

    log::debug!("Log files to process: {:?}", process_target.iter().map(|p| (p.1.file_name(), p.0)).collect::<Vec<_>>());

    let mut process_target = process_target.into_iter().peekable();
    while let Some((begin, entry)) = process_target.next() {
      let mut reader = BufReader::new(std::fs::File::open(entry.path())?);
      reader.seek(std::io::SeekFrom::Start(begin))?;

      let (len, dat) = process_file(reader)?;
      if crate::db::log_files::get_log(entry.file_name().to_str().unwrap()).await?.is_none() {
        crate::db::log_files::new_log(entry.file_name().to_str().unwrap()).await?;
      }
      crate::db::log_files::update_log_read_at(entry.file_name().to_str().ok_or(anyhow::anyhow!("Log file name missing"))?, len).await.unwrap();

      for session in dat.into_iter() {
        if !crate::db::worlds::does_world_exist(&session.world_uuid).await? {
          updated = true;
        }

        let id = crate::db::worlds::upsert_world(WorldQuery {
          uuid: session.world_uuid.clone(),
          title: Some(session.world_name.clone()),
          publisher_uuid: None,
          publisher_name: None,
          description: None,
          visits: None,
          favorites: None,
          capacity: None,
          published_at: None,
          does_support_windows: None,
          does_support_android: None,
          does_support_ios: None,
          latest_at: Some(session.ended_at.timestamp_millis()),
          registered_at: None,
        }).await?.id;

        crate::db::worlds::new_session(
          id,
          session.started_at.timestamp_millis(),
          session.ended_at.timestamp_millis()
        ).await?;

      }
    }

    if updated {
      app.emit("new-world", ()).unwrap();
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(1 * 1000)).await;
  }
}

fn process_file<R: std::io::BufRead + std::io::Seek>(mut reader: R) -> anyhow::Result<(i64, Vec<Session>)> {
  //! @returns line number by which processing is done

  let header_regex = Regex::new(r"^[0-9]{4}\.[0-9]{2}\.[0-9]{2} [0-9]{2}\:[0-9]{2}\:[0-9]{2}\ .+? -  ").unwrap();
  let enter_room_uuid_regex = Regex::new(r"^\[Behaviour\] Joining (wrld_.+?):").unwrap();
  let enter_room_name_regex = Regex::new(r"^\[Behaviour\] Entering Room\: (.+)\n?$").unwrap();
  let exit_room_regex = Regex::new(r"^\[Behaviour\] OnLeftRoom").unwrap();

  let mut now_header: Option<LogHeader> = None;
  let mut session_from: Option<chrono::DateTime<Utc>> = None;
  let mut session_world_name: Option<String> = None;
  let mut session_world_uuid: Option<String> = None;

  let mut read_until: i64 = reader.seek(std::io::SeekFrom::Current(0))? as i64;
  let mut sessions: Vec<Session> = vec![];

  let mut buf = String::new();
  loop {
    if buf.is_empty() {
      let lin = reader.read_line(&mut buf)?;
      if lin == 0 {
        break;
      }
    }

    // if header
    if header_regex.is_match(&buf) {
      let left: String;
      let right: String;
      {
        let (l, r) = buf.split_at(34); // length of header is 34
        left = l.to_string();
        right = r.to_string();
      }
      buf = right;

      let years: i32 = left[0..4].parse()?; // 0, 1, 2, 3
      // 4
      let months: u32 = left[5..7].parse()?; // 5, 6
      // 7
      let days: u32 = left[8..10].parse()?; // 8, 9
      // 10
      let hours: u32 = left[11..13].parse()?; // 11, 12
      // 13
      let minutes: u32 = left[14..16].parse()?; // 14, 15
      // 16
      let seconds: u32 = left[17..19].parse()?; // 17, 18
      // 19
      let level = &left[20..27];

      let time = chrono::Utc.with_ymd_and_hms(years, months, days, hours, minutes, seconds).unwrap();

      let level = match level.trim().to_uppercase().as_str() {
        "DEBUG" => LogLevel::Debug,
        "WARNING" => LogLevel::Warning,
        "ERROR" => LogLevel::Error,
        _ => LogLevel::Unknown,
      };

      now_header = Some(LogHeader::new(time, level));
      continue;
    }
    if enter_room_uuid_regex.is_match(&buf) {
      session_world_uuid = Some(enter_room_uuid_regex.captures(&buf).unwrap()[1].to_owned());
      session_from = Some(now_header.ok_or(anyhow::anyhow!("Missing log header"))?.time);
    } else if enter_room_name_regex.is_match(&buf) {
      session_world_name = Some(enter_room_name_regex.captures(&buf).unwrap()[1].to_owned());
    } else if exit_room_regex.is_match(&buf) {
      sessions.push(Session::new(
        session_from.clone().ok_or(anyhow::anyhow!("Missing session start time"))?,
        now_header.ok_or(anyhow::anyhow!("Missing session end time (in log header)"))?.time,
        session_world_uuid.clone().ok_or(anyhow::anyhow!("Missing world UUID"))?,
        session_world_name.clone().ok_or(anyhow::anyhow!("Missing world name"))?,
      ));

      session_from = None;
      session_world_name = None;
      session_world_uuid = None;
      read_until = reader.seek(std::io::SeekFrom::Current(0)).unwrap() as i64;
    }
    buf.clear();
  }

  Ok((read_until, sessions))
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct LogHeader {
  time: chrono::DateTime<chrono::Utc>,
  level: LogLevel,
}

impl LogHeader {
  pub fn new(time: chrono::DateTime<Utc>, level: LogLevel) -> Self {
    LogHeader {
      time,
      level,
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum LogLevel {
  Debug,
  Warning,
  Error,
  Unknown,
}

impl LogLevel {
  fn into_str(self) -> &'static str {
    match self {
      LogLevel::Debug => "Debug",
      LogLevel::Warning => "Warning",
      LogLevel::Error => "Error",
      LogLevel::Unknown => "Unknown",
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Session {
  started_at: chrono::DateTime<Utc>,
  ended_at: chrono::DateTime<Utc>,
  world_uuid: String,
  world_name: String,
}

impl Session {
  pub fn new(started_at: chrono::DateTime<Utc>, ended_at: chrono::DateTime<Utc>, world_uuid: String, world_name: String) -> Self {
    Self {
      started_at,
      ended_at,
      world_name,
      world_uuid,
    }
  }
}

#[cfg(test)]
mod tests {
  use chrono::{Datelike, Timelike};

use super::*;
  use std::io::{Cursor, BufReader};

  struct LogBuilder {
    header: LogHeader,
    body: String,
  }

  impl LogBuilder {
    fn new(header: LogHeader, body: String) -> Self {
      Self {
        header,
        body,
      }
    }
  }

  impl Into<String> for LogBuilder {
    fn into(self) -> String {
      //2026.05.03 22:46:35 Debug      -  [Behaviour] Spent 0.004882813s attaching initial component handlers.
      format!("{:04}.{:02}.{:02} {:02}:{:02}:{:02} {:<11}-  {}\n", self.header.time.year(), self.header.time.month(), self.header.time.day(), self.header.time.hour(), self.header.time.minute(), self.header.time.second(), self.header.level.into_str(), self.body)
    }
  }

  fn get_bufreader_from_string(s: &str) -> BufReader<Cursor<String>> {
    BufReader::new(
      Cursor::new(
        s.to_owned()
      )
    )
  }

  #[test]
  fn check_empty() {
    assert_eq!(
      (0i64, vec![]),
      process_file(get_bufreader_from_string("")).unwrap(),
    );
  }

  #[test]
  fn check_single_session() {
    let log: Vec<(LogHeader, &str)> = vec![
      ( LogHeader::new(Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap(), LogLevel::Debug), "[Behaviour] Entering Room: test" ),
      ( LogHeader::new(Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 2).unwrap(), LogLevel::Debug), "[Behaviour] Joining wrld_1234567890:00000" ),
      ( LogHeader::new(Utc.with_ymd_and_hms(2000, 1, 1, 0, 5, 0).unwrap(), LogLevel::Debug), "[Behaviour] OnLeftRoom" ),
    ];
    let log: Vec<String> = log.iter().map(|p| { LogBuilder::new(p.0, p.1.to_owned()).into() }).collect();

    assert_eq!(
      (log.iter().map(|s| s.len() as i64).sum(), vec![Session::new(
        Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 2).unwrap(),
        Utc.with_ymd_and_hms(2000, 1, 1, 0, 5, 0).unwrap(),
        "wrld_1234567890".to_owned(),
        "test".to_owned(),
      )]),
      process_file(get_bufreader_from_string(&log.join(""))).unwrap(),
    );
  }

  #[test]
  fn check_multiple_sessions() {
      let log: Vec<(LogHeader, &str)> = vec![
          ( LogHeader::new(Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap(), LogLevel::Debug), "[Behaviour] Entering Room: test" ),
          ( LogHeader::new(Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 2).unwrap(), LogLevel::Debug), "[Behaviour] Joining wrld_1234567890:00000" ),
          ( LogHeader::new(Utc.with_ymd_and_hms(2000, 1, 1, 0, 5, 0).unwrap(), LogLevel::Debug), "[Behaviour] OnLeftRoom" ),
          ( LogHeader::new(Utc.with_ymd_and_hms(2000, 1, 2, 0, 0, 0).unwrap(), LogLevel::Debug), "[Behaviour] Entering Room: test2" ),
          ( LogHeader::new(Utc.with_ymd_and_hms(2000, 1, 2, 0, 0, 2).unwrap(), LogLevel::Debug), "[Behaviour] Joining wrld_2234567890:00000" ),
          ( LogHeader::new(Utc.with_ymd_and_hms(2000, 1, 2, 0, 5, 0).unwrap(), LogLevel::Debug), "[Behaviour] OnLeftRoom" ),
      ];
      let log: Vec<String> = log.iter().map(|p| { LogBuilder::new(p.0, p.1.to_owned()).into() }).collect();

      assert_eq!(
        (log.iter().map(|s| s.len() as i64).sum(), vec![Session::new(
          Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 2).unwrap(),
          Utc.with_ymd_and_hms(2000, 1, 1, 0, 5, 0).unwrap(),
          "wrld_1234567890".to_owned(),
          "test".to_owned(),
        ), Session::new(
          Utc.with_ymd_and_hms(2000, 1, 2, 0, 0, 2).unwrap(),
          Utc.with_ymd_and_hms(2000, 1, 2, 0, 5, 0).unwrap(),
          "wrld_2234567890".to_owned(),
          "test2".to_owned(),
        )]),
        process_file(get_bufreader_from_string(&log.join(""))).unwrap(),
      );
    }
}
