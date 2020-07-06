mod entity;
mod game;
mod level;
mod physics;
mod ui;
mod tile_map;

use entity::*;
use game::*;
use level::*;
use physics::*;
use ui::*;
use tile_map::*;

use pyo3::prelude::*;

#[pymodule]
/// Bindings to the autogamer native module
pub fn autogamer_bindings(_py: Python, pymod: &PyModule) -> PyResult<()> {
    pymod.add_wrapped(pyo3::wrap_pymodule!(ui))?;

    pymod.add_class::<Game>()?;
    pymod.add_class::<Level>()?;
    pymod.add_class::<Physics>()?;
    pymod.add_class::<Entity>()?;
    pymod.add_class::<TileMap>()?;

    Ok(())
}
