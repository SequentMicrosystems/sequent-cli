use linux_embedded_hal::{I2CError, i2cdev::linux::LinuxI2CError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{self:?}")]
    InvalidParameter { param: String, value: String }, // Only rarely used

    #[error("{self:?}")]
    UnsupportedCapability { capability: &'static str },
    #[error("{self:?}")]
    UnknownCommand { command: String },
    #[error("{self:?}")]
    NoCommand(),

    #[error("{self:?}")]
    LinuxI2C(#[from] LinuxI2CError),
    #[error("{self:?}")]
    I2C(#[from] I2CError),

    #[error("{self:?}")]
    Clap(#[from] clap::Error),
}
