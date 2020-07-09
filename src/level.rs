mod load_tilesets;
mod load_layers;

use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use thiserror::Error;
use sdl2::{pixels::Color, rect::{Point, Rect}};
use specs::{World, WorldExt, Entity, Builder};

use crate::{
    Game,
    TileMap,
    Size,
    Physics,
    Player,
    Position,
    Vec2,
    ExtraLayers,
    TemplateError,
    Renderer,
    ImageCache,
    TileLayer,
    Image,
    ImageParams,
    SdlError,
};

use load_tilesets::load_tilesets;
use load_layers::load_layers;

/// The draw order value of tiles inserted into the world from the map layer
const TILE_DRAW_ORDER: u8 = 0;
/// The draw order value of objects inserted into the world from objects
const OBJECT_DRAW_ORDER: u8 = 1;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LoadError {
    #[error("Level.load() may only be called once")]
    MultipleLoads,
    #[error("Error with path `{0}`: {1}")]
    IOError(PathBuf, io::Error),

    TemplateError(#[from] TemplateError),
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
pub struct TileId(u32);

impl fmt::Display for TileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let TileId(id) = self;
        write!(f, "{}", id)
    }
}

macro_rules! assert_support {
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            Err(Unsupported(format!($($arg)+)))?
        }
    };
}

fn resolve_image_path(base_dir: &Path, image_path: &str) -> Result<PathBuf, LoadError> {
    let path = Path::new(image_path);
    let path = if path.is_relative() {
        base_dir.join(path)
    } else {
        path.to_path_buf()
    };

    Ok(path.canonicalize().map_err(|err| (path.to_path_buf(), err))?)
}

pub struct Level {
    world: World,
    viewport: Rect,
    level_start: Option<Vec2>,
    tile_size: Size,
    extra_layers: ExtraLayers,
    background_color: Color,
    /// True if load() has completed successfully
    loaded: bool,
}

impl fmt::Debug for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            world: _,
            viewport,
            level_start,
            tile_size,
            extra_layers,
            background_color,
            loaded,
        } = self;

        f.debug_struct("Level")
            // World doesn't implement Debug
            .field("world", &"World")
            .field("viewport", &viewport)
            .field("level_start", &level_start)
            .field("tile_size", &tile_size)
            .field("extra_layers", &extra_layers)
            .field("background_color", &background_color)
            .field("loaded", &loaded)
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
            tile_size: Size {width: 1, height: 1},
            extra_layers: ExtraLayers::default(),
            background_color: Color::BLACK,
            loaded: false,
        }
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn load(
        &mut self,
        base_dir: &Path,
        map: &TileMap,
        image_cache: &mut ImageCache,
    ) -> Result<(), LoadError> {
        let Self {
            world,
            viewport: _,
            level_start: _,
            extra_layers,
            tile_size,
            background_color,
            loaded,
        } = self;

        if *loaded {
            return Err(LoadError::MultipleLoads);
        }

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
            background_colour: map_background_color,
        } = *map.as_map();

        assert_support!(orientation == tiled::Orientation::Orthogonal,
            "only maps with orthogonal orientation are supported");

        if !image_layers.is_empty() {
            println!("Warning: image layers are not supported yet and will be ignored");
        }

        if let Some(tiled::Colour {red, green, blue}) = map_background_color {
            *background_color = Color {r: red, g: green, b: blue, a: 255};
        }

        tile_size.width = tile_width;
        tile_size.height = tile_height;

        let tiles = load_tilesets(base_dir, tilesets, image_cache)?;
        load_layers(nrows, ncols, layers, &tiles, world, extra_layers)?;

        //TODO: Store the level_start position
        //TODO: Check if we have an entity with the Player component, and if so
        // add a Position component.

        *loaded = true;

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

    pub fn update(&mut self, events: (/* TODO */), physics: &mut Physics) {
        //TODO: Update level
        //TODO: Update physics + physics step + copy changes back to ECS
    }

    pub fn draw(&self, renderer: &mut Renderer) -> Result<(), SdlError> {
        let Self {
            ref world,
            viewport,
            level_start: _,
            tile_size,
            ref extra_layers,
            background_color,
            loaded: _,
        } = *self;

        let Size {width, height} = renderer.size();

        // Compute the scale factor required to fit the viewport in the canvas
        let scale_x = width as f64 / viewport.width() as f64;
        let scale_y = height as f64 / viewport.height() as f64;

        renderer.clear(background_color);

        let ExtraLayers {front_layers, back_layers} = extra_layers;

        for layer in back_layers {
            draw_layer(renderer, layer, viewport, tile_size, (scale_x, scale_y))?;
        }

        //TODO: Render world

        for layer in front_layers {
            draw_layer(renderer, layer, viewport, tile_size, (scale_x, scale_y))?;
        }

        renderer.present();

        Ok(())
    }
}

fn draw_layer(
    renderer: &mut Renderer,
    layer: &TileLayer,
    viewport: Rect,
    tile_size: Size,
    (scale_x, scale_y): (f64, f64),
) -> Result<(), SdlError> {
    let TileLayer {
        offset,
        nrows: _,
        ncols: _,
        tiles,
    } = layer;

    // Draw tiles in right-down order
    for (row_i, row) in (0u32..).zip(tiles) {
        for (col_i, image) in (0u32..).zip(row) {
            let image = match image {
                Some(image) => image,
                None => continue,
            };

            // Compute the position of the tile in world coordinates
            let world_pos = Vec2::new(
                (col_i * tile_size.width) as f64 + offset.x,
                (row_i * tile_size.height) as f64 + offset.y,
            );

            render_image(
                renderer,
                image,
                world_pos,
                viewport,
                (scale_x, scale_y),
            )?;
        }
    }

    Ok(())
}

fn render_image(
    renderer: &mut Renderer,
    image: &Image,
    world_pos: Vec2,
    viewport: Rect,
    (scale_x, scale_y): (f64, f64),
) -> Result<(), SdlError> {
    let &Image {
        id,
        align,
        ref params,
    } = image;

    // Update the size to be the size in screen coordinates
    let size = params.size;
    let size = Size {
        width: (size.width as f64 * scale_x) as u32,
        height: (size.height as f64 * scale_y) as u32,
    };
    let params = ImageParams {size, ..params.clone()};

    let screen_pos = Point::new(
        (world_pos.x * scale_x) as i32,
        (world_pos.y * scale_y) as i32,
    );

    //TODO: Compute this rectangle based on `align`
    let screen_rect = Rect::new(
        screen_pos.x(),
        screen_pos.y(),
        size.width,
        size.height,
    );

    if viewport.has_intersection(screen_rect) {
        renderer.render_image(
            id,
            params,
            // Position is relative to the top left of the viewport
            screen_rect.top_left() - viewport.top_left(),
        )?;
    }

    Ok(())
}
