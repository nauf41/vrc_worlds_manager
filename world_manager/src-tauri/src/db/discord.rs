use super::*;

pub struct TagDiscordLink {
  pub tag_id: i64,
  pub discord_channel_id: i64, // official id
  pub do_auto_fetch: i64,
  pub do_auto_post: i64,
}

pub async fn get_handlers() -> Result<Vec<TagDiscordLink>, sqlx::Error> {
  sqlx::query_as!(
    TagDiscordLink,
    "
    SELECT
      *
    FROM tags_discord_channels
    WHERE do_auto_fetch = 1;
    "
  ).fetch_all(get_pool().await).await
}

pub async fn upsert_channel(id: i64, name: String, latest_read_id: Option<i64>) -> Result<(), sqlx::Error> {
  sqlx::query!(
    r#"
    INSERT INTO discord_channels (id, name, latest_read_id)
    VALUES ($1, $2, $3)
    ON CONFLICT (id) DO UPDATE SET
      name = COALESCE(EXCLUDED.name, name),
      latest_read_id = COALESCE(EXCLUDED.latest_read_id, latest_read_id);
    "#,
    id,
    name,
    latest_read_id
  ).execute(get_pool().await).await?;
  Ok(())
}

pub async fn create_link(tag_id: i64, discord_channel_id: i64, do_auto_fetch: bool, do_auto_post: bool) -> Result<(), sqlx::Error> {
  sqlx::query!(
    "
    INSERT INTO tags_discord_channels (tag_id, discord_channel_id, do_auto_fetch, do_auto_post)
    VALUES ($1, $2, $3, $4);
    ",
    tag_id,
    discord_channel_id,
    do_auto_fetch,
    do_auto_post
  ).execute(get_pool().await).await?;
  Ok(())
}

pub async fn remove_link_by_tag_id(tag_id: i64) -> Result<(), sqlx::Error> {
  sqlx::query!(
    "
    DELETE FROM tags_discord_channels
    WHERE tag_id = $1;
    ",
    tag_id
  ).execute(get_pool().await).await?;
  Ok(())
}

pub async fn get_all_links() -> Result<Vec<TagDiscordLink>, sqlx::Error> {
  sqlx::query_as!(
    TagDiscordLink,
    "
    SELECT * FROM tags_discord_channels;
    "
  ).fetch_all(get_pool().await).await
}
