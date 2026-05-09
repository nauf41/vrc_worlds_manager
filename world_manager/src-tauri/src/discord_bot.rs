use std::sync::{Arc, LazyLock, OnceLock};
use serenity::{all::{ChannelId, ShardManager}, async_trait, prelude::*};

use crate::db::discord::TagDiscordLink;

pub mod http;

type SerenityResult<T> = std::result::Result<T, serenity::Error>;

// LazyLock<Mutex<SHARD_MANAGER_ARGUMENT_TYPE>>
static SHARD_MANAGER: OnceLock<Arc<ShardManager>> = OnceLock::new();

struct Handler {
  watch_lists: Vec<TagDiscordLink>,
}

impl Handler {
  pub fn new(watch_lists: Vec<TagDiscordLink>) -> Self {
    Self {
      watch_lists,
    }
  }
}

#[async_trait]
impl serenity::client::EventHandler for Handler {
  async fn ready(&self, ctx: Context, ready: serenity::model::gateway::Ready) {
    log::info!("Connected as {}", ready.user.name);
    http::set_http(ctx.http).unwrap();
  }
}

pub async fn shutdown() {
  SHARD_MANAGER.get().unwrap().shutdown_all().await;
}

pub async fn main(token: &str) -> SerenityResult<()> {
  let intents = GatewayIntents::GUILD_MESSAGES
  | GatewayIntents::MESSAGE_CONTENT;

  let mut client = serenity::Client::builder(token, intents)
    .event_handler(Handler::new(crate::db::discord::get_handlers().await.unwrap()))
    .await?;

  SHARD_MANAGER.set(client.shard_manager.clone()).unwrap();

  client.start().await?;

  Ok(())
}
