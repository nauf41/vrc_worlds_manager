use crate::db::{tag_groups, tags, worlds};

// === World ===
#[tauri::command]
pub async fn add_world(uuid: String, publisher: Option<i64>) -> bool {
  worlds::add_new_world(&uuid, publisher).await.is_ok()
}

#[tauri::command]
pub async fn get_worlds(filter: worlds::WorldQueryFilters, sort_by: worlds::SortBy) -> Vec<worlds::World> {
  worlds::get_worlds(&filter, &sort_by).await.unwrap()
}

#[tauri::command]
pub async fn add_world_cache(
  uuid: String,
  description: Option<String>,
  title: Option<String>,
  visits: Option<i64>,
  favorites: Option<i64>,
  capacity: Option<i64>,
  published_at: Option<i64>,
  does_support_windows: Option<bool>,
  does_support_android: Option<bool>,
  does_support_ios: Option<bool>,
) -> bool {
  worlds::add_world_cache(&crate::ipc::native_messaging::World {
    uuid,
  }, &crate::ipc::native_messaging::WorldCache {
    description,
    title,
    visits,
    favorites,
    capacity,
    published_at,
    does_support_windows,
    does_support_android,
    does_support_ios,
  }).await.is_ok()
}

// === Publisher ===
#[tauri::command]
pub async fn upsert_publisher(uuid: String, name: Option<String>) -> bool {
  worlds::upsert_publisher(&uuid, &name).await.is_ok()
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
pub async fn get_tags_without_tagggroup() -> Option<Vec<tags::Tag>> {
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
