use std::vec::IntoIter;

use autogamer as ag;
use pyo3::prelude::*;
use pyo3::PyIterProtocol;

use crate::Event;

#[pyclass]
#[derive(Debug, Default)]
pub struct EventStream {
    events: Vec<Py<Event>>,
}

#[pyproto]
impl PyIterProtocol for EventStream {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<EventStreamIter>> {
        let py = slf.py();
        let iter = slf.iter(py);

        Py::new(py, iter)
    }
}

impl EventStream {
    /// Iterate through the events currently stored in the stream
    pub fn iter(&self, py: Python) -> EventStreamIter {
        let events: Vec<_> = self.events.iter()
            .filter(|event| event.borrow(py).inner().should_propagate())
            .map(|event| event.clone_ref(py))
            .collect();

        EventStreamIter {
            inner: events.into_iter(),
        }
    }

    /// Clears all events currently stored in the stream
    ///
    /// This does not impact the memory allocated for the events. That memory
    /// will be reused for the next set of events.
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Push an event into the event stream
    pub fn push(&mut self, event: Py<Event>) {
        self.events.push(event);
    }

    /// Append the entire contents of a `Vec` into the event stream
    pub fn append(&mut self, mut other: Vec<Py<Event>>) {
        self.events.append(&mut other);
    }
}

impl Extend<Py<Event>> for EventStream {
    fn extend<T: IntoIterator<Item = Py<Event>>>(&mut self, iter: T) {
        self.events.extend(iter);
    }
}

impl ag::EventStreamSource for EventStream {
    fn len(&self) -> usize {
        self.events.len()
    }

    fn for_each_event<F>(&self, mut f: F)
        where F: FnMut(&mut ag::Event)
    {
        let gil = GILGuard::acquire();
        let py = gil.python();
        for event in &self.events {
            let mut event = event.borrow_mut(py);
            let event = event.inner_mut();
            if event.should_propagate() {
                f(event);
            }
        }
    }
}

#[pyclass]
#[derive(Debug)]
pub struct EventStreamIter {
    inner: IntoIter<Py<Event>>,
}

#[pyproto]
impl PyIterProtocol for EventStreamIter {
    fn __iter__(slf: PyRefMut<Self>) -> Py<Self> {
        slf.into()
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<Py<Event>> {
        slf.next()
    }
}

impl ExactSizeIterator for EventStreamIter {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl Iterator for EventStreamIter {
    type Item = Py<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let event = self.inner.next()?;

            let gil = GILGuard::acquire();
            let py = gil.python();
            if event.borrow(py).inner().should_propagate() {
                break Some(event);
            }
        }
    }
}
