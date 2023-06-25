use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::user::UserModel;

#[tauri::command]
pub async fn add_user(user_code: String, user_name: String, user_gender: String, user_status: String, last_modified_user_id: String, organization_ids: Vec<String>, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  println!("{}", last_modified_user_id);
  Ok(1)
}


// #[tauri::command]
// pub async fn edit_user(UserEditRequest request,state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
//   return ResponseWrapper.apply(userService::editUser, request);
// }
//
//
// #[tauri::command]
// pub async fn delete_user(UserIdRequest request, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
//   return ResponseWrapper.apply(userService::deleteUser, request);
// }
//
//
// #[tauri::command]
// pub async fn disable_user(UserIdRequest request, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
//   return ResponseWrapper.apply(userService::disableUser, request);
// }
//
//
// #[tauri::command]
// pub async fn enable_user(UserIdRequest request, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
//   return ResponseWrapper.apply(userService::enableUser, request);
// }
//
//
// public ResponseWrapper<UseGroupPair> getUserGroupList( UserGroupQueryRequest request,state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
// return ResponseWrapper.apply(userService::getUserGroupList, request);
// }
//
//
// public ResponseWrapper<UserRolePair> getUserRoleList( UserRoleQueryRequest request,state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
// return ResponseWrapper.apply(userService::getUserRoleList, request);
// }
//
//
// #[tauri::command]
// pub async fn add_user_role(UserRoleAddRequest request, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
//   return ResponseWrapper.apply(userService::addUserRole, request);
// }
//
//

#[derive(Serialize, Deserialize, Default)]
pub struct UserCondition {
  #[serde(rename = "userCode")]
  user_code: Option<String>,
  #[serde(rename = "userName")]
  user_name: Option<String>,
  #[serde(rename = "userStatus")]
  user_status: Option<String>,
  #[serde(rename = "organizationId")]
  organization_id: String,
}

// #[tauri::command]
// pub async fn get_user_list_by_pagination(current_page: i64, page_size: i64, condition: UserCondition, state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<UserModel>, String> {
//   if current_page == 0 || page_size == 0 {
//     return Err(format!("分页参数不正确"));
//   }
//   let mut pagination_result = PaginationResult::<UserModel>::default();
//   pagination_result.current_page = current_page;
//   pagination_result.page_size = page_size;
//   let offset = (current_page - 1) * page_size;
//   let version_control_list_result = sqlx::query_as!(
//         UserModel,
//         r#"SELECT
//         id,
//         user_id,
//         user_name,
//         user_code,
//         user_gender,
//         user_avatar,
//         user_type,
//         user_status,
//         user_token,
//         last_modified_user_id,
//         create_time,
//         update_time
//          FROM bos_user ORDER BY id desc LIMIT ? OFFSET ?"#,
//         page_size,
//         offset
//     )
//     .fetch_all(&state.pool)
//     .await;
//   match version_control_list_result {
//     Ok(version_control_list) => {
//       pagination_result.data = version_control_list
//     }
//     Err(_error) => {
//       return Err(format!("查询分页数据异常"));
//     }
//   }
//   let total_result: Result<i64, Error> = sqlx::query_scalar(r#"SELECT count(*) FROM bos_user"#)
//     .fetch_one(&state.pool)
//     .await;
//   match total_result {
//     Ok(total) => {
//       pagination_result.total = total;
//     }
//     Err(_error) => {
//       return Err(format!("查询总数异常"));
//     }
//   }
//   Ok(pagination_result)
// }

//
//
// #[tauri::command]
// pub async fn add_user_group(UserGroupAddRequest request, state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
//   return ResponseWrapper.apply(userService::addUserGroup, request);
// }
