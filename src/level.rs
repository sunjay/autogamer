mod image_params;
mod load_tilesets;
mod load_layers;
mod load_objects;

use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use thiserror::Error;
use sdl2::{pixels::Color, rect::{Point, Rect}};
use specs::{
    World,
    WorldExt,
    Join,
    Entity,
    Builder,
    SystemData,
    ReadStorage,
    WriteStorage,
    prelude::ResourceId,
};

use crate::{
    Game,
    TileMap,
    Size,
    PhysicsEngine,
    Player,
    Position,
    Sprite,
    Vec2,
    ExtraLayers,
    TemplateError,
    Renderer,
    ImageCache,
    TileLayer,
    Image,
    ImageParams,
    SdlError,
    Align,
    EventStream,
    EventStreamSource,
    EventKind,
    Modifiers,
    Systems,
    Viewport,
};

use load_tilesets::load_tilesets;
use load_layers::load_layers;
use load_objects::load_objects;

/// The draw order value of tiles inserted into the world from the map layer
pub(crate) const TILE_DRAW_ORDER: u8 = 0;
/// The draw order value of objects inserted into the world from objects
pub(crate) const OBJECT_DRAW_ORDER: u8 = 1;
/// The draw order value of sprites inserted into the world from character spritesheets
pub(crate) const CHARACTER_DRAW_ORDER: u8 = 2;

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

#[macro_export]
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

#[derive(SystemData)]
struct RenderData<'a> {
    pub positions: ReadStorage<'a, Position>,
    pub sprites: ReadStorage<'a, Sprite>,
}

pub struct Level {
    world: World,
    systems: Systems,
    /// The area of the world (in world coordinates) that will be drawn by the
    /// renderer and scaled to fit in the window
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
            systems: _,
            viewport,
            level_start,
            tile_size,
            extra_layers,
            background_color,
            loaded,
        } = self;

        f.debug_struct("Level")
            .field("world", &"World {..}")
            .field("systems", &"Systems {..}")
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
        let default_viewport = Rect::new(
            0,
            0,
            game.window_width(),
            game.window_height(),
        );

        let mut world = World::new();
        crate::register_components(&mut world);
        // Setup resources
        world.insert(EventStream::default());
        world.insert(Viewport(default_viewport));

        let mut systems = Systems::default();
        systems.setup(&mut world);

        Self {
            world,
            systems,
            viewport: default_viewport,
            level_start: None,
            tile_size: Size {width: 1, height: 1},
            extra_layers: ExtraLayers::default(),
            background_color: Color::BLACK,
            loaded: false,
        }
    }

    pub fn world(&self) -> &World {
        &self.world
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
            systems: _,
            viewport: _,
            level_start,
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
        load_layers(nrows, ncols, layers, *tile_size, &tiles, world, extra_layers)?;
        load_objects(object_groups, &tiles, world, level_start)?;

        // Update any existing players based on the loaded level start
        if let Some(level_start) = *level_start {
            let (players, mut positions): (ReadStorage<Player>, WriteStorage<Position>) = world.system_data();
            for (Player, Position(pos)) in (&players, &mut positions).join() {
                *pos = level_start;
            }
        }

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

    pub fn update<E>(&mut self, events: &E, physics: &mut PhysicsEngine)
        where E: EventStreamSource,
    {
        // Update events
        self.world.write_resource::<EventStream>().refill(events);

        // Allow debug controls to handle events first (and potentially override
        // game behaviour)
        self.handle_debug_controls();

        // Update physics parameters
        self.systems.physics.set_gravity(physics.gravity());

        // Store the value of the viewport resource before the system is run
        let prev_viewport = (*self.world.read_resource::<Viewport>()).clone();

        // Run dispatcher
        self.systems.run(&mut self.world);
        self.world.maintain();

        // Only update the actual viewport if the systems changed the viewport
        // resource
        //
        // This allows the debug controls to work but ensures the viewport
        // target moving in the world still causes an update
        let next_viewport = (*self.world.read_resource::<Viewport>()).clone();
        if prev_viewport != next_viewport {
            let Viewport(viewport) = next_viewport;
            self.viewport = viewport;
        }
    }

    fn handle_debug_controls(&mut self) {
        let viewport = &mut self.viewport;

        let events = self.world.read_resource::<EventStream>();
        for event in events.iter() {
            use crate::Key;
            match event.kind() {
                EventKind::KeyDown {
                    key: Key::Up,
                    modifiers: Modifiers {ctrl_pressed: true, ..},
                    ..
                } => {
                    viewport.set_y(viewport.y() - 35);
                    event.stop_propagation();
                },
                EventKind::KeyDown {
                    key: Key::Down,
                    modifiers: Modifiers {ctrl_pressed: true, ..},
                    ..
                } => {
                    viewport.set_y(viewport.y() + 35);
                    event.stop_propagation();
                },
                EventKind::KeyDown {
                    key: Key::Left,
                    modifiers: Modifiers {ctrl_pressed: true, ..},
                    ..
                } => {
                    viewport.set_x(viewport.x() - 35);
                    event.stop_propagation();
                },
                EventKind::KeyDown {
                    key: Key::Right,
                    modifiers: Modifiers {ctrl_pressed: true, ..},
                    ..
                } => {
                    viewport.set_x(viewport.x() + 35);
                    event.stop_propagation();
                },
                _ => {},
            }
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) -> Result<(), SdlError> {
        let Self {
            ref world,
            systems: _,
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

        // Scale the viewport coordinates so they are in screen coordinates
        let screen_viewport = Rect::new(
            (viewport.x() as f64 * scale_x) as i32,
            (viewport.y() as f64 * scale_y) as i32,
            viewport.width(),
            viewport.height(),
        );

        renderer.clear(background_color);

        let ExtraLayers {front_layers, back_layers} = extra_layers;

        for layer in back_layers {
            draw_layer(
                renderer,
                layer,
                screen_viewport,
                tile_size,
                (scale_x, scale_y),
            )?;
        }

        let RenderData {
            positions,
            sprites,
        } = world.system_data();

        for (Position(world_pos), sprite) in (&positions, &sprites).join() {
            let &world_pos = world_pos;
            let &Sprite {ref image, align_size, pivot, draw_order} = sprite;

            draw_image(
                renderer,
                image,
                world_pos,
                align_size,
                pivot,
                screen_viewport,
                (scale_x, scale_y),
            )?;
        }

        for layer in front_layers {
            draw_layer(
                renderer,
                layer,
                screen_viewport,
                tile_size,
                (scale_x, scale_y),
            )?;
        }

        renderer.present();

        Ok(())
    }
}

fn draw_layer(
    renderer: &mut Renderer,
    layer: &TileLayer,
    screen_viewport: Rect,
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

            draw_image(
                renderer,
                image,
                world_pos,
                tile_size,
                // Tiles should rotate about the center of their image
                //TODO: This probably won't work...we need to rotate and then
                // translate to move back to the right position (so align is enforced)
                None,
                screen_viewport,
                (scale_x, scale_y),
            )?;
        }
    }

    Ok(())
}

/// The `world_pos` and `world_size` parameters represent the top-left corner
/// size of the rectangle used to align the image. Use a size of (0, 0) when
/// drawing something that is only represented by a position (e.g. an entity).
fn draw_image(
    renderer: &mut Renderer,
    image: &Image,
    world_pos: Vec2,
    world_size: Size,
    pivot: Option<Point>,
    screen_viewport: Rect,
    (scale_x, scale_y): (f64, f64),
) -> Result<(), SdlError> {
    let &Image {
        id,
        src,
        align,
        ref params,
    } = image;

    // Update the size to be the size in screen coordinates
    let image_width = (params.size.width as f64 * scale_x) as i32;
    let image_height = (params.size.height as f64 * scale_y) as i32;

    // The position and size of the object we're drawing (potentially different
    // than the image position and size because of alignment)
    let screen_pos = Point::new(
        (world_pos.x * scale_x) as i32,
        (world_pos.y * scale_y) as i32,
    );
    let screen_width = (world_size.width as f64 * scale_x) as i32;
    let screen_height = (world_size.height as f64 * scale_y) as i32;

    // The screen position for the **top left** of the image, computed based on
    // the alignment
    let image_screen_top_left = match align {
        Align::TopLeft => {
            screen_pos
        },
        Align::Top => {
            screen_pos + Point::new(screen_width/2 - image_width/2, 0)
        },
        Align::TopRight => {
            screen_pos + Point::new(screen_width - image_width, 0)
        },
        Align::Left => {
            screen_pos + Point::new(0, screen_height/2 - image_height/2)
        },
        Align::Center => {
            screen_pos + Point::new(
                screen_width/2 - image_width/2,
                screen_height/2 - image_height/2,
            )
        },
        Align::Right => {
            screen_pos + Point::new(
                screen_width - image_width,
                screen_height/2 - image_height/2,
            )
        },
        Align::BottomLeft => {
            screen_pos + Point::new(0, screen_height - image_height)
        },
        Align::Bottom => {
            screen_pos + Point::new(
                screen_width/2 - image_width/2,
                screen_height - image_height,
            )
        },
        Align::BottomRight => {
            screen_pos + Point::new(
                screen_width - image_width,
                screen_height - image_height,
            )
        },
    };

    // The area of the screen used by this image
    let image_screen_rect = Rect::new(
        image_screen_top_left.x(),
        image_screen_top_left.y(),
        // Using image_size not screen_width/screen_height because the image
        // dimensions can be different than the size of the item being drawn
        image_width as u32,
        image_height as u32,
    );

    // Render the image if it is even partially in the screen viewport
    if screen_viewport.has_intersection(image_screen_rect) {
        let params = ImageParams {
            size: Size {
                width: image_screen_rect.width(),
                height: image_screen_rect.height(),
            },
            ..params.clone()
        };

        renderer.draw_image(
            id,
            src,
            pivot,
            params,
            // Position is relative to the top left of the viewport
            image_screen_rect.top_left() - screen_viewport.top_left(),
        )?;
    }

    Ok(())
}
