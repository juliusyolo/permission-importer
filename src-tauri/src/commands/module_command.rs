use serde::{Deserialize, Serialize};
use sqlx::Error;
use tauri::State;

use crate::common::types::{DatabaseConnectionPool, PaginationResult};
use crate::models::module::{ModuleCondition, ModuleModel, ModuleVO};

#[tauri::command]
pub async fn add_module(
  module_id: String,
  module_name: String,
  module_remark: String,
  module_status: String,
  last_modified_user_id: String,
  system_id: String,
  create_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}

#[tauri::command]
pub async fn edit_module(
  module_id: String,
  module_name: String,
  module_remark: String,
  module_status: String,
  last_modified_user_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn delete_module(
  module_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn disable_module(
  module_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn enable_module(
  module_id: String,
  system_id: String,
  update_time: String,
  state: State<'_, DatabaseConnectionPool>) -> Result<u64, String> {
  Ok(1)
}


#[tauri::command]
pub async fn get_module_list_by_pagination(
  current_page: i64,
  page_size: i64,
  condition: ModuleCondition,
  state: State<'_, DatabaseConnectionPool>) -> Result<PaginationResult<ModuleVO>, String> {
  if current_page == 0 || page_size == 0 {
    return Err(format!("分页参数不正确"));
  }
  let mut pagination_result = PaginationResult::<ModuleVO>::default();
  pagination_result.current_page = current_page;
  pagination_result.page_size = page_size;
  let offset = (current_page - 1) * page_size;
  let mut module_list_result:Result<Vec<ModuleModel>,Error>;
  if condition.module_name.is_some()||condition.module_status.is_some() {
    if condition.module_name.is_some()&&condition.module_status.is_some() {
      module_list_result = sqlx::query_as!(
        ModuleModel,
        r#"SELECT
        bm.id as id,
        module_id,
        module_name,
        module_remark,
        module_status,
        bm.last_modified_user_id as last_modified_user_id,
        bm.create_time as create_time,
        bm.update_time as update_time
        from bos_module bm left join bos_system_belong_relation bsbr on bm.MODULE_ID = bsbr.BELONG_RESOURCE_ID where bsbr.SYSTEM_ID = ? and bm.module_name = ? and bm.module_status = ? and bsbr.BELONG_RESOURCE_TYPE = '3' ORDER BY bm.UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.system_id,
        condition.module_name.unwrap(),
        condition.module_status.unwrap(),
        page_size,
        offset
        ).fetch_all(&state.pool)
        .await;
    }else if condition.module_name.is_some() && condition.module_status.is_none() {
      module_list_result = sqlx::query_as!(
        ModuleModel,
        r#"SELECT
        bm.id as id,
        module_id,
        module_name,
        module_remark,
        module_status,
        bm.last_modified_user_id as last_modified_user_id,
        bm.create_time as create_time,
        bm.update_time as update_time
        from bos_module bm left join bos_system_belong_relation bsbr on bm.MODULE_ID = bsbr.BELONG_RESOURCE_ID where bsbr.SYSTEM_ID = ? and bm.module_name = ? and bsbr.BELONG_RESOURCE_TYPE = '3' ORDER BY bm.UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.system_id,
        condition.module_name.unwrap(),
        page_size,
        offset
        ).fetch_all(&state.pool)
        .await;
    }else {  }
  }else {
    module_list_result = sqlx::query_as!(
        ModuleModel,
        r#"SELECT
        bm.id as id,
        module_id,
        module_name,
        module_remark,
        module_status,
        bm.last_modified_user_id as last_modified_user_id,
        bm.create_time as create_time,
        bm.update_time as update_time
        from bos_module bm left join bos_system_belong_relation bsbr on bm.MODULE_ID = bsbr.BELONG_RESOURCE_ID where bsbr.SYSTEM_ID = ? and bm.module_status = ? and bsbr.BELONG_RESOURCE_TYPE = '3' ORDER BY bm.UPDATE_TIME desc LIMIT ? OFFSET ?"#,
        condition.system_id,
        condition.module_status.unwrap(),
        page_size,
        offset
      ).fetch_all(&state.pool)
      .await;
  }
  let module_list = module_list_result.map_err(|err| format!("查询分页用户数据异常: {}", err))?;
  pagination_result.data_list = module_list.into_iter().map(ModuleVO::from).collect();
  let total_result: Result<i64, Error> = sqlx::query_scalar!(r#"SELECT count(*) FROM bos_module"#).fetch_one(&state.pool).await;
  let total = total_result.map_err(|err| format!("查询模块总数异常: {}", err))?;
  pagination_result.total = total;
  Ok(pagination_result)
}
