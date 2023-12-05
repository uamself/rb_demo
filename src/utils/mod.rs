use crate::utils::{constant::GLOBAL_CONFIG, id_generator::SnowflakeIdWorker};
use anyhow::Error;
use lazy_static::lazy_static;

pub mod constant;
pub mod id_generator;
pub mod mysql_init;

lazy_static! {
    pub static ref SNOW_FLAKE_ID_WORKER: SnowflakeIdWorker = {
        let work_id = GLOBAL_CONFIG.snow_flake_id_worker.work_id as u128;
        let data_center_id = GLOBAL_CONFIG.snow_flake_id_worker.data_center_id as u128;
        SnowflakeIdWorker::new(work_id, data_center_id).unwrap()
    };
}

// 生成主键id
pub fn generate_id() -> Result<u128, Error> {
    SNOW_FLAKE_ID_WORKER.next_id()
}
