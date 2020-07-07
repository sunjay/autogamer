use std::fmt;

use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};

use crate::math::Vec2;

pub struct Physics {
    mechanical_world: DefaultMechanicalWorld<f64>,
    geometrical_world: DefaultGeometricalWorld<f64>,
    bodies: DefaultBodySet<f64>,
    colliders: DefaultColliderSet<f64>,
    joint_constraints: DefaultJointConstraintSet<f64>,
    force_generators: DefaultForceGeneratorSet<f64>,
}

impl fmt::Debug for Physics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Physics {{ ... }}")
    }
}

impl Physics {
    pub fn new() -> Self {
        Self {
            mechanical_world: DefaultMechanicalWorld::new(Vec2::new(0.0, 0.0)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: DefaultForceGeneratorSet::new(),
        }
    }

    pub fn set_gravity(&mut self, gravity: Vec2) {
        self.mechanical_world.gravity = gravity;
    }

    pub fn update(&mut self) {
        let Self {
            mechanical_world,
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators,
        } = self;

        mechanical_world.step(
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators
        )
    }
}
