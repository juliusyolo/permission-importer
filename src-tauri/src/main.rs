// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPool;
use sqlx::types::chrono::NaiveDateTime;
use tauri::State;

#[derive(Serialize, Deserialize)]
struct VersionControlModel {
  id: u64,
  version: String,
  #[serde(rename = "releaseTime", with = "serde_naive_datetime")]
  release_time: NaiveDateTime,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  update_time: NaiveDateTime,
}
#[derive(Serialize, Deserialize)]
struct PaginationResult<T>{
  total: i64,
  #[serde(rename = "currentPage")]
  current_page: i64,
  #[serde(rename = "pageSize")]
  page_size: i64,
  data:Vec<T>,
}

mod serde_naive_datetime {
  use serde::{self, Deserialize, Deserializer, Serializer};
  use sqlx::types::chrono::NaiveDateTime;

  const FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f";

  pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
  {
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
      D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
  }
}

struct DatabaseConnectionPool {
  pool: Pool<MySql>,
}

#[tauri::command]
async fn get_version_control_list_by_pagination(current_page: i64, page_size: i64, state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<VersionControlModel>, String> {
  if current_page == 0 || page_size == 0 {
    return Err(String::from("Invalid input parameters"));
  }
  let offset = (current_page - 1) * page_size;
  let version_control_list = sqlx::query_as!(
        VersionControlModel,
        r#"SELECT id, version, release_time, create_time, update_time FROM version_control LIMIT ? OFFSET ?"#,
        page_size,
        offset
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

  let total: i64 = sqlx::query_scalar(r#"SELECT count(*) FROM version_control"#)
    .fetch_one(&state.pool)
    .await.map_err(|e| format!("Database error: {}", e))?;

  if version_control_list.is_empty() {
    return Err(String::from("No version control found"));
  }
  let pagination_result = PaginationResult::<VersionControlModel>{
    total,
    current_page,
    page_size,
    data:version_control_list,
  };
  Ok(pagination_result)
}

#[tokio::main]
async fn main() {
  let pool = MySqlPool::connect_lazy("mysql://root:527901748@localhost:3306/permission_control").expect("");
  tauri::Builder::default()
    .manage(DatabaseConnectionPool { pool })
    .invoke_handler(tauri::generate_handler![get_version_control_list_by_pagination])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
