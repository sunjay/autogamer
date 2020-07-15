use std::collections::HashMap;

use noisy_float::types::R64;
use sdl2::rect::Rect;

use crate::{Size, Vec2, TileId, ImageId};

/// Defines how an image is to be aligned
///
/// For images within a tile, this defines how the image is to be aligned with
/// the tile rectangle. For example, an alignment of `BottomLeft` means that the
/// image will be aligned with the bottom right corner of the tile.
///
/// For images associated with an entity (a tile object in Tiled), this defines
/// how the image will be aligned with the position of that entity. For example,
/// an alignment of `BottomLeft` means that the image will be aligned so that
/// its bottom left corner is at the position of the entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Align {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Default for Align {
    fn default() -> Self {
        // See: https://doc.mapeditor.org/en/stable/reference/tmx-map-format/#tileset
        // This default is only valid in orthogonal mode
        Align::BottomLeft
    }
}

#[derive(Debug, Clone)]
pub struct TileImage {
    /// The ID of the image in the renderer image cache (for quick lookups
    /// without needing to store the path here)
    pub id: ImageId,
    /// The size of the image in pixels
    pub size: Size,
    /// The alignment of this image within its containing tile
    pub align: Align,
}

#[derive(Debug, Clone)]
pub enum Shape {
    Rect {width: f64, height: f64},
    Ellipse {width: f64, height: f64},
    Polyline {points: Vec<Vec2>},
    Polygon {points: Vec<Vec2>},
}

#[derive(Debug, Clone)]
pub struct CollisionGeometry {
    pub position: Vec2,
    /// Coordinates in the shape are relative to the position of this geometry
    pub shape: Shape,
}

#[derive(Debug)]
pub struct Tile {
    pub id: TileId,
    /// The image drawn at this tile's position
    pub image: TileImage,
    /// Any coordinates in the geometry are relative to the position of the tile
    pub collision_geometry: Vec<CollisionGeometry>,
    /// The type field provided in the tile map (possibly an empty string)
    pub tile_type: String,
    /// Any custom properites on the tile itself
    ///
    /// These will be merged (and potentially overridden) by custom properties
    /// on an object containing this tile.
    pub props: HashMap<String, tiled::PropertyValue>,
}

/// An image that can be drawn on the screen
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    /// The ID of the image in the renderer image cache (for quick lookups
    /// without needing to store the path here)
    pub id: ImageId,
    /// The region of the image to draw
    ///
    /// If `None`, the entire image is drawn.
    pub src: Option<Rect>,
    /// Defines the alignment of the image
    ///
    /// See the documentation on the `Align` type for more info.
    pub align: Align,
    /// Additional parameters used when drawing the image
    pub params: ImageParams,
}

/// Additional parameters used when rendering an image
///
/// Flips are performed before rotation. Rotation is performed around the
/// center of the image being drawn.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageParams {
    /// The size in pixels at which to draw the image (can be different from the
    /// size of the image stored in the image file)
    pub size: Size,
    /// true if the image should be flipped horizontally
    pub flip_horizontal: bool,
    /// true if the image should be flipped vertically
    pub flip_vertical: bool,
    /// The angle in degrees by which to rotate the image clockwise
    pub angle: R64,
    /// The alpha with which to draw the image (between 0 and 255)
    pub alpha: u8,
}

#[derive(Debug)]
pub struct TileLayer {
    /// The offset of all tiles in this layer
    pub offset: Vec2,
    /// The number of rows in the grid of tiles
    pub nrows: usize,
    /// The number of columns in the grid of tiles
    pub ncols: usize,
    /// The tiles in the layer, stored row-wise
    pub tiles: Vec<Vec<Option<Image>>>,
}

#[derive(Debug, Default)]
pub struct ExtraLayers {
    /// The layers that should be drawn in front of the map layer, in drawing
    /// order (back to front)
    ///
    /// These layers appear above the map layer in Tiled.
    pub front_layers: Vec<TileLayer>,
    /// The layers that should be drawn behind the map layer, in drawing
    /// order (back to front)
    ///
    /// These layers appear below the map layer in Tiled.
    pub back_layers: Vec<TileLayer>,
}
