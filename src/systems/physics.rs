use specs::{System, SystemData, ReadStorage, World, prelude::ResourceId};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};

use crate::math::Vec2;
use crate::Position;

#[derive(SystemData)]
pub struct Data<'a> {
    pub positions: ReadStorage<'a, Position>,
}

pub struct Physics {
    mechanical_world: DefaultMechanicalWorld<f64>,
    geometrical_world: DefaultGeometricalWorld<f64>,
    bodies: DefaultBodySet<f64>,
    colliders: DefaultColliderSet<f64>,
    joint_constraints: DefaultJointConstraintSet<f64>,
    force_generators: DefaultForceGeneratorSet<f64>,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            mechanical_world: DefaultMechanicalWorld::new(Vec2::new(0.0, 0.0)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: DefaultForceGeneratorSet::new(),
        }
    }
}

impl Physics {
    pub fn set_gravity(&mut self, gravity: Vec2) {
        let old_gravity = &mut self.mechanical_world.gravity;
        if gravity != *old_gravity {
            *old_gravity = gravity;
        }
    }
}

impl<'a> System<'a> for Physics {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let Self {
            mechanical_world,
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators,
        } = self;

        let Data {
            positions,
        } = data;

        //TODO: Update physics + physics step + copy changes back to ECS

        mechanical_world.step(
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators
        );
    }
}
