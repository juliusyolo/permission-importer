use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::module::{ModuleCondition, ModuleModel, ModuleVO};

#[tauri::command]
pub async fn add_module(
  module_id: String,
  module_name: String,
  module_remark: String,
  module_status: String,
  last_modified_user_id: String,
  system_id: String,
  create_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}

#[tauri::command]
pub async fn edit_module(
  module_id: String,
  module_name: String,
  module_remark: String,
  module_status: String,
  last_modified_user_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn delete_module(
  module_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn disable_module(
  module_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn enable_module(
  module_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn get_module_list_by_pagination(
  current_page: i64,
  page_size: i64,
  condition: ModuleCondition,
  state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<ModuleVO>, String> {
  if current_page == 0 || page_size == 0 {
    return Err(format!("分页参数不正确"));
  }
  let mut pagination_result = PaginationResult::<ModuleVO>::default();
  pagination_result.current_page = current_page;
  pagination_result.page_size = page_size;
  let offset = (current_page - 1) * page_size;
  let module_list_result = sqlx::query_as!(
        ModuleModel,
        r#"SELECT
        id,
        module_id,
        module_name,
        module_remark,
        module_status,
        last_modified_user_id,
        create_time,
        update_time
        FROM bos_module ORDER BY id desc LIMIT ? OFFSET ?"#,
        page_size,
        offset
  )
    .fetch_all(&state.pool)
    .await;
  match module_list_result {
    Ok(module_list) => {
      pagination_result.data_list = module_list.into_iter().map(ModuleVO::from).collect();
    }
    Err(_error) => {
      return Err(format!("查询分页用户数据异常"));
    }
  }
  let total_result: Result<i64, Error> = sqlx::query_scalar(r#"SELECT count(*) FROM bos_module"#)
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
