use std::fmt;

use thiserror::Error;
use sdl2::{pixels::Color, rect::{Point, Rect}};
use specs::{World, WorldExt, Entity};

use crate::{Game, TileMap, Size, Renderer};

#[derive(Debug, Clone, Error)]
#[error("{0}")]
pub struct Unsupported(String);

macro_rules! assert_support {
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            Err(Unsupported(format!($($arg)+)))?
        }
    };
}

#[derive(Debug)]
struct Markers {
    level_start: Option<Point>,
}

impl Markers {
    pub fn from_object_groups(groups: &[tiled::ObjectGroup]) -> Self {
        let mut level_start = None;

        for group in groups {
            let tiled::ObjectGroup {
                name,
                opacity: _,
                visible: _,
                objects,
                colour: _,
                layer_index: _,
                properties: _,
            } = group;

            if name != "markers" {
                continue;
            }

            for obj in objects {
                let tiled::Object {
                    id: _,
                    gid: _,
                    name,
                    obj_type: _,
                    width: _,
                    height: _,
                    x,
                    y,
                    rotation: _,
                    visible: _,
                    shape,
                    properties: _,
                } = obj;

                let point_shape = tiled::ObjectShape::Rect {width: 0.0, height: 0.0};
                if name == "level_start" && *shape == point_shape {
                    if level_start.is_some() {
                        println!("Warning: multiple `level_start` markers defined");
                    }
                    level_start = Some(Point::new(*x as i32, *y as i32));
                }
            }
        }

        Self {level_start}
    }
}

pub struct Level {
    world: World,
    viewport: Rect,
    level_start: Option<Point>,
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
        Self {
            world: World::new(),
            viewport: Rect::new(
                0,
                0,
                game.window_width(),
                game.window_height(),
            ),
            level_start: None,
        }
    }

    pub fn load(&mut self, map: &TileMap, renderer: &mut Renderer) -> Result<(), Unsupported> {
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

        let markers = Markers::from_object_groups(&object_groups);
        dbg!(markers);

        let background_color = match background_color {
            Some(tiled::Colour {red, green, blue}) => Color {r: red, g: green, b: blue, a: 255},
            None => Color::RGBA(0, 0, 0, 0),
        };

        //TODO: Check if we have an entity with the Player component, and if so
        // add a Position component. Otherwise just store the position for later

        Ok(())
    }

    pub fn add_player(&mut self) -> Entity {
        let level_start = self.level_start.unwrap_or_else(|| Point::new(0, 0));
        todo!()
    }

    pub fn set_viewport_dimensions(&mut self, size: Size) {
        let Size {width, height} = size;
        self.viewport.set_width(width);
        self.viewport.set_height(height);
    }
}
