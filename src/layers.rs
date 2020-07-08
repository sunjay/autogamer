use std::collections::HashMap;

use crate::{Size, Vec2, TileId, ImageId};

/// Defines how a tile image is aligned with respect to its position
///
/// For example, an alignment of `BottomLeft` means that the image will be drawn
/// so that its bottom left corner is at its position.
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
    pub image: TileImage,
    /// Any coordinates in the geometry are relative to the position of the tile
    pub collision_geometry: Vec<CollisionGeometry>,
    //TODO: inspect tile type field and generate a ComponentTemplate that knows
    // how to add those components to an entity
}

#[derive(Debug)]
pub struct TileLayerItem {
    pub tile_id: TileId,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    pub flip_diagonal: bool,
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
    pub tiles: Vec<Vec<Option<TileLayerItem>>>,
    /// The opacity at which all tiles in this layer will be rendered
    pub opacity: f64,
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
