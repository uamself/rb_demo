use super::constant::GLOBAL_CONFIG;

// 读取配置文件并生成mysql的URL
pub fn mysql_init(config: &GLOBAL_CONFIG) -> String {
    let mysql = config.mysql.clone();
    let username = mysql.username;
    let password = mysql.password;
    let host = mysql.host;
    let port = mysql.port;
    let db_name = mysql.db_name;

    let url = "mysql://".to_owned()
        + &username
        + ":"
        + &password
        + "@"
        + &host
        + ":"
        + port.to_string().as_str()
        + "/"
        + &db_name;

    url
}
