use sqlx::types::chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::common::serde_naive_datetime;
#[derive(Serialize, Deserialize, Default)]
pub struct OrganizationModel{
  pub id: u64,
  pub organization_id:String,
  pub up_organization_id:String,
  
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

pub struct OrganizationGroupRelationModel{

}

pub struct OrganizationRoleRelationModel{

}
