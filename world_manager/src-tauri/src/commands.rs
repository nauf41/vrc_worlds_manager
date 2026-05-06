use crate::db::{tag_groups, tags, worlds::{self, WorldQuery}};

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
  tags::attach(tagid, worldid).await.is_ok()
}

#[tauri::command]
pub async fn detach_world(tagid: i64, worldid: i64) -> bool {
  tags::detach(tagid, worldid).await.is_ok()
}

#[tauri::command]
pub async fn change_tag(tagid: i64, data: tags::Tag) -> bool {
  tags::update(tagid, data).await.is_ok()
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
