use tauri::{generate_context, generate_handler, Builder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .invoke_handler(generate_handler![])
        .run(generate_context!())
        .expect("error while running tauri application");
}
