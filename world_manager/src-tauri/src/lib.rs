mod db;
mod ipc;
mod commands;
mod log_watcher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::new()
        .format(|buf, rec| {
            use std::io::Write;
            writeln!(
                buf,
                "{}:{} [{}] - {}",
                rec.file().unwrap_or("unknown"),
                rec.line().unwrap_or(0),
                rec.level(),
                rec.args(),
            )
        })
        .filter(None, log::LevelFilter::Debug)
        .target(env_logger::Target::Stderr)
        .init();

    log::debug!("Application started");

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

                // wait for DB init, then start IPC server and log watcher
                tauri::async_runtime::spawn(async {
                    loop {
                        if let Err(err) = ipc::main().await {
                            eprintln!("Error while processing IPC: {err}");
                        }
                    }
                });
                tauri::async_runtime::spawn(async {
                    if let Err(err) = log_watcher::main().await {
                        eprintln!("Error while running log watcher: {err}");
                    }
                });
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
