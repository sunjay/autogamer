mod image_cache;

pub use image_cache::*;
pub use sdl2::render::WindowCanvas;

use std::{sync::Arc, fmt};

use parking_lot::Mutex;
use sdl2::pixels::Color;

pub struct Renderer {
    canvas: WindowCanvas,
    image_cache: Arc<Mutex<ImageCache>>,
    scale_factor: f64,
}

impl fmt::Debug for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            canvas: _,
            image_cache,
            scale_factor,
        } = self;

        f.debug_struct("Renderer")
            .field("canvas", &"WindowCanvas")
            .field("image_cache", &image_cache)
            .field("scale_factor", &scale_factor)
            .finish()
    }
}

impl Renderer {
    pub fn new(canvas: WindowCanvas, image_cache: Arc<Mutex<ImageCache>>) -> Self {
        Self {
            canvas,
            image_cache,
            scale_factor: 1.0,
        }
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    pub fn set_scale_factor(&mut self, scale_factor: f64) {
        self.scale_factor = scale_factor;
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
