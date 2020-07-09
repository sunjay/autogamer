use sdl2::{
    Sdl,
    EventPump,
    event::EventPollIterator,
    image::{InitFlag, Sdl2ImageContext},
    render::WindowCanvas,
};
use thiserror::Error;

use crate::Size;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct SdlError(String);

pub struct Window {
    _sdl_context: Sdl,
    _image_context: Sdl2ImageContext,
    event_pump: EventPump,
}

impl Window {
    pub fn new(title: &str, size: Size) -> Result<(Self, WindowCanvas), SdlError> {
        let _sdl_context = sdl2::init().map_err(SdlError)?;
        let video_subsystem = _sdl_context.video().map_err(SdlError)?;
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)
            .map_err(SdlError)?;

        let window = video_subsystem.window(title, size.width, size.height)
            .position_centered()
            .build()
            .map_err(|e| SdlError(e.to_string()))?;
        let canvas = window.into_canvas()
            .build()
            .map_err(|e| SdlError(e.to_string()))?;
        let event_pump = _sdl_context.event_pump().map_err(SdlError)?;

        Ok((Self {_sdl_context, _image_context, event_pump}, canvas))
    }

    pub fn poll_iter(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }
}
