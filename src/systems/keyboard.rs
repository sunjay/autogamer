use std::collections::HashMap;

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

/// The state of all keys on the keyboard
#[derive(Debug, Default)]
struct KeyboardState {
    /// Stores true if the key is currently pressed, false otherwise
    ///
    /// Assumes key is currently released if no events for that key have been
    /// observed
    key_is_pressed: HashMap<Key, bool>,
}

impl KeyboardState {
    /// Returns true if the given key is currently pressed
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.key_is_pressed.get(&key).copied().unwrap_or_default()
    }

    /// Updates the current state of keys based on the given events
    pub fn update(&mut self, events: &EventStream) {
        for event in events {
            use EventKind::*;
            match event.kind() {
                &KeyDown {key, repeat: false, ..} => {
                    self.key_is_pressed.insert(key, true);
                },

                &KeyUp {key, ..} => {
                    self.key_is_pressed.insert(key, false);
                },

                _ => {},
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Keyboard {
    // Note: this is currently stored here for convenience. It could later
    // become a resource managed by this system.
    keyboard_state: KeyboardState,
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

        self.keyboard_state.update(&events);

        // Update entities based on the current state
        for (entity, controls, body) in (&entities, &platformer_controls, &mut physics_bodies).join() {
            let &PlatformerControls {
                left_key,
                right_key,
                jump_key,
                horizontal_velocity,
                jump_velocity,
                midair_horizontal_multiplier,
            } = controls;

            let left_pressed = self.keyboard_state.is_key_pressed(left_key);
            let right_pressed = self.keyboard_state.is_key_pressed(right_key);
            let jump_pressed = self.keyboard_state.is_key_pressed(jump_key);

            let collisions = collisions.get(entity);
            let touching_ground = !collisions.touching_bottom.is_empty();

            // Potentially change movement in midair
            let hori_multiplier = if touching_ground {
                1.0
            } else {
                midair_horizontal_multiplier
            };

            // If neither is pressed or both are pressed, set speed to zero
            if left_pressed == right_pressed {
                body.velocity.linear.x = 0.0;
            } else if left_pressed {
                body.velocity.linear.x = -horizontal_velocity * hori_multiplier;
            } else if right_pressed {
                body.velocity.linear.x = horizontal_velocity * hori_multiplier;
            }

            // Only jump if currently touching the ground
            if jump_pressed && touching_ground {
                body.velocity.linear.y = jump_velocity;
            }
        }
    }
}
