use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;
#[derive(Serialize, Deserialize, Default)]
pub struct SystemModel {
  pub id: u64,
  #[serde(rename = "systemId")]
  pub system_id: String,
  #[serde(rename = "systemVersion")]
  pub system_version: String,
  #[serde(rename = "systemName")]
  pub system_name: String,
  #[serde(rename = "systemRemark")]
  pub system_remark: String,
  #[serde(rename = "systemStatus")]
  pub system_status: String,
  #[serde(rename = "systemCode")]
  pub system_code: String,
  #[serde(rename = "i18nKey")]
  pub i18n_key: String,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}
