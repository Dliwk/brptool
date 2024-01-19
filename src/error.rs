use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrpError {
    #[error("not a brp file")]
    NotABrpFile,
    #[error("unsupported protocol version {0}")]
    UnsupportedProtocolVersion(u16),
}
