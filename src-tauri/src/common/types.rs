use sqlx::{ MySql, Pool};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct PaginationResult<T> {
  pub(crate) total: i64,
  #[serde(rename = "currentPage")]
  pub(crate) current_page: i64,
  #[serde(rename = "pageSize")]
  pub(crate) page_size: i64,
  pub(crate) data: Vec<T>,
}

pub struct DatabaseConnectionPool {
  pub(crate) pool: Pool<MySql>,
}
