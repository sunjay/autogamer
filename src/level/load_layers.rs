use std::collections::HashMap;

use specs::{World, WorldExt};

use crate::{TileId, ExtraLayers, Tile, TileLayer};

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
            name,
            opacity,
            // A layer's visibility in Tiled does not affect its visibility here
            visible: _,
            tiles: layer_tiles,
            properties: _,
            layer_index,
        } = layer;

        assert!(*layer_index >= prev_layer_index,
            "bug: this code assumes that the layers are stored in draw order");
        prev_layer_index = *layer_index;

        if name.trim().eq_ignore_ascii_case("map") {
            load_map_layer(tiles, world, layer_tiles, opacity);
            found_map = true;
        } else {
            let layer = read_extra_layer(nrows, ncols, tiles, extra_layers, layer_tiles, opacity);

            if found_map {
                extra_layers.front_layers.push(layer);
            } else {
                extra_layers.back_layers.push(layer);
            }
        }
    }
}

fn load_map_layer(
    tiles: &HashMap<TileId, Tile>,
    world: &mut World,
    layer_tiles: &[Vec<tiled::LayerTile>],
    opacity: &f32,
) {
    todo!()
}

fn read_extra_layer(
    nrows: u32,
    ncols: u32,
    tiles: &HashMap<TileId, Tile>,
    extra_layers: &mut ExtraLayers,
    layer_tiles: &[Vec<tiled::LayerTile>],
    opacity: &f32,
) -> TileLayer {
    todo!()
}
