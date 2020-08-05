use specs::{System, SystemData, World, Read, ReadExpect, Entities, ReadStorage, WriteStorage, Join, prelude::ResourceId};

use crate::{PhysicsBody, PlatformerControls, EventStream, EventKind, Key, CollisionsMap};

#[derive(SystemData)]
pub struct Data<'a> {
    pub events: ReadExpect<'a, EventStream>,
    pub collisions: Read<'a, CollisionsMap>,
    pub entities: Entities<'a>,
    pub platformer_controls: ReadStorage<'a, PlatformerControls>,
    pub physics_bodies: WriteStorage<'a, PhysicsBody>,
}

#[derive(Debug, Default)]
pub struct Keyboard {
    left_pressed: bool,
    right_pressed: bool,
}

impl<'a> System<'a> for Keyboard {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let Data {
            events,
            collisions,
            entities,
            platformer_controls,
            mut physics_bodies,
        } = data;

        // Update the current state based on the events
        let mut initiate_jump = false;
        for event in events.iter() {
            use EventKind::*;
            match event.kind() {
                KeyDown {key: Key::Left, repeat: false, ..} => {
                    self.left_pressed = true;
                },
                KeyDown {key: Key::Right, repeat: false, ..} => {
                    self.right_pressed = true;
                },

                KeyUp {key: Key::Left, ..} => {
                    self.left_pressed = false;
                },
                KeyUp {key: Key::Right, ..} => {
                    self.right_pressed = false;
                },

                KeyDown {key: Key::Space, repeat: false, ..} => {
                    initiate_jump = true;
                },

                _ => {},
            }
        }

        // Update entities based on the current state
        for (entity, controls, body) in (&entities, &platformer_controls, &mut physics_bodies).join() {
            let collisions = collisions.get(entity);
            let touching_ground = !collisions.touching_bottom.is_empty();

            // Slow down movement in midair
            //TODO: Make this configurable
            let hori_multiplier = if touching_ground { 1.0 } else { 0.5 };

            // If neither are pressed or both are pressed, set speed to zero
            if !(self.left_pressed ^ self.right_pressed) {
                body.velocity.linear.x = 0.0;
            } else if self.left_pressed {
                body.velocity.linear.x = controls.left_velocity * hori_multiplier;
            } else if self.right_pressed {
                body.velocity.linear.x = controls.right_velocity * hori_multiplier;
            }

            // Only jump if currently touching the ground
            if initiate_jump && touching_ground {
                body.velocity.linear.y = controls.jump_velocity;
            }
        }
    }
}
