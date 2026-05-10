use crate::db::get_pool;
use serde::{Deserialize, Serialize};

pub async fn create(name: String) -> Result<TagGroup, sqlx::Error> {
    sqlx::query_as!(
        TagGroup,
        "
    INSERT INTO tag_groups (name)
    VALUES ($1)
    RETURNING id, name
    ;
    ",
        name
    )
    .fetch_one(get_pool().await)
    .await
}

pub async fn get() -> Result<Vec<TagGroup>, sqlx::Error> {
    sqlx::query_as!(
        TagGroup,
        "
    SELECT
      id,
      name
    FROM tag_groups
    ORDER BY name ASC
    ;
    ",
    )
    .fetch_all(get_pool().await)
    .await
}

pub async fn get_with_tags() -> Result<Vec<(TagGroup, Vec<i64>)>, sqlx::Error> {
    let dat = sqlx::query_as!(
        TagWithTagGroup,
        "
    SELECT
      tg.id AS group_id,
      tg.name AS group_name,
      tgt.tags_id AS tag_id
    FROM tag_groups tg
    LEFT JOIN tag_groups_tags tgt
      ON tgt.tag_groups_id = tg.id
    ORDER BY tg.id
    ;
    "
    )
    .fetch_all(get_pool().await)
    .await?;

    let mut res: Vec<(TagGroup, Vec<i64>)> = vec![];

    for dt in dat.into_iter() {
        if let Some(e) = res.last_mut() {
            if e.0.id == dt.group_id {
                if let Some(v) = dt.tag_id {
                    e.1.push(v);
                }
                continue;
            }
        }

        res.push((
            TagGroup {
                id: dt.group_id,
                name: dt.group_name,
            },
            if let Some(v) = dt.tag_id {
                vec![v]
            } else {
                vec![]
            },
        ));
    }

    Ok(res)
}

pub async fn update_name(id: i64, name: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
    UPDATE tag_groups
    SET name = $1
    WHERE id = $2
    ",
        name,
        id
    )
    .execute(get_pool().await)
    .await?;

    Ok(())
}

pub async fn upsert_attachment(tag_id: i64, group_id: Option<i64>) -> Result<(), sqlx::Error> {
    let pool = get_pool().await;

    if let Some(gid) = group_id {
        sqlx::query!(
            "
        INSERT INTO tag_groups_tags (tag_groups_id, tags_id)
        VALUES ($1, $2)
        ON CONFLICT (tags_id) DO UPDATE SET tag_groups_id = excluded.tag_groups_id;
      ",
            gid,
            tag_id
        )
        .execute(pool)
        .await?;
    } else {
        sqlx::query!(
            "
      DELETE FROM tag_groups_tags
      WHERE tags_id = $1
      ;
      ",
            tag_id
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn delete(id: i64) -> Result<(), sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query!(
        "
    DELETE FROM tag_groups_tags
    WHERE tag_groups_id = $1
    ;
    ",
        id
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "
    DELETE FROM tag_groups
    WHERE id = $1
    ;
    ",
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TagGroup {
    pub id: i64,
    pub name: String,
}

struct TagWithTagGroup {
    pub tag_id: Option<i64>,
    pub group_id: i64,
    pub group_name: String,
}
