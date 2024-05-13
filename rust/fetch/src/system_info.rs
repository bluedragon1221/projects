use std::time::Duration;

use pretty_duration::pretty_duration;
use sysinfo::System;

use crate::FetchError;

pub struct Sysinfo {}

impl Sysinfo {
    pub fn get_distro() -> Result<String, FetchError> {
        Ok(whoami::distro())
    }
    pub fn get_short_hostname() -> Result<String, FetchError> {
        Ok(whoami::devicename())
    }
    pub fn get_username() -> Result<String, FetchError> {
        Ok(whoami::username())
    }
    pub fn get_hostname() -> Result<String, FetchError> {
        Ok(format!(
            "{}@{}",
            Self::get_username()?,
            Self::get_short_hostname()?
        ))
    }
    pub fn get_kernel() -> Result<String, FetchError> {
        System::kernel_version().ok_or(FetchError::GetKernelError)
    }
    pub fn get_desktop() -> Result<String, FetchError> {
        Ok(match whoami::desktop_env() {
            whoami::DesktopEnv::Unknown(_) => std::env::var("XDG_CURRENT_DESKTOP")?,
            a => a.to_string(),
        })
    }

    fn round_number(num: u64, percision: u64) -> u64 {
        ((num + percision - 1) / percision) * percision
    }
    pub fn get_uptime() -> Result<String, FetchError> {
        Ok(pretty_duration(
            &Duration::from_secs(Self::round_number(System::uptime(), 60)),
            None,
        )
        .to_string())
    }
}
