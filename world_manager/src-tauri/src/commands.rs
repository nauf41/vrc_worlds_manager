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
