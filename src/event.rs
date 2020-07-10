mod key;
mod event_kind;

pub use key::*;
pub use event_kind::*;

pub trait EventStream {
    fn for_each_event<F>(&self, f: F)
        where F: FnMut(&mut Event);
}

#[derive(Debug, Clone)]
pub struct Event {
    kind: EventKind,
    /// true if this event should continue to be propagated
    propagate: bool,
}

impl Event {
    pub fn new(kind: EventKind) -> Self {
        Self {
            kind,
            propagate: true,
        }
    }

    /// Returns the event kind
    pub fn kind(&self) -> &EventKind {
        &self.kind
    }

    /// Returns true if this event should continue to propagate
    pub fn should_propagate(&self) -> bool {
        self.propagate
    }

    /// Stops this event from ever being yielded by the event stream again
    pub fn stop_propagation(&mut self) {
        self.propagate = false;
    }
}
