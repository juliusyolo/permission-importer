use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::menu::{MenuModel, MenuTreeVO};

#[tauri::command]
pub async fn get_menu_list(up_menu_id: Option<String>, menu_code: Option<String>, menu_name: Option<String>, menu_status: Option<String>, system_id: String, state: State<'_, DatabaseConnectionPool>) -> Result<Vec<MenuTreeVO>, String> {
  Ok(vec![])
}


#[tauri::command]
pub async fn get_menu_tree_list(up_menu_id: Option<String>, menu_code: Option<String>, menu_name: Option<String>, menu_status: Option<String>, system_id: String, state: State<'_, DatabaseConnectionPool>) -> Result<Vec<MenuTreeVO>, String> {
  Ok(vec![])
}


#[tauri::command]
pub async fn add_menu(
  up_menu_id: String,
  menu_name: String,
  menu_order: u32,
  menu_status: String,
  menu_remark: String,
  menu_icon: String,
  function_id: String,
  i18n_key: String,
  menu_path: String,
  menu_component: String,
  last_modified_user_id: String,
  system_id: String,
  create_time:String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn edit_menu(
  menu_id: String,
  up_menu_id: String,
  menu_name: String,
  menu_order: u32,
  menu_status: String,
  menu_remark: String,
  menu_icon: String,
  function_id: String,
  i18n_key: String,
  menu_path: String,
  menu_component: String,
  last_modified_user_id: String,
  system_id: String,
  update_time:String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn delete_menu(menu_id: String, system_id: String,update_time:String, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn disable_menu(menu_id: String, system_id: String,update_time:String, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn enable_menu(menu_id: String, system_id: String,update_time:String, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}
