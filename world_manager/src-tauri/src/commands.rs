use crate::db::{tags, worlds};

#[tauri::command]
pub async fn get_worlds(filter: worlds::WorldQueryFilters, sort_by: worlds::SortBy) -> Vec<worlds::World> {
  worlds::get_worlds(&filter, &sort_by).await.unwrap()
}

#[tauri::command]
pub async fn get_tags() -> Vec<tags::Tag> {
  tags::get_tags().await.unwrap()
}

#[tauri::command]
pub async fn create_tag(name: String) -> tags::Tag {
  tags::create_tag(name).await.unwrap()
}

#[tauri::command]
pub async fn delete_tag(tagid: i64) -> bool {
  tags::delete_tag(tagid).await.unwrap()
}

#[tauri::command]
pub async fn change_tag(tagid: i64, data: tags::Tag) -> bool {
  tags::change(tagid, data).await.is_ok()
}

#[tauri::command]
pub async fn create_tag_group(name: String) -> Option<tags::sql_return_structs::TagGroup> {
  if let Ok(r) = tags::create_tag_group(name).await {
    Some(r)
  } else {
    None
  }
}

#[tauri::command]
pub async fn get_tag_groups() -> Option<Vec<tags::sql_return_structs::TagGroup>> {
  tags::get_tag_groups().await.ok()
}

#[tauri::command]
pub async fn edit_tag_group_name(taggroupid: i64, name: String) -> bool {
  tags::edit_tag_group_name(taggroupid, name).await.is_ok()
}

#[tauri::command]
pub async fn delete_tag_group(taggroupid: i64) -> bool {
  tags::delete_tag_group(taggroupid).await.ok().unwrap_or(false)
}

#[tauri::command]
pub async fn upsert_tag_group_attachment(tagid: i64, taggroupid: Option<i64>) -> bool {
  tags::upsert_tag_group_attachment(tagid, taggroupid).await.is_ok()
}
