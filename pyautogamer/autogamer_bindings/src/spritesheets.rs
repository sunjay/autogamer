use std::path::Path;

use autogamer as ag;
use pyo3::prelude::*;
use pyo3::exceptions::ValueError;

#[pyclass]
#[derive(Debug)]
pub struct CharacterSpritesheet {
    spritesheet: ag::CharacterSpritesheet,
}

#[pymethods]
impl CharacterSpritesheet {
    #[new]
    pub fn new(spritesheet: &str, config: &str) -> PyResult<Self> {
        let spritesheet = ag::CharacterSpritesheet::open(Path::new(spritesheet), Path::new(config))
            .map_err(|err| ValueError::py_err(err.to_string()))?;

        Ok(Self {spritesheet})
    }
}
