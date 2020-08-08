mod physics;
mod keyboard;
mod viewport_updater;
mod collision_detector;
mod currency_system;

use specs::{World, System};

#[derive(Default)]
pub struct Systems {
    pub keyboard: keyboard::Keyboard,
    pub physics: physics::Physics,
    pub collision_detector: collision_detector::CollisionsDetector,
    pub viewport_updater: viewport_updater::ViewportUpdater,
    pub currency_system: currency_system::CurrencySystem,
}

impl Systems {
    pub fn setup(&mut self, world: &mut World) {
        let Self {
            keyboard,
            physics,
            collision_detector,
            viewport_updater,
            currency_system,
        } = self;

        keyboard.setup(world);
        physics.setup(world);
        collision_detector.setup(world);
        viewport_updater.setup(world);
        currency_system.setup(world);
    }

    pub fn run(&mut self, world: &World) {
        let Self {
            keyboard,
            physics,
            collision_detector,
            viewport_updater,
            currency_system,
        } = self;

        keyboard.run(world.system_data());
        physics.run(world.system_data());

        rayon::join(
            || collision_detector.run(world.system_data()),
            || viewport_updater.run(world.system_data()),
        );

        currency_system.run(world.system_data());
    }
}
