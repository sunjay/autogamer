use pyo3::prelude::*;

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
}

#[pymethods]
impl Map {
    #[new]
    fn new(path: &str) -> Self {
        Self {}
    }
}
