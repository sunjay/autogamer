mod physics;
mod keyboard;
mod viewport_updater;

use specs::{World, System};

#[derive(Default)]
pub struct Systems {
    pub keyboard: keyboard::Keyboard,
    pub physics: physics::Physics,
    pub viewport_updater: viewport_updater::ViewportUpdater,
}

impl Systems {
    pub fn setup(&mut self, world: &mut World) {
        let Self {
            keyboard,
            physics,
            viewport_updater,
        } = self;

        keyboard.setup(world);
        physics.setup(world);
        viewport_updater.setup(world);
    }

    pub fn run(&mut self, world: &World) {
        let Self {
            keyboard,
            physics,
            viewport_updater,
        } = self;

        keyboard.run(world.system_data());
        physics.run(world.system_data());
        viewport_updater.run(world.system_data());
    }
}
