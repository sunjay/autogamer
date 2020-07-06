use std::path::Path;

#[derive(Debug)]
pub struct TileMap {
    map: tiled::Map,
}

impl TileMap {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, tiled::TiledError> {
        let map = tiled::parse_file(path.as_ref())?;
        Ok(Self {map})
    }

    pub fn as_map(&self) -> &tiled::Map {
        &self.map
    }

    pub fn tile_width(&self) -> u32 {
        self.map.tile_width
    }

    pub fn tile_height(&self) -> u32 {
        self.map.tile_height
    }
}
