use ab_glyph::FontArc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::io;
use std::ops::Deref;
use std::path::PathBuf;

/// Font wrapper that pairs a FontArc with its source path.
///
/// Serializes to just the path; deserializes by loading from disk.
/// Dereferences to the FontArc for transparent API usage.
#[derive(Clone)]
pub struct PathFont {
    font: FontArc,
    path: PathBuf,
}

impl PathFont {
    /// Create a PathFont by loading a FontArc from disk.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, io::Error> {
        let path = path.into();
        let data = std::fs::read(&path)?;
        let font = FontArc::try_from_vec(data)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid font data"))?;
        Ok(Self { font, path })
    }

    /// Get a reference to the loaded font.
    pub fn font(&self) -> &FontArc {
        &self.font
    }

    /// Get the path this font was loaded from.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Consume and return the inner font.
    pub fn into_font(self) -> FontArc {
        self.font
    }

    /// Clone the inner font.
    pub fn clone_font(&self) -> FontArc {
        self.font.clone()
    }
}

impl Deref for PathFont {
    type Target = FontArc;

    fn deref(&self) -> &Self::Target {
        &self.font
    }
}

impl Serialize for PathFont {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.path.serialize(serializer)
    }
}

/// Deserialization loads the font from disk using the stored path.
impl<'de> Deserialize<'de> for PathFont {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let path = PathBuf::deserialize(deserializer)?;
        Self::new(&path).map_err(serde::de::Error::custom)
    }
}
