// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::mysql::MySqlPool;

use commands::version_control_command::{get_version_control_list_by_pagination, save_version_control};
use commands::user_command::{add_user,get_user_list_by_pagination};
use common::types::{DatabaseConnectionPool};

mod models;
mod common;
mod commands;


#[tokio::main]
async fn main() {
  let pool = MySqlPool::connect_lazy("mysql://root:527901748@localhost:3306/permission_control").expect("");
  tauri::Builder::default()
    .manage(DatabaseConnectionPool { pool })
    .invoke_handler(tauri::generate_handler![get_version_control_list_by_pagination,save_version_control,add_user,get_user_list_by_pagination])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
