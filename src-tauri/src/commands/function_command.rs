use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::function::{FunctionCondition, FunctionModel, FunctionVO, ModuleFunctionCondition};

#[tauri::command]
pub async fn add_function(
  function_name: String,
  function_remark: String,
  module_id: String,
  function_url: String,
  function_icon: String,
  function_order: String,
  function_status: String,
  last_modified_user_id: String,
  system_id: String,
  create_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn edit_function(
  function_id: String,
  function_name: String,
  function_remark: String,
  module_id: String,
  function_url: String,
  function_icon: String,
  function_order: String,
  function_status: String,
  last_modified_user_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn delete_function(
  function_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn disable_function(
  function_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn enable_function(
  function_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn get_function_list_by_pagination(
  current_page: i64,
  page_size: i64,
  condition: FunctionCondition,
  state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<FunctionVO>, String> {
  if current_page == 0 || page_size == 0 {
    return Err(format!("分页参数不正确"));
  }
  let mut pagination_result = PaginationResult::<FunctionVO>::default();
  pagination_result.current_page = current_page;
  pagination_result.page_size = page_size;
  let offset = (current_page - 1) * page_size;
  let mut function_list_result: Result<Vec<FunctionModel>, Error>;
  if condition.function_name.is_some() || condition.function_status.is_some() {
    if condition.function_name.is_some() && condition.function_status.is_some() {
      function_list_result = sqlx::query_as!(
        FunctionModel,
        r#"SELECT
        id,
        function_id,
        function_name,
        function_remark,
        module_id,
        function_order,
        function_status,
        last_modified_user_id,
        create_time,
        update_time
        from bos_function where module_id = ? and function_name like ? and function_status = ? ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.module_id,
        format!("{}%%",condition.function_name.unwrap()),
        condition.function_status.unwrap(),
        page_size,
        offset
      ).fetch_all(&state.pool)
        .await;
    } else if condition.function_name.is_some() && condition.function_status.is_none() {
      function_list_result = sqlx::query_as!(
        FunctionModel,
        r#"SELECT
        id,
        function_id,
        function_name,
        function_remark,
        module_id,
        function_order,
        function_status,
        last_modified_user_id,
        create_time,
        update_time
        from bos_function where module_id = ? and function_name like ? ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.module_id,
        format!("{}%%",condition.function_name.unwrap()),
        page_size,
        offset
      ).fetch_all(&state.pool)
        .await;
    } else {
      function_list_result = sqlx::query_as!(
        FunctionModel,
        r#"SELECT
        id,
        function_id,
        function_name,
        function_remark,
        module_id,
        function_order,
        function_status,
        last_modified_user_id,
        create_time,
        update_time
        from bos_function where module_id = ? and function_status = ? ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.module_id,
        condition.function_status.unwrap(),
        page_size,
        offset
      ).fetch_all(&state.pool)
        .await;
    }
  } else {
    function_list_result = sqlx::query_as!(
        FunctionModel,
        r#"SELECT
        id,
        function_id,
        function_name,
        function_remark,
        module_id,
        function_order,
        function_status,
        last_modified_user_id,
        create_time,
        update_time
        from bos_function where module_id = ? ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.module_id,
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  }
  let function_list = function_list_result.map_err(|err| format!("查询业务功能分页数据异常: {}", err))?;
  pagination_result.data_list = function_list.into_iter().map(FunctionVO::from).collect();
  let total_result: Result<i64, Error> = sqlx::query_scalar!(r#"SELECT count(*) FROM bos_function where module_id = ?"#,condition.module_id).fetch_one(&state.pool).await;
  let total = total_result.map_err(|err| format!("查询业务功能总数异常: {}", err))?;
  pagination_result.total = total;
  Ok(pagination_result)
}


#[tauri::command]
pub async fn get_module_function_list_by_pagination(
  current_page: i64,
  page_size: i64,
  condition: ModuleFunctionCondition,
  state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<FunctionVO>, String> {
  Ok(PaginationResult::default())
}
