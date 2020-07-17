use std::collections::HashMap;

use specs::{
    System,
    SystemData,
    World,
    Entities,
    WriteStorage,
    Join,
    prelude::ResourceId,
    world::{EntitiesRes, Index},
};
use nphysics2d::{
    object::{
        DefaultBodySet,
        DefaultColliderSet,
        DefaultBodyHandle,
        DefaultColliderHandle,
        BodyPartHandle,
        Ground,
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
    ground: DefaultBodyHandle,

    body_handles: HashMap<Index, DefaultBodyHandle>,
    collider_handles: HashMap<Index, DefaultColliderHandle>,
}

impl Default for Physics {
    fn default() -> Self {
        let mut bodies = DefaultBodySet::new();
        let ground = bodies.insert(Ground::new());
        Self {
            mechanical_world: DefaultMechanicalWorld::new(Vec2::new(0.0, 0.0)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies,
            colliders: DefaultColliderSet::new(),
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: DefaultForceGeneratorSet::new(),
            ground,

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
            ground,
        } = self;

        let Data {
            entities,
            positions,
            mut physics_bodies,
            mut physics_colliders,
        } = data;

        // Sync to the physics world. Bodies must go first since colliders rely
        // on the bodies being fully synced.
        sync_physics_bodies_to_engine(
            &entities,
            &positions,
            &mut physics_bodies,
            body_handles,
            bodies,
        );
        sync_physics_colliders_to_engine(
            &entities,
            &positions,
            &mut physics_colliders,
            collider_handles,
            colliders,
            body_handles,
            *ground,
        );

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

fn sync_physics_bodies_to_engine(
    entities: &EntitiesRes,
    positions: &WriteStorage<Position>,
    physics_bodies: &mut WriteStorage<PhysicsBody>,
    body_handles: &mut HashMap<Index, DefaultBodyHandle>,
    bodies: &mut DefaultBodySet<f64>,
) {
    // Handle removals
    while let Some(&id) = body_handles.keys().next() {
        let entity = entities.entity(id);
        if !physics_bodies.contains(entity) {
            if let Some(handle) = body_handles.remove(&id) {
                bodies.remove(handle);
            }
        }
    }

    // Add or update the physics bodies
    for (entity, &Position(pos), body) in (entities, positions, physics_bodies).join() {
        let id = entity.id();
        match body_handles.get(&id) {
            // Update existing rigid body
            Some(&handle) => {
                let rigid_body = bodies.rigid_body_mut(handle)
                    .expect("bug: invalid physics body handle");

                body.apply_to_rigid_body(rigid_body);
                rigid_body.set_position(Isometry::new(pos, 0.0));
            },

            // Add a new rigid body
            None => {
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
}

fn sync_physics_colliders_to_engine(
    entities: &EntitiesRes,
    positions: &WriteStorage<Position>,
    physics_colliders: &mut WriteStorage<PhysicsCollider>,
    collider_handles: &mut HashMap<Index, DefaultColliderHandle>,
    colliders: &mut DefaultColliderSet<f64>,
    body_handles: &HashMap<Index, DefaultBodyHandle>,
    ground: DefaultBodyHandle,
) {
    // Handle removals
    while let Some(&id) = collider_handles.keys().next() {
        let entity = entities.entity(id);
        if !physics_colliders.contains(entity) {
            if let Some(handle) = collider_handles.remove(&id) {
                // Check if collider still exists since colliders are implicitly
                // removed when the parent body is removed.
                if colliders.get(handle).is_some() {
                    colliders.remove(handle);
                }
            }
        }
    }

    // Add or update the physics colliders
    for (entity, &Position(pos), physics_collider) in (entities, positions, physics_colliders).join() {
        let id = entity.id();
        match collider_handles.get(&id) {
            // Update existing collider
            Some(&handle) => {
                let collider = colliders.get_mut(handle)
                    .expect("bug: invalid physics collider handle");

                physics_collider.update_collider(collider);
                collider.set_position(Isometry::new(pos, 0.0));
            },

            // Add a new collider
            None => {
                // Attempt to find an existing body associated with the same ID
                // so we can use it as the parent of the collider (default: ground)
                let body_handle = body_handles.get(&id)
                    .map(|&handle| BodyPartHandle(handle, 0))
                    .unwrap_or_else(|| BodyPartHandle(ground, 0));


                let collider = physics_collider.to_collider_desc()
                    .position(Isometry::new(pos, 0.0))
                    // Store ID so updating from the physics world is easy
                    .user_data(id)
                    .build(body_handle);

                let handle = colliders.insert(collider);

                debug_assert!(physics_collider.handle.is_none());
                physics_collider.handle = Some(handle);
                debug_assert!(!collider_handles.contains_key(&id));
                collider_handles.insert(id, handle);
            },
        }
    }
}
