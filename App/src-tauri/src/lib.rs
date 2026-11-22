// GATE-MKP offline client. Server/Django remains untouched.
mod db;
mod entities;
mod services;
mod repositories;
mod commands;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let db = tauri::async_runtime::block_on(db::initialize(app.handle()))
                .map_err(std::io::Error::other)?;
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::local_data::local_list,
            commands::local_data::local_get,
            commands::local_data::local_create,
            commands::local_data::local_update,
            commands::local_data::local_delete,
            commands::local_data::local_import,
            commands::local_data::local_import_links,
            commands::local_data::local_mirror,
            commands::local_data::local_save_media,
            commands::local_data::local_attach_media,
            commands::local_data::local_read_media,
            commands::local_data::local_create_or_link_bohrkern,
            commands::dashboard::dashboard_snapshot,
            commands::dashboard::dashboard_record_baselines,
            commands::dashboard::dashboard_record_server_check,
            commands::dashboard::dashboard_record_opened,
            commands::sync::sync_take_pending,
            commands::sync::sync_apply,
            commands::sync::sync_list_conflicts,
            commands::sync::sync_take_creates,
            commands::sync::sync_apply_creates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
