use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;

#[derive(Serialize, Deserialize, Default)]
pub struct ModuleModel {
  id: u64,
  #[serde(rename = "moduleId")]
  module_id: String,
  #[serde(rename = "moduleName")]
  module_name: String,
  #[serde(rename = "moduleRemark")]
  module_remark: String,
  #[serde(rename = "moduleStatus")]
  module_status: String,
  #[serde(rename = "lastModifiedUserId")]
  last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  update_time: NaiveDateTime,
}
