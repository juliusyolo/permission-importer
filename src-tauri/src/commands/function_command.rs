use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::function::{FunctionCondition, FunctionVO, ModuleFunctionCondition};

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
  Ok(PaginationResult::default())
}


#[tauri::command]
pub async fn get_module_function_list_by_pagination(
  current_page: i64,
  page_size: i64,
  condition: ModuleFunctionCondition,
  state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<FunctionVO>, String> {
  Ok(PaginationResult::default())
}
