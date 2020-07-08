mod image_cache;

pub use image_cache::*;

#[derive(Debug)]
pub struct Renderer {
    image_cache: ImageCache,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            image_cache: ImageCache::default(),
        }
    }

    pub fn image_cache_mut(&mut self) -> &mut ImageCache {
        &mut self.image_cache
    }
}
