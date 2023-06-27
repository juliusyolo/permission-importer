use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;

#[derive(Serialize, Deserialize, Default)]
pub struct MenuModel {
  pub id: u64,
  #[serde(rename = "menuId")]
  pub menu_id: String,
  #[serde(rename = "upMenuId")]
  pub up_menu_id: Option<String>,
  #[serde(rename = "upMenuName")]
  pub up_menu_name: String,
  #[serde(rename = "menuName")]
  pub menu_name: String,
  #[serde(rename = "menuOrder")]
  pub menu_order: Option<i32>,
  #[serde(rename = "menuStatus")]
  pub menu_status: Option<String>,
  #[serde(rename = "menuRemark")]
  pub menu_remark: Option<String>,
  #[serde(rename = "menuIcon")]
  pub menu_icon: Option<String>,
  #[serde(rename = "functionId")]
  pub function_id: Option<String>,
  #[serde(rename = "functionName")]
  pub function_name: String,
  #[serde(rename = "i18nKey")]
  pub i18n_key: Option<String>,
  #[serde(rename = "menuPath")]
  pub menu_path: Option<String>,
  #[serde(rename = "menuComponent")]
  pub menu_component: Option<String>,
  #[serde(rename = "subMenuCount")]
  pub sub_menu_count: i64,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  pub create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MenuFunctionModel {
  id: u64,
  #[serde(rename = "menuId")]
  pub menu_id: String,
  #[serde(rename = "upMenuId")]
  pub up_menu_id: String,
  #[serde(rename = "menuName")]
  pub menu_name: String,
  #[serde(rename = "menuOrder")]
  pub menu_order: String,
  #[serde(rename = "menuStatus")]
  pub menu_status: String,
  #[serde(rename = "menuRemark")]
  pub menu_remark: String,
  #[serde(rename = "menuIcon")]
  pub menu_icon: String,
  #[serde(rename = "functionId")]
  pub function_id: String,
  #[serde(rename = "systemId")]
  pub system_id: String,
  #[serde(rename = "lastModifiedUserId")]
  last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  update_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MenuTreeVO {
  #[serde(rename = "menuId")]
  pub menu_id: String,
  #[serde(rename = "upMenuId")]
  pub up_menu_id: Option<String>,
  #[serde(rename = "upMenuName")]
  pub up_menu_name: String,
  #[serde(rename = "menuName")]
  pub menu_name: String,
  #[serde(rename = "menuOrder")]
  pub menu_order: Option<i32>,
  #[serde(rename = "menuStatus")]
  pub menu_status: Option<String>,
  #[serde(rename = "menuRemark")]
  pub menu_remark: Option<String>,
  #[serde(rename = "menuIcon")]
  pub menu_icon: Option<String>,
  #[serde(rename = "functionId")]
  pub function_id: Option<String>,
  #[serde(rename = "functionName")]
  pub function_name: String,
  #[serde(rename = "i18nKey")]
  pub i18n_key: Option<String>,
  #[serde(rename = "menuPath")]
  pub menu_path: Option<String>,
  #[serde(rename = "menuComponent")]
  pub menu_component: Option<String>,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
  pub children: Vec<MenuTreeVO>,
  #[serde(rename = "isLeaf")]
  pub is_leaf: bool,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
}

impl From<MenuModel> for MenuTreeVO {
  fn from(value: MenuModel) -> Self {
    MenuTreeVO {
      menu_id: value.menu_id,
      up_menu_id: value.up_menu_id,
      up_menu_name: value.up_menu_name,
      menu_name: value.menu_name,
      menu_order: value.menu_order,
      menu_status: value.menu_status,
      menu_remark: value.menu_remark,
      menu_icon: value.menu_icon,
      function_id: value.function_id,
      function_name: value.function_name,
      i18n_key: value.i18n_key,
      menu_path: value.menu_path,
      menu_component: value.menu_component,
      update_time: value.update_time,
      children: vec![],
      is_leaf: value.sub_menu_count == 0,
      last_modified_user_id: value.last_modified_user_id,
    }
  }
}

#[derive(Serialize, Deserialize, Default)]
pub struct MenuVO {}
