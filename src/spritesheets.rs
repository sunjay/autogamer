use std::path::{PathBuf, Path};
use std::fs::File;
use std::io;

use serde::{Serialize, Deserialize};
use noisy_float::prelude::R64;
use sdl2::rect::Rect;
use thiserror::Error;

use crate::{
    CHARACTER_DRAW_ORDER,
    CharacterSprites,
    ImageCache,
    Image,
    Align,
    ImageParams,
    Sprite,
    Size,
};

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
struct Poses {
    pub idle: Option<TilePos>,
    pub jump_midair: Option<TilePos>,
    pub fall_midair: Option<TilePos>,
    pub crouch: Option<TilePos>,
    pub hurt: Option<TilePos>,
    pub kick: Option<TilePos>,
    pub talk: Option<TilePos>,
    pub slide: Option<TilePos>,
    pub hang: Option<TilePos>,
    pub skid: Option<TilePos>,
    pub back: Option<TilePos>,
    pub stand: Option<TilePos>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Animations {
    walk: Option<Vec<Frame>>,
    climb: Option<Vec<Frame>>,
    cheer: Option<Vec<Frame>>,
    swim: Option<Vec<Frame>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "layout")]
#[serde(deny_unknown_fields)]
enum SpritesheetConfig {
    Grid {
        tile_width: u32,
        tile_height: u32,
        poses: Poses,
        animations: Animations,
    },
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LoadSpritesError {
    IOError(#[from] io::Error),
    JsonError(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct CharacterSpritesheet {
    spritesheet: PathBuf,
    config: SpritesheetConfig,
}

impl CharacterSpritesheet {
    pub fn open(spritesheet: &Path, config: &Path) -> Result<Self, LoadSpritesError> {
        let spritesheet = spritesheet.to_path_buf();
        let config = serde_json::from_reader(File::open(config)?)?;
        Ok(Self {spritesheet, config})
    }

    pub fn load(&self, image_cache: &mut ImageCache) -> Result<CharacterSprites, LoadSpritesError> {
        let id = image_cache.add(&self.spritesheet);

        let make_image = |src, size| {
            let src = Some(src);
            let align = Align::default();
            let params = ImageParams {
                size,
                flip_horizontal: false,
                flip_vertical: false,
                angle: R64::new(0.0),
                alpha: u8::MAX,
            };
            Image {id, src, align, params}
        };

        use SpritesheetConfig::*;
        match &self.config {
            &Grid {tile_width, tile_height, ref poses, ref animations} => {
                let grid_sprite = |TilePos {row, col}| {
                    let src = Rect::new(
                        (col * tile_width) as i32,
                        (row * tile_height) as i32,
                        tile_width,
                        tile_height,
                    );

                    let size = Size {
                        width: tile_width,
                        height: tile_height,
                    };

                    let image = make_image(src, size);
                    let align_size = size;
                    let pivot = None;
                    let draw_order = CHARACTER_DRAW_ORDER;

                    Sprite {image, align_size, pivot, draw_order}
                };

                Ok(CharacterSprites {
                    idle: poses.idle.map(grid_sprite),
                })
            }
        }
    }
}
