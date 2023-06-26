use sqlx::{ MySql, Pool};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct PaginationResult<T> {
  pub(crate) total: i64,
  #[serde(rename = "currentPage")]
  pub(crate) current_page: i64,
  #[serde(rename = "pageSize")]
  pub(crate) page_size: i64,
  #[serde(rename = "dataList")]
  pub(crate) data_list: Vec<T>,
}

pub struct DatabaseConnectionPool {
  pub(crate) pool: Pool<MySql>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TreeResult<T>{
  data:T,
}
