use crate::commands::ChannelInfo;
use crate::db::discord::{get_link_by_tag_id, ChannelDBStructure};
use crate::db::worlds::{World, WorldQuery};
use regex::Regex;
use serenity::all::Message;
use serenity::{
    all::{ChannelId, ChannelType, GuildChannel, GuildId, GuildInfo, Http, MessageId},
    prelude::*,
};
use std::sync::LazyLock;
use std::{
    collections::BTreeMap,
    sync::{Arc, OnceLock},
};

use super::SerenityResult;

static HTTP: OnceLock<Arc<Http>> = OnceLock::new();

pub fn set_http(http: Arc<Http>) -> SerenityResult<()> {
    HTTP.get_or_init(|| http);
    Ok(())
}

fn get_http() -> SerenityResult<&'static Arc<Http>> {
    HTTP.get()
        .ok_or(serenity::Error::Other("http client is not initialized"))
}

pub async fn get_guilds() -> SerenityResult<Vec<GuildInfo>> {
    let http = get_http()?;
    let guilds = http.get_guilds(None, None).await?;
    log::info!("{:#?}", guilds);
    Ok(guilds)
}

pub async fn get_channels(
    guild_id: GuildId,
) -> SerenityResult<BTreeMap<Option<ChannelId>, Vec<GuildChannel>>> {
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

pub async fn get_worlds_from_channel(
    channel: ChannelInfo,
    processed_until: u64,
) -> SerenityResult<(Option<u64>, Vec<World>)> {
    /// (read_until, world[])
    /// 過去に遡って取得する
    let http = get_http()?;
    let mut worlds: Vec<World> = vec![];

    let mut message_id_earliest: Option<MessageId> = None;
    let mut message_id_latest: Option<MessageId> = None;
    'outer: loop {
        let now_messages = http
            .get_messages(
                ChannelId::new(channel.id.parse().unwrap()),
                if let Some(id) = message_id_earliest {
                    Some(serenity::all::MessagePagination::Before(id))
                } else {
                    None
                },
                Some(100u8),
            )
            .await;

        if let Err(e) = now_messages {
            log::error!("An error occurred while processing messages: {:?}", e);
            break;
        } else {
            let now_messages = now_messages.unwrap();
            if now_messages.is_empty() {
                break;
            }

            for msg in now_messages {
                if process_message(processed_until, &msg, &mut worlds).await {
                    break 'outer;
                }

                if let Some(bef) = message_id_earliest {
                    message_id_earliest = Some(bef.min(msg.id));
                } else {
                    message_id_earliest = Some(msg.id);
                }

                if let Some(bef) = message_id_latest {
                    message_id_latest = Some(bef.max(msg.id));
                } else {
                    message_id_latest = Some(msg.id);
                }
            }
        }
    }

    let _ = crate::db::discord::upsert_channel(channel.id.parse().unwrap(), channel.name).await;

    Ok((message_id_latest.and_then(|v| Some(v.get())), worlds))
}

pub async fn process_message(
  processed_until: u64,
  msg: &Message,
  worlds: &mut Vec<World>,
) -> bool {
    /// @returns boolean whether it should break loop
    static REG_WORLD_URL: std::sync::LazyLock<Regex> = LazyLock::new(|| {
        regex::Regex::new(r#"https:\/\/vrchat\.com\/.*(wrld_[0-9a-f\-]+)"#).unwrap()
    });
    static REG_WORLD_TITLE: std::sync::LazyLock<Regex> =
        LazyLock::new(|| regex::Regex::new(r#"^(.+) by (.+?)$"#).unwrap());

    if processed_until >= msg.id.get() {
        log::info!(
            "Reached already processed messages. Stopping. (message id: {}, processed until: {})",
            msg.id,
            processed_until
        );
        return true;
    }

    if let Some(v) = REG_WORLD_URL.captures(&msg.content) {
        if let Some(vv) = v.get(1) {
            let world_id = vv.as_str();

            log::info!(r#"detected world: {} from message "{:#?}""#, world_id, msg);

            let (title, publisher_name, description): (
                Option<String>,
                Option<String>,
                Option<String>,
            ) = {
                if let Some(embed) = msg.embeds.get(0) {
                    let (title, name): (Option<String>, Option<String>) =
                        if let Some(embed_title) = &embed.title {
                            if let Some(v) = REG_WORLD_TITLE.captures(embed_title) {
                                (
                                    v.get(1).and_then(|v| Some(v.as_str().to_owned())),
                                    v.get(2).and_then(|v| Some(v.as_str().to_owned())),
                                )
                            } else {
                                (None, None)
                            }
                        } else {
                            (None, None)
                        };

                    let description = embed.description.clone();

                    (title, name, description)
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
                latest_at: Some(msg.timestamp.timestamp_millis()),
            })
            .await;

            worlds.push(world.unwrap().1);

            // TODO fetch image url

            false
        } else {
            false
        }
    } else {
        false
    }
}

pub async fn post_world(tag_id: i64, world: World) -> SerenityResult<()> {
    let http = get_http()?;
    let connection = get_link_by_tag_id(tag_id).await.unwrap();
    if let Some(con) = connection {
        http.send_message(
            ChannelId::new(con.discord_channel_id.try_into().unwrap()),
            vec![],
            &serenity::all::CreateMessage::new()
                .content(format!("https://vrchat.com/home/world/{}/info", world.uuid)),
        )
        .await?;
    }
    Ok(())
}
