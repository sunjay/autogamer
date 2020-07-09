use sdl2::{
    Sdl,
    EventPump,
    VideoSubsystem,
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
    video_subsystem: VideoSubsystem,
    event_pump: EventPump,
}

impl Window {
    pub fn new(title: &str, size: Size) -> Result<(Self, WindowCanvas), SdlError> {
        let _sdl_context = sdl2::init().map_err(SdlError)?;
        let video_subsystem = _sdl_context.video().map_err(SdlError)?;
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)
            .map_err(SdlError)?;

        let window = video_subsystem.window(title, size.width, size.height)
            .allow_highdpi()
            .position_centered()
            .build()
            .map_err(|e| SdlError(e.to_string()))?;
        let canvas = window.into_canvas()
            .build()
            .map_err(|e| SdlError(e.to_string()))?;
        let event_pump = _sdl_context.event_pump().map_err(SdlError)?;

        Ok((Self {_sdl_context, _image_context, video_subsystem, event_pump}, canvas))
    }

    /// Returns the DPI scale factor
    pub fn scale_factor(&self) -> f64 {
        // Ignoring error and using 1.0 if no scale factor could be determined
        //
        // See: https://nlguillemot.wordpress.com/2016/12/11/high-dpi-rendering/
        self.video_subsystem.display_dpi(0)
            .map(|(scale, _, _)| scale as f64)
            .unwrap_or(1.0)
    }

    pub fn poll_iter(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }
}
