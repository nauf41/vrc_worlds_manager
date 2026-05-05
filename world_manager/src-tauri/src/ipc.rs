#[cfg(windows)]
use tauri::AppHandle;
use tokio::net::windows::named_pipe::ServerOptions;

#[cfg(windows)]
pub async fn main(app: AppHandle) -> anyhow::Result<()> {
  let receiver = tokio::spawn(async move {
    let server = ServerOptions::new()
      .first_pipe_instance(false)
      .create(r"\\.\pipe\io.github.nauf41.world_manager.tauri").unwrap();

    server.connect().await.unwrap();

    let (mut reader, mut writer) = tokio::io::split(server);
    // TODO 接続相手先のパスなどで認証をする

    loop {
      use tokio::io::AsyncReadExt;

      let mut id_buf = [0u8;4];
      let mut len_buf = [0u8;4];

      reader.read_exact(&mut id_buf).await.unwrap();
      reader.read_exact(&mut len_buf).await.unwrap();

      let id = u32::from_le_bytes(id_buf);
      let len = u32::from_le_bytes(len_buf);

      if len > 64 * 1024 * 1024 { // Since there's 64 MiB outbound limitation on Chrome Native Messaging API
        panic!("Message length is too large; client-id: {id}, length: {len}");
      }

      let mut dat_buf = vec![0u8; len as usize];
      reader.read_exact(&mut dat_buf).await.unwrap();

      if let Ok(s) = String::from_utf8(dat_buf.clone()) {
        println!("received message: {s}");
      }

      let st: Result<native_messaging::Message, _> = serde_json::from_slice(&dat_buf);
      if let Ok(msg) = st {
        let response_object = process_and_gen_response(&app, msg).await;
        if let Ok(r) = response_object {
          println!("Returning {r:?}");
          let res = serde_json::to_vec(&r);

          if let Ok(mut resp) = res {
            use tokio::io::AsyncWriteExt;

            let mut result: Vec<u8> = id_buf.into();
            result.extend_from_slice(&(resp.len() as u32).to_le_bytes());
            result.append(&mut resp);

            writer.write_all(&result).await.unwrap();
          } else {
            eprintln!("Failed serializing Object and continuing: {res:?}");
          }
        } else {
          eprintln!("Failed to generate response: {response_object:?}")
        }
      } else {
        eprintln!("Failed to parse message: {st:?}");
      }
    }
  });

  tokio::try_join!(receiver)?;

  Ok(())
}

pub async fn process_and_gen_response(app: &AppHandle, msg: native_messaging::Message) -> anyhow::Result<native_messaging::Response> {
  use native_messaging::{MessageBody, Response, CheckFavoriteResponse, ResponseBody};
  use tauri::Emitter;

  match msg.body {
    MessageBody::FavoriteStatus(dat) => {
      let uuid = dat.uuid;
      let is_favorite = crate::db::worlds::does_world_exist(&uuid).await?;

      app.emit("favorite-status-updated", ()).unwrap();

      Ok(Response {
        id: msg.id,
        body: ResponseBody::FavoriteStatus(CheckFavoriteResponse {
          uuid,
          is_favorite,
        })
      })
    }

    MessageBody::UpdateCache(cache) => {
      crate::db::worlds::add_new_world_if_not_exists(&cache.world.uuid).await?;
      crate::db::worlds::add_world_cache(&cache.world, &cache.cache).await?;

      app.emit("world-cache-updated", ()).unwrap();

      Ok(Response {
        id: msg.id,
        body: ResponseBody::UpdateCache(true)
      })
    }

    MessageBody::SetRegistered(req) => {
      if let Some(id) = crate::db::worlds::get_world_id_by_uuid(&req.world.uuid).await? {
        crate::db::worlds::update_registered(id, req.is_registered).await?;

        app.emit("registered-status-updated", ()).unwrap();

        Ok(Response {
          id: msg.id,
          body: ResponseBody::SetRegistered(true)
        })
      } else {
        Ok(Response {
          id: msg.id,
          body: ResponseBody::SetRegistered(false)
        })
      }
    }
  }
}

pub mod native_messaging {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct Message {
    pub id: f64,

    pub body: MessageBody,
  }

  #[derive(Debug, Serialize, Deserialize, Clone)]
  #[serde(tag = "type", content = "body")]
  pub enum MessageBody {
    #[serde(rename = "favorite-status")]
    FavoriteStatus(CheckFavorite),

    #[serde(rename = "update-cache")]
    UpdateCache(UpdateCache),

    #[serde(rename = "set-registered")]
    SetRegistered(SetRegistered),
  }

  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct CheckFavorite {
    pub uuid: String,
  }

  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct UpdateCache {
    pub world: World,
    pub cache: WorldCache,
  }

  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct SetRegistered {
    #[serde(rename = "isRegistered")]
    pub is_registered: bool,
    pub world: World,
  }

  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct Response {
    pub id: f64,
    pub body: ResponseBody,
  }

  #[derive(Debug, Serialize, Deserialize, Clone)]
  #[serde(tag = "type", content = "body")]
  pub enum ResponseBody{
    #[serde(rename = "favorite-status")]
    FavoriteStatus(CheckFavoriteResponse),

    #[serde(rename = "update-cache")]
    UpdateCache(bool),

    #[serde(rename = "set-registered")]
    SetRegistered(bool),
  }

  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct CheckFavoriteResponse {
    pub uuid: String,
    #[serde(rename = "isFavorite")]
    pub is_favorite: bool,
  }

  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct World {
    pub uuid: String,
  }

  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct WorldCache {
    pub description: Option<String>,
    pub title: Option<String>,
    pub visits: Option<i64>,
    pub favorites: Option<i64>,
    pub capacity: Option<i64>,
    pub published_at: Option<i64>,
    pub does_support_windows: Option<bool>,
    pub does_support_android: Option<bool>,
    pub does_support_ios: Option<bool>,
  }
}
