use std::{collections::BTreeMap, sync::{Arc, OnceLock}};
use serenity::{all::{ChannelId, ChannelType, GuildChannel, GuildId, GuildInfo, Http, MessageId}, prelude::*};
use crate::db::worlds::{World, WorldDBStructure, WorldQuery};

use super::SerenityResult;

static HTTP: OnceLock<Arc<Http>> = OnceLock::new();

pub fn set_http(http: Arc<Http>) -> SerenityResult<()> {
  if let Err(e) = HTTP.set(http) {
    Err(serenity::Error::Other("Failed to set http client"))
  } else {
    Ok(())
  }
}

fn get_http() -> SerenityResult<&'static Arc<Http>> {
  HTTP.get().ok_or(serenity::Error::Other("http client is not intialized"))
}

pub async fn get_guilds() -> SerenityResult<Vec<GuildInfo>> {
  let http = get_http()?;
  let guilds = http.get_guilds(None, None).await?;
  log::info!("{:#?}", guilds);
  Ok(guilds)
}

pub async fn get_channels(guild_id: GuildId) -> SerenityResult<BTreeMap<Option<ChannelId>, Vec<GuildChannel>>> {
  let http = get_http()?;
  let channels = http.get_channels(guild_id).await?;
  log::warn!("Channels: {:#?}", channels);

  let mut res: BTreeMap<Option<ChannelId>, Vec<GuildChannel>> = BTreeMap::new();
  for channel in channels {
    let key = if let Some(v) = channel.parent_id {
      Some(v)
    } else {
      None
    };

    if !res.contains_key(&key) {
      res.insert(key, vec![]);
    }
    if let Some(v) = res.get_mut(&key) {
      v.push(channel);
    }
  }

  log::warn!("resulting with {:#?}", res);

  Ok(res)
}

pub async fn get_worlds_from_channel(channel: ChannelId) -> SerenityResult<Vec<crate::db::worlds::WorldDBStructure>> {
  /// 過去に遡って取得する
  let reg_world_url = regex::Regex::new(r#"https:\/\/vrchat\.com\/home\/launch\?worldId=(wrld_[0-9a-f\-]+)"#).unwrap();
  let reg_world_title = regex::Regex::new(r#"^(.+) by (.+?)$"#).unwrap();

  let http = get_http()?;
  let mut worlds: Vec<WorldDBStructure> = vec![];

  let mut message_id: Option<MessageId> = None;
  loop {
    let now_messages = http.get_messages(
      channel,
      if let Some(id) = message_id { Some(serenity::all::MessagePagination::Before(id)) } else { None },
      Some(100u8)
    ).await;

    if let Err(e) = now_messages {
      log::error!("An error occurred while processing messages: {:?}", e);
      break;
    } else {
      let now_messages = now_messages.unwrap();
      if now_messages.is_empty() { break; }

      for msg in now_messages {
        if let Some(v) = reg_world_url.captures(&msg.content) {
          if let Some(vv) = v.get(1) {
            let world_id = vv.as_str();

            log::info!(r#"detected world: {} from message "{:#?}""#, world_id, msg);

            let (title, publisher_name, description): (Option<String>, Option<String>, Option<String>) = {
              if let Some(embed) = msg.embeds.get(0) {
                if let Some(embed_title) = &embed.title {
                  if let Some(embed_description) = &embed.description {
                    if let Some(v) = reg_world_title.captures(embed_title) {
                      (
                        v.get(1).and_then(|v| Some(v.as_str().to_owned())),
                        v.get(2).and_then(|v| Some(v.as_str().to_owned())),
                        Some(embed_description.to_owned())
                      )
                    } else {
                      (None, None, None)
                    }
                  } else {
                    (None, None, None)
                  }
                } else {
                  (None, None, None)
                }
              } else {
                (None, None, None)
              }
            };

            let world = crate::db::worlds::upsert_world(WorldQuery {
              uuid: world_id.to_owned(),
              publisher_uuid: None,
              publisher_name,
              registered_at: Some(msg.timestamp.timestamp_millis()),
              description,
              title,
              visits: None,
              favorites: None,
              capacity: None,
              published_at: None,
              does_support_windows: None,
              does_support_android: None,
              does_support_ios: None,
              latest_at: Some(msg.timestamp.timestamp_millis())
            }).await;

            if let Ok(world) = world {
              log::info!("registered world: {:?}", world);
              worlds.push(world);
            } else {
              log::error!("Failed to register a world.");
            }

            // TODO fetch image url
          }
        }

        if let Some(bef) = message_id {
          message_id = Some(bef.min(msg.id));
        } else {
          message_id = Some(msg.id);
        }
      }
    }
  }

  Ok(worlds)
}
