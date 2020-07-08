use std::collections::HashMap;

use specs::{World, WorldExt};

use crate::{ExtraLayers, Tile, TileLayer, TileLayerItem, Vec2};

use super::TileId;

pub fn load_layers(
    nrows: u32,
    ncols: u32,
    layers: &[tiled::Layer],
    tiles: &HashMap<TileId, Tile>,
    world: &mut World,
    extra_layers: &mut ExtraLayers,
) {
    let mut prev_layer_index = 0;
    let mut found_map = false;
    for layer in layers {
        let tiled::Layer {
            ref name,
            opacity,
            // A layer's visibility in Tiled does not affect its visibility here
            visible: _,
            tiles: ref layer_tiles,
            properties: _,
            layer_index,
        } = *layer;

        assert!(layer_index >= prev_layer_index,
            "bug: this code assumes that the layers are stored in draw order");
        prev_layer_index = layer_index;

        if name.trim().eq_ignore_ascii_case("map") {
            if found_map {
                // Not sure if having multiple map layers will cause problems.
                // Going to disable it for now until someone asks for it.
                println!("Warning: only a single layer should be named `map` (ignoring layer)");
                continue;
            }

            load_map_layer(layer_tiles, opacity as f64, tiles, world);
            found_map = true;

        } else {
            let layer = to_extra_layer(
                layer_tiles,
                opacity as f64,
                nrows as usize,
                ncols as usize,
            );

            if found_map {
                extra_layers.front_layers.push(layer);
            } else {
                extra_layers.back_layers.push(layer);
            }
        }
    }
}

fn load_map_layer(
    layer_tiles: &[Vec<tiled::LayerTile>],
    opacity: f64,
    tiles: &HashMap<TileId, Tile>,
    world: &mut World,
) {
    for tile in layer_tiles.iter().flatten() {
        let &tiled::LayerTile {gid, flip_h, flip_v, flip_d} = tile;
        let id = TileId(gid);
        todo!()
    }
}

fn to_extra_layer(
    layer_tiles: &[Vec<tiled::LayerTile>],
    opacity: f64,
    nrows: usize,
    ncols: usize,
) -> TileLayer {
    //TODO: Use the actual layer offset once we are using a library that
    // actually provides that to us
    let offset = Vec2::default();

    let mut grid_tiles = Vec::new();
    for row in layer_tiles {
        assert!(row.len() <= ncols,
            "expected `{}` tiles in layer row, found `{}` tiles", ncols, row.len());

        let mut grid_row = Vec::new();
        for tile in row {
            let &tiled::LayerTile {
                gid,
                flip_h: flip_horizontal,
                flip_v: flip_vertical,
                flip_d: flip_diagonal,
            } = tile;

            // Tiled global IDs always start at 1, so 0 is used to indicate an
            // empty tile
            if gid == 0 {
                grid_row.push(None)

            } else {
                let tile_id = TileId(gid);
                grid_row.push(Some(TileLayerItem {
                    tile_id,
                    flip_horizontal,
                    flip_vertical,
                    flip_diagonal,
                }));
            }
        }

        grid_tiles.push(grid_row);
    }

    TileLayer {offset, nrows, ncols, tiles: grid_tiles, opacity}
}
