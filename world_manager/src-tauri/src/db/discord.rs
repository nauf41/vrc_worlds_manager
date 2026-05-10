use super::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TagDiscordLink {
    pub tag_id: i64,
    pub discord_channel_id: i64, // official id
    pub do_auto_fetch: i64,
    pub do_auto_post: i64,
    pub latest_read_id: Option<i64>,
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
    )
    .fetch_all(get_pool().await)
    .await
}

pub async fn upsert_channel(id: i64, name: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO discord_channels (id, name)
    VALUES ($1, $2)
    ON CONFLICT (id) DO UPDATE SET
      name = COALESCE(EXCLUDED.name, name)
    ;
    "#,
        id,
        name,
    )
    .execute(get_pool().await)
    .await?;
    Ok(())
}

pub struct ChannelDBStructure {
    pub id: i64,
    pub name: String,
}
pub async fn get_channel_by_id(id: i64) -> Result<Option<ChannelDBStructure>, sqlx::Error> {
    sqlx::query_as!(
        ChannelDBStructure,
        "
    SELECT id, name
    FROM discord_channels
    WHERE id = $1;
    ",
        id
    )
    .fetch_optional(get_pool().await)
    .await
}

pub async fn create_link(
    tag_id: i64,
    discord_channel_id: i64,
    do_auto_fetch: bool,
    do_auto_post: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
    INSERT INTO tags_discord_channels (tag_id, discord_channel_id, do_auto_fetch, do_auto_post)
    VALUES ($1, $2, $3, $4);
    ",
        tag_id,
        discord_channel_id,
        do_auto_fetch,
        do_auto_post
    )
    .execute(get_pool().await)
    .await?;
    Ok(())
}

pub async fn update_link(
    tag_id: i64,
    discord_channel_id: i64,
    read_until: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
    UPDATE tags_discord_channels
    SET latest_read_id = $1
    WHERE tag_id = $2 AND discord_channel_id = $3;
    ",
        read_until,
        tag_id,
        discord_channel_id
    )
    .execute(get_pool().await)
    .await?;
    Ok(())
}

pub async fn get_link_by_tag_id(tag_id: i64) -> Result<Option<TagDiscordLink>, sqlx::Error> {
    sqlx::query_as!(
        TagDiscordLink,
        "
    SELECT
      *
    FROM tags_discord_channels
    WHERE tag_id = $1;
    ",
        tag_id
    )
    .fetch_optional(get_pool().await)
    .await
}

pub async fn remove_link_by_tag_id(tag_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
    DELETE FROM tags_discord_channels
    WHERE tag_id = $1;
    ",
        tag_id
    )
    .execute(get_pool().await)
    .await?;
    Ok(())
}

pub async fn get_all_links() -> Result<Vec<TagDiscordLink>, sqlx::Error> {
    sqlx::query_as!(
        TagDiscordLink,
        "
    SELECT * FROM tags_discord_channels;
    "
    )
    .fetch_all(get_pool().await)
    .await
}
