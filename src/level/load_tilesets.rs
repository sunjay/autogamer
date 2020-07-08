use std::path::Path;
use std::collections::HashMap;

use tiled::Tileset;

use crate::{Renderer, Size, Vec2, Tile, TileImage, CollisionGeometry, Shape, Align};

use super::{TileId, LoadError, resolve_image_path};

fn object_to_collision_geometry(obj: &tiled::Object) -> CollisionGeometry {
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

    CollisionGeometry {position, shape}
}

pub fn load_tilesets(
    base_dir: &Path,
    tilesets: &[Tileset],
    renderer: &mut Renderer,
) -> Result<HashMap<TileId, Tile>, LoadError> {
    let mut tiles = HashMap::new();

    for tileset in tilesets {
        let Tileset {
            first_gid,
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
                animation: _,
                tile_type,
                probability: _,
            } = tile;

            if images.len() != 1 {
                println!("Warning: Tile with ID {} does not have exactly 1 image (ignoring tile)", id);
                continue;
            }

            let id = TileId(*first_gid + *id);

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

            let image_id = renderer.image_cache_mut().add(image_path);

            let image = TileImage {
                id: image_id,
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

                objects.iter().map(object_to_collision_geometry).collect()
            }).unwrap_or_default();

            let tile = Tile {
                id,
                image,
                collision_geometry,
                tile_type: tile_type.clone().unwrap_or_default(),
                props: properties.clone(),
            };

            assert!(tiles.insert(id, tile).is_none(),
                "bug: tile ID should be unique");
        }
    }

    Ok(tiles)
}
