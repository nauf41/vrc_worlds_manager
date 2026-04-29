mod db;
mod ipc;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_worlds,
            commands::get_tags,
            commands::create_tag,
        ])
        .setup(|_app| {
            tauri::async_runtime::spawn(async {
                if let Err(err) = db::init().await {
                    eprintln!("failed to initialize database: {err}");
                }
            });
            tauri::async_runtime::spawn(async {
                loop {
                    if let Err(err) = ipc::main().await {
                        eprintln!("Error while processing IPC: {err}");
                    }
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
