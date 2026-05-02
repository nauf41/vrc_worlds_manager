use std::fs::DirEntry;
use std::path::PathBuf;
use std::env;
use std::io::{BufRead, BufReader, Seek};
use regex::Regex;

use chrono::{TimeZone, Utc};

use crate::ipc::native_messaging::{World, WorldCache};

pub async fn main() {
  let vrc_dir = {
    let mut path = PathBuf::from(env::home_dir().unwrap());
    path.push("AppData");
    path.push("LocalLow");
    path.push("VRChat");
    path.push("VRChat");
    path
  };

  loop {
    let mut log_files: Vec<DirEntry> = std::fs::read_dir(&vrc_dir).unwrap()
      .map(|entry| { entry.unwrap() }).collect();

    log_files.sort_by(|a,b| a.file_name().cmp(&b.file_name()));

    let mut process_target: Vec<(u64, DirEntry)> = vec![]; // begin_at, file
    for entry in log_files.into_iter() {
      if let Some(v) = crate::db::log_files::get_log(&entry.file_name().into_string().unwrap()).await.unwrap() {
        if (v.read_at as u64) + 1 < std::fs::metadata(entry.path()).unwrap().len() {
          process_target.push(((v.read_at as u64) + 1, entry));
        }
      } else {
        process_target.push((0, entry));
      }
    }

    let mut process_target = process_target.into_iter().peekable();
    while let Some((begin, entry)) = process_target.next() {
      let mut reader = BufReader::new(std::fs::File::open(entry.path()).unwrap());
      reader.seek(std::io::SeekFrom::Start(begin)).unwrap();

      process_file(entry.path().to_str().unwrap().to_owned(), reader).await;
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
  }
}

async fn process_file(filepath: String, mut reader: BufReader<std::fs::File>) {

  let header_regex = Regex::new(r"^[0-9]{4}\.[0-9]{2}\.[0-9]{2} [0-9]{4}\:[0-9]{4}\:[0-9]{4}\ .+? -  ").unwrap();
  let enter_room_uuid_lin_1_regex = Regex::new(r"^\[Behaviour\] Joining *$").unwrap();
  let enter_room_uuid_lin_2_regex = Regex::new(r"(wrld_.+?):").unwrap();
  let enter_room_name_regex = Regex::new(r"^\[Behaviour\] Entering Room: (.+)$").unwrap();
  let exit_room_regex = Regex::new(r"OnLeftRoom").unwrap();

  let mut now_header: Option<LogHeader> = None;
  let mut session_from: Option<chrono::DateTime<Utc>> = None;
  let mut session_world_name: Option<String> = None;
  let mut session_world_uuid: Option<String> = None;

  let mut buf = String::new();
  loop {
    let lin = reader.read_line(&mut buf).unwrap();
    if lin == 0 {
      break;
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

      let years: i32 = left[0..4].parse().unwrap(); // 0, 1, 2, 3
      // 4
      let months: u32 = left[5..7].parse().unwrap(); // 5, 6
      // 7
      let days: u32 = left[8..10].parse().unwrap(); // 8, 9
      // 10
      let hours: u32 = left[11..13].parse().unwrap(); // 11, 12
      // 13
      let minutes: u32 = left[14..16].parse().unwrap(); // 14, 15
      // 16
      let seconds: u32 = left[17..19].parse().unwrap(); // 17, 18
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
    } else if enter_room_uuid_lin_1_regex.is_match(&buf) {
      buf.clear();
      assert_ne!(0, reader.read_line(&mut buf).unwrap());
      if let Some(v) = enter_room_uuid_lin_2_regex.captures(&buf).unwrap().get(1) {
        session_world_uuid = Some(v.as_str().to_owned());
        session_from = Some(now_header.unwrap().time);
      }
    } else if enter_room_name_regex.is_match(&buf) {
      session_world_name = Some(enter_room_name_regex.captures(&buf).unwrap()[1].to_owned());
    } else if exit_room_regex.is_match(&buf) {
      if crate::db::worlds::get_world_id_by_uuid(&session_world_uuid.clone().unwrap()).await.unwrap().is_none() {
        crate::db::worlds::add_new_world(&session_world_uuid.clone().unwrap(), None).await.unwrap();
      }
      crate::db::worlds::new_session(
        crate::db::worlds::get_world_id_by_uuid(session_world_uuid.as_ref().unwrap()).await.unwrap().unwrap(),
        session_from.unwrap().timestamp(),
        now_header.unwrap().time.timestamp()
      ).await.unwrap();
      now_header = None;
      if crate::db::log_files::get_log(&filepath).await.unwrap().is_none() {
        crate::db::log_files::new_log(&filepath).await.unwrap();
      }

      crate::db::worlds::add_world_cache(&World {
        uuid: session_world_uuid.clone().unwrap(),
      }, &WorldCache {
        description: None,
        title: session_world_name.clone(),
        visits: None,
        favorites: None,
        capacity: None,
        published_at: None,
        does_support_windows: None,
        does_support_android: None,
        does_support_ios: None,
      }).await.unwrap();

      session_from = None;
      session_world_name = None;
      session_world_uuid = None;
      crate::db::log_files::update_log_read_at(&filepath, reader.seek(std::io::SeekFrom::Current(0)).unwrap() as i64).await.unwrap();
    }
    buf.clear();
  }
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
