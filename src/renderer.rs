mod image_cache;

pub use image_cache::*;

use sdl2::pixels::Color;

#[derive(Debug)]
pub struct Renderer {
    image_cache: ImageCache,
    background_color: Color,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            image_cache: ImageCache::default(),
            background_color: Color::RGBA(0, 0, 0, 0),
        }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn set_background_color(&mut self, background_color: Color) {
        self.background_color = background_color;
    }
}
