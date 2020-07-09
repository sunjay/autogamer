use std::sync::Arc;

use sdl2::render::WindowCanvas;
use parking_lot::{Mutex, MutexGuard};

use crate::{Size, Window, SdlError, ImageCache};

#[derive(Debug)]
pub struct Game {
    title: String,
    window_size: Size,
    /// The global image cache, shared by all screens and the renderer
    image_cache: Arc<Mutex<ImageCache>>,
}

impl Game {
    pub fn new(title: String, window_size: Size) -> Self {
        Self {
            title,
            window_size,
            image_cache: Default::default(),
        }
    }

    pub fn image_cache(&self) -> &Arc<Mutex<ImageCache>> {
        &self.image_cache
    }

    pub fn image_cache_mut(&mut self) -> MutexGuard<ImageCache> {
        self.image_cache.lock()
    }

    pub fn window_width(&self) -> u32 {
        self.window_size.width
    }

    pub fn window_height(&self) -> u32 {
        self.window_size.height
    }

    pub fn create_window(&self) -> Result<(Window, WindowCanvas), SdlError> {
        Window::new(&self.title, self.window_size)
    }
}
