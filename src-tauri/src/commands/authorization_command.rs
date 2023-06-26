use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult, TreeResult};
use crate::models::authorization::{AuthorizationResource, AuthorizedPair};
use crate::models::menu::MenuVO;

#[tauri::command]
pub async fn get_authorized_menu(
  authorization_type: String,
  authorization_key: String,
  last_modified_user_id: String,
  complete_authorization_resources: Vec<AuthorizationResource>,
  half_authorization_resources: Vec<AuthorizationResource>,
  system_id: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<AuthorizedPair<u64>, String> {
  Ok(AuthorizedPair::default())
}


#[tauri::command]
pub async fn get_authorized_permission(
  authorization_type: String,
  authorization_key: String,
  last_modified_user_id: String,
  complete_authorization_resources: Vec<AuthorizationResource>,
  half_authorization_resources: Vec<AuthorizationResource>,
  system_id: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<AuthorizedPair<u64>, String> {
  Ok(AuthorizedPair::default())
}


#[tauri::command]
pub async fn authorization_menu_record_save(
  authorization_type: String,
  authorization_key: String,
  last_modified_user_id: String,
  complete_authorization_resources: Vec<AuthorizationResource>,
  half_authorization_resources: Vec<AuthorizationResource>,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn authorization_permission_record_save(
  authorization_type: String,
  authorization_key: String,
  last_modified_user_id: String,
  complete_authorization_resources: Vec<AuthorizationResource>,
  half_authorization_resources: Vec<AuthorizationResource>,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn get_menu_tree_result(
  system_id: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<Vec<TreeResult<MenuVO>>, String> {
  Ok(vec![])
}


#[tauri::command]
pub async fn get_permission_tree_result(
  system_id: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<Vec<TreeResult<u64>>, String> {
  Ok(vec![])
}
