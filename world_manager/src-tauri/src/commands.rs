use crate::db::{tags, worlds};

#[tauri::command]
pub async fn get_worlds(filter: worlds::WorldQueryFilters, sort_by: worlds::SortBy) -> Vec<worlds::World> {
  worlds::get_worlds(filter, sort_by).await
}

#[tauri::command]
pub async fn get_tags() -> Vec<tags::Tag> {
  tags::get_tags().await
}

#[tauri::command]
pub async fn create_tag(name: String) -> tags::Tag {
  tags::create_tag(name).await
}
