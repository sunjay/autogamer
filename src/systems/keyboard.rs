use specs::{System, SystemData, World, ReadExpect, prelude::ResourceId};

use crate::EventStream;

#[derive(SystemData)]
pub struct Data<'a> {
    pub events: ReadExpect<'a, EventStream>,
}

#[derive(Debug, Default)]
pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let Data {
            events,
        } = data;
    }
}
