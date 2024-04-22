use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Failed to get kernel version")]
    GetKernelError,

    #[error("An error occured in IO")]
    IOError(#[from] std::io::Error),

    #[error("An error occured while getting the desktop/window manager")]
    GetDesktopError(#[from] std::env::VarError),

    #[error("An error occured getting the system uptime")]
    GetUptimeError,

    #[error("An error occured in parsing")]
    ParseError,
}
