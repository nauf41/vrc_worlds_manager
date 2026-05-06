use serde::{Deserialize, Serialize};

use super::get_pool;

pub async fn upsert_world(w: WorldQuery) -> Result<WorldDBStructure, sqlx::Error> {
  // store
  let f = if let Some(data) = w.image_cache {
    let img_uuid = uuid::Uuid::now_v7().to_string();
    let data = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, data).unwrap();
    let info = infer::get(&data).unwrap();
    let file_name = format!("{}.{}", img_uuid, info.extension());
    let path = std::path::Path::new("./thumbnail-cache").join(&file_name);
    std::fs::write(&path, data).unwrap();
    Some(file_name)
  } else {
    None
  };

  sqlx::query_as!(
    WorldDBStructure,
    "
    INSERT INTO worlds (uuid, publisher_uuid, publisher_name, registered_at, description, title, visits, favorites, capacity, published_at, does_support_windows, does_support_android, does_support_ios, latest_at, image_cache_file)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
    ON CONFLICT(uuid) DO UPDATE SET
      uuid = COALESCE(EXCLUDED.uuid, worlds.uuid),
      publisher_uuid = COALESCE(EXCLUDED.publisher_uuid, worlds.publisher_uuid),
      publisher_name = COALESCE(EXCLUDED.publisher_name, worlds.publisher_name),
      registered_at = COALESCE(EXCLUDED.registered_at, worlds.registered_at),
      description = COALESCE(EXCLUDED.description, worlds.description),
      title = COALESCE(EXCLUDED.title, worlds.title),
      visits = COALESCE(EXCLUDED.visits, worlds.visits),
      favorites = COALESCE(EXCLUDED.favorites, worlds.favorites),
      capacity = COALESCE(EXCLUDED.capacity, worlds.capacity),
      published_at = COALESCE(EXCLUDED.published_at, worlds.published_at),
      does_support_windows = COALESCE(EXCLUDED.does_support_windows, worlds.does_support_windows),
      does_support_android = COALESCE(EXCLUDED.does_support_android, worlds.does_support_android),
      does_support_ios = COALESCE(EXCLUDED.does_support_ios, worlds.does_support_ios),
      latest_at = COALESCE(EXCLUDED.latest_at, worlds.latest_at),
      image_cache_file = COALESCE(EXCLUDED.image_cache_file, worlds.image_cache_file)

      RETURNING *
    ;
    ",
    w.uuid,
    w.publisher_uuid,
    w.publisher_name,
    w.registered_at,
    w.description,
    w.title,
    w.visits,
    w.favorites,
    w.capacity,
    w.published_at,
    w.does_support_windows,
    w.does_support_android,
    w.does_support_ios,
    w.latest_at,
    f,
  ).fetch_one(get_pool().await).await
}

pub async fn get_id_from_uuid(uuid: &str) -> Result<Option<i64>, sqlx::Error> {
  let res = sqlx::query_scalar!("SELECT id FROM worlds WHERE uuid = ?;", uuid)
    .fetch_optional(get_pool().await)
    .await?;

  Ok(if let Some(Some(v)) = res { Some(v) } else { None })
}

pub async fn does_world_exist(uuid: &str) -> Result<bool, sqlx::Error> {
  let len: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM worlds WHERE uuid = ?;", uuid)
    .fetch_one(get_pool().await)
    .await?;

  Ok(len > 0)
}

// tag 0 is a magic number which shows that the world is favorite.
// -1: 0 AND not another
// null: any
// -2: not 0
pub async fn get_worlds(filter: &WorldQueryFilters, _sort_by: &SortBy) -> Result<Vec<World>, sqlx::Error> {
  match filter.tag_id {
    None => {
      // any
      sqlx::query_as!(
        World,
        "
        SELECT
          worlds.id AS id,
          worlds.uuid AS uuid,
          worlds.publisher_uuid AS publisher_uuid,
          worlds.publisher_name AS publisher_name,
          worlds.registered_at AS registered_at,
          worlds.description AS description,
          worlds.title AS title,
          worlds.visits AS visits,
          worlds.favorites AS favorites,
          worlds.capacity AS capacity,
          worlds.published_at AS published_at,
          worlds.does_support_windows AS does_support_windows,
          worlds.does_support_android AS does_support_android,
          worlds.does_support_ios AS does_support_ios,
          worlds.latest_at AS latest_at,
          worlds.image_cache_file AS image_cache_file,
          ac.cnt AS self_visits
        FROM worlds
        LEFT JOIN (
          SELECT world_id, COUNT(*) AS cnt
          FROM activities
          GROUP BY world_id
        ) ac

        ORDER BY worlds.latest_at DESC
      ;
        ",
      ).fetch_all(get_pool().await).await
    }
    Some(-2) => {
      sqlx::query_as!(
        World,
        "
        SELECT
          worlds.id AS id,
          worlds.uuid AS uuid,
          worlds.publisher_uuid AS publisher_uuid,
          worlds.publisher_name AS publisher_name,
          worlds.registered_at AS registered_at,
          worlds.description AS description,
          worlds.title AS title,
          worlds.visits AS visits,
          worlds.favorites AS favorites,
          worlds.capacity AS capacity,
          worlds.published_at AS published_at,
          worlds.does_support_windows AS does_support_windows,
          worlds.does_support_android AS does_support_android,
          worlds.does_support_ios AS does_support_ios,
          worlds.latest_at AS latest_at,
          worlds.image_cache_file AS image_cache_file,
          ac.cnt AS self_visits
        FROM worlds
        LEFT JOIN (
          SELECT world_id, COUNT(*) AS cnt
          FROM activities
          GROUP BY world_id
        ) ac
          ON worlds.id = ac.world_id
        LEFT JOIN tags_worlds
          ON worlds.id = tags_worlds.worlds_id

        WHERE
          tags_worlds.tags_id IS NULL

        ORDER BY worlds.latest_at DESC
      ;
        "
      ).fetch_all(get_pool().await).await
    }
    Some(-1) => {
      Ok(vec![])
    }
    _ => {
      let id = filter.tag_id.unwrap();

      sqlx::query_as!(
        World,
        "
        SELECT
          worlds.id AS id,
          worlds.uuid AS uuid,
          worlds.publisher_uuid AS publisher_uuid,
          worlds.publisher_name AS publisher_name,
          worlds.registered_at AS registered_at,
          worlds.description AS description,
          worlds.title AS title,
          worlds.visits AS visits,
          worlds.favorites AS favorites,
          worlds.capacity AS capacity,
          worlds.published_at AS published_at,
          worlds.does_support_windows AS does_support_windows,
          worlds.does_support_android AS does_support_android,
          worlds.does_support_ios AS does_support_ios,
          worlds.latest_at AS latest_at,
          worlds.image_cache_file AS image_cache_file,
          ac.cnt AS self_visits
        FROM worlds
        LEFT JOIN (
          SELECT world_id, COUNT(*) AS cnt
          FROM activities
          GROUP BY world_id
        ) ac
          ON worlds.id = ac.world_id
        LEFT JOIN tags_worlds
          ON worlds.id = tags_worlds.worlds_id

        WHERE
          tags_worlds.tags_id = ?

        ORDER BY worlds.latest_at DESC
        ;
        ",
        id
      ).fetch_all(get_pool().await).await
    }
  }
}

pub async fn new_session(world_id: i64, started_at: i64, ended_at: i64) -> Result<(), sqlx::Error> {
  sqlx::query!(
    "INSERT INTO activities (world_id, started_at, ended_at) VALUES (
      ?,
      ?,
      ?
    );",
    world_id,
    started_at,
    ended_at
  ).execute(get_pool().await).await?;
  Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WorldQueryFilters {
  tag_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum SortBy {
  Recency,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct World {
  pub id: i64,
  pub uuid: String,
  pub publisher_uuid: Option<String>,
  pub publisher_name: Option<String>,
  pub registered_at: Option<i64>,
  pub description: Option<String>,
  pub title: Option<String>,
  pub visits: Option<i64>,
  pub favorites: Option<i64>,
  pub capacity: Option<i64>,
  pub published_at: Option<i64>,
  pub does_support_windows: Option<i64>,
  pub does_support_android: Option<i64>,
  pub does_support_ios: Option<i64>,
  pub latest_at: Option<i64>,
  pub image_cache_file: Option<String>,
  pub self_visits: Option<i64>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WorldQuery {
  pub uuid: String,
  pub publisher_uuid: Option<String>,
  pub publisher_name: Option<String>,
  pub registered_at: Option<i64>,
  pub description: Option<String>,
  pub title: Option<String>,
  pub visits: Option<i64>,
  pub favorites: Option<i64>,
  pub capacity: Option<i64>,
  pub published_at: Option<i64>,
  pub does_support_windows: Option<i64>,
  pub does_support_android: Option<i64>,
  pub does_support_ios: Option<i64>,
  pub latest_at: Option<i64>,
  pub image_cache: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WorldDBStructure {
  pub id: i64,
  /*  1 */ pub uuid: String,
  /*  2 */ pub publisher_uuid: Option<String>,
  /*  3 */ pub publisher_name: Option<String>,
  /*  4 */ pub registered_at: Option<i64>,
  /*  5 */ pub description: Option<String>,
  /*  6 */ pub title: Option<String>,
  /*  7 */ pub visits: Option<i64>,
  /*  8 */ pub favorites: Option<i64>,
  /*  9 */ pub capacity: Option<i64>,
  /* 10 */ pub published_at: Option<i64>,
  /* 11 */ pub does_support_windows: Option<i64>,
  /* 12 */ pub does_support_android: Option<i64>,
  /* 13 */ pub does_support_ios: Option<i64>,
  /* 14 */ pub latest_at: Option<i64>,
  /* 15 */ pub image_cache_file: Option<String>,
}
