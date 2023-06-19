use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;
use crate::models::organization::OrganizationModel;
use crate::models::system::SystemModel;

#[derive(Serialize, Deserialize, Default)]
pub struct UserModel {
  pub id: u64,
  #[serde(rename = "userId")]
  pub user_id: String,
  #[serde(rename = "userName")]
  pub user_name: String,
  #[serde(rename = "userCode")]
  pub user_code: String,
  #[serde(rename = "userGender")]
  pub user_gender: String,
  #[serde(rename = "userAvatar")]
  pub user_avatar: String,
  #[serde(rename = "userType")]
  pub user_type: String,
  #[serde(rename = "userStatus")]
  pub user_status: String,
  #[serde(rename = "systems")]
  pub systems: Vec<SystemModel>,
  #[serde(rename = "organizations")]
  pub organizations: Vec<OrganizationModel>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

pub struct UserOrganizationRelationModel {

}

pub struct UserOrganizationGroupRelationModel {

}

pub struct UserOrganizationRoleRelationModel {

}
