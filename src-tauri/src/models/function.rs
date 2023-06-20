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
  pub function_remark: String,
  #[serde(rename = "moduleId")]
  pub module_id: String,
  #[serde(rename = "moduleName")]
  pub module_name: String,
  #[serde(rename = "functionUrl")]
  pub function_url: String,
  #[serde(rename = "functionIcon")]
  pub function_icon: String,
  #[serde(rename = "functionOrder")]
  pub function_order: String,
  #[serde(rename = "functionStatus")]
  pub function_status: String,
  #[serde(rename = "controlPoints")]
  control_points: Vec<ControlPointModel>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}


