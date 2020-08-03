use std::sync::Arc;

use autogamer as ag;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::exceptions::ValueError;
use specs::{World, WorldExt, BitSet};
use bstringify::bstringify;
use parking_lot::Mutex;

use crate::*;

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

        /// Represented a Python ECS component class
        ///
        /// Note that this represents the component class itself, not a
        /// downcasted instance of that class.
        #[derive(Debug, Clone)]
        pub enum PyComponentClass {
            /// Not a real component, but allows entities to be iterated along
            /// with their components.
            Entity,

            $($component),*
        }

        impl PyComponentClass {
            pub fn from_py(class: &PyAny) -> PyResult<Self> {
                //TODO: Using __name__ like this isn't foolproof. Would be
                // better to use the unique ID of the object for each component
                // class. No idea how to do that yet...
                let name = class.getattr("__name__")?;
                let name: &PyString = name.cast_as()?;
                let name = name.as_bytes()?;

                #[deny(unreachable_patterns)]
                Ok(match name {
                    b"Entity" => PyComponentClass::Entity,

                    $(bstringify!($component) => PyComponentClass::$component,)*

                    _ => return Err(ValueError::py_err("Unknown component")),
                })
            }

            /// Returns the name of this component class
            pub fn name(&self) -> &'static str {
                use PyComponentClass::*;
                match self {
                    Entity => "Entity",

                    $($component => stringify!($component),)*
                }
            }

            /// Filter the given bitset based on this component's storage
            ///
            /// The bitset represents which ECS entities have this component
            pub fn filter_bitset(&self, world: &World, bitset: &mut BitSet) {
                use PyComponentClass::*;
                match self {
                    // This does not filter out any components because it is not
                    // a real component
                    Entity => {},

                    $($component => {
                        let storage = world.read_component::<ag::$component>();
                        let mask = storage.mask();
                        *bitset &= mask;
                    },)*
                }
            }

            /// Reads a *copy* of a component from the world and returns it as a
            /// PyObject
            ///
            /// Modifying the copy does not update the component assocaited with
            /// this entity.
            ///
            /// Returns `None` if this component doesn't exist for this entity
            pub fn read_copy(
                &self,
                level: &Arc<Mutex<ag::Level>>,
                entity: specs::Entity,
                py: Python,
            ) -> PyResult<Option<PyObject>> {
                Ok(match self {
                    PyComponentClass::Entity => {
                        let entity = Entity::new(level.clone(), entity);
                        Some(PyCell::new(py, entity)?.to_object(py))
                    },

                    $(PyComponentClass::$component => {
                        let level = level.lock();
                        let world = level.world();

                        let storage = world.read_component::<ag::$component>();
                        match storage.get(entity) {
                            Some(component) => Some(PyCell::new(py, $component {
                                component: component.clone(),
                            })?.to_object(py)),

                            None => None,
                        }
                    },)*
                })
            }

            /// Removes this component class from the given entity and returns
            /// its previous value if any
            pub fn remove(
                &self,
                world: &World,
                entity: specs::Entity,
                py: Python,
            ) -> PyResult<Option<PyObject>> {
                Ok(match self {
                    PyComponentClass::Entity => {
                        return Err(ValueError::py_err("`Entity` is not a component, so it cannot be removed from an entity"));
                    },

                    $(PyComponentClass::$component => {
                        let mut storage = world.write_component::<ag::$component>();
                        let value = storage.get(entity).cloned();
                        storage.remove(entity);

                        match value {
                            Some(component) => Some(PyCell::new(py, $component {
                                component,
                            })?.to_object(py)),

                            None => None,
                        }
                    },)*
                })
            }
        }
    };
}

components! {
    Player,
    Position,
    PhysicsBody,
    PhysicsCollider,
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

/// Describes a body in the physics engine
#[pyclass]
#[derive(Debug, Clone)]
pub struct PhysicsBody {
    component: ag::PhysicsBody,
}

#[pymethods]
impl PhysicsBody {
    #[new]
    //TODO(PyO3/pyo3#1025): `mass` should be a keyword-only argument with no default
    #[args("*", mass="0.0")]
    pub fn new(mass: f64) -> Self {
        Self {
            component: ag::PhysicsBody {
                mass,
                ..ag::PhysicsBody::default()
            },
        }
    }

    #[getter]
    pub fn mass(&self) -> f64 {
        self.component.mass
    }

    //TODO: Should the setter have the side effect of updating this component
    // for a given entity? Maybe this struct should store Option<Entity>
    //#[setter]
    //pub fn set_mass(&mut self, mass: f64) {
    //    self.component.mass = mass;
    //}
}

/// Describes a collider in the physics engine
#[pyclass]
#[derive(Debug, Clone)]
pub struct PhysicsCollider {
    component: ag::PhysicsCollider,
}

#[pymethods]
impl PhysicsCollider {
    #[new]
    //TODO(PyO3/pyo3#1025): `shape` should be a keyword-only argument with no default
    #[args("*", shape="todo!()", offset="None", density="0.0", collision_groups="None")]
    pub fn new(shape: &PyAny, offset: Option<(f64, f64)>, density: f64, collision_groups: Option<CollisionGroups>) -> PyResult<Self> {
        let shape = Shape::to_shape(shape)
            .ok_or_else(|| ValueError::py_err("Unknown shape"))?;
        let (offset_x, offset_y) = offset.unwrap_or_default();
        let offset = ag::Vec2::new(offset_x, offset_y);
        let collision_groups = collision_groups.map(|groups| groups.inner().clone())
            .unwrap_or_default();

        Ok(Self {
            component: ag::PhysicsCollider {
                shape,
                offset,
                density,
                collision_groups,
                ..ag::PhysicsCollider::default()
            },
        })
    }
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
