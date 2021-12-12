use std::path::Path;
use std::collections::HashMap;

use tiled::Tileset;

use crate::{
    assert_support,
    unsupported,
    Size,
    Point2,
    Vec2,
    Tile,
    TileImage,
    Shape,
    Align,
    ImageCache,
    ShapeRect,
    ShapeCircle,
    ShapePolyline,
    ShapeConvexPolygon,
};

use super::{TileId, LoadError, Unsupported, resolve_image_path};

pub fn load_tilesets(
    base_dir: &Path,
    tilesets: &[Tileset],
    image_cache: &mut ImageCache,
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

            let image_id = image_cache.add(image_path);

            let image = TileImage {
                id: image_id,
                size: Size {
                    width: width as u32,
                    height: height as u32,
                },
                //TODO: Get alignment from <tileset> tag
                align: Align::default(),
            };

            let collision_geometry = match &objectgroup {
                Some(objectgroup) => {
                    let tiled::ObjectGroup {
                        name: _,
                        opacity: _,
                        visible: _,
                        objects,
                        colour: _,
                        layer_index: _,
                        properties: _,
                    } = objectgroup;

                    objects.iter()
                        .map(object_to_collision_geometry)
                        .collect::<Result<Vec<_>, _>>()?
                },
                None => Vec::new(),
            };

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

fn object_to_collision_geometry(obj: &tiled::Object) -> Result<(Vec2, Shape), Unsupported> {
    let tiled::Object {
        id,
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
        tiled::ObjectShape::Point(..) => {
            unsupported!("invalid collision geometry: single point shapes are not supported (object ID = {}", id);
        },

        &tiled::ObjectShape::Rect {width, height} => {
            let half_extents = Vec2::new(width as f64/2.0, height as f64/2.0);
            Shape::Rect(ShapeRect::new(half_extents))
        },

        &tiled::ObjectShape::Ellipse {width, height} => {
            assert_support!((width - height).abs() < 0.001,
                "invalid collision geometry: only circles that have an equal width and height are supported (object ID = {})", id);
            let radius = width as f64 / 2.0;
            Shape::Circle(ShapeCircle::new(radius))
        },

        tiled::ObjectShape::Polyline {points} => {
            let points = points.iter()
                .map(|&(x, y)| Point2::new(x as f64, y as f64))
                .collect();
            Shape::Polyline(ShapePolyline::new(points, None))
        },

        tiled::ObjectShape::Polygon {points} => {
            let points: Vec<_> = points.iter()
                .map(|&(x, y)| Point2::new(x as f64, y as f64))
                .collect();
            let polygon = ShapeConvexPolygon::try_from_points(&points)
                .ok_or_else(|| Unsupported(format!("invalid collision geometry: only convex polygons are supported (object ID = {})", id)))?;
            Shape::ConvexPolygon(polygon)
        },
    };

    Ok((position, shape))
}
