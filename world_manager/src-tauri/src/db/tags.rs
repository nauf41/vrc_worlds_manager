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

pub async fn delete_tag(tag_id: i64) -> Result<bool, sqlx::Error> {
  let res = sqlx::query!(
    "
    DELETE FROM tags
    WHERE id = $1
    ;
    ",
    tag_id
  ).execute(get_pool().await).await?;

  Ok(res.rows_affected() > 0)
}

pub async fn change(tag_id: i64, after: Tag) -> Result<(), sqlx::Error> {
  sqlx::query!(
    "
    UPDATE tags
    SET name = $1
    WHERE id = $2
    ;
    ",
    after.name,
    tag_id
  ).execute(get_pool().await).await?;

  Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Tag {
  pub id: i64,
  pub name: String,
}
