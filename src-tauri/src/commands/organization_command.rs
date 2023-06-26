use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::organization::{OrganizationGroupPair, OrganizationRolePair, OrganizationTreeVO, OrganizationUserVO};

#[tauri::command]
pub async fn add_organization(
  up_organization_id: String,
  organization_code: String,
  organization_name: String,
  organization_level: String,
  organization_status: String,
  last_modified_user_id: String,
  system_id: String,
  create_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}

#[tauri::command]
pub async fn edit_organization(
  organization_id: String,
  up_organization_id: String,
  organization_code: String,
  organization_name: String,
  organization_level: String,
  organization_status: String,
  last_modified_user_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn delete_organization(
  organization_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn disable_organization(
  organization_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn enable_organization(
  organization_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}

#[tauri::command]
pub async fn get_organization_list(
  up_organization_id: String,
  organization_code: String,
  organization_name: String,
  organization_status: String,
  system_id: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<Vec<OrganizationTreeVO>, String> {
  Ok(vec![])
}


#[tauri::command]
pub async fn get_all_organization_list(
  up_organization_id: String,
  organization_code: Option<String>,
  organization_name: Option<String>,
  organization_status: Option<String>,
  system_id: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<Vec<OrganizationTreeVO>, String> {
  Ok(vec![])
}


#[tauri::command]
pub async fn get_organization_tree_list(
  up_organization_id: String,
  organization_code: Option<String>,
  organization_name: Option<String>,
  organization_status: Option<String>,
  system_id: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<Vec<OrganizationTreeVO>, String> {
  Ok(vec![])
}

#[tauri::command]
pub async fn add_organization_role(
  organization_id: String,
  authorized_role_ids: Vec<String>,
  last_modified_user_id: String,
  system_id: String,
  create_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn get_organization_role_list(
  organization_id: String,
  system_id: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<OrganizationRolePair, String> {
  Ok(OrganizationRolePair::default())
}

#[tauri::command]
pub async fn add_organization_group(
  organization_id: String,
  authorized_group_ids: Vec<String>,
  last_modified_user_id: String,
  system_id: String,
  create_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn get_organization_group_list(
  organization_id: String,
  system_id: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<OrganizationGroupPair, String> {
  Ok(OrganizationGroupPair::default())
}

#[tauri::command]
pub async fn get_organization_user_list(
  organization_id: String,
  system_id: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<Vec<OrganizationUserVO>, String> {
  Ok(vec![])
}
