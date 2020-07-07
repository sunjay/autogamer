use std::path::{Path, PathBuf};
use std::collections::HashMap;

use tiled::Tileset;

use crate::{Size, Vec2};

use super::{TId, LoadError};

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

impl<'a> From<&'a tiled::Object> for CollisionGeometry {
    fn from(obj: &'a tiled::Object) -> Self {
        let tiled::Object {
            id: _,
            gid: _,
            name: _,
            obj_type: _,
            width: _,
            height: _,
            x,
            y,
            rotation: _,
            visible: _,
            ref shape,
            properties: _,
        } = *obj;

        let position = Vec2::new(x as f64, y as f64);
        let shape = match shape {
            &tiled::ObjectShape::Rect {width, height} => {
                Shape::Rect {width: width as f64, height: height as f64}
            },

            &tiled::ObjectShape::Ellipse {width, height} => {
                Shape::Ellipse {width: width as f64, height: height as f64}
            },

            tiled::ObjectShape::Polyline {points} => {
                Shape::Polyline {
                    points: points.iter().map(|&(x, y)| {
                        Vec2::new(x as f64, y as f64)
                    }).collect(),
                }
            },

            tiled::ObjectShape::Polygon {points} => {
                Shape::Polygon {
                    points: points.iter().map(|&(x, y)| {
                        Vec2::new(x as f64, y as f64)
                    }).collect(),
                }
            },
        };

        Self {position, shape}
    }
}

#[derive(Debug)]
pub struct Tile {
    pub id: TId,
    pub image: Image,
    /// Any coordinates in the geometry are relative to the position of the tile
    pub collision_geometry: Vec<CollisionGeometry>,
    //TODO: inspect tile type field and generate a ComponentTemplate that knows
    // how to add those components to an entity
}

pub fn load_tilesets(
    base_dir: &Path,
    tilesets: &[Tileset],
) -> Result<HashMap<TId, Tile>, LoadError> {
    let mut tiles = HashMap::new();

    for tileset in tilesets {
        let Tileset {
            first_gid: _,
            name,
            tile_width: _,
            tile_height: _,
            // Used when tileset is based on a single tileset image
            spacing: _,
            // Used when tileset is based on a single tileset image
            margin: _,
            //TODO: Could this be used with `tiles.reserve(...)` as a potential
            // optimization? Might be able to avoid a few allocations
            tilecount: _,
            images,
            tiles: tileset_tiles,
            properties: _,
        } = tileset;

        if !images.is_empty() {
            println!("Warning: Tileset `{}` is based on a single Tileset image and is not supported yet (ignoring tileset)", name);
            continue;
        }

        for tile in tileset_tiles {
            let tiled::Tile {
                id,
                images,
                properties,
                objectgroup,
                animation,
                tile_type,
                probability: _,
            } = tile;

            if images.len() != 1 {
                println!("Warning: Tile with ID {} does not have exactly 1 image (ignoring tile)", id);
                continue;
            }

            let id = TId(*id);

            let &tiled::Image {
                ref source,
                width,
                height,
                transparent_colour: transparent_color,
            } = &images[0];

            let image_path = resolve_image_path(base_dir, source)?;

            if transparent_color.is_some() {
                println!("Warning: image `{}` specifies a transparent color which is not supported yet (ignoring transparent color)", image_path.display());
            }

            let image = Image {
                path: image_path,
                size: Size {
                    width: width as u32,
                    height: height as u32,
                },
                align: Align::default(),
            };

            let collision_geometry = objectgroup.as_ref().map(|objectgroup| {
                let tiled::ObjectGroup {
                    name: _,
                    opacity: _,
                    visible: _,
                    objects,
                    colour: _,
                    layer_index: _,
                    properties: _,
                } = objectgroup;

                objects.iter().map(|object| object.into()).collect()
            }).unwrap_or_default();

            let tile = Tile {
                id,
                image,
                collision_geometry,
            };

            assert!(tiles.insert(id, tile).is_none(),
                "bug: tile ID should be unique");
        }
    }

    Ok(tiles)
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
