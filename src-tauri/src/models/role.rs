use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;

#[derive(Serialize, Deserialize, Default)]
pub struct RoleModel {
  pub id: u64,
  #[serde(rename = "roleId")]
  pub role_id: String,
  #[serde(rename = "roleCode")]
  pub role_code: String,
  #[serde(rename = "roleName")]
  pub role_name: String,
  #[serde(rename = "roleFlag")]
  pub role_flag: String,
  #[serde(rename = "roleStatus")]
  pub role_status: String,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct RoleVO{
  #[serde(rename = "roleId")]
  pub role_id: String,
  #[serde(rename = "roleCode")]
  pub role_code: String,
  #[serde(rename = "roleName")]
  pub role_name: String,
  #[serde(rename = "roleRemark")]
  pub role_remark: String,
  #[serde(rename = "roleStatus")]
  pub role_status: String,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct RoleCondition{
  #[serde(rename = "roleCode")]
  pub role_code:Option<String>,
  #[serde(rename = "roleName")]
  pub role_name:Option<String>,
  #[serde(rename = "roleStatus")]
  pub role_status:Option<String>,
  #[serde(rename = "systemId")]
  pub system_id:String
}

#[derive(Serialize, Deserialize, Default)]
pub struct RoleUserCondition{
  #[serde(rename = "roleId")]
  role_id:String,
  #[serde(rename = "systemId")]
  pub system_id:String
}
