use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;

#[derive(Serialize, Deserialize, Default)]
pub struct ControlPointModel {
  pub id: u64,
  #[serde(rename = "controlId")]
  pub control_id: String,
  #[serde(rename = "functionId")]
  pub function_id: String,
  #[serde(rename = "controlCode")]
  pub control_code: String,
  #[serde(rename = "controlName")]
  pub control_name: String,
  #[serde(rename = "controlStatus")]
  pub control_status: String,
  pub resources: Vec<ControlResourceModel>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ControlResourceModel {
  pub id: u64,
  #[serde(rename = "controlId")]
  pub control_id: String,
  #[serde(rename = "resourceId")]
  pub resource_id: String,
  #[serde(rename = "resourceRemark")]
  pub resource_remark: String,
  #[serde(rename = "resourceCode")]
  pub resource_code: String,
  #[serde(rename = "resourceUrl")]
  pub resource_url: String,
  #[serde(rename = "resourceMethodType")]
  pub resource_method_type: String,
  #[serde(rename = "resourceQueryFlag")]
  pub resource_query_flag: String,
  #[serde(rename = "resourceClipRule")]
  pub resource_clip_rule: String,
  pub authorizations: Vec<ResourceAuthorizationModel>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ResourceAuthorizationModel {
  pub id: u64,
  #[serde(rename = "authorizationId")]
  pub authorization_id: String,
  #[serde(rename = "authorizationName")]
  pub authorization_name: String,
  #[serde(rename = "resourceId")]
  pub resource_id: String,
  #[serde(rename = "authorizationStatus")]
  pub authorization_status: String,
  #[serde(rename = "authorizationType")]
  pub authorization_type: String,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ControlPointCondition {
  #[serde(rename = "functionId")]
  pub function_id: String,
  #[serde(rename = "controlCode")]
  pub control_code: String,
  #[serde(rename = "controlName")]
  pub control_name: String,
  #[serde(rename = "controlStatus")]
  pub control_status: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ControlPointVO {}

#[derive(Serialize, Deserialize, Default)]
pub struct ResourceClipRuleVO {}

#[derive(Serialize, Deserialize, Default)]
pub struct ControlResourceVO {}
