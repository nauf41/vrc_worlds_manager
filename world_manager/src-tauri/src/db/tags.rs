use serde::{Deserialize, Serialize};
use crate::db::get_pool;

pub async fn create(name: String) -> Result<Tag, sqlx::Error> {
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

pub async fn get() -> Result<Vec<Tag>, sqlx::Error> {
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

pub async fn get_with_children() -> Result<Vec<(Tag, Vec<i64>)>, sqlx::Error> {
  let tags = sqlx::query_as!(
    TagWithChild,
    "
    SELECT
      tg.id AS tag_id,
      tg.name AS tag_name,
      tw.worlds_id AS world_id
    FROM tags tg
    LEFT JOIN tags_worlds tw ON tg.id = tw.tags_id
    ORDER BY tg.id ASC
    ;
    "
  ).fetch_all(get_pool().await).await?;

  let mut res: Vec<(Tag, Vec<i64>)> = vec![];

  for tag in tags {
    if let Some(v) = res.last_mut() {
      if v.0.id == tag.tag_id {
        if let Some(vv) = tag.world_id {
          v.1.push(vv);
        }
        continue;
      }
    }

    res.push((
      Tag {
        id: tag.tag_id,
        name: tag.tag_name
      },
      if let Some(v) = tag.world_id {
        vec![v]
      } else {
        vec![]
      }
    ));
  }

  Ok(res)
}

pub async fn get_without_taggroup() -> Result<Vec<Tag>, sqlx::Error> {
  sqlx::query_as!(
    Tag,
    "
    SELECT
      id,
      name
    FROM tags
    WHERE id != 0 AND (
      SELECT COUNT(*)
      FROM tag_groups_tags tgt
      WHERE tgt.tags_id = tags.id
    ) = 0
    ORDER BY name ASC
    ;
    ",
  ).fetch_all(get_pool().await).await
}

pub async fn get_favorited_worlds() -> Result<Vec<i64>, sqlx::Error> {
  let res = sqlx::query_as!(
    WorldsIdOnly,
    "
    SELECT
      worlds_id
    FROM tags_worlds
    WHERE tags_id = 0
    ;
    "
  ).fetch_all(get_pool().await).await?;

  Ok(res.into_iter().map(|v| v.worlds_id).collect())
}

struct WorldsIdOnly {
  pub worlds_id: i64
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Origin {
  DiscordChannel,
  Other,
}
pub async fn attach(tag_id: i64, world_id: i64, is_from_discord: bool) -> Result<(), sqlx::Error> {
  sqlx::query!(
    "
    INSERT INTO tags_worlds (tags_id, worlds_id, is_sent_to_discord)
    VALUES ($1, $2, 0)
    ON CONFLICT DO NOTHING
    ;
    ",
    tag_id,
    world_id
  ).execute(get_pool().await).await?;

  if !is_from_discord {
    tauri::async_runtime::spawn(async move {
      if let Ok(Some(link)) = crate::db::discord::get_link_by_tag_id(tag_id).await {
        if link.do_auto_post != 0 {
          // send to discord
          let world = crate::db::worlds::get_world_by_id(world_id).await.unwrap().unwrap();
          crate::discord_bot::http::post_world(tag_id, world.into()).await.unwrap();
        }
      }
    });
  }

  Ok(())
}

pub async fn detach(tag_id: i64, world_id: i64) -> Result<(), sqlx::Error> {
  sqlx::query!(
    "
    DELETE FROM tags_worlds
    WHERE tags_id = $1 AND worlds_id = $2
    ;
    ",
    tag_id,
    world_id
  ).execute(get_pool().await).await?;

  Ok(())
}

pub async fn update(tag_id: i64, after: Tag) -> Result<(), sqlx::Error> {
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

pub async fn delete(tag_id: i64) -> Result<bool, sqlx::Error> {
  sqlx::query!(
    "
    DELETE FROM tag_groups_tags
    WHERE tags_id = $1
    ;
    ",
    tag_id
  ).execute(get_pool().await).await?;

  sqlx::query!(
    "
    DELETE FROM tags_worlds
    WHERE tags_id = $1
    ;
    ",
    tag_id
  ).execute(get_pool().await).await?;

  sqlx::query!(
    "
    DELETE FROM tags_discord_channels
    WHERE tag_id = $1
    ;
    ",
    tag_id
  ).execute(get_pool().await).await?;

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Tag {
  pub id: i64,
  pub name: String,
}

struct TagWithChild {
  pub tag_id: i64,
  pub tag_name: String,
  pub world_id: Option<i64>,
}

#[cfg(test)]
mod test {
  use super::*;

  #[tokio::test]
  async fn test_get_with_children() {
  }
}