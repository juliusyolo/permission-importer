use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use std::convert::From;
use crate::common::serde_naive_datetime;
use crate::models::group::GroupModel;
use crate::models::organization::OrganizationModel;
use crate::models::role::RoleModel;
use crate::models::system::SystemModel;

#[derive(Serialize, Deserialize, Default)]
pub struct UserModel {
  pub id: u64,
  #[serde(rename = "userId")]
  pub user_id: String,
  #[serde(rename = "userName")]
  pub user_name: String,
  #[serde(rename = "userCode")]
  pub user_code: Option<String>,
  #[serde(rename = "userGender")]
  pub user_gender: Option<String>,
  #[serde(rename = "userAvatar")]
  pub user_avatar: Option<String>,
  #[serde(rename = "userType")]
  pub user_type: Option<String>,
  #[serde(rename = "userStatus")]
  pub user_status: String,
  #[serde(rename = "userToken")]
  pub user_token: Option<String>,
  // #[serde(rename = "systems")]
  // pub systems: Vec<SystemModel>,
  // #[serde(rename = "organizations")]
  // pub organizations: Vec<OrganizationModel>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}
#[derive(Serialize, Deserialize, Default)]
pub struct UserVO {
  #[serde(rename = "userId")]
  pub user_id: String,
  #[serde(rename = "userName")]
  pub user_name: String,
  #[serde(rename = "userCode")]
  pub user_code: Option<String>,
  #[serde(rename = "userGender")]
  pub user_gender: Option<String>,
  #[serde(rename = "userStatus")]
  pub user_status: String,
  #[serde(rename = "organizations")]
  pub organizations: Vec<String>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

impl From<UserModel> for UserVO {
  fn from(value: UserModel) -> Self {
    UserVO{
      user_id: value.user_id,
      user_name: value.user_name,
      user_code: value.user_code,
      user_gender: value.user_gender,
      user_status: value.user_status,
      organizations: vec![],
      last_modified_user_id: value.last_modified_user_id,
      update_time: value.update_time,
    }
  }
}
#[derive(Serialize, Deserialize, Default)]
pub struct UserOrganizationRelationModel {
  pub id: u64,
  #[serde(rename = "userOrgRelId")]
  pub user_org_rel_id: String,
  #[serde(rename = "userId")]
  pub user_id: String,
  #[serde(rename = "systemId")]
  pub system_id: String,
  #[serde(rename = "organizationId")]
  pub organization_id: String,
  #[serde(rename = "organizationModel")]
  organization_model: OrganizationModel,
  #[serde(rename = "systemModel")]
  system_model: SystemModel,
  #[serde(rename = "userModel")]
  user_model: UserModel,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserOrganizationGroupRelationModel {
  pub id: u64,
  #[serde(rename = "userOrgGroupRelId")]
  pub user_org_group_rel_id: String,
  #[serde(rename = "groupId")]
  pub group_id: String,
  #[serde(rename = "GroupModel")]
  group_model: GroupModel,
  #[serde(rename = "userId")]
  pub user_id: String,
  #[serde(rename = "systemId")]
  pub system_id: String,
  #[serde(rename = "organizationId")]
  pub organization_id: String,
  #[serde(rename = "organizationModel")]
  organization_model: OrganizationModel,
  #[serde(rename = "systemModel")]
  system_model: SystemModel,
  #[serde(rename = "userModel")]
  user_model: UserModel,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserOrganizationRoleRelationModel {
  pub id: u64,
  #[serde(rename = "userOrgRoleRelId")]
  pub user_org_role_rel_id: String,
  #[serde(rename = "roleId")]
  pub role_id: String,
  #[serde(rename = "roleModel")]
  role_model: RoleModel,
  #[serde(rename = "userId")]
  pub user_id: String,
  #[serde(rename = "systemId")]
  pub system_id: String,
  #[serde(rename = "organizationId")]
  pub organization_id: String,
  #[serde(rename = "organizationModel")]
  organization_model: OrganizationModel,
  #[serde(rename = "systemModel")]
  system_model: SystemModel,
  #[serde(rename = "userModel")]
  user_model: UserModel,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,

}
