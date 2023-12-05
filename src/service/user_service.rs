use crate::utils::generate_id;
use crate::{entity::User, vo::req_vo::UserVo, RB};
use rbatis::{
    rbdc::DateTime,
    sql::{Page, PageRequest},
};

pub async fn query_all() -> Vec<User> {
    let res = User::select_all(RB.get().unwrap()).await;
    if let Ok(res) = res {
        return res;
    }
    vec![]
}

pub async fn query_by_page(page_no: u64, page_size: u64) -> Page<User> {
    let res = User::select_page(RB.get().unwrap(), &PageRequest::new(page_no, page_size)).await;
    if let Ok(res) = res {
        return res;
    }
    Page::default()
}

pub async fn query_by_id(id: String) -> Option<User> {
    let res = User::select_by_column(RB.get().unwrap(), "id", id).await;
    match res.unwrap().get(0) {
        Some(v) => Some(v.to_owned()),
        None => None,
    }
}

pub async fn insert(user_vo: UserVo) -> Option<u64> {
    let user = User {
        id: generate_id().unwrap().to_string(),
        name: user_vo.name.unwrap(),
        enabled: 1,
        password: user_vo.password.unwrap(),
        create_time: DateTime::now(),
        modify_time: DateTime::now(),
    };
    Some(
        User::insert(RB.get().unwrap(), &user)
            .await
            .unwrap()
            .rows_affected,
    )
}

pub async fn update(user_vo: UserVo) -> Option<u64> {
    let op_user = query_by_id(user_vo.id.clone().unwrap()).await;
    if op_user.is_none() {
        return None;
    }
    let op_user = op_user.unwrap();

    let user = User {
        id: user_vo.id.unwrap(),
        name: if let Some(v) = user_vo.name {
            v
        } else {
            op_user.name
        },
        enabled: if let Some(v) = user_vo.enabled {
            v
        } else {
            op_user.enabled
        },
        password: if let Some(v) = user_vo.password {
            v
        } else {
            op_user.password
        },
        create_time: if let Some(v) = user_vo.create_time {
            v
        } else {
            op_user.create_time
        },
        modify_time: DateTime::now(),
    };
    Some(
        User::update_by_column(RB.get().unwrap(), &user, "id")
            .await
            .unwrap()
            .rows_affected,
    )
}

pub async fn delete(id: String) -> Option<u64> {
    Some(
        User::delete_by_column(RB.get().unwrap(), "id", id)
            .await
            .unwrap()
            .rows_affected,
    )
}
