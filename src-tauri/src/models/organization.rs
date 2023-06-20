use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;
use crate::models::group::GroupModel;
use crate::models::role::RoleModel;

#[derive(Serialize, Deserialize, Default)]
pub struct OrganizationModel {
  pub id: u64,
  #[serde(rename = "organizationId")]
  pub organization_id: String,
  #[serde(rename = "upOrganizationId")]
  pub up_organization_id: String,
  #[serde(rename = "upOrganizationCode")]
  pub up_organization_code: String,
  #[serde(rename = "organizationCode")]
  pub organization_code: String,
  #[serde(rename = "organizationName")]
  pub organization_name: String,
  #[serde(rename = "organizationSequence")]
  pub organization_sequence: String,
  #[serde(rename = "organizationLevel")]
  pub organization_level: String,
  #[serde(rename = "organizationStatus")]
  pub organization_status: String,
  #[serde(rename = "organizationRemark")]
  pub organization_remark: String,
  #[serde(rename = "upOrganizationName")]
  pub up_organization_name: String,
  #[serde(rename = "subOrganizationCount")]
  sub_organization_count: u64,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OrganizationGroupRelationModel {
  pub id: u64,
  #[serde(rename = "orgGroupRelId")]
  pub org_group_rel_id: String,
  #[serde(rename = "organizationId")]
  pub organization_id: String,
  #[serde(rename = "groupId")]
  pub group_id: String,
  #[serde(rename = "systemId")]
  pub system_id: String,
  #[serde(rename = "groupModel")]
  group_model: GroupModel,
  #[serde(rename = "organizationModel")]
  organization_model: OrganizationModel,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OrganizationRoleRelationModel {
  pub id: u64,
  #[serde(rename = "orgRoleRelId")]
  pub org_role_rel_id: String,
  #[serde(rename = "organizationId")]
  pub organization_id: String,
  #[serde(rename = "roleId")]
  pub role_id: String,
  #[serde(rename = "systemId")]
  pub system_id: String,
  #[serde(rename = "roleModel")]
  role_model: RoleModel,
  #[serde(rename = "organizationModel")]
  organization_model: OrganizationModel,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}
