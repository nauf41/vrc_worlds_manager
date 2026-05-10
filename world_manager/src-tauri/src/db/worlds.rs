use serde::{Deserialize, Serialize};

use super::get_pool;

pub async fn upsert_world(w: WorldQuery) -> Result<(bool, WorldDBStructure), sqlx::Error> {
  /// bool: true if inserted, false if updated
  let r = sqlx::query_as!(
    WorldDBStructure,
    "
    INSERT INTO worlds (uuid, publisher_uuid, publisher_name, registered_at, description, title, visits, favorites, capacity, published_at, does_support_windows, does_support_android, does_support_ios, latest_at)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
    ON CONFLICT(uuid) DO NOTHING
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
  ).fetch_optional(get_pool().await).await?;

  if let Some(w) = r {
    Ok((true, w))
  } else {
    sqlx::query_as!(
      WorldDBStructure,
      r#"
    UPDATE worlds
    SET
      uuid = COALESCE($1, worlds.uuid),
      publisher_uuid = COALESCE($2, worlds.publisher_uuid),
      publisher_name = COALESCE($3, worlds.publisher_name),
      registered_at = COALESCE($4, worlds.registered_at),
      description = COALESCE($5, worlds.description),
      title = COALESCE($6, worlds.title),
      visits = COALESCE($7, worlds.visits),
      favorites = COALESCE($8, worlds.favorites),
      capacity = COALESCE($9, worlds.capacity),
      published_at = COALESCE($10, worlds.published_at),
      does_support_windows = COALESCE($11, worlds.does_support_windows),
      does_support_android = COALESCE($12, worlds.does_support_android),
      does_support_ios = COALESCE($13, worlds.does_support_ios),
      latest_at = MAX(worlds.latest_at, COALESCE($14, worlds.latest_at))
    WHERE worlds.uuid = $1
    RETURNING
      COALESCE(id, 0) AS "id!: i64",
      uuid,
      publisher_uuid,
      publisher_name,
      registered_at,
      description,
      title,
      visits,
      favorites,
      capacity,
      published_at,
      does_support_windows,
      does_support_android,
      does_support_ios,
      latest_at,
      image_cache_file
    ;
    "#,
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
    w.latest_at
    ).fetch_one(get_pool().await).await.and_then(|v| Ok((false, v)))
  }
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

pub async fn does_tag_contain_world(tagid: i64, worldid: i64) -> Result<bool, sqlx::Error> {
  sqlx::query_scalar!(
    "
    SELECT EXISTS (
      SELECT 1
      FROM tags_worlds
      WHERE tags_id = ? AND worlds_id = ?
    );
    ",
    tagid,
    worldid
  ).fetch_one(get_pool().await).await.map(|cnt| cnt > 0)
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
        r#"
        SELECT
          w.id AS "id!: i64",
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
          w.image_cache_file,
          COALESCE((SELECT COUNT(*) FROM activities a WHERE a.world_id = w.id), 0) AS "self_visits!: i64"
        FROM worlds w

        ORDER BY w.latest_at DESC
      ;
        "#,
      ).fetch_all(get_pool().await).await
    }
    Some(-2) => {
      sqlx::query_as!(
        World,
        r#"
        SELECT
          w.id AS "id!: i64",
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
          w.image_cache_file,
          COALESCE((SELECT COUNT(*) FROM activities a WHERE a.world_id = w.id), 0) AS "self_visits!: i64"
        FROM worlds w
        LEFT JOIN (
          SELECT world_id, COUNT(*) AS cnt
          FROM activities
          GROUP BY world_id
        ) ac
          ON w.id = ac.world_id
        LEFT JOIN tags_worlds tw
          ON w.id = tw.worlds_id

        WHERE
          tw.tags_id IS NULL

        ORDER BY w.latest_at DESC
      ;
        "#
      ).fetch_all(get_pool().await).await
    }
    Some(-1) => {
      sqlx::query_as!(
        World,
        r#"
        SELECT
          w.id AS "id!: i64",
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
          w.image_cache_file,
          COALESCE((SELECT COUNT(*) FROM activities a WHERE a.world_id = w.id), 0) AS "self_visits!: i64"
        FROM worlds w
        INNER JOIN tags_worlds tw
          ON w.id = tw.worlds_id
        WHERE tw.tags_id = 0
          AND NOT EXISTS (
            SELECT 1
            FROM tags_worlds tw_other
            WHERE tw_other.worlds_id = w.id
              AND tw_other.tags_id != 0
          )

        ORDER BY w.latest_at DESC
        ;
        "#
      ).fetch_all(get_pool().await).await
    }
    _ => {
      let id = filter.tag_id.unwrap();

      sqlx::query_as!(
        World,
        r#"
        SELECT
          w.id AS "id!: i64",
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
          w.image_cache_file,
          COALESCE((SELECT COUNT(*) FROM activities a WHERE a.world_id = w.id), 0) AS "self_visits!: i64"
        FROM worlds w
        INNER JOIN tags_worlds tw
          ON w.id = tw.worlds_id

        WHERE
          tw.tags_id = $1

        ORDER BY w.latest_at DESC
        ;
        "#,
        id
      ).fetch_all(get_pool().await).await
    }
  }
}

pub async fn get_world_by_id(id: i64) -> Result<Option<World>, sqlx::Error> {
  sqlx::query_as!(
    World,
    r#"
    SELECT
      w.id AS "id!: i64",
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
      w.image_cache_file,
      COALESCE((SELECT COUNT(*) FROM activities a WHERE a.world_id = w.id), 0) AS "self_visits!: i64"
    FROM worlds w
    WHERE w.id = $1
    ;
    "#,
    id
  ).fetch_optional(get_pool().await).await
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
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
  pub self_visits: i64,
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

impl Into<WorldDBStructure> for World {
  fn into(self) -> WorldDBStructure {
    WorldDBStructure {
      id: self.id,
      uuid: self.uuid,
      publisher_uuid: self.publisher_uuid,
      publisher_name: self.publisher_name,
      registered_at: self.registered_at,
      description: self.description,
      title: self.title,
      visits: self.visits,
      favorites: self.favorites,
      capacity: self.capacity,
      published_at: self.published_at,
      does_support_windows: self.does_support_windows,
      does_support_android: self.does_support_android,
      does_support_ios: self.does_support_ios,
      latest_at: self.latest_at,
      image_cache_file: self.image_cache_file,
    }
  }
}