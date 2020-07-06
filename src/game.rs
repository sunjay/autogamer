use crate::{Size, Renderer};

#[derive(Debug)]
pub struct Game {
    window_size: Size,
    renderer: Renderer,
}

impl Game {
    pub fn new(window_size: Size) -> Self {
        Self {
            window_size,
            renderer: Renderer::new(),
        }
    }

    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn window_width(&self) -> u32 {
        self.window_size.width
    }

    pub fn window_height(&self) -> u32 {
        self.window_size.height
    }
}
