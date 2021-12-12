use std::collections::HashMap;

use specs::{World, WorldExt, Builder};
use noisy_float::types::R64;

use crate::{
    Size,
    ExtraLayers,
    Tile,
    TileImage,
    TileLayer,
    Vec2,
    Image,
    ImageParams,
    Sprite,
    ApplyComponentTemplates,
    Position,
    Shape,
    PhysicsCollider,
    EntityEditor,
    unsupported,
};

use super::{TILE_DRAW_ORDER, TileId, LoadError, image_params::TiledImageParams};

pub fn load_layers(
    nrows: u32,
    ncols: u32,
    layers: &[tiled::Layer],
    tile_size: Size,
    tiles: &HashMap<TileId, Tile>,
    world: &mut World,
    extra_layers: &mut ExtraLayers,
) -> Result<(), LoadError> {
    let mut prev_layer_index = 0;
    let mut found_map = false;
    for layer in layers {
        let tiled::Layer {
            ref name,
            opacity,
            // A layer's visibility in Tiled does not affect its visibility here
            visible: _,
            tiles: ref layer_data,
            properties: _,
            layer_index,
            offset_x,
            offset_y,
        } = *layer;

        let layer_offset = Vec2::new(offset_x as f64, offset_y as f64);

        assert!(layer_index >= prev_layer_index,
            "bug: this code assumes that the layers are stored in draw order");
        prev_layer_index = layer_index;

        let layer_tiles = match layer_data {
            tiled::LayerData::Finite(layer_tiles) => layer_tiles,
            //TODO: Infinite maps should be reasonably easy to support if we make an iterator over
            // LayerData that yields world_pos and LayerTile
            tiled::LayerData::Infinite(..) => unsupported!("infinite map layers are not supported yet"),
        };

        if name.trim().eq_ignore_ascii_case("map") {
            if found_map {
                // Not sure if having multiple map layers will cause problems.
                // Going to disable it for now until someone asks for it.
                println!("Warning: only a single layer should be named `map` (ignoring layer)");
                continue;
            }

            load_map_layer(
                layer_tiles,
                layer_offset,
                tile_size,
                opacity as f64,
                tiles,
                world,
            )?;

            found_map = true;

        } else {
            let layer = to_extra_layer(
                layer_tiles,
                layer_offset,
                opacity as f64,
                nrows as usize,
                ncols as usize,
                tiles,
            );

            if found_map {
                extra_layers.front_layers.push(layer);
            } else {
                extra_layers.back_layers.push(layer);
            }
        }
    }

    Ok(())
}

fn load_map_layer(
    layer_tiles: &[Vec<tiled::LayerTile>],
    offset: Vec2,
    tile_size: Size,
    opacity: f64,
    tiles: &HashMap<TileId, Tile>,
    world: &mut World,
) -> Result<(), LoadError> {
    for (row_i, row) in (0u32..).zip(layer_tiles) {
        for (col_i, tile) in (0u32..).zip(row) {
            let (tile, image) = match process_layer_tile(tiles, tile, opacity) {
                Some((tile, image)) => (tile, image),
                None => continue,
            };

            // Compute the position of the tile in world coordinates
            let world_pos = Vec2::new(
                (col_i * tile_size.width) as f64 + offset.x,
                (row_i * tile_size.height) as f64 + offset.y,
            );

            let sprite = Sprite {
                image,
                align_size: tile_size,
                pivot: None,
                draw_order: TILE_DRAW_ORDER,
            };

            let Tile {
                id,
                image,
                collision_geometry,
                tile_type,
                props,
            } = tile;

            // Insert collision geometry as a collider, defaulting to a
            // rectangle based on the image size if no geometry was provided
            let shape = Shape::from_shapes(collision_geometry)
                .unwrap_or_else(|| Shape::rect(image.size));

            let collider = PhysicsCollider {
                shape,
                collision_groups: PhysicsCollider::ground_collision_groups(),
                ..PhysicsCollider::default()
            };

            let entity = world.create_entity()
                .with(Position(world_pos))
                .with(sprite)
                .with(collider)
                .build();

            let entity = EntityEditor::new(world, entity);
            entity.apply_templates(*id, tile_type, props)?
        }
    }

    Ok(())
}

fn to_extra_layer(
    layer_tiles: &[Vec<tiled::LayerTile>],
    offset: Vec2,
    opacity: f64,
    nrows: usize,
    ncols: usize,
    tiles: &HashMap<TileId, Tile>,
) -> TileLayer {
    let mut grid_tiles = Vec::new();
    for row in layer_tiles {
        assert!(row.len() <= ncols,
            "expected `{}` tiles in layer row, found `{}` tiles", ncols, row.len());

        let mut grid_row = Vec::new();
        for tile in row {
            let image = process_layer_tile(tiles, tile, opacity)
                .map(|(_, image)| image);
            grid_row.push(image);
        }

        grid_tiles.push(grid_row);
    }

    TileLayer {offset, nrows, ncols, tiles: grid_tiles}
}

/// Looks up a layer tile in the tiles loaded from the tilesets and computes
/// the complete image with all parameters that should be drawn for this tile.
///
/// Returns None if the tile is empty
fn process_layer_tile<'a>(
    tiles: &'a HashMap<TileId, Tile>,
    tile: &tiled::LayerTile,
    opacity: f64,
) -> Option<(&'a Tile, Image)> {
    let &tiled::LayerTile {
        gid,
        flip_h: flip_horizontal,
        flip_v: flip_vertical,
        flip_d: flip_diagonal,
    } = tile;

    let base_params = TiledImageParams {
        flip_horizontal,
        flip_vertical,
        flip_diagonal,
    }.normalize();

    // Tiled global IDs always start at 1, so 0 is used to indicate an
    // empty tile
    if gid == 0 {
        return None;
    }

    let id = TileId(gid);
    let tile = &tiles[&id];

    let TileImage {
        id: image_id,
        size,
        align,
    } = tile.image;

    let image = Image {
        id: image_id,
        src: None,
        align,
        params: ImageParams {
            size,
            flip_horizontal: base_params.flip_horizontal,
            flip_vertical: base_params.flip_vertical,
            angle: R64::new(base_params.angle),
            alpha: (opacity * u8::MAX as f64).round() as u8,
        },
    };

    Some((tile, image))
}
