//! Path-aware wrappers with automatic serialization to disk paths.
//!
//! `PathImage` pairs a DynamicImage with its source path. When serialized,
//! only the path is stored. When deserialized, the image is loaded from disk.
//!
//! `PathFont` pairs a FontArc with its source path. When serialized,
//! only the path is stored. When deserialized, the font is loaded from disk.
//!
//! # Example
//!
//! ```ignore
//! use path_image::PathImage;
//!
//! let img = PathImage::new("map.png")?;
//! let json = serde_json::to_string(&img)?;  // "map.png"
//! ```

mod context;
mod error;
mod path_image;
mod path_font;

pub use path_image::PathImage;
pub use path_font::PathFont;
pub use error::PathImageError;
pub use context::{set_deserialize_context, clear_deserialize_context, get_deserialize_context};
