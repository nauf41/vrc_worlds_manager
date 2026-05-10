use serenity::{
    all::{ChannelId, ShardManager},
    async_trait,
    prelude::*,
};
use std::sync::{Arc, LazyLock, OnceLock};

use crate::db::discord::TagDiscordLink;
use crate::db::worlds::World;
use crate::discord_bot::http::process_message;

pub mod http;

type SerenityResult<T> = std::result::Result<T, serenity::Error>;

// LazyLock<Mutex<SHARD_MANAGER_ARGUMENT_TYPE>>
static SHARD_MANAGER: OnceLock<Arc<ShardManager>> = OnceLock::new();

struct Handler {
    watch_lists: Mutex<Vec<TagDiscordLink>>,
}

impl Handler {
    pub fn new(watch_lists: Vec<TagDiscordLink>) -> Self {
        Self {
            watch_lists: Mutex::new(watch_lists),
        }
    }

    pub async fn set_wl(&self, watch_lists: Vec<TagDiscordLink>) {
        let mut lock = self.watch_lists.lock().await;
        *lock = watch_lists;
    }
}

#[async_trait]
impl serenity::client::EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: serenity::model::channel::Message) {
        let mut wl: Vec<_> = self.watch_lists.lock().await.clone();
        log::info!(
            "Received message in channel {}: {}",
            msg.channel_id,
            msg.content
        );
        // check match
        for wl in &mut wl {
            let tmp: i64 = msg.channel_id.get().try_into().unwrap();
            if wl.discord_channel_id == tmp {
                log::info!(
                    "Matched channel {} with tag_id {}",
                    wl.discord_channel_id,
                    wl.tag_id
                );
                let mut wrld: Vec<World> = vec![];
                process_message(
                    wl.latest_read_id.unwrap_or(0).try_into().unwrap(),
                    &msg,
                    &mut wrld,
                )
                .await;
                if wrld.len() > 0 {
                    crate::db::tags::attach(wl.tag_id, wrld[0].id, true)
                      .await
                      .unwrap();
                    crate::db::tags::attach(0, wrld[0].id, true)
                      .await
                      .unwrap();
                }
                crate::db::discord::upsert_channel(
                    wl.discord_channel_id,
                    msg.channel_id.to_string(),
                )
                .await
                .unwrap();
                crate::db::discord::update_link(
                    wl.tag_id,
                    wl.discord_channel_id,
                    msg.id.get() as i64,
                )
                .await
                .unwrap();
                wl.latest_read_id = Some(msg.id.get() as i64);
            }
        }

        self.set_wl(wl).await;
    }
    async fn ready(&self, ctx: Context, ready: serenity::model::gateway::Ready) {
        log::info!("Connected as {}", ready.user.name);
        http::set_http(ctx.http).unwrap();
    }
}

pub async fn shutdown() {
    SHARD_MANAGER.get().unwrap().shutdown_all().await;
}

pub async fn main(token: &str) -> SerenityResult<()> {
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::Client::builder(token, intents)
        .event_handler(Handler::new(
            crate::db::discord::get_handlers().await.unwrap(),
        ))
        .await?;

    SHARD_MANAGER.set(client.shard_manager.clone()).unwrap();

    client.start().await?;

    Ok(())
}
