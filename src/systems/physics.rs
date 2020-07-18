use std::collections::HashMap;

use specs::{
    System,
    SystemData,
    World,
    WorldExt,
    WriteStorage,
    Join,
    ReaderId,
    BitSet,
    prelude::{ComponentEvent, ResourceId},
    world::Index,
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

    positions_reader_id: Option<ReaderId<ComponentEvent>>,
    physics_bodies_reader_id: Option<ReaderId<ComponentEvent>>,
    physics_colliders_reader_id: Option<ReaderId<ComponentEvent>>,

    removed_physics_bodies: BitSet,
    modified_physics_bodies: BitSet,
    removed_physics_colliders: BitSet,
    modified_physics_colliders: BitSet,
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

            positions_reader_id: None,
            physics_bodies_reader_id: None,
            physics_colliders_reader_id: None,

            removed_physics_bodies: BitSet::default(),
            modified_physics_bodies: BitSet::default(),
            removed_physics_colliders: BitSet::default(),
            modified_physics_colliders: BitSet::default(),
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
            ground,

            body_handles,
            collider_handles,

            positions_reader_id,
            physics_bodies_reader_id,
            physics_colliders_reader_id,

            removed_physics_bodies,
            modified_physics_bodies,
            removed_physics_colliders,
            modified_physics_colliders,
        } = self;

        let Data {
            mut positions,
            mut physics_bodies,
            mut physics_colliders,
        } = data;

        let positions_reader_id = positions_reader_id.as_mut()
            .expect("reader_id should have been configured during setup");
        let physics_bodies_reader_id = physics_bodies_reader_id.as_mut()
            .expect("reader_id should have been configured during setup");
        let physics_colliders_reader_id = physics_colliders_reader_id.as_mut()
            .expect("reader_id should have been configured during setup");

        // Determine which entities have been removed or changed
        resolve_removals_modifications(
            positions.channel().read(positions_reader_id),
            physics_bodies.channel().read(physics_bodies_reader_id),
            physics_colliders.channel().read(physics_colliders_reader_id),
            removed_physics_bodies,
            modified_physics_bodies,
            removed_physics_colliders,
            modified_physics_colliders,
        );

        // Sync to the physics world

        sync_physics_bodies_to_engine(
            removed_physics_bodies,
            modified_physics_bodies,
            &positions,
            &mut physics_bodies,
            body_handles,
            bodies,
        );
        // Syncing the colliders depends on the bodies being fully synced first
        sync_physics_colliders_to_engine(
            removed_physics_colliders,
            modified_physics_colliders,
            &positions,
            &mut physics_colliders,
            collider_handles,
            colliders,
            body_handles,
            *ground,
        );

        // Run the next step of the simulation

        mechanical_world.step(
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators
        );

        //TODO: Copy collider events

        // Sync the results back from the physics world
        sync_engine_to_physics_bodies(&mut positions, &mut physics_bodies, bodies);

        // Drain events caused by this system since we don't want to end up in
        // an infinite loop where we update things that we just updated
        positions.channel().read(positions_reader_id).for_each(drop);
        physics_bodies.channel().read(physics_bodies_reader_id).for_each(drop);
        physics_colliders.channel().read(physics_colliders_reader_id).for_each(drop);
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        // register reader id for the Position storage
        let mut positions = world.write_storage::<Position>();
        self.positions_reader_id = Some(positions.register_reader());

        // register reader id for the PhysicsBody storage
        let mut physics_bodies = world.write_storage::<PhysicsBody>();
        self.physics_bodies_reader_id = Some(physics_bodies.register_reader());

        // register reader id for the PhysicsCollider storage
        let mut physics_colliders = world.write_storage::<PhysicsCollider>();
        self.physics_colliders_reader_id = Some(physics_colliders.register_reader());
    }
}

fn resolve_removals_modifications<'a>(
    positions_events: impl Iterator<Item=&'a ComponentEvent>,
    physics_bodies_events: impl Iterator<Item=&'a ComponentEvent>,
    physics_colliders_events: impl Iterator<Item=&'a ComponentEvent>,

    removed_physics_bodies: &mut BitSet,
    modified_physics_bodies: &mut BitSet,
    removed_physics_colliders: &mut BitSet,
    modified_physics_colliders: &mut BitSet,
) {
    removed_physics_bodies.clear();
    modified_physics_bodies.clear();
    removed_physics_colliders.clear();
    modified_physics_colliders.clear();

    for event in positions_events {
        match event {
            // Adding or modifying a Position component affects both bodies
            // and colliders
            &ComponentEvent::Inserted(id) |
            &ComponentEvent::Modified(id) => {
                modified_physics_bodies.add(id);
                modified_physics_colliders.add(id);
            },

            // Removing a Position component removes all bodies and
            // colliders associated with that position
            &ComponentEvent::Removed(id) => {
                removed_physics_bodies.add(id);
                removed_physics_colliders.add(id);
            },
        }
    }

    for event in physics_bodies_events {
        match event {
            &ComponentEvent::Inserted(id) |
            &ComponentEvent::Modified(id) => {
                modified_physics_bodies.add(id);
            },

            &ComponentEvent::Removed(id) => {
                removed_physics_bodies.add(id);
            },
        }
    }

    for event in physics_colliders_events {
        match event {
            &ComponentEvent::Inserted(id) |
            &ComponentEvent::Modified(id) => {
                modified_physics_colliders.add(id);
            },

            &ComponentEvent::Removed(id) => {
                removed_physics_colliders.add(id);
            },
        }
    }
}

fn sync_physics_bodies_to_engine(
    removed_physics_bodies: &BitSet,
    modified_physics_bodies: &BitSet,
    positions: &WriteStorage<Position>,
    physics_bodies: &mut WriteStorage<PhysicsBody>,
    body_handles: &mut HashMap<Index, DefaultBodyHandle>,
    bodies: &mut DefaultBodySet<f64>,
) {
    // Handle removals
    for id in removed_physics_bodies {
        if let Some(handle) = body_handles.remove(&id) {
            bodies.remove(handle);
        }
    }

    // Add or update the physics bodies
    for (id, &Position(pos), body) in (modified_physics_bodies, positions, physics_bodies).join() {
        match body.handle {
            // Update existing rigid body
            Some(handle) => {
                let rigid_body = bodies.rigid_body_mut(handle)
                    .expect("bug: invalid physics body handle");

                body.apply_to_rigid_body(rigid_body);
                if rigid_body.position().translation.vector != pos {
                    rigid_body.set_position(Isometry::new(pos, 0.0));
                }
            },

            // Add a new rigid body
            None => {
                // Check if we previously stored a handle for this ID. If that
                // is the case, this means that the PhysicsBody component was
                // removed and re-added.
                if let Some(handle) = body_handles.remove(&id) {
                    // Remove the handle for the previous PhysicsBody component
                    bodies.remove(handle);
                }

                let rigid_body = body.to_rigid_body_desc()
                    .position(Isometry::new(pos, 0.0))
                    // Store ID so updating from the physics world is easy
                    .user_data(id)
                    .build();

                let handle = bodies.insert(rigid_body);

                body.handle = Some(handle);
                body_handles.insert(id, handle);
            },
        }
    }
}

fn sync_physics_colliders_to_engine(
    removed_physics_colliders: &BitSet,
    modified_physics_colliders: &BitSet,
    positions: &WriteStorage<Position>,
    physics_colliders: &mut WriteStorage<PhysicsCollider>,
    collider_handles: &mut HashMap<Index, DefaultColliderHandle>,
    colliders: &mut DefaultColliderSet<f64>,
    body_handles: &HashMap<Index, DefaultBodyHandle>,
    ground: DefaultBodyHandle,
) {
    // Handle removals
    for id in removed_physics_colliders {
        if let Some(handle) = collider_handles.remove(&id) {
            // Check if collider still exists since colliders are implicitly
            // removed when the parent body is removed.
            if colliders.get(handle).is_some() {
                colliders.remove(handle);
            }
        }
    }

    // Add or update the physics colliders
    for (id, &Position(pos), physics_collider) in (modified_physics_colliders, positions, physics_colliders).join() {
        match physics_collider.handle {
            // Update existing collider
            Some(handle) => {
                let collider = colliders.get_mut(handle)
                    .expect("bug: invalid physics collider handle");

                physics_collider.update_collider(collider);
            },

            // Add a new collider
            None => {
                // Check if we previously stored a handle for this ID. If that
                // is the case, this means that the PhysicsCollider component
                // was removed and re-added.
                if let Some(handle) = collider_handles.remove(&id) {
                    // Remove the handle for the previous PhysicsCollider component
                    //
                    // Check if collider still exists since colliders are
                    // implicitly removed when the parent body is removed.
                    if colliders.get(handle).is_some() {
                        colliders.remove(handle);
                    }
                }

                // Attempt to find an existing body associated with the same ID
                // so we can use it as the parent of the collider (default: ground)
                let (body_handle, rel_pos) = match body_handles.get(&id) {
                    Some(&handle) => {
                        let body_handle = BodyPartHandle(handle, 0);
                        // Position relative to parent body
                        let rel_pos = Vec2::new(0.0, 0.0);
                        (body_handle, rel_pos)
                    },

                    None => {
                        let body_handle = BodyPartHandle(ground, 0);
                        // Position relative to ground (i.e. the origin)
                        let rel_pos = pos;
                        (body_handle, rel_pos)
                    },
                };

                let collider = physics_collider.to_collider_desc()
                    .position(Isometry::new(rel_pos, 0.0))
                    // Store ID so updating from the physics world is easy
                    .user_data(id)
                    .build(body_handle);

                let handle = colliders.insert(collider);

                physics_collider.handle = Some(handle);
                collider_handles.insert(id, handle);
            },
        }
    }
}

fn sync_engine_to_physics_bodies(
    positions: &mut WriteStorage<Position>,
    physics_bodies: &mut WriteStorage<PhysicsBody>,
    bodies: &mut DefaultBodySet<f64>,
) {
    for (pos, body) in (positions, physics_bodies).join() {
        let Position(pos) = pos;

        let handle = body.handle
            .expect("bug: all bodies should have handles at this point");
        let rigid_body = bodies.rigid_body(handle)
            .expect("bug: invalid body handle");

        body.update_from_rigid_body(rigid_body);
        *pos = rigid_body.position().translation.vector;
    }
}
