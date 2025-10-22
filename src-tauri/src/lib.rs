// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::path::PathBuf;
use std::str::FromStr;

use directories::ProjectDirs;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use tauri::Manager;

struct AppState {
    pool: sqlx::SqlitePool,
    db_path: PathBuf,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn db_health(state: tauri::State<'_, AppState>) -> Result<String, String> {
    // Simple query to verify connectivity
    let (one,): (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(format!("ok:{}; path={}", one, state.db_path.display()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Resolve persistent data directory for the app
            let proj = ProjectDirs::from("com", "edc", "edc-finc")
                .expect("unable to resolve project directories");
            let data_dir = proj.data_dir();
            std::fs::create_dir_all(data_dir).expect("failed to create data dir");
//             let db_path = data_dir.join("edc-finc.sqlite3");
            let db_path = std::env::current_dir()?.join("edc-finc.sqlite3");



            // Build sqlite connection options
            let db_url = format!("sqlite://{}", db_path.to_string_lossy());
            let connect_options = SqliteConnectOptions::from_str(&db_url)
                .expect("invalid sqlite URL")
                .create_if_missing(true)
                .foreign_keys(true)
                .journal_mode(SqliteJournalMode::Wal);

            // Create pool and run migrations synchronously during setup
            let pool = tauri::async_runtime::block_on(async move {
                let pool = SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect_with(connect_options)
                    .await
                    .expect("failed to connect to sqlite");

                // Run embedded migrations
                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .expect("failed to run database migrations");

                pool
            });

            app.manage(AppState { pool, db_path });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, db_health])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
