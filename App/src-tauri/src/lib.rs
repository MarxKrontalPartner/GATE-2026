pub mod login;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(login::AuthState::default())
        .invoke_handler(tauri::generate_handler![
            login::login,
            login::logout,
            login::is_logged_in,
            login::api,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
