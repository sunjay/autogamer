#![deny(unused_must_use)]

mod entity;
mod renderer;
mod event;
mod event_stream;
mod game;
mod geometry;
mod join;
mod components;
mod level;
mod physics;
mod ui;
mod tile_map;
mod spritesheets;

use entity::*;
use renderer::*;
use event::*;
use event_stream::*;
use game::*;
use geometry::*;
use join::*;
use components::*;
use level::*;
use physics::*;
use ui::*;
use tile_map::*;
use spritesheets::*;

use pyo3::prelude::*;

#[pymodule]
/// Bindings to the autogamer native module
pub fn autogamer_bindings(py: Python, pymod: &PyModule) -> PyResult<()> {
    pymod.add_wrapped(pyo3::wrap_pymodule!(ui))?;

    pymod.add_class::<Game>()?;
    pymod.add_class::<Level>()?;
    pymod.add_class::<Entity>()?;
    pymod.add_class::<TileMap>()?;
    pymod.add_class::<CharacterSpritesheet>()?;

    add_physics_mod(py, pymod)?;
    add_geometry_mod(py, pymod)?;

    add_components(pymod)?;

    Ok(())
}
