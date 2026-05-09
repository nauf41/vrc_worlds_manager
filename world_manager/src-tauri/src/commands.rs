use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, ChannelType};

use crate::db::{tag_groups, tags, worlds::{self, WorldQuery}};
use crate::config;

// === World ===
#[tauri::command]
pub async fn upsert_world(query: WorldQuery) -> bool {
  worlds::upsert_world(query).await.is_ok()
}

#[tauri::command]
pub async fn get_worlds(filter: worlds::WorldQueryFilters, sort_by: worlds::SortBy) -> Option<Vec<worlds::World>> {
  worlds::get_worlds(&filter, &sort_by).await.ok()
}

// === Tags ===
#[tauri::command]
pub async fn create_tag(name: String) -> Option<tags::Tag> {
  tags::create(name).await.ok()
}

#[tauri::command]
pub async fn get_tags() -> Option<Vec<tags::Tag>> {
  tags::get().await.ok()
}

#[tauri::command]
pub async fn get_tags_with_children() -> Option<Vec<(tags::Tag, Vec<i64>)>> {
  tags::get_with_children().await.ok()
}

#[tauri::command]
pub async fn get_tags_without_taggroup() -> Option<Vec<tags::Tag>> {
  tags::get_without_taggroup().await.ok()
}

#[tauri::command]
pub async fn get_favorited_worlds() -> Option<Vec<i64>> {
  tags::get_favorited_worlds().await.ok()
}

#[tauri::command]
pub async fn attach_world(tagid: i64, worldid: i64) -> bool {
  tags::attach(tagid, worldid).await.unwrap();
  true
}

#[tauri::command]
pub async fn detach_world(tagid: i64, worldid: i64) -> bool {
  tags::detach(tagid, worldid).await.unwrap();
  true
}

#[tauri::command]
pub async fn change_tag(tagid: i64, data: tags::Tag) -> bool {
  tags::update(tagid, data).await.unwrap();
  true
}

#[tauri::command]
pub async fn delete_tag(tagid: i64) -> bool {
  log::debug!("Deleting tag with id: {}", tagid);
  tags::delete(tagid).await.unwrap_or(false)
}

// === Tag Groups ===
#[tauri::command]
pub async fn create_tag_group(name: String) -> Option<tag_groups::TagGroup> {
  if let Ok(r) = tag_groups::create(name).await {
    Some(r)
  } else {
    None
  }
}

#[tauri::command]
pub async fn get_tag_groups() -> Option<Vec<tag_groups::TagGroup>> {
  tag_groups::get().await.ok()
}

#[tauri::command]
pub async fn get_tag_groups_with_tags() -> Option<Vec<(tag_groups::TagGroup, Vec<i64>)>> {
  tag_groups::get_with_tags().await.ok()
}

#[tauri::command]
pub async fn edit_tag_group_name(taggroupid: i64, name: String) -> bool {
  tag_groups::update_name(taggroupid, name).await.is_ok()
}

#[tauri::command]
pub async fn delete_tag_group(taggroupid: i64) -> bool {
  tag_groups::delete(taggroupid).await.is_ok()
}

#[tauri::command]
pub async fn upsert_tag_group_attachment(tagid: i64, taggroupid: Option<i64>) -> bool {
  tag_groups::upsert_attachment(tagid, taggroupid).await.is_ok()
}

// === config ===
#[tauri::command]
pub async fn get_config() -> Option<config::Config> {
  config::get_conf().ok().cloned()
}

#[tauri::command]
pub async fn update_config(new: config::Config) -> bool {
  config::update_conf(new).is_ok()
}

// === discord bot ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildInfo {
  pub id: String,
  pub name: String,
}
#[tauri::command]
pub async fn get_discord_guilds() -> Option<Vec<GuildInfo>> {
  if let Ok(v) = crate::discord_bot::http::get_guilds().await {
    Some(v.into_iter().map(|g| GuildInfo { id: g.id.to_string(), name: g.name }).collect())
  } else {
    None
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
  pub id: String,
  pub name: String,
  pub is_category: bool,
  pub parent_id: Option<String>,
}
#[tauri::command]
pub async fn get_discord_channels(guild_id: String) -> Option<Vec<(Option<String>, Vec<ChannelInfo>)>> {
  if let Ok(v) = crate::discord_bot::http::get_channels(serenity::all::GuildId::new(guild_id.parse().unwrap())).await {
    Some(v.iter().map(|p| {
      (
        p.0.and_then(|v| Some(v.get().to_string())),
        p.1.into_iter().map(|a| ChannelInfo {
          id: a.id.to_string(),
          name: a.name.clone(),
          is_category: a.kind == ChannelType::Category,
          parent_id: a.parent_id.and_then(|v| Some(v.to_string()))
        }).collect()
      )
    }).collect())
  } else {
    None
  }
}

// u63なため、signedとして扱ってもOK
#[tauri::command]
pub async fn add_discord_link(tag_id: String, channel: ChannelInfo, do_auto_fetch: bool, do_auto_post: bool) -> bool {
  if let Err(_) = crate::db::discord::upsert_channel(channel.id.parse().unwrap(), channel.name, None).await {
    return false;
  }
  if let Err(_) = crate::db::discord::create_link(tag_id.parse().unwrap(), channel.id.parse().unwrap(), do_auto_fetch, do_auto_post).await {
    return false;
  }

  return true;
}

#[tauri::command]
pub async fn parse_channel(channel_id: String) -> Option<Vec<crate::db::worlds::WorldDBStructure>> {
  crate::discord_bot::http::get_worlds_from_channel(ChannelId::new(channel_id.parse().unwrap())).await.ok()
}
