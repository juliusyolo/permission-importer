use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;
use crate::models::control::ControlPointModel;

#[derive(Serialize, Deserialize, Default)]
pub struct FunctionModel {
  pub id: u64,
  #[serde(rename = "functionId")]
  pub function_id: String,
  #[serde(rename = "functionName")]
  pub function_name: String,
  #[serde(rename = "functionRemark")]
  pub function_remark: Option<String>,
  #[serde(rename = "moduleId")]
  pub module_id: String,
  #[serde(rename = "functionOrder")]
  pub function_order: Option<i32>,
  #[serde(rename = "functionStatus")]
  pub function_status: Option<String>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct FunctionVO {
  #[serde(rename = "functionId")]
  pub function_id: String,
  #[serde(rename = "functionName")]
  pub function_name: String,
  #[serde(rename = "functionRemark")]
  pub function_remark: Option<String>,
  #[serde(rename = "moduleId")]
  pub module_id: String,
  #[serde(rename = "functionOrder")]
  pub function_order: Option<i32>,
  #[serde(rename = "functionStatus")]
  pub function_status: Option<String>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

impl From<FunctionModel> for FunctionVO {
  fn from(value: FunctionModel) -> Self {
    FunctionVO {
      function_id: value.function_id,
      function_name: value.function_name,
      function_remark: value.function_remark,
      module_id: value.module_id,
      function_order: value.function_order,
      function_status: value.function_status,
      last_modified_user_id: value.last_modified_user_id,
      update_time: value.update_time,
    }
  }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ModuleFunctionCondition {
  #[serde(rename = "moduleName")]
  module_name: String,
  #[serde(rename = "functionName")]
  function_name: String,
  #[serde(rename = "systemId")]
  system_id: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct FunctionCondition {
  #[serde(rename = "moduleId")]
  pub module_id: String,
  #[serde(rename = "functionName")]
  pub function_name: Option<String>,
  #[serde(rename = "functionStatus")]
  pub function_status: Option<String>,
}
