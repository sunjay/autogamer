use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io;

use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
enum SpritesheetLayout {
    Grid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TilePos {
    /// The index of a row in the spritesheet
    pub row: u32,
    /// The index of a column in the spritesheet
    pub col: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Frame {
    /// The index of a row in the spritesheet
    pub row: u32,
    /// The index of a column in the spritesheet
    pub col: u32,
    /// The duration of the frame in ms
    pub duration: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SpritesheetConfig {
    pub layout: SpritesheetLayout,
    pub tile_width: u32,
    pub tile_height: u32,
    pub poses: HashMap<String, TilePos>,
    pub animations: HashMap<String, Vec<Frame>>,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LoadSpritesError {
    IOError(#[from] io::Error),
    JsonError(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct CharacterSpritesheet {
}

impl CharacterSpritesheet {
    pub fn open(spritesheet: &Path, config: &Path) -> Result<Self, LoadSpritesError> {
        let config: SpritesheetConfig = serde_json::from_reader(File::open(config)?)?;
        Ok(Self {})
    }
}
