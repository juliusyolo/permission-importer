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
  pub group_remark: Option<String>,
  #[serde(rename = "groupStatus")]
  pub group_status: String,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GroupVO{
  #[serde(rename = "groupId")]
  pub group_id: String,
  #[serde(rename = "groupCode")]
  pub group_code: String,
  #[serde(rename = "groupName")]
  pub group_name: String,
  #[serde(rename = "groupRemark")]
  pub group_remark: Option<String>,
  #[serde(rename = "groupStatus")]
  pub group_status: String,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

impl From<GroupModel> for GroupVO{
  fn from(value: GroupModel) -> Self {
    GroupVO{
      group_id: value.group_id,
      group_code: value.group_code,
      group_name: value.group_name,
      group_remark: value.group_remark,
      group_status: value.group_status,
      last_modified_user_id: value.last_modified_user_id,
      update_time: value.update_time
    }
  }
}

#[derive(Serialize, Deserialize, Default)]
pub struct GroupCondition{
  #[serde(rename = "groupCode")]
  pub group_code:Option<String>,
  #[serde(rename = "groupName")]
  pub group_name:Option<String>,
  #[serde(rename = "groupStatus")]
  pub group_status:Option<String>,
  #[serde(rename = "systemId")]
  pub system_id:String
}

#[derive(Serialize, Deserialize, Default)]
pub struct GroupUserCondition{
  #[serde(rename = "groupId")]
  group_id:String,
  #[serde(rename = "systemId")]
  pub system_id:String
}
