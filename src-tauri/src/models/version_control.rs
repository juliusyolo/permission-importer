use sqlx::types::chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::common::serde_naive_datetime;

#[derive(Serialize, Deserialize, Default)]
pub struct VersionControlModel {
  pub id: u64,
  pub version: String,
  #[serde(rename = "releaseTime", with = "serde_naive_datetime")]
  pub release_time: NaiveDateTime,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}
