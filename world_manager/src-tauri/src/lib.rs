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
        .filter(None, log::LevelFilter::Info)
        .target(env_logger::Target::Stderr)
        .init();

    log::info!("Application started");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_worlds,
            commands::get_tags,
            commands::create_tag,
            commands::delete_tag,
            commands::change_tag,
            commands::create_tag_group,
            commands::get_tag_groups,
            commands::edit_tag_group_name,
            commands::delete_tag_group,
            commands::upsert_tag_group_attachment,
            commands::get_tag_groups_with_tags,
            commands::add_world,
            commands::add_world_cache,
            commands::get_tags_with_children,
            commands::get_favorited_worlds,
            commands::attach_world,
            commands::detach_world,
            commands::upsert_publisher,
        ])
        .setup(|app| {
            let handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                if let Err(err) = db::init().await {
                    eprintln!("failed to initialize database: {err}");
                }

                let h1 = handle.clone();
                let h2 = handle.clone();
                // wait for DB init, then start IPC server and log watcher
                tauri::async_runtime::spawn(async move {
                    loop {
                        if let Err(err) = ipc::main(h1.clone()).await {
                            eprintln!("Error while processing IPC: {err}");
                        }
                    }
                });
                tauri::async_runtime::spawn(async move {
                    if let Err(err) = log_watcher::main(h2).await {
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
