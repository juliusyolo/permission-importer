use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;

#[derive(Serialize, Deserialize, Default, Clone)]
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
  pub control_status: Option<String>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ControlResourceModel {
  pub id: u64,
  #[serde(rename = "controlId")]
  pub control_id: Option<String>,
  #[serde(rename = "resourceId")]
  pub resource_id: Option<String>,
  #[serde(rename = "resourceRemark")]
  pub resource_remark: Option<String>,
  #[serde(rename = "resourceCode")]
  pub resource_code: Option<String>,
  #[serde(rename = "resourceUrl")]
  pub resource_url: Option<String>,
  #[serde(rename = "resourceMethodType")]
  pub resource_method_type: Option<String>,
  #[serde(rename = "resourceQueryFlag")]
  pub resource_query_flag: Option<String>,
  #[serde(rename = "resourceClipRule")]
  pub resource_clip_rule: Option<String>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: Option<String>,
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
  pub control_code: Option<String>,
  #[serde(rename = "controlName")]
  pub control_name: Option<String>,
  #[serde(rename = "controlStatus")]
  pub control_status: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ControlPointVO {
  #[serde(rename = "controlId")]
  pub control_id: String,
  #[serde(rename = "functionId")]
  pub function_id: String,
  #[serde(rename = "controlCode")]
  pub control_code: String,
  #[serde(rename = "controlName")]
  pub control_name: String,
  #[serde(rename = "controlStatus")]
  pub control_status: Option<String>,
  pub resources: Vec<ControlResourceVO>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

impl From<ControlPointModel> for ControlPointVO {
  fn from(value: ControlPointModel) -> Self {
    ControlPointVO {
      control_id: value.control_id,
      function_id: value.function_id,
      control_code: value.control_code,
      control_name: value.control_name,
      control_status: value.control_status,
      resources: vec![],
      last_modified_user_id: value.last_modified_user_id,
      create_time: value.create_time,
      update_time: value.update_time,
    }
  }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ResourceClipRuleVO {
  pub r#type: Option<String>,
  pub keyword: Option<String>,
  pub regex: Option<String>,
  pub replacement: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ControlResourceVO {
  #[serde(rename = "controlId")]
  pub control_id: Option<String>,
  #[serde(rename = "resourceId")]
  pub resource_id: Option<String>,
  #[serde(rename = "resourceRemark")]
  pub resource_remark: Option<String>,
  #[serde(rename = "resourceCode")]
  pub resource_code: Option<String>,
  #[serde(rename = "resourceUrl")]
  pub resource_url: Option<String>,
  #[serde(rename = "resourceMethodType")]
  pub resource_method_type: Option<String>,
  #[serde(rename = "resourceQueryFlag")]
  pub resource_query_flag: Option<String>,
  #[serde(rename = "resourceClipRule")]
  pub resource_clip_rule: Option<Vec<ResourceClipRuleVO>>,
  pub authorizations: Vec<String>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: Option<String>,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

impl From<ControlResourceModel> for ControlResourceVO {
  fn from(value: ControlResourceModel) -> Self {
    ControlResourceVO {
      control_id: value.control_id,
      resource_id: value.resource_id,
      resource_remark: value.resource_remark,
      resource_code: value.resource_code,
      resource_url: value.resource_url,
      resource_method_type: value.resource_method_type,
      resource_query_flag: value.resource_query_flag,
      resource_clip_rule: {
        if value.resource_clip_rule.is_some() {
          Some(serde_json::from_str(&*format!(r#"{}"#, &*value.resource_clip_rule.unwrap())).unwrap())
        } else {
          None
        }
      },
      last_modified_user_id: value.last_modified_user_id,
      create_time: value.create_time,
      update_time: value.update_time,
      authorizations: vec![],
    }
  }
}
