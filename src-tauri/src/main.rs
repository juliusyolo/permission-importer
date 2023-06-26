// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::mysql::MySqlPool;

use commands::authorization_command::{authorization_menu_record_save, authorization_permission_record_save, get_authorized_menu, get_authorized_permission, get_menu_tree_result, get_permission_tree_result};
use commands::control_command::{add_control_point, add_control_resource, delete_control_point, delete_control_resource, disable_control_point, edit_control_point, edit_control_resource, enable_control_point, get_control_point_list_by_pagination, get_control_resource_list};
use commands::function_command::{add_function, delete_function, disable_function, edit_function, enable_function, get_function_list_by_pagination, get_module_function_list_by_pagination};
use commands::group_command::{add_group, delete_group, disable_group, edit_group, enable_group, get_group_list_by_pagination, get_group_user_list_by_pagination};
use commands::menu_command::{add_menu, delete_menu, disable_menu, edit_menu, enable_menu, get_menu_list, get_menu_tree_list};
use commands::module_command::{add_module, delete_module, disable_module, edit_module, enable_module, get_module_list_by_pagination};
use commands::organization_command::{add_organization, add_organization_group, add_organization_role, delete_organization, disable_organization, edit_organization, enable_organization, get_all_organization_list, get_organization_group_list, get_organization_list, get_organization_role_list, get_organization_tree_list, get_organization_user_list};
use commands::role_command::{add_role, delete_role, disable_role, edit_role, enable_role, get_role_list_by_pagination, get_role_user_list_by_pagination};
use commands::user_command::{add_user, add_user_group, add_user_role, delete_user, disable_user, edit_user, enable_user, get_user_group_list, get_user_list_by_pagination, get_user_role_list};
use commands::version_control_command::{get_version_control_list_by_pagination, save_version_control};
use common::types::DatabaseConnectionPool;

mod models;
mod common;
mod commands;


#[tokio::main]
async fn main() {
  let pool = MySqlPool::connect_lazy("mysql://root:527901748@localhost:3306/permission_control").expect("");
  tauri::Builder::default()
    .manage(DatabaseConnectionPool { pool })
    .invoke_handler(tauri::generate_handler![
      get_authorized_menu,get_authorized_permission,authorization_menu_record_save,authorization_permission_record_save,get_menu_tree_result,get_permission_tree_result,
      add_control_point,edit_control_point,delete_control_point,disable_control_point,enable_control_point,get_control_point_list_by_pagination,add_control_resource,edit_control_resource,delete_control_resource,get_control_resource_list,
      add_function,edit_function,delete_function,disable_function,enable_function,get_function_list_by_pagination,get_module_function_list_by_pagination,
      add_group, delete_group, disable_group, edit_group, enable_group, get_group_list_by_pagination, get_group_user_list_by_pagination,
      add_menu, delete_menu, disable_menu, edit_menu, enable_menu, get_menu_list, get_menu_tree_list,
      add_module,edit_module,delete_module,disable_module,enable_module,get_module_list_by_pagination,
      add_organization,edit_organization,delete_organization,disable_organization,enable_organization,get_organization_list,get_all_organization_list,get_organization_tree_list,add_organization_role,get_organization_role_list,add_organization_group,get_organization_group_list,get_organization_user_list,
      add_role, delete_role, disable_role, edit_role, enable_role, get_role_list_by_pagination, get_role_user_list_by_pagination,
      add_user, add_user_group, add_user_role, delete_user, disable_user, edit_user, enable_user, get_user_group_list, get_user_list_by_pagination, get_user_role_list,
      get_version_control_list_by_pagination, save_version_control,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
