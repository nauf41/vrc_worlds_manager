use tokio::net::windows::named_pipe::ServerOptions;

#[cfg(windows)]
pub async fn main() -> anyhow::Result<()> {
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

      let st: Result<native_messaging::Message, _> = serde_json::from_slice(&dat_buf);
      if let Ok(msg) = st {
        let response_object = process_and_gen_response(msg).await;
        if let Ok(r) = response_object {
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

pub async fn process_and_gen_response(msg: native_messaging::Message) -> anyhow::Result<native_messaging::Response> {
  use native_messaging::{Message, Response, CheckFavoriteResponse};

  match msg {
    Message::FavoriteStatus(dat) => {
      let uuid = dat.uuid;
      let is_favorite = crate::db::does_world_exist(&uuid).await?;

      Ok(Response::FavoriteStatus(CheckFavoriteResponse {
        uuid,
        is_favorite,
      }))
    }
  }
}

pub mod native_messaging {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  #[serde(tag = "type", content = "body")]
  pub enum Message {
    #[serde(rename = "favorite-status")]
    FavoriteStatus(CheckFavorite),
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct CheckFavorite {
    pub uuid: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  #[serde(tag = "type", content = "body")]
  pub enum Response {
    #[serde(rename = "favorite-status")]
    FavoriteStatus(CheckFavoriteResponse),
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct CheckFavoriteResponse {
    pub uuid: String,
    #[serde(rename = "isFavorite")]
    pub is_favorite: bool,
  }
}