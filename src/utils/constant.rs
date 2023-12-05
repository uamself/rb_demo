use crate::load_config::init_load_config;
use crate::load_config::models::GlobalConfig;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_CONFIG: GlobalConfig = init_load_config::load_global_config().unwrap();
}
