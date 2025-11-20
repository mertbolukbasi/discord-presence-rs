use thiserror::Error;

/// Represents the possible errors that can occur.
#[derive(Debug, Error)]
pub enum Error {
    /// An I/O error occurred.
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    /// A JSON error occurred.
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
    /// The IPC connection was not found.
    #[error("IPC Connection Not Found")]
    ConnectionNotFound,
    /// The IPC handshake failed.
    #[error("IPC Handshake Failed")]
    HandshakeFailed,
}
