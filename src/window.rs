use std::env;

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
    scale_factor: f64,
}

impl Window {
    pub fn new(title: &str, size: Size) -> Result<(Self, WindowCanvas), SdlError> {
        let _sdl_context = sdl2::init().map_err(SdlError)?;
        let video_subsystem = _sdl_context.video().map_err(SdlError)?;
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)
            .map_err(SdlError)?;

        //TODO: SDL2 doesn't provide a reliable scale factor through its DPI
        // support. Rather than deal with that in some fragile way using
        // hard-coded values, I have chosen to just use an environment variable.
        // See: https://nlguillemot.wordpress.com/2016/12/11/high-dpi-rendering/
        let scale_factor = env::var("DISPLAY_SCALE")
            .map(|x| x.parse().expect("DISPLAY_SCALE must be a number"))
            .unwrap_or(1.0);

        let width = (size.width as f64 * scale_factor) as u32;
        let height = (size.height as f64 * scale_factor) as u32;
        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| SdlError(e.to_string()))?;
        let canvas = window.into_canvas()
            .build()
            .map_err(|e| SdlError(e.to_string()))?;
        let event_pump = _sdl_context.event_pump().map_err(SdlError)?;

        Ok((Self {
            _sdl_context,
            _image_context,
            event_pump,
            scale_factor,
        }, canvas))
    }

    /// Returns the DPI scale factor
    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    pub fn poll_iter(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }
}
