use crate::Size;

#[derive(Debug)]
pub struct Game {
    window_size: Size,
}

impl Game {
    pub fn new(window_size: Size) -> Self {
        Self {window_size}
    }

    pub fn window_width(&self) -> u32 {
        self.window_size.width
    }

    pub fn window_height(&self) -> u32 {
        self.window_size.height
    }
}
