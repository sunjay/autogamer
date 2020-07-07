use std::path::PathBuf;

use crate::{Size, Vec2, TileId};

/// Defines how a tile image is aligned within the tile
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
pub struct Image {
    /// The absolute path to this image
    pub path: PathBuf,
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
    pub image: Image,
    /// Any coordinates in the geometry are relative to the position of the tile
    pub collision_geometry: Vec<CollisionGeometry>,
    //TODO: inspect tile type field and generate a ComponentTemplate that knows
    // how to add those components to an entity
}
