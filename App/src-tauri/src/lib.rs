// GATE-MKP Step 1: client database + SeaORM; no Vue API changes yet.
mod db;
mod entities;

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
