use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathImageError {
    #[error("path '{0}' escapes the context folder")]
    PathEscapeAttempt(PathBuf),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("path '{0}' not found")]
    NotFound(PathBuf),
}
