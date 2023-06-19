use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;

#[derive(Serialize, Deserialize, Default)]
pub struct GroupModel {
  pub id: u64,
  #[serde(rename = "groupId")]
  pub group_id: String,
  #[serde(rename = "groupCode")]
  pub group_code: String,
  #[serde(rename = "groupName")]
  pub group_name: String,
  #[serde(rename = "groupRemark")]
  pub group_remark: String,
  #[serde(rename = "groupStatus")]
  pub group_status: String,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}
