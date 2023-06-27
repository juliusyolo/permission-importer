use serde::{Deserialize, Serialize};
use sqlx::encode::IsNull::No;
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::menu::{MenuModel, MenuTreeVO};

#[tauri::command]
pub async fn get_menu_list(up_menu_id: Option<String>, menu_code: Option<String>, menu_name: Option<String>, menu_status: Option<String>, system_id: String, state: State<'_, DatabaseConnectionPool>) -> Result<Vec<MenuTreeVO>, String> {
  Ok(vec![])
}


#[tauri::command]
pub async fn get_menu_tree_list(up_menu_id: Option<String>, system_id: String, state: State<'_, DatabaseConnectionPool>) -> Result<Vec<MenuTreeVO>, String> {
  let mut query_sql: String;
  let mut menu_tree: Vec<MenuTreeVO> = vec![];
  let menu_list_result: Result<Vec<MenuModel>, Error>;
  if up_menu_id.is_none() {
    menu_list_result = sqlx::query_as!(
        MenuModel,
        r#"SELECT
        bm.id as id,
        menu_id,
        up_menu_id,
        menu_name,
        menu_order,
        menu_status,
        menu_remark,
        menu_icon,
        function_id,
        i18n_key,
        menu_path,
        menu_component,
        '' as up_menu_name,
        '' as function_name,
        0 as sub_menu_count,
        bm.last_modified_user_id as last_modified_user_id,
        bm.create_time as create_time,
        bm.update_time as update_time
        from bos_menu bm left join bos_system_belong_relation bsbr on bm.MENU_ID = bsbr.BELONG_RESOURCE_ID where bsbr.SYSTEM_ID = ? and bsbr.BELONG_RESOURCE_TYPE = '2' and up_menu_id is null order by bm.UPDATE_TIME desc"#,
        system_id
      ).fetch_all(&state.pool)
      .await;
  } else {
    menu_list_result = sqlx::query_as!(
        MenuModel,
        r#"SELECT
        bm.id as id,
        menu_id,
        up_menu_id,
        menu_name,
        menu_order,
        menu_status,
        menu_remark,
        menu_icon,
        function_id,
        i18n_key,
        menu_path,
        menu_component,
        '' as up_menu_name,
        '' as function_name,
        0 as sub_menu_count,
        bm.last_modified_user_id as last_modified_user_id,
        bm.create_time as create_time,
        bm.update_time as update_time
        from bos_menu bm left join bos_system_belong_relation bsbr on bm.MENU_ID = bsbr.BELONG_RESOURCE_ID where bsbr.SYSTEM_ID = ? and bsbr.BELONG_RESOURCE_TYPE = '2' and up_menu_id = ? order by bm.UPDATE_TIME desc"#,
        system_id,
        up_menu_id.unwrap()
      ).fetch_all(&state.pool)
      .await;
  }

  let menu_list = menu_list_result.map_err(|err| format!("查询菜单列表异常: {}", err))?;
  let mut all_menu_tree_nodes = Vec::<MenuTreeVO>::new();
  for mut menu_model in menu_list {
    if menu_model.up_menu_id.is_some() {
      let up_menu_name_result: Result<String, Error> = sqlx::query_scalar!(r#"SELECT menu_name FROM bos_menu where menu_id = ?"#,menu_model.up_menu_id.unwrap())
        .fetch_one(&state.pool)
        .await;
      match up_menu_name_result {
        Ok(up_menu_name) => {
          menu_model.up_menu_name = up_menu_name;
        }
        Err(_) => {
          return Err(format!("查询上级菜单名常"));
        }
      }
    }
    if menu_model.function_id.is_some() {
      let up_menu_name = sqlx::query_scalar!(r#"SELECT function_name FROM bos_function where function_id = ?"#,menu_model.function_id.unwrap())
        .fetch_one(&state.pool)
        .await.map_err(|err| format!("查询上级菜单名常: {}", err))?;
      menu_model.up_menu_name = up_menu_name;
    }
    let sub_menu_count_result = sqlx::query_scalar!(r#"SELECT count(*) FROM bos_menu where up_menu_id = ?"#,menu_model.menu_id)
      .fetch_one(&state.pool)
      .await.map_err(|err| format!("查询子菜单数异常: {}", err))?;
    menu_model.sub_menu_count = sub_menu_count;
    all_menu_tree_nodes.push(MenuTreeVO::from(menu_model));
  }
  menu_tree = recursive_generate_menu_tree(&all_menu_tree_nodes, None);

  Ok(menu_tree)
}

fn recursive_generate_menu_tree(menu_list: &Vec<MenuTreeVO>, up_menu_id: Option<String>) -> Vec<MenuTreeVO> {
  let mut menu_nodes: Vec<MenuTreeVO> = Vec::new();

  if up_menu_id.is_none() {
    menu_nodes = menu_list
      .into_iter()
      .filter(|menu_node| menu_node.up_menu_id.is_none())
      .collect();
  } else {
    menu_nodes = menu_list
      .into_iter()
      .filter(|menu_node| {
        menu_node.up_menu_id.is_some() && menu_node.up_menu_id.as_ref().unwrap() == up_menu_id.as_ref().unwrap()
      })
      .collect();
  }
  for mut menu_node in menu_nodes.iter_mut() {
    menu_node.children = recursive_generate_menu_tree(menu_list, Some(menu_node.menu_id.clone()));
  }
  menu_nodes
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
  create_time: String,
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
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn delete_menu(menu_id: String, system_id: String, update_time: String, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn disable_menu(menu_id: String, system_id: String, update_time: String, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn enable_menu(menu_id: String, system_id: String, update_time: String, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}
