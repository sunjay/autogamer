use specs::{System, SystemData, World, WorldExt, ReadStorage, Write, Read, ReaderId, prelude::ResourceId};

use crate::{ContactEvents, ProximityEvents, CollisionsMap, ContactEvent, ProximityEvent, ContactType, Proximity, Position, PhysicsCollider};

#[derive(SystemData)]
pub struct Data<'a> {
    pub positions: ReadStorage<'a, Position>,
    pub physics_colliders: ReadStorage<'a, PhysicsCollider>,
    pub contact_events: Read<'a, ContactEvents>,
    pub proximity_events: Read<'a, ProximityEvents>,
    pub collisions: Write<'a, CollisionsMap>,
}

#[derive(Debug, Default)]
pub struct CollisionsDetector {
    contact_events_reader_id: Option<ReaderId<ContactEvent>>,
    proximity_events_reader_id: Option<ReaderId<ProximityEvent>>,
}

impl<'a> System<'a> for CollisionsDetector {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let Self {
            contact_events_reader_id,
            proximity_events_reader_id,
        } = self;

        let Data {
            positions,
            physics_colliders,
            contact_events,
            proximity_events,
            mut collisions,
        } = data;

        let contact_events_reader_id = contact_events_reader_id.as_mut()
            .expect("reader_id should have been configured during setup");
        let proximity_events_reader_id = proximity_events_reader_id.as_mut()
            .expect("reader_id should have been configured during setup");

        // Go through contact and proximity events and update collision map

        for event in contact_events.read(contact_events_reader_id) {
            let &ContactEvent {collider1, collider2, contact_type} = event;

            let components = (
                positions.get(collider1),
                physics_colliders.get(collider1),
                positions.get(collider2),
                physics_colliders.get(collider2),
            );
            let (
                pos1,
                shape1,
                offset1,
                pos2,
                shape2,
                offset2,
            ) = match components {
                (
                    Some(&Position(pos1)),
                    Some(PhysicsCollider {shape: shape1, offset: offset1, ..}),
                    Some(&Position(pos2)),
                    Some(PhysicsCollider {shape: shape2, offset: offset2, ..}),
                ) => (pos1, shape1, offset1, pos2, shape2, offset2),

                // Only colliders with all the necessary components can be used
                _ => continue,
            };

            // We use the centers because the bounding boxes can sometimes
            // overlap a bit when two entities are in contact. This still isn't
            // foolproof though and can fail with smaller entities and more
            // overlap.
            //TODO: Is there a better way to detect which side of an entity is
            // being touched? For example, we could take the midpoint of the
            // bottom an entity and compare it to the top of another entity to
            // see if they are very "close". This is more precise than just
            // doing a <= comparision, but has issues of its own with choosing
            // how to decide what "close" means.
            let center1 = pos1 + shape1.center().coords + offset1;
            let center2 = pos2 + shape2.center().coords + offset2;

            let (collisions1, collisions2) = collisions.get_or_default2(collider1, collider2);

            let colliders1;
            let colliders2;
            // collider1 is above collider2
            if center1.y <= center2.y {
                colliders1 = &mut collisions1.touching_bottom;
                colliders2 = &mut collisions2.touching_top;

            // collider1 is below collider2
            } else if center1.y > center2.y {
                colliders1 = &mut collisions1.touching_top;
                colliders2 = &mut collisions2.touching_bottom;

            // collider1 is to the left of collider2
            } else if center1.x <= center2.x {
                colliders1 = &mut collisions1.touching_right;
                colliders2 = &mut collisions2.touching_left;

            // collider1 is to the right of collider2
            } else if center1.x > center2.x {
                colliders1 = &mut collisions1.touching_left;
                colliders2 = &mut collisions2.touching_right;

            } else {
                // One of the colliders is inside the other one (handled through
                // proximity events)
                continue;
            }

            match contact_type {
                ContactType::Started => {
                    colliders1.push(collider2);
                    colliders2.push(collider1);
                },

                ContactType::Stopped => {
                    //TODO: Replace with `remove_item` when that is stable
                    // See: https://github.com/rust-lang/rust/issues/40062
                    if let Some(collider2_index) = colliders1.iter().position(|&x| x == collider2) {
                        colliders1.remove(collider2_index);
                    }
                    if let Some(collider1_index) = colliders2.iter().position(|&x| x == collider1) {
                        colliders2.remove(collider1_index);
                    }
                },
            }
        }

        for event in proximity_events.read(proximity_events_reader_id) {
            let &ProximityEvent {collider1, collider2, prev_status, current_status} = event;
            let (collisions1, collisions2) = collisions.get_or_default2(collider1, collider2);

            // This code assumes that both `Intersecting` and `WithinMargin`
            // indicate that the two entities are intersecting
            match (prev_status, current_status) {
                // No change since both of these just mean intersecting
                (Proximity::Intersecting, Proximity::WithinMargin) |
                (Proximity::WithinMargin, Proximity::Intersecting) => {},

                // Begin intersection
                (Proximity::Disjoint, Proximity::Intersecting) |
                (Proximity::Disjoint, Proximity::WithinMargin) => {
                    collisions1.intersecting.push(collider2);
                    collisions2.intersecting.push(collider1);
                },

                // Stop intersecting
                (Proximity::Intersecting, Proximity::Disjoint) |
                (Proximity::WithinMargin, Proximity::Disjoint) => {
                    let colliders1 = &mut collisions1.intersecting;
                    let colliders2 = &mut collisions2.intersecting;

                    //TODO: Replace with `remove_item` when that is stable
                    // See: https://github.com/rust-lang/rust/issues/40062
                    if let Some(collider2_index) = colliders1.iter().position(|&x| x == collider2) {
                        colliders1.remove(collider2_index);
                    }
                    if let Some(collider1_index) = colliders2.iter().position(|&x| x == collider1) {
                        colliders2.remove(collider1_index);
                    }
                },

                (Proximity::Intersecting, Proximity::Intersecting) |
                (Proximity::WithinMargin, Proximity::WithinMargin) |
                (Proximity::Disjoint, Proximity::Disjoint) => {
                    unreachable!("bug: prev_status is guaranteed to be different from current_status");
                },
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        // register reader id for the contact events
        let mut contact_events = world.write_resource::<ContactEvents>();
        self.contact_events_reader_id = Some(contact_events.register_reader());

        // register reader id for the proximity events
        let mut proximity_events = world.write_resource::<ProximityEvents>();
        self.proximity_events_reader_id = Some(proximity_events.register_reader());
    }
}
