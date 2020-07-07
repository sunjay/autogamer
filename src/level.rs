mod load_tilesets;

use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use thiserror::Error;
use sdl2::{pixels::Color, rect::Rect};
use specs::{World, WorldExt, Entity, Builder};

use crate::{Game, TileMap, Size, Renderer, Player, Position, Vec2};

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LoadError {
    #[error("Error with path `{0}`: {1}")]
    IOError(PathBuf, io::Error),
    Unsupported(#[from] Unsupported),
}

impl From<(PathBuf, io::Error)> for LoadError {
    fn from((path, err): (PathBuf, io::Error)) -> Self {
        LoadError::IOError(path, err)
    }
}

#[derive(Debug, Clone, Error)]
#[error("{0}")]
pub struct Unsupported(String);

/// A unique ID for a value retrieved from a tiled map file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TId(u32);

macro_rules! assert_support {
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            Err(Unsupported(format!($($arg)+)))?
        }
    };
}

pub struct Level {
    world: World,
    viewport: Rect,
    level_start: Option<Vec2>,
}

impl fmt::Debug for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {world: _, viewport, level_start} = self;

        f.debug_struct("Level")
            // World doesn't implement Debug
            .field("world", &"World")
            .field("viewport", &viewport)
            .field("level_start", &level_start)
            .finish()
    }
}

impl Level {
    pub fn new(game: &Game) -> Self {
        let mut world = World::new();
        crate::register_components(&mut world);

        Self {
            world,
            viewport: Rect::new(
                0,
                0,
                game.window_width(),
                game.window_height(),
            ),
            level_start: None,
        }
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn load(
        &mut self,
        base_dir: &Path,
        map: &TileMap,
        renderer: &mut Renderer,
    ) -> Result<(), LoadError> {
        let tiled::Map {
            version: _,
            orientation,
            width: ncols,
            height: nrows,
            tile_width,
            tile_height,
            ref tilesets,
            ref layers,
            ref image_layers,
            ref object_groups,
            properties: _,
            background_colour: background_color,
        } = *map.as_map();

        assert_support!(orientation == tiled::Orientation::Orthogonal,
            "only maps with orthogonal orientation are supported");

        if !image_layers.is_empty() {
            println!("Warning: image layers are not supported yet and will be ignored");
        }

        let background_color = match background_color {
            Some(tiled::Colour {red, green, blue}) => Color {r: red, g: green, b: blue, a: 255},
            None => Color::RGBA(0, 0, 0, 0),
        };
        renderer.set_background_color(background_color);

        let tiles = load_tilesets::load_tilesets(
            base_dir,
            tilesets,
        )?;

        //TODO: Check if we have an entity with the Player component, and if so
        // add a Position component. Otherwise just store the position for later

        Ok(())
    }

    pub fn add_player(&mut self) -> Entity {
        let level_start = self.level_start.unwrap_or_default();

        self.world.create_entity()
            .with(Player)
            .with(Position(level_start))
            .build()
    }

    pub fn set_viewport_dimensions(&mut self, size: Size) {
        let Size {width, height} = size;
        self.viewport.set_width(width);
        self.viewport.set_height(height);
    }
}
