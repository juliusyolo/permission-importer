use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;

#[derive(Serialize, Deserialize, Default)]
pub struct AuthorizationRecordModel {
  pub id: u64,
  #[serde(rename = "authorizationRecordId")]
  pub authorization_record_id: String,
  #[serde(rename = "authorizationObjectType")]
  pub authorization_object_type: String,
  #[serde(rename = "authorizationObjectId")]
  pub authorization_object_id: String,
  #[serde(rename = "authorizationResourceType")]
  pub authorization_resource_type: String,
  #[serde(rename = "authorizationResourceId")]
  pub authorization_resource_id: String,
  #[serde(rename = "systemId")]
  pub system_id: String,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AuthorizationRecordHelperModel {
  pub id: u64,
  #[serde(rename = "authorizationRecordType")]
  pub authorization_record_type: String,
  #[serde(rename = "authorizationObjectType")]
  pub authorization_object_type: String,
  #[serde(rename = "authorizationObjectId")]
  pub authorization_object_id: String,
  #[serde(rename = "halfKeys")]
  pub half_keys: String,
  #[serde(rename = "completeKeys")]
  pub complete_keys: String,
  #[serde(rename = "systemId")]
  pub system_id: String,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AuthorizationResource{

}
#[derive(Serialize, Deserialize, Default)]
pub struct AuthorizedPair<T>{
   data:T,
}
