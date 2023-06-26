use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::user::{UserGroupPair, UserCondition, UserModel, UserRolePair, UserVO};

#[tauri::command]
pub async fn add_user(
  user_code: String,
  user_name: String,
  user_gender: String,
  user_status: String,
  last_modified_user_id: String,
  organization_ids: Vec<String>,
  create_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  println!("{}", last_modified_user_id);
  Ok(1)
}


#[tauri::command]
pub async fn edit_user(
  user_id: String,
  user_code: String,
  user_name: String,
  user_gender: String,
  user_status: String,
  last_modified_user_id: String,
  organization_ids: Vec<String>,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn delete_user(user_id: String, system_id: String, update_time: String, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn disable_user(user_id: String, system_id: String, update_time: String, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn enable_user(user_id: String, system_id: String, update_time: String, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn get_user_group_list(
  user_id: String,
  organization_id: String,
  system_id: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<UserGroupPair, String> {
  Ok(UserGroupPair::default())
}


#[tauri::command]
pub async fn get_user_role_list(
  user_id: String,
  organization_id: String,
  system_id: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<UserRolePair, String> {
  Ok(UserRolePair::default())
}


#[tauri::command]
pub async fn add_user_role(
  user_id: String,
  authorized_org_role_rel_ids: Vec<String>,
  last_modified_user_id: String,
  system_id: String,
  create_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}

#[tauri::command]
pub async fn get_user_list_by_pagination(
  current_page: i64,
  page_size: i64,
  condition: UserCondition,
  state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<UserVO>, String> {
  if current_page == 0 || page_size == 0 {
    return Err(format!("分页参数不正确"));
  }
  let mut pagination_result = PaginationResult::<UserVO>::default();
  pagination_result.current_page = current_page;
  pagination_result.page_size = page_size;
  let offset = (current_page - 1) * page_size;
  let user_list_result = sqlx::query_as!(
        UserModel,
        r#"SELECT
        id,
        user_id,
        user_name,
        user_code,
        user_gender,
        user_avatar,
        user_type,
        user_status,
        user_token,
        last_modified_user_id,
        create_time,
        update_time
        FROM bos_user ORDER BY id desc LIMIT ? OFFSET ?"#,
        page_size,
        offset
  )
    .fetch_all(&state.pool)
    .await;
  match user_list_result {
    Ok(user_list) => {
      let mut data_list = Vec::<UserVO>::new();
      for user_model in user_list {
        let organization_ids_result: Result<Vec<Option<String>>, Error> = sqlx::query_scalar!(r#"SELECT organization_id FROM bos_user_organization_relation where user_id = ?"#,user_model.user_id)
          .fetch_all(&state.pool).await;
        match organization_ids_result {
          Ok(organization_ids) => {
            let mut user_vo: UserVO = UserVO::from(user_model);
            user_vo.organizations = organization_ids;
            data_list.push(user_vo);
          }
          Err(_error) => {
            return Err(format!("查询用户机构信息异常"));
          }
        }
      }
      pagination_result.data_list = data_list;
    }
    Err(_error) => {
      return Err(format!("查询分页用户数据异常"));
    }
  }
  let total_result: Result<i64, Error> = sqlx::query_scalar(r#"SELECT count(*) FROM bos_user"#)
    .fetch_one(&state.pool)
    .await;
  match total_result {
    Ok(total) => {
      pagination_result.total = total;
    }
    Err(_error) => {
      return Err(format!("查询总数异常"));
    }
  }
  Ok(pagination_result)
}


#[tauri::command]
pub async fn add_user_group(
  user_id: String,
  authorized_org_group_rel_ids: Vec<String>,
  last_modified_user_id: String,
  system_id: String,
  create_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}
