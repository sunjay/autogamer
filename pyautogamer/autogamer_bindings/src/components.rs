use std::sync::Arc;

use autogamer as ag;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::exceptions::PyValueError;
use specs::{World, WorldExt, BitSet};
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
                    let component = cell.borrow();
                    component.write(world, entity);
                    return Ok(());
                }
            )*

            Err(PyValueError::new_err("Unknown component"))
        }

        pub trait PyWriteComponent {
            fn write(&self, world: &mut World, entity: specs::Entity);
        }

        $(
            impl PyWriteComponent for $component {
                fn write(&self, world: &mut World, entity: specs::Entity) {
                    world.write_component().insert(entity, self.component.clone())
                        .expect(concat!("unable to insert component `", stringify!($component), "`"));
                }
            }
        )*

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
                let name = name.to_str()?;

                #[deny(unreachable_patterns)]
                Ok(match name {
                    "Entity" => PyComponentClass::Entity,

                    $(stringify!($component) => PyComponentClass::$component,)*

                    _ => return Err(PyValueError::new_err("Unknown component")),
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

            /// Reads a component from the world and returns it as a PyObject
            ///
            /// Modifying the component will update the component assocaited
            /// with this entity.
            ///
            /// Returns `None` if this component doesn't exist for this entity
            pub fn read(
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
                        let level_lock = level.lock();
                        let world = level_lock.world();

                        let storage = world.read_component::<ag::$component>();
                        match storage.get(entity) {
                            Some(component) => Some(PyCell::new(
                                py,
                                $component::from((level.clone(), entity, component.clone())),
                            )?.to_object(py)),

                            None => None,
                        }
                    },)*
                })
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
                            Some(component) => Some(PyCell::new(
                                py,
                                $component::from(component.clone()),
                            )?.to_object(py)),

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
                        return Err(PyValueError::new_err("`Entity` is not a component, so it cannot be removed from an entity"));
                    },

                    $(PyComponentClass::$component => {
                        let mut storage = world.write_component::<ag::$component>();
                        let value = storage.get(entity).cloned();
                        storage.remove(entity);

                        match value {
                            Some(component) => Some(PyCell::new(
                                py,
                                $component::from(component),
                            )?.to_object(py)),

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
    Wallet,
}

fn update_component<C: PyWriteComponent>(
    entity: &Option<(Arc<Mutex<ag::Level>>, specs::Entity)>,
    component: &C,
) {
    if let Some((level, entity)) = entity {
        let mut level = level.lock();
        let world = level.world_mut();
        component.write(world, *entity);
    }
}

/// A marker component given to an entity to indicate that it represents one of
/// the players of the game. This component is automatically added when you call
/// `Game.add_player`.
#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player {
    component: ag::Player,
}

impl From<ag::Player> for Player {
    fn from(component: ag::Player) -> Self {
        Self {component}
    }
}

impl From<(Arc<Mutex<ag::Level>>, specs::Entity, ag::Player)> for Player {
    fn from((_, _, component): (Arc<Mutex<ag::Level>>, specs::Entity, ag::Player)) -> Self {
        Self {component}
    }
}

#[pymethods]
impl Player {
    #[new]
    pub fn new() -> Self {
        Self {
            component: ag::Player,
        }
    }

    /// Returns a copy of this component
    ///
    /// Modifying a copy of a component does not modify the original component
    /// it was copied from
    pub fn copy(&self) -> Self {
        Self {
            component: self.component.clone(),
        }
    }
}

/// The position of an entity
#[pyclass]
#[derive(Debug, Clone)]
pub struct Position {
    entity: Option<(Arc<Mutex<ag::Level>>, specs::Entity)>,
    component: ag::Position,
}

impl From<ag::Position> for Position {
    fn from(component: ag::Position) -> Self {
        Self {
            entity: None,
            component,
        }
    }
}

impl From<(Arc<Mutex<ag::Level>>, specs::Entity, ag::Position)> for Position {
    fn from((level, entity, component): (Arc<Mutex<ag::Level>>, specs::Entity, ag::Position)) -> Self {
        let entity = Some((level, entity));
        Self {entity, component}
    }
}

#[pymethods]
impl Position {
    #[new]
    #[args("*", x="0.0", y="0.0")]
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            entity: None,
            component: ag::Position(ag::Vec2::new(x, y)),
        }
    }

    /// Returns a copy of this component
    ///
    /// Modifying a copy of a component does not modify the original component
    /// it was copied from
    pub fn copy(&self) -> Self {
        Self {
            entity: None,
            component: self.component.clone(),
        }
    }

    #[getter]
    pub fn x(&self) -> f64 {
        self.component.0.x
    }

    #[setter]
    pub fn set_x(&mut self, x: f64) {
        self.component.0.x = x;
        update_component(&self.entity, self);
    }

    #[getter]
    pub fn y(&self) -> f64 {
        self.component.0.y
    }

    #[setter]
    pub fn set_y(&mut self, y: f64) {
        self.component.0.y = y;
        update_component(&self.entity, self);
    }
}

/// Describes a body in the physics engine
#[pyclass]
#[derive(Debug, Clone)]
pub struct PhysicsBody {
    entity: Option<(Arc<Mutex<ag::Level>>, specs::Entity)>,
    component: ag::PhysicsBody,
}

impl From<ag::PhysicsBody> for PhysicsBody {
    fn from(component: ag::PhysicsBody) -> Self {
        Self {
            entity: None,
            component,
        }
    }
}

impl From<(Arc<Mutex<ag::Level>>, specs::Entity, ag::PhysicsBody)> for PhysicsBody {
    fn from((level, entity, component): (Arc<Mutex<ag::Level>>, specs::Entity, ag::PhysicsBody)) -> Self {
        let entity = Some((level, entity));
        Self {entity, component}
    }
}

#[pymethods]
impl PhysicsBody {
    #[new]
    #[args("*", mass)]
    pub fn new(mass: f64) -> Self {
        Self {
            entity: None,
            component: ag::PhysicsBody {
                mass,
                ..ag::PhysicsBody::default()
            },
        }
    }

    /// Returns a copy of this component
    ///
    /// Modifying a copy of a component does not modify the original component
    /// it was copied from
    pub fn copy(&self) -> Self {
        Self {
            entity: None,
            component: self.component.clone(),
        }
    }

    #[getter]
    pub fn mass(&self) -> f64 {
        self.component.mass
    }

    #[setter]
    pub fn set_mass(&mut self, mass: f64) {
        self.component.mass = mass;
        update_component(&self.entity, self);
    }
}

/// Describes a collider in the physics engine
#[pyclass]
#[derive(Debug, Clone)]
pub struct PhysicsCollider {
    entity: Option<(Arc<Mutex<ag::Level>>, specs::Entity)>,
    component: ag::PhysicsCollider,
}

impl From<ag::PhysicsCollider> for PhysicsCollider {
    fn from(component: ag::PhysicsCollider) -> Self {
        Self {
            entity: None,
            component,
        }
    }
}

impl From<(Arc<Mutex<ag::Level>>, specs::Entity, ag::PhysicsCollider)> for PhysicsCollider {
    fn from((level, entity, component): (Arc<Mutex<ag::Level>>, specs::Entity, ag::PhysicsCollider)) -> Self {
        let entity = Some((level, entity));
        Self {entity, component}
    }
}

#[pymethods]
impl PhysicsCollider {
    #[new]
    #[args("*", shape, offset="None", density="0.0", collision_groups="None")]
    pub fn new(shape: &PyAny, offset: Option<(f64, f64)>, density: f64, collision_groups: Option<CollisionGroups>) -> PyResult<Self> {
        let shape = Shape::to_shape(shape)
            .ok_or_else(|| PyValueError::new_err("Unknown shape"))?;
        let (offset_x, offset_y) = offset.unwrap_or_default();
        let offset = ag::Vec2::new(offset_x, offset_y);
        let collision_groups = collision_groups.map(|groups| groups.inner().clone())
            .unwrap_or_default();

        Ok(Self {
            entity: None,
            component: ag::PhysicsCollider {
                shape,
                offset,
                density,
                collision_groups,
                ..ag::PhysicsCollider::default()
            },
        })
    }

    /// Returns a copy of this component
    ///
    /// Modifying a copy of a component does not modify the original component
    /// it was copied from
    pub fn copy(&self) -> Self {
        Self {
            entity: None,
            component: self.component.clone(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Sprite {
    entity: Option<(Arc<Mutex<ag::Level>>, specs::Entity)>,
    component: ag::Sprite,
}

impl From<ag::Sprite> for Sprite {
    fn from(component: ag::Sprite) -> Self {
        Self {
            entity: None,
            component,
        }
    }
}

impl From<(Arc<Mutex<ag::Level>>, specs::Entity, ag::Sprite)> for Sprite {
    fn from((level, entity, component): (Arc<Mutex<ag::Level>>, specs::Entity, ag::Sprite)) -> Self {
        let entity = Some((level, entity));
        Self {entity, component}
    }
}

#[pymethods]
impl Sprite {
    /// Returns a copy of this component
    ///
    /// Modifying a copy of a component does not modify the original component
    /// it was copied from
    pub fn copy(&self) -> Self {
        Self {
            entity: None,
            component: self.component.clone(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct CharacterSprites {
    entity: Option<(Arc<Mutex<ag::Level>>, specs::Entity)>,
    component: ag::CharacterSprites,
}

impl From<ag::CharacterSprites> for CharacterSprites {
    fn from(component: ag::CharacterSprites) -> Self {
        Self {
            entity: None,
            component,
        }
    }
}

impl From<(Arc<Mutex<ag::Level>>, specs::Entity, ag::CharacterSprites)> for CharacterSprites {
    fn from((level, entity, component): (Arc<Mutex<ag::Level>>, specs::Entity, ag::CharacterSprites)) -> Self {
        let entity = Some((level, entity));
        Self {entity, component}
    }
}

#[pymethods]
impl CharacterSprites {
    pub fn default_sprite(&self) -> Option<Sprite> {
        self.component.default_sprite().map(Into::into)
    }

    /// Returns a copy of this component
    ///
    /// Modifying a copy of a component does not modify the original component
    /// it was copied from
    pub fn copy(&self) -> Self {
        Self {
            entity: None,
            component: self.component.clone(),
        }
    }
}

/// An entity with this component will respond to arrow key presses by setting
/// its velocity to the configured values. `left_velocity` and `right_velocity`
/// will be applied to the x-axis velocity. `jump_velocity` will be applied to
/// the y-axis velocity.
#[pyclass]
#[derive(Debug, Clone)]
pub struct PlatformerControls {
    entity: Option<(Arc<Mutex<ag::Level>>, specs::Entity)>,
    component: ag::PlatformerControls,
}

impl From<ag::PlatformerControls> for PlatformerControls {
    fn from(component: ag::PlatformerControls) -> Self {
        Self {
            entity: None,
            component,
        }
    }
}

impl From<(Arc<Mutex<ag::Level>>, specs::Entity, ag::PlatformerControls)> for PlatformerControls {
    fn from((level, entity, component): (Arc<Mutex<ag::Level>>, specs::Entity, ag::PlatformerControls)) -> Self {
        let entity = Some((level, entity));
        Self {entity, component}
    }
}

#[pymethods]
impl PlatformerControls {
    #[new]
    #[args(
        "*",
        horizontal_velocity,
        jump_velocity,
        midair_horizontal_multiplier = "1.0",
    )]
    pub fn new(
        horizontal_velocity: f64,
        jump_velocity: f64,
        midair_horizontal_multiplier: f64,
    ) -> Self {
        Self {
            entity: None,
            component: ag::PlatformerControls {
                horizontal_velocity,
                jump_velocity,
                midair_horizontal_multiplier,
                ..ag::PlatformerControls::default()
            },
        }
    }

    /// Returns a copy of this component
    ///
    /// Modifying a copy of a component does not modify the original component
    /// it was copied from
    pub fn copy(&self) -> Self {
        Self {
            entity: None,
            component: self.component.clone(),
        }
    }

    #[getter]
    pub fn horizontal_velocity(&self) -> f64 {
        self.component.horizontal_velocity
    }

    #[setter]
    pub fn set_horizontal_velocity(&mut self, horizontal_velocity: f64) {
        self.component.horizontal_velocity = horizontal_velocity;
        update_component(&self.entity, self);
    }

    #[getter]
    pub fn jump_velocity(&self) -> f64 {
        self.component.jump_velocity
    }

    #[setter]
    pub fn set_jump_velocity(&mut self, jump_velocity: f64) {
        self.component.jump_velocity = jump_velocity;
        update_component(&self.entity, self);
    }

    #[getter]
    pub fn midair_horizontal_multiplier(&self) -> f64 {
        self.component.midair_horizontal_multiplier
    }

    #[setter]
    pub fn set_midair_horizontal_multiplier(&mut self, midair_horizontal_multiplier: f64) {
        self.component.midair_horizontal_multiplier = midair_horizontal_multiplier;
        update_component(&self.entity, self);
    }
}

/// The health of an entity
#[pyclass]
#[derive(Debug, Clone)]
pub struct Health {
    entity: Option<(Arc<Mutex<ag::Level>>, specs::Entity)>,
    component: ag::Health,
}

impl From<ag::Health> for Health {
    fn from(component: ag::Health) -> Self {
        Self {
            entity: None,
            component,
        }
    }
}

impl From<(Arc<Mutex<ag::Level>>, specs::Entity, ag::Health)> for Health {
    fn from((level, entity, component): (Arc<Mutex<ag::Level>>, specs::Entity, ag::Health)) -> Self {
        let entity = Some((level, entity));
        Self {entity, component}
    }
}

#[pymethods]
impl Health {
    #[new]
    pub fn new(initial_health: u32) -> Self {
        Self {
            entity: None,
            component: ag::Health(initial_health),
        }
    }

    /// Returns a copy of this component
    ///
    /// Modifying a copy of a component does not modify the original component
    /// it was copied from
    pub fn copy(&self) -> Self {
        Self {
            entity: None,
            component: self.component.clone(),
        }
    }

    #[getter]
    pub fn health(&self) -> u32 {
        self.component.0
    }

    #[setter]
    pub fn set_health(&mut self, health: u32) {
        self.component.0 = health;
        update_component(&self.entity, self);
    }
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

impl From<ag::ViewportTarget> for ViewportTarget {
    fn from(component: ag::ViewportTarget) -> Self {
        Self {component}
    }
}

impl From<(Arc<Mutex<ag::Level>>, specs::Entity, ag::ViewportTarget)> for ViewportTarget {
    fn from((_, _, component): (Arc<Mutex<ag::Level>>, specs::Entity, ag::ViewportTarget)) -> Self {
        Self {component}
    }
}

#[pymethods]
impl ViewportTarget {
    #[new]
    pub fn new() -> Self {
        Self {
            component: ag::ViewportTarget,
        }
    }

    /// Returns a copy of this component
    ///
    /// Modifying a copy of a component does not modify the original component
    /// it was copied from
    pub fn copy(&self) -> Self {
        Self {
            component: self.component.clone(),
        }
    }
}

/// The amount of currency collected by this entity so far.
///
/// The amount may become negative if enough negative-value currency components
/// are collected.
///
/// This component must be present for an entity to be able to interact with
/// other entities that have `Currency` components.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Wallet {
    entity: Option<(Arc<Mutex<ag::Level>>, specs::Entity)>,
    component: ag::Wallet,
}

impl From<ag::Wallet> for Wallet {
    fn from(component: ag::Wallet) -> Self {
        Self {
            entity: None,
            component,
        }
    }
}

impl From<(Arc<Mutex<ag::Level>>, specs::Entity, ag::Wallet)> for Wallet {
    fn from((level, entity, component): (Arc<Mutex<ag::Level>>, specs::Entity, ag::Wallet)) -> Self {
        let entity = Some((level, entity));
        Self {entity, component}
    }
}

#[pymethods]
impl Wallet {
    #[new]
    #[args(initial_balance = 0)]
    pub fn new(initial_balance: i32) -> Self {
        Self {
            entity: None,
            component: ag::Wallet(initial_balance),
        }
    }

    /// Returns a copy of this component
    ///
    /// Modifying a copy of a component does not modify the original component
    /// it was copied from
    pub fn copy(&self) -> Self {
        Self {
            entity: None,
            component: self.component.clone(),
        }
    }

    #[getter]
    pub fn balance(&self) -> i32 {
        self.component.0
    }

    #[setter]
    pub fn set_balance(&mut self, balance: i32) {
        self.component.0 = balance;
        update_component(&self.entity, self);
    }
}
