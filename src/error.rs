use std::fmt;

/// Main error type for the nexus-core library
#[derive(Debug)]
pub enum NexusError {
    /// Database-related errors
    Database(rusqlite::Error),
    /// Validation errors (e.g., empty fields, invalid input)
    Validation(String),
    /// Authentication/authorization errors
    Auth(String),
    /// Resource not found errors
    NotFound(String),
    /// Serialization/deserialization errors
    Serialization(String),
    /// P2P synchronization errors
    Sync(String),
    /// I/O errors
    Io(std::io::Error),
    /// Generic errors with custom messages
    Other(String),
}

impl fmt::Display for NexusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NexusError::Database(e) => write!(f, "Database error: {}", e),
            NexusError::Validation(msg) => write!(f, "Validation error: {}", msg),
            NexusError::Auth(msg) => write!(f, "Authentication error: {}", msg),
            NexusError::NotFound(msg) => write!(f, "Not found: {}", msg),
            NexusError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            NexusError::Sync(msg) => write!(f, "Synchronization error: {}", msg),
            NexusError::Io(e) => write!(f, "I/O error: {}", e),
            NexusError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for NexusError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NexusError::Database(e) => Some(e),
            NexusError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<rusqlite::Error> for NexusError {
    fn from(err: rusqlite::Error) -> Self {
        NexusError::Database(err)
    }
}

impl From<std::io::Error> for NexusError {
    fn from(err: std::io::Error) -> Self {
        NexusError::Io(err)
    }
}

impl From<String> for NexusError {
    fn from(msg: String) -> Self {
        NexusError::Other(msg)
    }
}

impl From<&str> for NexusError {
    fn from(msg: &str) -> Self {
        NexusError::Other(msg.to_string())
    }
}

/// Result type alias for nexus-core operations
pub type Result<T> = std::result::Result<T, NexusError>;
