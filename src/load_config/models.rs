use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalConfig {
    pub mysql: Mysql,
    pub application: Application,
    pub snow_flake_id_worker: SnowFlakeIdWorkerConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Application {
    pub server: Server,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub port: usize,
    pub host: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mysql {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db_name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profiles {
    pub active: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvConfig {
    pub profiles: Profiles,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnowFlakeIdWorkerConfig {
    pub work_id: u32,
    pub data_center_id: u32,
}
