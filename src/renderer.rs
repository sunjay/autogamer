mod image_cache;

pub use image_cache::*;
pub use sdl2::render::WindowCanvas;

use std::{sync::Arc, fmt};

use parking_lot::Mutex;
use sdl2::{rect::{Rect, Point}, pixels::Color};

use crate::{SdlError, Size, ImageParams};

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
    pub fn new(
        canvas: WindowCanvas,
        image_cache: Arc<Mutex<ImageCache>>,
    ) -> Self {
        Self {canvas, image_cache}
    }

    pub fn size(&self) -> Size {
        let (width, height) = self.canvas.logical_size();
        Size {width, height}
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn draw_image(
        &mut self,
        image: ImageId,
        params: ImageParams,
        top_left: Point,
    ) -> Result<(), SdlError> {
        let size = params.size;
        let dest = Rect::new(
            top_left.x(),
            top_left.y(),
            size.width,
            size.height,
        );

        let mut image_cache = self.image_cache.lock();
        let tex = image_cache.load(image, params)?;

        self.canvas.copy(tex, None, dest)?;

        Ok(())
    }
}
