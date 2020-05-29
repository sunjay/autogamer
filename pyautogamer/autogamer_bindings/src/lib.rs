use autogamer as ag;
use pyo3::prelude::*;
use pyo3::exceptions::ValueError;

#[pymodule]
/// Bindings to the autogamer native module
pub fn autogamer_bindings(_py: Python, pymod: &PyModule) -> PyResult<()> {
    pymod.add_class::<Game>()?;
    pymod.add_class::<Map>()?;
    Ok(())
}

#[pyclass]
#[derive(Debug)]
pub struct Game {
}

#[pymethods]
impl Game {
    #[new]
    fn new() -> Self {
        Self {}
    }

    fn add(&mut self, map: &Map) {
    }

    fn fullscreen(&self) {
    }

    fn run(&self) {
    }
}

#[pyclass]
#[derive(Debug)]
pub struct Map {
    map: ag::Map,
}

#[pymethods]
impl Map {
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let map = ag::Map::load(path)
            .map_err(|err| ValueError::py_err(err.to_string()))?;
        Ok(Self {map})
    }
}
