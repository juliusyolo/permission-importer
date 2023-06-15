// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use sqlx::{Error, MySql, Pool};
use sqlx::mysql::MySqlPool;
use sqlx::types::chrono::NaiveDateTime;
use tauri::State;

#[derive(Serialize, Deserialize, Default)]
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

#[derive(Serialize, Deserialize, Default)]
struct PaginationResult<T> {
  total: i64,
  #[serde(rename = "currentPage")]
  current_page: i64,
  #[serde(rename = "pageSize")]
  page_size: i64,
  data: Vec<T>,
}

#[derive(Serialize, Deserialize, Default)]
struct ExecuteResult<T> {
  error: bool,
  #[serde(rename = "errorMsg")]
  error_msg: String,
  result: T,
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
    return Err(format!("分页参数不正确"));
  }
  let mut pagination_result = PaginationResult::<VersionControlModel>::default();
  pagination_result.current_page = current_page;
  pagination_result.page_size = page_size;
  let offset = (current_page - 1) * page_size;
  let version_control_list_result = sqlx::query_as!(
        VersionControlModel,
        r#"SELECT id, version, release_time, create_time, update_time FROM version_control ORDER BY id desc LIMIT ? OFFSET ?"#,
        page_size,
        offset
    )
    .fetch_all(&state.pool)
    .await;
  match version_control_list_result {
    Ok(version_control_list) => {
      pagination_result.data = version_control_list
    }
    Err(_error) => {
      return Err(format!("查询分页数据异常"));
    }
  }
  let total_result: Result<i64, Error> = sqlx::query_scalar(r#"SELECT count(*) FROM version_control"#)
    .fetch_one(&state.pool)
    .await;
  match total_result {
    Ok(total) => {
      pagination_result.total = total;
    }
    Err(_error) => {
      return Err(format!("查询总数异常"));
    }
  }
  Ok(pagination_result)
}

#[tauri::command]
async fn save_version_control(version: String, release_time: String, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  let save_result = sqlx::query!(
        r#"INSERT INTO version_control (version, release_time) VALUES (?, ?)"#,
        version,
        release_time
    ).execute(&state.pool)
    .await;
  return match save_result {
    Ok(query_result) => {
      Ok(query_result.last_insert_id())
    }
    Err(error) => {
      println!("{}",error);
      Err(format!("新增异常"))
    }
  }
}

#[tokio::main]
async fn main() {
  let pool = MySqlPool::connect_lazy("mysql://root:527901748@localhost:3306/permission_control").expect("");
  tauri::Builder::default()
    .manage(DatabaseConnectionPool { pool })
    .invoke_handler(tauri::generate_handler![get_version_control_list_by_pagination,save_version_control])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
