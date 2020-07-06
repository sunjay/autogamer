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
}
