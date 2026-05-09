use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  pub discord_bot_token: Option<String>,
}

pub fn get_conf() -> anyhow::Result<&'static Config> {
  static CONF: OnceLock<Config> = OnceLock::new();

  if let Some(v) = CONF.get() {
    Ok(&v)
  } else {
    let file = std::fs::File::open("config.json")?;
    let conf: Config = serde_json::from_reader(file)?;
    Ok(CONF.get_or_init(|| conf))
  }
}

pub fn update_conf(new: Config) -> anyhow::Result<()> {
  /// the application must be restarted for loading updated config.

  let file = std::fs::OpenOptions::new()
    .read(false)
    .write(true)
    .create(true)
    .open("config.json")?;

  serde_json::to_writer_pretty(&file, &new)?;
  Ok(())
}
