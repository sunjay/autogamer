use autogamer as ag;
use pyo3::prelude::*;

pub fn add_physics_mod(py: Python, pymod: &PyModule) -> PyResult<()> {
    pymod.add_class::<PhysicsEngine>()?;
    pymod.add_class::<CollisionGroups>()?;

    let ground_groups = CollisionGroups::from(ag::PhysicsCollider::ground_collision_groups());
    pymod.add("GROUND_COLLISION_GROUPS", &PyCell::new(py, ground_groups)?)?;
    let player_groups = CollisionGroups::from(ag::PhysicsCollider::player_collision_groups());
    pymod.add("PLAYER_COLLISION_GROUPS", &PyCell::new(py, player_groups)?)?;
    let enemy_groups = CollisionGroups::from(ag::PhysicsCollider::enemy_collision_groups());
    pymod.add("ENEMY_COLLISION_GROUPS", &PyCell::new(py, enemy_groups)?)?;

    Ok(())
}

#[pyclass]
#[derive(Debug)]
pub struct PhysicsEngine {
    physics: ag::PhysicsEngine,
}

impl PhysicsEngine {
    pub fn inner(&self) -> &ag::PhysicsEngine {
        &self.physics
    }

    pub fn inner_mut(&mut self) -> &mut ag::PhysicsEngine {
        &mut self.physics
    }
}

#[pymethods]
impl PhysicsEngine {
    #[new]
    pub fn new() -> Self {
        Self {
            physics: ag::PhysicsEngine::new(),
        }
    }

    pub fn set_gravity(&mut self, gravity: (f64, f64)) {
        let (x_gravity, y_gravity) = gravity;
        self.physics.set_gravity(ag::Vec2::new(x_gravity, y_gravity))
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct CollisionGroups {
    groups: ag::CollisionGroups,
}

impl From<ag::CollisionGroups> for CollisionGroups {
    fn from(groups: ag::CollisionGroups) -> Self {
        Self {groups}
    }
}

impl CollisionGroups {
    pub fn inner(&self) -> &ag::CollisionGroups {
        &self.groups
    }

    pub fn inner_mut(&mut self) -> &mut ag::CollisionGroups {
        &mut self.groups
    }
}
