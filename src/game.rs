use crate::{Size, Renderer, Window, SdlError};

#[derive(Debug)]
pub struct Game {
    title: String,
    window_size: Size,
    renderer: Renderer,
}

impl Game {
    pub fn new(title: String, window_size: Size) -> Self {
        Self {
            title,
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

    pub fn create_window(&self) -> Result<Window, SdlError> {
        Window::new(&self.title, self.window_size)
    }
}
