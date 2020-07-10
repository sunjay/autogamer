use std::thread;
use std::time::{Instant, Duration};

use autogamer as ag;
use pyo3::prelude::*;
use pyo3::PyTraverseError;
use pyo3::gc::{PyGCProtocol, PyVisit};
use pyo3::exceptions::ValueError;

use crate::*;

#[pyclass(gc, unsendable)]
#[derive(Debug)]
pub struct Game {
    game: ag::Game,
    current_screen: Option<Py<Screen>>
}

impl Game {
    pub fn inner(&self) -> &ag::Game {
        &self.game
    }

    pub fn inner_mut(&mut self) -> &mut ag::Game {
        &mut self.game
    }
}

#[pyproto]
impl PyGCProtocol for Game {
    fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        let Self {
            game: _,
            current_screen,
        } = self;

        if let Some(current_screen) = current_screen {
            visit.call(current_screen)?;
        }

        Ok(())
    }

    fn __clear__(&mut self) {
        let Self {
            game: _,
            current_screen,
        } = self;

        // Release reference, this decrements the ref counter
        let gil = GILGuard::acquire();
        let py = gil.python();

        if let Some(current_screen) = current_screen.take() {
            py.release(&current_screen);
        }
    }
}

#[pymethods]
impl Game {
    #[new]
    #[args(
        "*",
        title = "\"autogamer\".to_string()",
        window_width = 800,
        window_height = 600,
    )]
    pub fn new(
        title: String,
        window_width: u32,
        window_height: u32,
    ) -> Self {
        Self {
            game: ag::Game::new(title, ag::Size {
                width: window_width,
                height: window_height,
            }),
            current_screen: None,
        }
    }

    /// Sets the current screen of the game to the given screen
    pub fn set_screen(&mut self, screen: Py<Screen>) {
        self.current_screen = Some(screen);
    }

    /// Runs the game main loop until either the window is closed or the game
    /// loop is ended by the game itself
    pub fn run(slf: PyRefMut<Self>) -> PyResult<()> {
        /// The maximum frames per second - used to limit the speed at which
        /// update() and render() are called
        const MAX_FPS: u64 = 60;
        /// 1,000,000 us in 1 s
        const MICROS_PER_SEC: u64 = 1_000_000;

        // No screen configured, quit immediately
        if slf.current_screen.is_none() {
            return Ok(());
        }

        let (mut window, canvas) = slf.game.create_window()
            .map_err(|err| ValueError::py_err(err.to_string()))?;
        let image_cache = slf.game.image_cache().clone();

        // Create the texture creator that will load images
        let texture_creator = canvas.texture_creator();
        image_cache.lock().set_texture_creator(texture_creator);

        let renderer = Py::new(slf.py(), Renderer::new(canvas, image_cache))?;

        let frame_duration = Duration::from_micros(MICROS_PER_SEC / MAX_FPS);
        let mut last_frame = Instant::now();

        let events = Py::new(slf.py(), EventStream::default())?;

        let mut running = true;
        while running {
            let current_events = window.poll_events()
                .map(|event| Py::new(slf.py(), Event::new(event)))
                .collect::<Result<Vec<_>, _>>()?;
            events.borrow_mut(slf.py()).append(current_events);

            // Make sure we don't update too often or we may mess up physics
            // calculations or cause rendering bottlenecks
            let time_elapsed = last_frame.elapsed();
            let frames_elapsed = time_elapsed.as_micros() / frame_duration.as_micros();
            if frames_elapsed >= 1 {
                // Note: technically, we could make the simulation more accurate
                // by simulating multiple frames (calling update() multiple
                // times) if more than one frame has elapsed. This is dangerous
                // though because there's a chance that we might enter a
                // never-ending cycle of trying to catch up to a point where
                // only 1 or 0 frames have elapsed.
                // Skipping any additional frames that we may have missed works
                // around this at the cost of the game potentially lagging a bit
                // if either update or render are particularly slow.

                // Get the current screen from self directly since it may have
                // changed during the previous iteration
                let current_screen = slf.current_screen.as_ref()
                    .expect("bug: should be impossible to set current_screen to None once initialized");
                //TODO: Calling trait method in fully-qualified form because
                // rust-analyzer couldn't handle two different as_ref methods
                // taking a different number of arguments
                let current_screen = AsPyRef::as_ref(current_screen, slf.py());

                // Need to use call_method because we want to call the
                // overridden versions of these methods, not just the methods on
                // the base Screen class
                current_screen.call_method1("update", (&events,))?;

                // Check if we need to quit
                //
                // Doing this after update so the game code has the opportunity
                // to stop propagation on the quit or keyboard events used here
                for event in events.borrow(slf.py()).iter(slf.py()) {
                    match event.borrow(slf.py()).inner().kind() {
                        ag::EventKind::Quit {..} |
                        ag::EventKind::KeyUp {key: ag::Key::Escape, ..} => {
                            running = false;
                        },
                        _ => {},
                    }
                }

                // Clear events so we don't get stale data next time
                // Reuses the previously allocated memory for the events
                events.borrow_mut(slf.py()).clear();

                // Render the updated state to the screen
                current_screen.call_method1("draw", (&renderer,))?;

                // Record the time the frame was completed
                last_frame = Instant::now();

            } else {
                thread::sleep(frame_duration - time_elapsed);
            }
        }

        Ok(())
    }
}
