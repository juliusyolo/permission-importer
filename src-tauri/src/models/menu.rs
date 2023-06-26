use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::common::serde_naive_datetime;
use crate::models::function::FunctionModel;

#[derive(Serialize, Deserialize, Default)]
pub struct MenuModel {
  id: u64,
  #[serde(rename = "menuId")]
  pub menu_id: String,
  #[serde(rename = "upMenuId")]
  pub up_menu_id: String,
  #[serde(rename = "upMenuName")]
  pub up_menu_name: String,
  #[serde(rename = "menuName")]
  pub menu_name: String,
  #[serde(rename = "menuOrder")]
  menu_order: u32,
  #[serde(rename = "menuStatus")]
  pub menu_status: String,
  #[serde(rename = "menuRemark")]
  pub menu_remark: String,
  #[serde(rename = "menuIcon")]
  pub menu_icon: String,
  #[serde(rename = "functionId")]
  pub function_id: String,
  #[serde(rename = "functionName")]
  pub function_name: String,
  function: FunctionModel,
  #[serde(rename = "i18nKey")]
  pub i18n_key: String,
  #[serde(rename = "menuPath")]
  pub menu_path: String,
  #[serde(rename = "menuComponent")]
  pub menu_component: String,
  #[serde(rename = "subMenuCount")]
  sub_menu_count: u64,
  #[serde(rename = "lastModifiedUserId")]
  last_modified_user_id: String,
  #[serde(rename = "createTime", with = "serde_naive_datetime")]
  create_time: NaiveDateTime,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  update_time: NaiveDateTime,
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
  pub up_menu_id: String,
  #[serde(rename = "upMenuName")]
  pub up_menu_name: String,
  #[serde(rename = "menuName")]
  pub menu_name: String,
  #[serde(rename = "menuOrder")]
  menu_order: u32,
  #[serde(rename = "menuStatus")]
  pub menu_status: String,
  #[serde(rename = "menuRemark")]
  pub menu_remark: String,
  #[serde(rename = "menuIcon")]
  pub menu_icon: String,
  #[serde(rename = "functionId")]
  pub function_id: String,
  #[serde(rename = "functionName")]
  pub function_name: String,
  #[serde(rename = "i18nKey")]
  pub i18n_key: String,
  #[serde(rename = "menuPath")]
  pub menu_path: String,
  #[serde(rename = "menuComponent")]
  pub menu_component: String,
  #[serde(rename = "updateTime", with = "serde_naive_datetime")]
  pub update_time: NaiveDateTime,
  pub children: Vec<MenuTreeVO>,
  #[serde(rename = "isLeaf")]
  pub is_leaf: bool,
  #[serde(rename = "lastModifiedUserId")]
  pub last_modified_user_id: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MenuVO{

}
