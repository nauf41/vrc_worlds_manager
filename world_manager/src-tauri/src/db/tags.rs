use serde::{Deserialize, Serialize};

use crate::db::get_pool;

pub async fn get_tags() -> Result<Vec<Tag>, sqlx::Error> {
  sqlx::query_as!(
    Tag,
    "
    SELECT
      id,
      name
    FROM tags
    ORDER BY name ASC
    ;
    ",
  ).fetch_all(get_pool().await).await
}

pub async fn create_tag(name: String) -> Result<Tag, sqlx::Error> {
  sqlx::query_as!(
    Tag,
    "
    INSERT INTO tags (name)
    VALUES ($1)
    RETURNING id, name
    ;
    ",
    name
  ).fetch_one(get_pool().await).await
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Tag {
  pub id: i64,
  pub name: String,
}
