mod physics;
mod keyboard;

use specs::{World, System};

#[derive(Default)]
pub struct Systems {
    keyboard: keyboard::Keyboard,
    physics: physics::Physics,
}

impl Systems {
    pub fn run(&mut self, world: &World) {
        let Self {
            keyboard,
            physics,
        } = self;

        keyboard.run(world.system_data());
        physics.run(world.system_data());
    }
}
