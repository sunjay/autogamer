use autogamer as ag;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Event {
    event: ag::Event,
}

impl Event {
    pub fn new(event: ag::Event) -> Self {
        Self {event}
    }

    pub fn inner(&self) -> &ag::Event {
        &self.event
    }

    pub fn inner_mut(&mut self) -> &mut ag::Event {
        &mut self.event
    }
}

#[pymethods]
impl Event {
    /// Stops this event from ever being yielded by the event stream again
    pub fn stop_propagation(&mut self) {
        self.event.stop_propagation();
    }
}
