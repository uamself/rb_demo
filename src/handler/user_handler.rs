use std::collections::HashMap;

use crate::service::user_service;
use crate::vo::req_vo::{PageVo, UserVo};
use crate::vo::resp_vo::RespVO;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{routing::get, Json, Router};

// 创建用户路由收集器
pub fn get_user_routes() -> Router {
    Router::new()
        .route("/user/test", get(test))
        .route("/user/query_all", get(query_all))
        .route("/user/query_by_page", post(query_by_page))
        .route("/user/query_by_id", get(query_by_id))
        .route("/user/insert", post(insert))
        .route("/user/update", post(update))
        .route("/user/delete", get(delete))
}

// 获取所有用户信息
pub async fn query_all() -> impl IntoResponse {
    Json(RespVO::from_result(&user_service::query_all().await))
}

// 分页获取所有用户信息
pub async fn query_by_page(Json(pv): Json<PageVo>) -> impl IntoResponse {
    let res = user_service::query_by_page(pv.page_no, pv.page_size);
    Json(RespVO::from_result(&res.await))
}

// 根据ID查询用户
pub async fn query_by_id(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    match params.get("id") {
        Some(id) => Json(RespVO::from_result(
            &user_service::query_by_id(id.clone()).await,
        )),
        None => Json(RespVO::from_error("no data found !")),
    }
}

// 添加用户
pub async fn insert(Json(user_vo): Json<UserVo>) -> impl IntoResponse {
    Json(RespVO::from_result(&user_service::insert(user_vo).await))
}

// 修改用户
pub async fn update(Json(user_vo): Json<UserVo>) -> impl IntoResponse {
    Json(RespVO::from_result(&user_service::update(user_vo).await))
}

// 删除用户
pub async fn delete(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    match params.get("id") {
        Some(id) => Json(RespVO::from_result(&user_service::delete(id.clone()).await)),
        None => Json(RespVO::from_error("no data found !")),
    }
}

// 异步函数，返回字符串"test"
pub async fn test() -> impl IntoResponse {
    Json(RespVO::from_result(&"test".to_string()))
}
