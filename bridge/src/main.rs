use tokio::net::windows::named_pipe::ClientOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rand::{RngExt};
use std::io::Read;

#[tokio::main]
#[cfg(windows)]
async fn main() -> anyhow::Result<()> {
  let mut rng = rand::rng();
  let proc_uniq_id = rng.random::<u32>();

  let client = loop {
    if let Ok(v) = ClientOptions::new().open(r"\\.\pipe\io.github.nauf41.world_manager.tauri") {
      break v;
    }

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
  };


  let (mut reader, mut writer) = tokio::io::split(client);

  // Stdin -> Pipe
  let stdin_to_pipe = tokio::spawn(async move {
    let mut stdin = std::io::stdin();

    loop {
      use std::io::Write;

      let mut len_buf = [0u8;4];
      stdin.read_exact(&mut len_buf).unwrap();
      let len = u32::from_le_bytes(len_buf);
      let mut msg_buf = vec![0u8; len as usize];
      stdin.read_exact(&mut msg_buf).unwrap();

      let mut msg: Vec<u8> = vec![];
      msg.extend_from_slice(&proc_uniq_id.to_be_bytes());
      msg.extend_from_slice(&len_buf);
      msg.extend_from_slice(&msg_buf);

      writer.write_all(&mut msg).await.unwrap();
    }
  });

  // Pipe -> Stdout
  let pipe_to_stdout = tokio::spawn(async move {
    let mut stdout = std::io::stdout();

    loop {
      use std::io::Write;

      let mut header_buf = [0u8;4];
      reader.read_exact(&mut header_buf).await.unwrap();

      let mut siz_buf = [0u8;4];
      reader.read_exact(&mut siz_buf).await.unwrap();

      let siz = u32::from_le_bytes(siz_buf) as usize;

      let mut dat_buf = vec![0u8; siz];
      reader.read_exact(&mut dat_buf).await.unwrap();

      let mut msg: Vec<u8> = siz_buf.into();
      msg.append(&mut dat_buf);
      stdout.write_all(&mut msg).unwrap();
      stdout.flush().unwrap();
    }
  });

  tokio::try_join!(stdin_to_pipe, pipe_to_stdout)?;

  Ok(())
}
