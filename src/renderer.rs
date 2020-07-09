mod image_cache;

pub use image_cache::*;
pub use sdl2::render::WindowCanvas;

use std::{sync::Arc, fmt};

use parking_lot::Mutex;

pub struct Renderer {
    canvas: WindowCanvas,
    image_cache: Arc<Mutex<ImageCache>>,
}

impl fmt::Debug for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            canvas: _,
            image_cache,
        } = self;

        f.debug_struct("Renderer")
            .field("canvas", &"WindowCanvas")
            .field("image_cache", &image_cache)
            .finish()
    }
}

impl Renderer {
    pub fn new(canvas: WindowCanvas, image_cache: Arc<Mutex<ImageCache>>) -> Self {
        Self {canvas, image_cache}
    }
}
