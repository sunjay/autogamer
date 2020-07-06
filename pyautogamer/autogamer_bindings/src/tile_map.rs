use autogamer as ag;
use pyo3::prelude::*;
use pyo3::exceptions::ValueError;

/// Represents the raw data in a Tiled TMX file.
///
/// This can be queried and modified as needed before being loaded into the
/// game. Any modifications after this has been added to the game will be
/// ignored.
#[pyclass]
#[derive(Debug)]
pub struct TileMap {
    map: ag::TileMap,
}

#[pymethods]
impl TileMap {
    #[new]
    pub fn new(path: &str) -> PyResult<Self> {
        let map = ag::TileMap::load(path)
            .map_err(|err| ValueError::py_err(err.to_string()))?;
        Ok(Self {map})
    }

    #[getter]
    pub fn tile_width(&self) -> u32 {
        todo!()
    }

    #[getter]
    pub fn tile_height(&self) -> u32 {
        todo!()
    }
}
