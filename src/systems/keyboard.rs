use specs::{System, SystemData, World, ReadExpect, ReadStorage, WriteStorage, Join, prelude::ResourceId};

use crate::{PhysicsBody, PlatformerControls, EventStream, EventKind, Key};

#[derive(SystemData)]
pub struct Data<'a> {
    pub events: ReadExpect<'a, EventStream>,
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
            platformer_controls,
            mut physics_bodies,
        } = data;

        // Update the current state based on the events
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

                _ => {},
            }
        }

        // Update entities based on the current state
        for (controls, body) in (&platformer_controls, &mut physics_bodies).join() {
            // Assuming that only a single arrow key can be held down at a time.
            if self.left_pressed {
                body.velocity.linear.x = controls.left_velocity;
            } else if self.right_pressed {
                body.velocity.linear.x = controls.right_velocity;
            } else {
                body.velocity.linear.x = 0.0;
            }
        }
    }
}
