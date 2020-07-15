use specs::{System, SystemData, ReadStorage, World, prelude::ResourceId};

use crate::{Position};

#[derive(SystemData)]
pub struct Data<'a> {
    pub positions: ReadStorage<'a, Position>,
}

#[derive(Debug, Default)]
pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let Data {
            positions,
        } = data;
    }
}
