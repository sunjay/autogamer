mod screen;

pub use screen::*;

use pyo3::prelude::*;

#[pymodule]
/// Bindings to the autogamer native UI module
pub fn ui(_py: Python, pymod: &PyModule) -> PyResult<()> {
    pymod.add_class::<Screen>()?;

    Ok(())
}
