use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::version_control::VersionControlModel;

#[tauri::command]
pub async fn get_version_control_list_by_pagination(current_page: i64, page_size: i64, state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<VersionControlModel>, String> {
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
pub async fn save_version_control(version: String, release_time: String, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
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
      println!("{}", error);
      Err(format!("新增异常"))
    }
  };
}
