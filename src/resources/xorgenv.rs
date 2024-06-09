use crate::config;
#[allow(non_snake_case)]
pub struct EnvInfo {
    pub XORG_ENV: String,
}

use std::error::Error;

pub fn env_info() -> Result<Option<EnvInfo>, Box<dyn Error>> {
    let xorg_env = config::env_wm();
    Ok(Some(EnvInfo {
        XORG_ENV: xorg_env.to_string(),
    }))
}
