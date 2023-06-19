use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;

pub struct SystemModel {
  pub id: u64,
  pub systemId: String,
  pub systemVersion: String,
  pub systemName: String,
  pub systemRemark: String,
  pub systemStatus: String,
  pub systemCode: String,
  pub i18nKey: String,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}
