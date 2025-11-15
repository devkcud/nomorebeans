pub mod command;
pub mod models;
pub mod repositories;
pub mod services;
pub mod state;
pub mod utils;

use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tauri::{async_runtime, generate_context, generate_handler, Builder};

use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();
    async_runtime::block_on(async {
        let pool = PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not set"))
            .await
            .expect("Failed to connect to database");

        Builder::default()
            .manage(AppState::new(pool))
            .plugin(tauri_plugin_fs::init())
            .invoke_handler(generate_handler![
                command::get_profiles,
                command::create_profile,
                command::get_profile_by_id,
                command::get_profile_by_username,
                command::delete_profile,
                command::update_profile
            ])
            .run(generate_context!())
            .expect("error while running tauri application");
    });
}
