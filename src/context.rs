use std::cell::RefCell;
use std::path::{Path, PathBuf};
use crate::error::PathImageError;

thread_local! {
    static DESERIALIZE_CONTEXT: RefCell<Option<PathBuf>> = RefCell::new(None);
}

/// Set the deserialization context for resolving relative paths.
pub fn set_deserialize_context(path: &Path) {
    DESERIALIZE_CONTEXT.with(|ctx| {
        *ctx.borrow_mut() = Some(path.to_path_buf());
    });
}

/// Clear the deserialization context.
pub fn clear_deserialize_context() {
    DESERIALIZE_CONTEXT.with(|ctx| {
        *ctx.borrow_mut() = None;
    });
}

/// Get the current deserialization context.
pub fn get_deserialize_context() -> Option<PathBuf> {
    DESERIALIZE_CONTEXT.with(|ctx| ctx.borrow().clone())
}

/// Resolve a path within the current context, validating it doesn't escape the context folder.
/// When a context is set, absolute paths and escape attempts are rejected.
pub fn resolve_path_in_context(path: &Path) -> Result<PathBuf, PathImageError> {
    if let Some(context) = get_deserialize_context() {
        // With context: reject absolute paths and escape attempts
        if path.is_absolute() {
            return Err(PathImageError::PathEscapeAttempt(path.to_path_buf()));
        }

        let resolved = context.join(path);

        // Canonicalize both paths to resolve . and .. components
        let canonical_resolved = match resolved.canonicalize() {
            Ok(p) => p,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(PathImageError::NotFound(resolved));
            }
            Err(e) => return Err(PathImageError::Io(e)),
        };

        let canonical_context = context.canonicalize()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    PathImageError::NotFound(context.clone())
                } else {
                    PathImageError::Io(e)
                }
            })?;

        // Ensure the resolved path is within the context folder
        if !canonical_resolved.starts_with(&canonical_context) {
            return Err(PathImageError::PathEscapeAttempt(path.to_path_buf()));
        }

        Ok(canonical_resolved)
    } else {
        // Without context: allow any path (absolute or relative)
        Ok(path.to_path_buf())
    }
}
