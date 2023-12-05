//#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'
#[macro_use]
extern crate rbatis;

use axum::Router;
use log::info;
use once_cell::sync::OnceCell;
use rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use utils::{constant::GLOBAL_CONFIG, mysql_init};

mod entity;
mod handler;
mod load_config;
mod service;
mod utils;
mod vo;
static RB: OnceCell<RBatis> = OnceCell::new();

#[tokio::main]
async fn main() {
    // 读取yml配置文件
    let config = &GLOBAL_CONFIG;
    let server = &config.application.server;

    // 初始化打印sql的日志
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");

    // 初始化mysql连接信息
    let url = mysql_init::mysql_init(config);

    // 以下代码设置连接池
    // let opts = MySqlConnectOptions::new()
    //     .host(&server.host)
    //     .username(config.mysql.username.as_str())
    //     .password(config.mysql.password.as_str())
    //     .database(config.mysql.db_name.as_str())
    //     .port(server.port.try_into().unwrap());

    // let pool = DefaultPool::new(ConnManager::new_opt(MysqlDriver {}, opts)).unwrap();
    // let _ = pool.set_max_idle_conns(10);
    // let _ = pool.set_max_open_conns(100);
    // let _ = rb.init_pool(pool);
    // 设置连接池结束

    // 初始化rbatis
    let rb = RBatis::new();
    // 连接mysql
    rb.link(MysqlDriver {}, url.as_str()).await.unwrap();
    let _ = RB.set(rb);

    // 初始化请求路由
    info!("收集所有的请求路由配置---------------开始---------------");
    let app = Router::new().merge(handler::user_handler::get_user_routes());
    info!("收集所有的请求路由配置---------------结束---------------");

    info!("server start success");

    // 构建web服务
    let listener =
        tokio::net::TcpListener::bind(server.host.clone() + ":" + server.port.to_string().as_str())
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}
