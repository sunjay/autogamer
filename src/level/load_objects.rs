use std::collections::HashMap;
use std::convert::TryInto;

use specs::World;

use crate::{Size, Vec2, Tile, TileImage, Image, ImageParams};

use super::{LoadError, TileId};

pub fn load_objects(
    object_groups: &[tiled::ObjectGroup],
    tiles: &HashMap<TileId, Tile>,
    world: &mut World,
    level_start: &mut Option<Vec2>,
) -> Result<(), LoadError> {
    for group in object_groups {
        let &tiled::ObjectGroup {
            name: _,
            opacity,
            visible: _,
            ref objects,
            // Objects aren't drawn, so we can ignore the color
            colour: _,
            layer_index: _,
            properties: _,
        } = group;
        let opacity = opacity as f64;

        for object in objects {
            let &tiled::Object {
                id,
                gid,
                name: _,
                ref obj_type,
                width,
                height,
                x,
                y,
                rotation,
                visible: _,
                ref shape,
                ref properties,
            } = object;

            if rotation != 0.0 {
                println!("Warning: rotation field on objects is not supported yet (ID = {})", id);
            }

            let pos = Vec2::new(x as f64, y as f64);

            // Tiled global IDs always start at 1, so 0 is used to indicate that
            // no tile is associated with this object
            if gid == 0 {
                apply_object_templates(
                    obj_type,
                    pos,
                    shape,
                    properties,
                    world,
                    level_start,
                )?;

            } else {
                // The tiled crate doesn't account for flipping info in the gid
                // so we have to do it manually
                // See: https://docs.rs/tiled/0.9.2/src/tiled/lib.rs.html#639-660
                const FLIPPED_HORIZONTALLY_FLAG: u32 = 0x80000000;
                const FLIPPED_VERTICALLY_FLAG: u32 = 0x40000000;
                const FLIPPED_DIAGONALLY_FLAG: u32 = 0x20000000;
                const ALL_FLIP_FLAGS: u32 = FLIPPED_HORIZONTALLY_FLAG
                    | FLIPPED_VERTICALLY_FLAG
                    | FLIPPED_DIAGONALLY_FLAG;

                let flags = gid & ALL_FLIP_FLAGS;
                let gid = gid & !ALL_FLIP_FLAGS;
                // Swap x and y axis (anti-diagonally) [flips over y = -x line]
                let flip_diagonal = flags & FLIPPED_DIAGONALLY_FLAG == FLIPPED_DIAGONALLY_FLAG;
                // Flip tile over y axis
                let flip_horizontal = flags & FLIPPED_HORIZONTALLY_FLAG == FLIPPED_HORIZONTALLY_FLAG;
                // Flip tile over x axis
                let flip_vertical = flags & FLIPPED_VERTICALLY_FLAG == FLIPPED_VERTICALLY_FLAG;

                let tile_id = TileId(gid);
                let tile = &tiles[&tile_id];

                let TileImage {
                    id: image_id,
                    size,
                    align,
                } = tile.image;

                let image = Image {
                    id: image_id,
                    align,
                    params: ImageParams {
                        size,
                        flip_horizontal,
                        flip_vertical,
                        flip_diagonal,
                        opacity: opacity.try_into()
                            .expect("opacity should have been between 0.0 and 1.0"),
                    },
                };

                apply_tile_object_templates(
                    tile,
                    image,
                    obj_type,
                    pos,
                    shape,
                    properties,
                )?;
            }
        }
    }
    Ok(())
}

fn apply_object_templates(
    obj_type: &str,
    pos: Vec2,
    shape: &tiled::ObjectShape,
    props: &HashMap<String, tiled::PropertyValue>,
    world: &mut World,
    level_start: &mut Option<Vec2>,
) -> Result<(), LoadError> {
    //TODO
    // let point_shape = tiled::ObjectShape::Rect {width: 0.0, height: 0.0};
    // if name == "level_start" && *shape == point_shape
    Ok(())
}

fn apply_tile_object_templates(
    tile: &Tile,
    image: Image,
    obj_type: &str,
    pos: Vec2,
    shape: &tiled::ObjectShape,
    props: &HashMap<String, tiled::PropertyValue>,
) -> Result<(), LoadError> {
    todo!()
}
