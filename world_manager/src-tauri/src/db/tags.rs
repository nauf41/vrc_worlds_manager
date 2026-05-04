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

pub async fn create_tag_group(name: String) -> Result<sql_return_structs::TagGroup, sqlx::Error> {
  sqlx::query_as!(
    sql_return_structs::TagGroup,
    "
    INSERT INTO tag_groups (name) VALUES ($1) RETURNING id, name;", name
  ).fetch_one(get_pool().await).await
}

pub async fn get_tag_groups() -> Result<Vec<sql_return_structs::TagGroup>, sqlx::Error> {
  sqlx::query_as!(
    sql_return_structs::TagGroup,
    "
    SELECT id, name FROM tag_groups ORDER BY name ASC;"
  ).fetch_all(get_pool().await).await
}

pub async fn edit_tag_group_name(tag_group_id: i64, name: String) -> Result<(), sqlx::Error> {
  sqlx::query!(
    "
    UPDATE tag_groups
    SET name = $1
    WHERE id = $2
    ;
    ",
    name, tag_group_id
  ).execute(get_pool().await).await?;

  Ok(())
}

pub async fn delete_tag_group(tag_group_id: i64) -> Result<bool, sqlx::Error> {
  sqlx::query!(
    "
    DELETE FROM tag_groups
    WHERE id = $1
    ;
    ",
    tag_group_id
  ).execute(get_pool().await).await?;
  Ok(true)
}

pub async fn upsert_tag_group_attachment(tag_id: i64, tag_group_id: Option<i64>) -> Result<(), sqlx::Error> {
  if tag_group_id.is_none() {
    sqlx::query!(
      "
      DELETE FROM tag_groups_tags
      WHERE tags_id = $1;
      ",
      tag_id
    ).execute(get_pool().await).await?;
  } else {
    sqlx::query!(
      "
      INSERT INTO tag_groups_tags (tag_groups_id, tags_id) VALUES ($1, $2)
      ON CONFLICT (tags_id) DO UPDATE SET tag_groups_id = $1;",
      tag_group_id,
      tag_id
    ).execute(get_pool().await).await?;
  }

  Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Tag {
  pub id: i64,
  pub name: String,
}

pub mod sql_return_structs {
  use super::*;

  #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
  pub struct TagGroup {
    pub id: i64,
    pub name: String,
  }
}