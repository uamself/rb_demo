use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct PageVo {
    pub page_no: u64,
    pub page_size: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UserVo {
    pub id: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub enabled: Option<i32>,
    pub create_time: Option<DateTime>,
}
