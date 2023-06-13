// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::error::Error;
use tauri::{async_runtime};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
use tauri::State;

#[derive(Serialize, Deserialize)]
struct VersionControlModel {
  id: u8,
  version: String,
  #[serde(rename = "releaseTime")]
  release_time: String,
  #[serde(rename = "createTime")]
  create_time: String,
  #[serde(rename = "updateTime")]
  update_time: String,
}

struct DatabaseConnectionPool{
  pool: Pool<MySql>
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_version_control_list_by_pagination(current_page: u8, page_size: u8, state: State<Pool<MySql>>) -> Vec<VersionControlModel> {
  let version_control_list = vec![
    VersionControlModel {
      id: 1,
      version: "BH_RCMP.PLS_23.05.02".to_string(),
      release_time: "2023-06-18 15:00:00".to_string(),
      create_time: "2023-06-18 15:00:00".to_string(),
      update_time: "2023-06-18 15:00:00".to_string(),
    },
    VersionControlModel {
      id: 2,
      version: "BH_RCMP.PLS_23.05.01".to_string(),
      release_time: "2023-06-18 15:00:00".to_string(),
      create_time: "2023-06-18 15:00:00".to_string(),
      update_time: "2023-06-18 15:00:00".to_string(),
    },
  ];
  version_control_list
}
async fn runner() -> Result<(), dyn Error> {
  let pool = MySqlPoolOptions::new()
    .max_connections(10)
    .connect(&env::var("DATABASE_URL")?).await?;

  tauri::Builder::default()
    .manage(DatabaseConnectionPool { pool })
    .invoke_handler(tauri::generate_handler![greet,get_version_control_list_by_pagination])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  Ok(())
}

fn main(){
  async_runtime::block_on(runner()).expect("TODO: panic message");

}
