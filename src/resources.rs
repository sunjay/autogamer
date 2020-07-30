mod physics_events;

pub use physics_events::*;

use sdl2::rect::Rect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Viewport(pub Rect);
