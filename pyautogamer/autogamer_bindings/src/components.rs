use autogamer as ag;
use pyo3::prelude::*;
use pyo3::exceptions::ValueError;
use specs::{World, WorldExt};

macro_rules! components {
    ($($component:ident),* $(,)?) => {
        /// Adds all component classes to a module
        pub fn add_components(pymod: &PyModule) -> PyResult<()> {
            $(pymod.add_class::<$component>()?;)*

            Ok(())
        }

        pub fn write_component(world: &mut World, entity: specs::Entity, component: &PyAny) -> PyResult<()> {
            $(
                if let Ok(cell) = component.downcast::<PyCell<$component>>() {
                    let component = cell.borrow().component.clone();

                    world.write_component().insert(entity, component)
                        .expect(concat!("unable to insert component `", stringify!($component), "`"));

                    return Ok(());
                }
            )*

            Err(ValueError::py_err("Unknown component"))
        }
    };
}

components! {
    Player,
    Position,
    Sprite,
    CharacterSprites,
    PlatformerControls,
    Health,
    ViewportTarget,
}

/// A marker component given to an entity to indicate that it represents one of
/// the players of the game. This component is automatically added when you call
/// `Game.add_player`.
#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player {
    component: ag::Player,
}

#[pymethods]
impl Player {
    #[new]
    pub fn new() -> Self {
        Self {
            component: ag::Player,
        }
    }
}

/// The position of an entity
#[pyclass]
#[derive(Debug, Clone)]
pub struct Position {
    component: ag::Position,
}

#[pymethods]
impl Position {
    #[new]
    #[args("*", x="0.0", y="0.0")]
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            component: ag::Position(ag::Vec2::new(x, y)),
        }
    }

    #[getter]
    pub fn x(&self) -> f64 {
        self.component.0.x
    }

    //TODO: Should the setter have the side effect of updating this component
    // for a given entity? Maybe this struct should store Option<Entity>
    //#[setter]
    //pub fn set_x(&mut self, x: f64) {
    //    self.component.0.x = x;
    //}

    #[getter]
    pub fn y(&self) -> f64 {
        self.component.0.y
    }

    //TODO: Should the setter have the side effect of updating this component
    // for a given entity? Maybe this struct should store Option<Entity>
    //#[setter]
    //pub fn set_y(&mut self, y: f64) {
    //    self.component.0.y = y;
    //}
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Sprite {
    component: ag::Sprite,
}

impl From<ag::Sprite> for Sprite {
    fn from(component: ag::Sprite) -> Self {
        Self {component}
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct CharacterSprites {
    component: ag::CharacterSprites,
}

impl From<ag::CharacterSprites> for CharacterSprites {
    fn from(component: ag::CharacterSprites) -> Self {
        Self {component}
    }
}

#[pymethods]
impl CharacterSprites {
    pub fn default_sprite(&self) -> Option<Sprite> {
        self.component.default_sprite().map(Into::into)
    }
}

/// An entity with this component will respond to arrow key presses by setting
/// its velocity to the configured values. `left_velocity` and `right_velocity`
/// will be applied to the x-axis velocity. `jump_velocity` will be applied to
/// the y-axis velocity.
#[pyclass]
#[derive(Debug, Clone)]
pub struct PlatformerControls {
    component: ag::PlatformerControls,
}

#[pymethods]
impl PlatformerControls {
    #[new]
    #[args(
        "*",
        //TODO(PyO3/pyo3#1025): These should be keyword-only arguments with no defaults
        left_velocity = "0.0",
        right_velocity = "0.0",
        jump_velocity = "0.0",
    )]
    pub fn new(
        left_velocity: f64,
        right_velocity: f64,
        jump_velocity: f64,
    ) -> Self {
        Self {
            component: ag::PlatformerControls {
                left_velocity,
                right_velocity,
                jump_velocity,
            },
        }
    }

    #[getter]
    pub fn left_velocity(&self) -> f64 {
        self.component.left_velocity
    }

    //TODO: Should the setter have the side effect of updating this component
    // for a given entity? Maybe this struct should store Option<Entity>
    //#[setter]
    //pub fn set_left_velocity(&mut self, left_velocity: f64) {
    //    self.component.left_velocity = left_velocity;
    //}

    #[getter]
    pub fn right_velocity(&self) -> f64 {
        self.component.right_velocity
    }

    //TODO: Should the setter have the side effect of updating this component
    // for a given entity? Maybe this struct should store Option<Entity>
    //#[setter]
    //pub fn set_right_velocity(&mut self, right_velocity: f64) {
    //    self.component.right_velocity = right_velocity;
    //}

    #[getter]
    pub fn jump_velocity(&self) -> f64 {
        self.component.jump_velocity
    }

    //TODO: Should the setter have the side effect of updating this component
    // for a given entity? Maybe this struct should store Option<Entity>
    //#[setter]
    //pub fn set_jump_velocity(&mut self, jump_velocity: f64) {
    //   self.component.jump_velocity = jump_velocity;
    //}
}

/// The health of an entity
#[pyclass]
#[derive(Debug, Clone)]
pub struct Health {
    component: ag::Health,
}

#[pymethods]
impl Health {
    #[new]
    pub fn new(initial_health: u32) -> Self {
        Self {
            component: ag::Health(initial_health),
        }
    }

    #[getter]
    pub fn health(&self) -> u32 {
        self.component.0
    }

    //TODO: Should the setter have the side effect of updating this component
    // for a given entity? Maybe this struct should store Option<Entity>
    //#[setter]
    //pub fn set_health(&mut self, health: u32) {
    //    self.component.0 = health;
    //}
}

/// If an entity is given this component, the viewport will attempt to center
/// itself around the position of the entity.
///
/// Warning: Multiple entities should not have this component at the same time.
#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ViewportTarget {
    component: ag::ViewportTarget,
}

#[pymethods]
impl ViewportTarget {
    #[new]
    pub fn new() -> Self {
        Self {
            component: ag::ViewportTarget,
        }
    }
}
