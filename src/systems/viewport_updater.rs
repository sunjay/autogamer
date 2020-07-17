use specs::{System, SystemData, World, WriteExpect, ReadStorage, Join, prelude::ResourceId};

use crate::{Position, Viewport, ViewportTarget};

#[derive(SystemData)]
pub struct Data<'a> {
    pub viewport: WriteExpect<'a, Viewport>,
    pub positions: ReadStorage<'a, Position>,
    pub viewport_targets: ReadStorage<'a, ViewportTarget>,
}

#[derive(Debug, Default)]
pub struct ViewportUpdater;

impl<'a> System<'a> for ViewportUpdater {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let Data {
            mut viewport,
            positions,
            viewport_targets,
        } = data;

        let Viewport(viewport) = &mut *viewport;

        let mut found = false;
        for (Position(pos), ViewportTarget) in (&positions, &viewport_targets).join() {
            if found {
                println!("Warning: multiple viewport targets detected");
            }
            found = true;

            // Center viewport around position
            viewport.set_x(pos.x as i32 - viewport.width() as i32 / 2);
            viewport.set_y(pos.y as i32 - viewport.height() as i32 / 2);

            //TODO: Deal with viewport boundaries
        }
    }
}
