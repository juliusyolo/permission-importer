use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::group::{GroupCondition, GroupUserCondition, GroupVO};
use crate::models::user::UserOrganizationGroupVO;

#[tauri::command]
pub async fn add_group(
  group_code: String,
  group_name: String,
  group_remark: String,
  group_status: String,
  last_modified_user_id: String,
  system_id: String,
  create_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn edit_group(
  group_id: String,
  group_code: String,
  group_name: String,
  group_remark: String,
  group_status: String,
  last_modified_user_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn delete_group(
  group_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}

#[tauri::command]
pub async fn disable_group(
  group_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}

#[tauri::command]
pub async fn enable_group(
  group_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn get_group_user_list_by_pagination(
  current_page: i64,
  page_size: i64,
  condition: GroupUserCondition,
  state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<UserOrganizationGroupVO>, String> {
  Ok(PaginationResult::default())
}

#[tauri::command]
pub async fn get_group_list_by_pagination(
  current_page: i64,
  page_size: i64,
  condition: GroupCondition,
  state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<GroupVO>, String> {
  Ok(PaginationResult::default())
}
