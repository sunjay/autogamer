use std::collections::HashMap;
use std::convert::TryInto;

use specs::{World, WorldExt, Builder};

use crate::{
    Size,
    Vec2,
    Tile,
    TileImage,
    Image,
    ImageParams,
    Sprite,
    Position,
    ApplyComponentTemplates,
    JointHashMap,
};

use super::{LoadError, TileId, OBJECT_DRAW_ORDER};

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

        //TODO: Use the actual layer offset once we are using a library that
        // actually provides that to us
        let layer_offset = Vec2::default();

        for object in objects {
            let &tiled::Object {
                id,
                gid,
                name: _,
                ref obj_type,
                width: _,
                height: _,
                x,
                y,
                rotation,
                visible: _,
                ref shape,
                ref properties,
            } = object;

            //TODO: Tiled sometimes uses rotation to simulate flips **instead**
            // of using the horizontal or vertical flip properties.
            // (e.g. pressing the horizontal flip button will sometimes result
            // in rotation = 180 and flip_vertical = true)
            if rotation != 0.0 {
                println!("Warning: rotation field on objects is not supported yet (ID = {})", id);
            }

            let world_pos = Vec2::new(
                x as f64 + layer_offset.x,
                y as f64 + layer_offset.y,
            );

            // Tiled global IDs always start at 1, so 0 is used to indicate that
            // no tile is associated with this object
            if gid == 0 {
                apply_object_templates(
                    id,
                    obj_type,
                    world_pos,
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
                    world_pos,
                    shape,
                    properties,
                    world,
                )?;
            }
        }
    }

    Ok(())
}

fn apply_object_templates(
    id: u32,
    obj_type: &str,
    world_pos: Vec2,
    shape: &tiled::ObjectShape,
    props: &HashMap<String, tiled::PropertyValue>,
    world: &mut World,
    level_start: &mut Option<Vec2>,
) -> Result<(), LoadError> {
    match obj_type {
        "level_start" => {
            let point_shape = tiled::ObjectShape::Rect {width: 0.0, height: 0.0};
            if *shape == point_shape {
                if level_start.is_some() {
                    println!("Warning: ignoring duplicate `level_start` indicator (ID = {})", id);

                } else {
                    *level_start = Some(world_pos);
                }

            } else {
                println!("Warning: The `level_start` indicator should to be a single point (ID = {})", id);
            }
        },

        //TODO: Process other object types

        _ => {},
    }

    Ok(())
}

fn apply_tile_object_templates(
    tile: &Tile,
    image: Image,
    obj_type: &str,
    world_pos: Vec2,
    shape: &tiled::ObjectShape,
    obj_props: &HashMap<String, tiled::PropertyValue>,
    world: &mut World,
) -> Result<(), LoadError> {
    // Tile object positions are already set with the alignment in mind so we
    // can get the correct alignment by assuming that we're aligning with a
    // single point
    let align_size = Size {width: 0, height: 0};

    let sprite = Sprite {
        image,
        align_size,
        draw_order: OBJECT_DRAW_ORDER,
    };

    let Tile {
        id,
        image: _,
        //TODO: Insert collision geometry or a default rectangle geometry
        // based on the object shape if this is empty
        collision_geometry,
        tile_type,
        props: tile_props,
    } = tile;

    // The type specified on the object overrides the type specified on the tile
    let obj_tile_type = if obj_type.is_empty() {
        tile_type
    } else {
        obj_type
    };
    // Allow object properties to override tile properties
    let props = JointHashMap {
        base: tile_props,
        data: obj_props,
    };

    world.create_entity()
        .with(Position(world_pos))
        .with(sprite)
        .apply_templates(*id, obj_tile_type, &props)?
        .build();

    Ok(())
}
