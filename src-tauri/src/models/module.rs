use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;

#[derive(Serialize, Deserialize, Default)]
pub struct ModuleModel {
  pub id: u64,
  #[serde(rename = "moduleId")]
  pub module_id: String,
  #[serde(rename = "moduleName")]
  pub module_name: String,
  #[serde(rename = "moduleRemark")]
  pub module_remark: Option<String>,
  #[serde(rename = "moduleStatus")]
  pub module_status: Option<String>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ModuleVO {
  #[serde(rename = "moduleId")]
  pub module_id: String,
  #[serde(rename = "moduleName")]
  pub module_name: String,
  #[serde(rename = "moduleRemark")]
  pub module_remark: Option<String>,
  #[serde(rename = "moduleStatus")]
  pub module_status: Option<String>,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

impl From<ModuleModel> for ModuleVO {
  fn from(value: ModuleModel) -> Self {
    ModuleVO {
      module_id: value.module_id,
      module_name: value.module_name,
      module_remark: value.module_remark,
      module_status: value.module_status,
      last_modified_user_id: value.last_modified_user_id,
      update_time: value.update_time,
    }
  }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ModuleCondition {
  #[serde(rename = "moduleName")]
  module_name: Option<String>,
  #[serde(rename = "moduleStatus")]
  module_status: Option<String>,
  #[serde(rename = "systemId")]
  system_id: String,
}
