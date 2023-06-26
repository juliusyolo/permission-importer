use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::role::{RoleCondition, RoleUserCondition, RoleVO};
use crate::models::user::UserOrganizationRoleVO;

#[tauri::command]
pub async fn add_role(
  role_code: String,
  role_name: String,
  role_remark: String,
  role_status: String,
  last_modified_user_id: String,
  system_id: String,
  create_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn edit_role(
  role_id: String,
  role_code: String,
  role_name: String,
  role_remark: String,
  role_status: String,
  last_modified_user_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}

#[tauri::command]
pub async fn delete_role(
  role_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn disable_role(
  role_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn enable_role(
  role_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn get_role_user_list_by_pagination(
  current_page: i64,
  page_size: i64,
  condition: RoleUserCondition,
  state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<UserOrganizationRoleVO>, String> {
  Ok(PaginationResult::default())
}


#[tauri::command]
pub async fn get_role_list_by_pagination(
  current_page: i64,
  page_size: i64,
  condition: RoleCondition,
  state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<RoleVO>, String> {
  Ok(PaginationResult::default())
}
