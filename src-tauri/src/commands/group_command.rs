use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::group::{GroupCondition, GroupModel, GroupUserCondition, GroupVO};
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
  if current_page == 0 || page_size == 0 {
    return Err(format!("分页参数不正确"));
  }
  let mut pagination_result = PaginationResult::<GroupVO>::default();
  pagination_result.current_page = current_page;
  pagination_result.page_size = page_size;
  let offset = (current_page - 1) * page_size;
  let mut group_list_result: Result<Vec<GroupModel>, Error>;
  if condition.group_name.is_some() && condition.group_code.is_some() && condition.group_status.is_some() {
    group_list_result = sqlx::query_as!(
        GroupModel,
        r#"SELECT
        bg.id as id,
        group_id,
        group_code,
        group_name,
        group_remark,
        group_status,
        bg.last_modified_user_id as last_modified_user_id,
        bg.create_time as create_time,
        bg.update_time as update_time
        FROM bos_group bg left join bos_system_belong_relation bsbr on bg.GROUP_ID = bsbr.BELONG_RESOURCE_ID where group_name = ? and group_code = ? and group_status = ? and bsbr.SYSTEM_ID = ? and bsbr.BELONG_RESOURCE_TYPE = '5' ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.group_name.unwrap(),
        condition.group_code.unwrap(),
        condition.group_status.unwrap(),
        condition.system_id,
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else if condition.group_name.is_some() && condition.group_code.is_some() && condition.group_status.is_none() {
    group_list_result = sqlx::query_as!(
        GroupModel,
        r#"SELECT
        bg.id as id,
        group_id,
        group_code,
        group_name,
        group_remark,
        group_status,
        bg.last_modified_user_id as last_modified_user_id,
        bg.create_time as create_time,
        bg.update_time as update_time
        FROM bos_group bg left join bos_system_belong_relation bsbr on bg.GROUP_ID = bsbr.BELONG_RESOURCE_ID where group_name = ? and group_code = ? and group_status = ? and bsbr.SYSTEM_ID = ? and bsbr.BELONG_RESOURCE_TYPE = '5' ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.group_name.unwrap(),
        condition.group_code.unwrap(),
        condition.group_status.unwrap(),
        condition.system_id,
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else if condition.group_name.is_some() && condition.group_code.is_none() && condition.group_status.is_some() {
    group_list_result = sqlx::query_as!(
        GroupModel,
        r#"SELECT
        bg.id as id,
        group_id,
        group_code,
        group_name,
        group_remark,
        group_status,
        bg.last_modified_user_id as last_modified_user_id,
        bg.create_time as create_time,
        bg.update_time as update_time
        FROM bos_group bg left join bos_system_belong_relation bsbr on bg.GROUP_ID = bsbr.BELONG_RESOURCE_ID where group_name = ? and group_code = ? and group_status = ? and bsbr.SYSTEM_ID = ? and bsbr.BELONG_RESOURCE_TYPE = '5' ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.group_name.unwrap(),
        condition.group_code.unwrap(),
        condition.group_status.unwrap(),
        condition.system_id,
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else if condition.group_name.is_none() && condition.group_code.is_some() && condition.group_status.is_some() {
    group_list_result = sqlx::query_as!(
        GroupModel,
        r#"SELECT
        bg.id as id,
        group_id,
        group_code,
        group_name,
        group_remark,
        group_status,
        bg.last_modified_user_id as last_modified_user_id,
        bg.create_time as create_time,
        bg.update_time as update_time
        FROM bos_group bg left join bos_system_belong_relation bsbr on bg.GROUP_ID = bsbr.BELONG_RESOURCE_ID where group_name = ? and group_code = ? and group_status = ? and bsbr.SYSTEM_ID = ? and bsbr.BELONG_RESOURCE_TYPE = '5' ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.group_name.unwrap(),
        condition.group_code.unwrap(),
        condition.group_status.unwrap(),
        condition.system_id,
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else if condition.group_name.is_none() && condition.group_code.is_none() && condition.group_status.is_some() {
    group_list_result = sqlx::query_as!(
        GroupModel,
        r#"SELECT
        bg.id as id,
        group_id,
        group_code,
        group_name,
        group_remark,
        group_status,
        bg.last_modified_user_id as last_modified_user_id,
        bg.create_time as create_time,
        bg.update_time as update_time
        FROM bos_group bg left join bos_system_belong_relation bsbr on bg.GROUP_ID = bsbr.BELONG_RESOURCE_ID where group_name = ? and group_code = ? and group_status = ? and bsbr.SYSTEM_ID = ? and bsbr.BELONG_RESOURCE_TYPE = '5' ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.group_name.unwrap(),
        condition.group_code.unwrap(),
        condition.group_status.unwrap(),
        condition.system_id,
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else if condition.group_name.is_some() && condition.group_code.is_none() && condition.group_status.is_none() {
    group_list_result = sqlx::query_as!(
        GroupModel,
        r#"SELECT
        bg.id as id,
        group_id,
        group_code,
        group_name,
        group_remark,
        group_status,
        bg.last_modified_user_id as last_modified_user_id,
        bg.create_time as create_time,
        bg.update_time as update_time
        FROM bos_group bg left join bos_system_belong_relation bsbr on bg.GROUP_ID = bsbr.BELONG_RESOURCE_ID where group_name = ? and group_code = ? and group_status = ? and bsbr.SYSTEM_ID = ? and bsbr.BELONG_RESOURCE_TYPE = '5' ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.group_name.unwrap(),
        condition.group_code.unwrap(),
        condition.group_status.unwrap(),
        condition.system_id,
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else if condition.group_name.is_none() && condition.group_code.is_some() && condition.group_status.is_none() {
    group_list_result = sqlx::query_as!(
        GroupModel,
        r#"SELECT
        bg.id as id,
        group_id,
        group_code,
        group_name,
        group_remark,
        group_status,
        bg.last_modified_user_id as last_modified_user_id,
        bg.create_time as create_time,
        bg.update_time as update_time
        FROM bos_group bg left join bos_system_belong_relation bsbr on bg.GROUP_ID = bsbr.BELONG_RESOURCE_ID where group_name = ? and group_code = ? and group_status = ? and bsbr.SYSTEM_ID = ? and bsbr.BELONG_RESOURCE_TYPE = '5' ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.group_name.unwrap(),
        condition.group_code.unwrap(),
        condition.group_status.unwrap(),
        condition.system_id,
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else {
    group_list_result = sqlx::query_as!(
        GroupModel,
        r#"SELECT
        bg.id as id,
        group_id,
        group_code,
        group_name,
        group_remark,
        group_status,
        bg.last_modified_user_id as last_modified_user_id,
        bg.create_time as create_time,
        bg.update_time as update_time
        FROM bos_group bg left join bos_system_belong_relation bsbr on bg.GROUP_ID = bsbr.BELONG_RESOURCE_ID where group_name = ? and group_code = ? and group_status = ? and bsbr.SYSTEM_ID = ? and bsbr.BELONG_RESOURCE_TYPE = '5' ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.group_name.unwrap(),
        condition.group_code.unwrap(),
        condition.group_status.unwrap(),
        condition.system_id,
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  }
  let control_point_list = group_list_result.map_err(|err| format!("查询岗位分页数据异常: {}", err))?;
  pagination_result.data_list = control_point_list.into_iter().map(GroupVO::from).collect();
  let total_result: Result<i64, Error> = sqlx::query_scalar!(r#"SELECT count(*) FROM bos_group bg left join bos_system_belong_relation bsbr on bg.GROUP_ID = bsbr.BELONG_RESOURCE_ID where bsbr.SYSTEM_ID = ? and bsbr.BELONG_RESOURCE_TYPE = '5'"#,condition.system_id).fetch_one(&state.pool).await;
  let total = total_result.map_err(|err| format!("查询岗位总数异常: {}", err))?;
  pagination_result.total = total;
  Ok(pagination_result)
}
