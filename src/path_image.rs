use image::{DynamicImage, ImageError};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use crate::context;

/// Image wrapper that pairs a DynamicImage with its source path.
///
/// Serializes to just the path; deserializes by loading from disk.
/// Dereferences to the DynamicImage for transparent API usage.
#[derive(Clone)]
pub struct PathImage {
    image: DynamicImage,
    path: PathBuf,
}

impl PathImage {
    /// Create a PathImage by loading a DynamicImage from disk.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, ImageError> {
        let path = path.into();
        let image = image::open(&path)?;
        Ok(Self { image, path })
    }

    /// Get a reference to the loaded image.
    pub fn image(&self) -> &DynamicImage {
        &self.image
    }

    /// Get the path this image was loaded from.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Consume and return the inner image.
    pub fn into_image(self) -> DynamicImage {
        self.image
    }

    /// Clone the inner image.
    pub fn clone_image(&self) -> DynamicImage {
        self.image.clone()
    }
}

impl Deref for PathImage {
    type Target = DynamicImage;

    fn deref(&self) -> &Self::Target {
        &self.image
    }
}

impl DerefMut for PathImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.image
    }
}

impl Serialize for PathImage {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.path.serialize(serializer)
    }
}


/// Deserialization loads the image from disk using the stored path.
/// If a deserialization context is set, validates that the path doesn't escape it.
impl<'de> Deserialize<'de> for PathImage {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let path = PathBuf::deserialize(deserializer)?;

        // Validate and resolve path within context
        let resolved_path = context::resolve_path_in_context(&path)
            .map_err(serde::de::Error::custom)?;

        Self::new(&resolved_path).map_err(serde::de::Error::custom)
    }
}
