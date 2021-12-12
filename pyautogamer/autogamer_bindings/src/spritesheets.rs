use std::path::Path;

use autogamer as ag;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[pyclass]
#[derive(Debug)]
pub struct CharacterSpritesheet {
    spritesheet: ag::CharacterSpritesheet,
}

impl CharacterSpritesheet {
    pub fn inner(&self) -> &ag::CharacterSpritesheet {
        &self.spritesheet
    }

    pub fn inner_mut(&mut self) -> &mut ag::CharacterSpritesheet {
        &mut self.spritesheet
    }
}

#[pymethods]
impl CharacterSpritesheet {
    #[new]
    pub fn new(spritesheet: &str, config: &str) -> PyResult<Self> {
        let spritesheet = ag::CharacterSpritesheet::open(Path::new(spritesheet), Path::new(config))
            .map_err(|err| PyValueError::new_err(err.to_string()))?;

        Ok(Self {spritesheet})
    }
}
