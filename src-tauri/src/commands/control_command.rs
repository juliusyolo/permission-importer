use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::control::{ControlPointCondition, ControlPointModel, ControlPointVO, ControlResourceModel, ControlResourceVO, ResourceAuthorizationModel, ResourceClipRuleVO};

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
  if current_page == 0 || page_size == 0 {
    return Err(format!("分页参数不正确"));
  }
  let mut pagination_result = PaginationResult::<ControlPointVO>::default();
  pagination_result.current_page = current_page;
  pagination_result.page_size = page_size;
  let offset = (current_page - 1) * page_size;
  let mut control_point_list_result: Result<Vec<ControlPointModel>, Error>;
  if condition.control_name.is_some() && condition.control_code.is_some() && condition.control_status.is_some() {
    control_point_list_result = sqlx::query_as!(
        ControlPointModel,
        r#"SELECT
        id,
        control_id,
        function_id,
        control_code,
        control_name,
        control_status,
        last_modified_user_id,
        create_time,
        update_time
        from bos_control_point where function_id = ? and control_name like ? and control_code like ? and control_status = ? ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.function_id,
        format!("{}%%",condition.control_name.unwrap()),
        format!("{}%%",condition.control_code.unwrap()),
        condition.control_status.unwrap(),
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else if condition.control_name.is_some() && condition.control_code.is_some() && condition.control_status.is_none() {
    control_point_list_result = sqlx::query_as!(
        ControlPointModel,
        r#"SELECT
        id,
        control_id,
        function_id,
        control_code,
        control_name,
        control_status,
        last_modified_user_id,
        create_time,
        update_time
        from bos_control_point where function_id = ? and control_name like ? and control_code like ? ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.function_id,
        format!("{}%%",condition.control_name.unwrap()),
        format!("{}%%",condition.control_code.unwrap()),
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else if condition.control_name.is_some() && condition.control_code.is_none() && condition.control_status.is_some() {
    control_point_list_result = sqlx::query_as!(
        ControlPointModel,
        r#"SELECT
        id,
        control_id,
        function_id,
        control_code,
        control_name,
        control_status,
        last_modified_user_id,
        create_time,
        update_time
        from bos_control_point where function_id = ? and control_name like ? and control_status = ? ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.function_id,
        format!("{}%%",condition.control_name.unwrap()),
        condition.control_status.unwrap(),
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else if condition.control_name.is_none() && condition.control_code.is_some() && condition.control_status.is_some() {
    control_point_list_result = sqlx::query_as!(
        ControlPointModel,
        r#"SELECT
        id,
        control_id,
        function_id,
        control_code,
        control_name,
        control_status,
        last_modified_user_id,
        create_time,
        update_time
        from bos_control_point where function_id = ? and control_code like ? and control_status = ? ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.function_id,
        format!("{}%%",condition.control_code.unwrap()),
        condition.control_status.unwrap(),
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else if condition.control_name.is_none() && condition.control_code.is_none() && condition.control_status.is_some() {
    control_point_list_result = sqlx::query_as!(
        ControlPointModel,
        r#"SELECT
        id,
        control_id,
        function_id,
        control_code,
        control_name,
        control_status,
        last_modified_user_id,
        create_time,
        update_time
        from bos_control_point where function_id = ? and control_status = ? ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.function_id,
        condition.control_status.unwrap(),
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else if condition.control_name.is_some() && condition.control_code.is_none() && condition.control_status.is_none() {
    control_point_list_result = sqlx::query_as!(
        ControlPointModel,
        r#"SELECT
        id,
        control_id,
        function_id,
        control_code,
        control_name,
        control_status,
        last_modified_user_id,
        create_time,
        update_time
        from bos_control_point where function_id = ? and control_name like ? ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.function_id,
        format!("{}%%",condition.control_name.unwrap()),
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else if condition.control_name.is_none() && condition.control_code.is_some() && condition.control_status.is_none() {
    control_point_list_result = sqlx::query_as!(
        ControlPointModel,
        r#"SELECT
        id,
        control_id,
        function_id,
        control_code,
        control_name,
        control_status,
        last_modified_user_id,
        create_time,
        update_time
        from bos_control_point where function_id = ? and control_code like ? ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.function_id,
        format!("{}%%",condition.control_code.unwrap()),
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  } else {
    control_point_list_result = sqlx::query_as!(
        ControlPointModel,
        r#"SELECT
        id,
        control_id,
        function_id,
        control_code,
        control_name,
        control_status,
        last_modified_user_id,
        create_time,
        update_time
        from bos_control_point where function_id = ? ORDER BY UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.function_id,
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  }
  let control_point_list = control_point_list_result.map_err(|err| format!("查询控制点分页数据异常: {}", err))?;
  let mut control_point_vo_list = Vec::<ControlPointVO>::new();
  for control_point_model in control_point_list {
    let mut control_point_vo = ControlPointVO::from(control_point_model.clone());
    control_point_vo.resources = get_control_resource_list(control_point_model.control_id.clone(), state.clone()).await?;
    control_point_vo_list.push(control_point_vo);
  }
  pagination_result.data_list = control_point_vo_list;
  let total_result: Result<i64, Error> = sqlx::query_scalar!(r#"SELECT count(*) FROM bos_control_point where function_id = ?"#,condition.function_id).fetch_one(&state.pool).await;
  let total = total_result.map_err(|err| format!("查询控制点总数异常: {}", err))?;
  pagination_result.total = total;
  Ok(pagination_result)
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
  let control_resource_list_result: Result<Vec<ControlResourceModel>, Error> = sqlx::query_as!(
        ControlResourceModel,
        r#"SELECT
        id,
        control_id,
        resource_id,
        resource_remark,
        resource_code,
        resource_url,
        resource_method_type,
        resource_query_flag,
        resource_clip_rule,
        last_modified_user_id,
        create_time,
        update_time
        from bos_control_resource where CONTROL_ID = ?"#,
        control_id,
      ).fetch_all(&state.pool)
    .await;
  let control_resource_list = control_resource_list_result.map_err(|err| format!("查询资源点数据异常: {}", err))?;
  let mut control_resource_vo_list = Vec::<ControlResourceVO>::new();
  for control_resource_model in control_resource_list {
    let mut control_resource_vo = ControlResourceVO::from(control_resource_model.clone());
    let resource_authorization_list = sqlx::query_scalar!(
        r#"select
        authorization_type
        from bos_resource_authorization where RESOURCE_ID = ?"#,
        control_resource_model.resource_id.clone(),
      ).fetch_all(&state.pool)
      .await.map_err(|err| format!("查询数据权限点数据异常: {}", err))?;
    control_resource_vo.authorizations = resource_authorization_list;
    control_resource_vo_list.push(control_resource_vo);
  }
  Ok(control_resource_vo_list)
}
