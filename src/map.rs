use thiserror::Error;
use sdl2::pixels::Color;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LoadError {
    TiledError(#[from] tiled::TiledError),
    Unsupported(#[from] Unsupported),
}

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
pub struct Map {
    ncols: u32,
    nrows: u32,
    tile_width: u32,
    tile_height: u32,
    background_color: Color,
}

impl Map {
    pub fn load(path: &str) -> Result<Self, LoadError> {
        let tiled::Map {
            version: _,
            orientation,
            width: ncols,
            height: nrows,
            tile_width,
            tile_height,
            tilesets,
            layers,
            image_layers,
            object_groups,
            properties: _,
            background_colour: background_color,
        } = tiled::parse_file(path.as_ref())?;
        assert_support!(orientation == tiled::Orientation::Orthogonal,
            "only maps with orthogonal orientation are supported");

        if !image_layers.is_empty() {
            println!("Warning: image layers are not supported yet");
        }

        let background_color = match background_color {
            Some(tiled::Colour {red, green, blue}) => Color {r: red, g: green, b: blue, a: 255},
            None => Color::RGBA(0, 0, 0, 0),
        };

        Ok(Self {ncols, nrows, tile_width, tile_height, background_color})
    }
}
