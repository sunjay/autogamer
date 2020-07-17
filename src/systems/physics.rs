use std::collections::HashMap;

use specs::{
    System,
    SystemData,
    World,
    Entities,
    WriteStorage,
    Join,
    prelude::ResourceId,
    world::Index,
};
use nphysics2d::{
    object::{
        DefaultBodySet,
        DefaultColliderSet,
        DefaultBodyHandle,
        DefaultColliderHandle,
    },
    force_generator::DefaultForceGeneratorSet,
    joint::DefaultJointConstraintSet,
    world::{DefaultMechanicalWorld, DefaultGeometricalWorld},
};

use crate::math::Vec2;
use crate::{Position, PhysicsBody, PhysicsCollider, Isometry};

#[derive(SystemData)]
pub struct Data<'a> {
    pub entities: Entities<'a>,
    pub positions: WriteStorage<'a, Position>,
    pub physics_bodies: WriteStorage<'a, PhysicsBody>,
    pub physics_colliders: WriteStorage<'a, PhysicsCollider>,
}

pub struct Physics {
    mechanical_world: DefaultMechanicalWorld<f64>,
    geometrical_world: DefaultGeometricalWorld<f64>,
    bodies: DefaultBodySet<f64>,
    colliders: DefaultColliderSet<f64>,
    joint_constraints: DefaultJointConstraintSet<f64>,
    force_generators: DefaultForceGeneratorSet<f64>,

    body_handles: HashMap<Index, DefaultBodyHandle>,
    collider_handles: HashMap<Index, DefaultColliderHandle>,
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

            body_handles: HashMap::new(),
            collider_handles: HashMap::new(),
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
            body_handles,
            collider_handles,
        } = self;

        let Data {
            entities,
            positions,
            mut physics_bodies,
            mut physics_colliders,
        } = data;

        // Handle removals
        while let Some(&id) = body_handles.keys().next() {
            let entity = entities.entity(id);
            if !physics_bodies.contains(entity) {
                if let Some(handle) = body_handles.remove(&id) {
                    bodies.remove(handle);
                }
            }
        }
        while let Some(&id) = collider_handles.keys().next() {
            let entity = entities.entity(id);
            if !physics_colliders.contains(entity) {
                if let Some(handle) = collider_handles.remove(&id) {
                    colliders.remove(handle);
                }
            }
        }

        // Add or update the physics bodies
        for (entity, &Position(pos), body) in (&entities, &positions, &mut physics_bodies).join() {
            let id = entity.id();
            match body_handles.get(&id) {
                Some(&handle) => {
                    let rigid_body = bodies.rigid_body_mut(handle)
                        .expect("bug: invalid physics body handle");
                    body.apply_to_rigid_body(rigid_body);
                    rigid_body.set_position(Isometry::new(pos, 0.0));
                },
                None => {
                    // Add a new rigid body
                    let rigid_body = body.to_rigid_body_desc()
                        .position(Isometry::new(pos, 0.0))
                        // Store ID so updating from the physics world is easy
                        .user_data(id)
                        .build();

                    let handle = bodies.insert(rigid_body);

                    debug_assert!(body.handle.is_none());
                    body.handle = Some(handle);
                    debug_assert!(!body_handles.contains_key(&id));
                    body_handles.insert(id, handle);
                }
            }
        }
        // Add or update the physics colliders
        //TODO: Add and update colliders

        mechanical_world.step(
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators
        );

        //TODO: Copy collider events

        //TODO: Copy from physics worlds to World
    }
}
