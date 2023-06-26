use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::control::{ControlPointCondition, ControlPointVO, ControlResourceVO, ResourceClipRuleVO};

#[tauri::command]
pub async fn add_control_point(
  control_id: String,
  function_id: String,
  control_code: String,
  control_name: String,
  control_status: String,
  last_modified_user_id: String,
  create_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}

#[tauri::command]
pub async fn edit_control_point(
  control_id: String,
  function_id: String,
  control_code: String,
  control_name: String,
  control_status: String,
  last_modified_user_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn delete_control_point(
  control_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn disable_control_point(
  control_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn enable_control_point(
  control_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn get_control_point_list_by_pagination(
  current_page: i64,
  page_size: i64,
  condition: ControlPointCondition,
  state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<ControlPointVO>, String> {
  Ok(PaginationResult::default())
}

#[tauri::command]
pub async fn add_control_resource(
  control_id: String,
  resource_remark: String,
  resource_code: String,
  resource_url: String,
  resource_method_type: String,
  resource_query_flag: String,
  resource_clip_rules: Vec<ResourceClipRuleVO>,
  authorizations: Vec<String>,
  last_modified_user_id: String,
  create_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}

#[tauri::command]
pub async fn edit_control_resource(
  control_id: String,
  resource_id: String,
  resource_remark: String,
  resource_code: String,
  resource_url: String,
  resource_method_type: String,
  resource_query_flag: String,
  resource_clip_rules: Vec<ResourceClipRuleVO>,
  authorizations: Vec<String>,
  last_modified_user_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn delete_control_resource(
  resource_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn get_control_resource_list(
  control_id: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<Vec<ControlResourceVO>, String> {
  Ok(vec![])
}
