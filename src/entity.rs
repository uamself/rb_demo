use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub enabled: i32,
    pub create_time: DateTime,
    pub modify_time: DateTime,
}
rbatis::crud!(User {}); //crud = insert+select_by_column+update_by_column+delete_by_column
impl_select_page!(User{select_page() => "
if !sql.contains('count(1)'):
       `order by create_time desc`"
});

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub create_time: DateTime,
    pub modify_time: DateTime,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub create_time: DateTime,
    pub modify_time: DateTime,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Permissions {
    pub id: String,
    pub name: String,
    pub create_time: DateTime,
    pub modify_time: DateTime,
}
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RolePermissions {
    pub id: String,
    pub role_id: String,
    pub permissions_id: String,
    pub create_time: DateTime,
    pub modify_time: DateTime,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Menu {
    pub id: String,
    pub name: String,
    pub permissions_id: String,
    pub url: String,
    pub sort: i32,
    pub style: String,
    pub parent_id: String,
    pub create_time: DateTime,
    pub modify_time: DateTime,
}
