mod key;
mod event_kind;

pub use key::*;
pub use event_kind::*;

use std::sync::{atomic::{Ordering, AtomicBool}, Arc};

#[derive(Debug, Clone)]
pub struct Event {
    kind: EventKind,
    /// true if this event should continue to be propagated
    ///
    /// Every clone of this event shares the same `propagate` flag
    propagate: Arc<AtomicBool>,
}

impl Event {
    pub fn new(kind: EventKind) -> Self {
        Self {
            kind,
            propagate: Arc::new(AtomicBool::new(true)),
        }
    }

    /// Returns the event kind
    pub fn kind(&self) -> &EventKind {
        &self.kind
    }

    /// Returns true if this event should continue to propagate
    pub fn should_propagate(&self) -> bool {
        self.propagate.load(Ordering::SeqCst)
    }

    /// Stops this event from ever being yielded by the event stream again
    pub fn stop_propagation(&mut self) {
        self.propagate.store(false, Ordering::SeqCst)
    }
}
