use std::path::{Path, PathBuf};

use autogamer as ag;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

/// Represents the raw data in a Tiled TMX file.
///
/// This can be queried and modified as needed before being loaded into the
/// game. Any modifications after this has been added to the game will be
/// ignored.
#[pyclass]
#[derive(Debug)]
pub struct TileMap {
    base_dir: PathBuf,
    map: ag::TileMap,
}

impl TileMap {
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    pub fn inner(&self) -> &ag::TileMap {
        &self.map
    }
}

#[pymethods]
impl TileMap {
    #[new]
    pub fn new(path: &str) -> PyResult<Self> {
        let path: &Path = path.as_ref();
        let base_dir = path.parent()
            .ok_or_else(|| PyValueError::new_err("Path to tiled map file did not have a valid parent directory"))?
            .to_path_buf();

        let map = ag::TileMap::open(path)
            .map_err(|err| PyValueError::new_err(err.to_string()))?;

        Ok(Self {base_dir, map})
    }

    #[getter]
    pub fn tile_width(&self) -> u32 {
        self.map.tile_width()
    }

    #[getter]
    pub fn tile_height(&self) -> u32 {
        self.map.tile_height()
    }
}
