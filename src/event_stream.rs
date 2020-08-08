use crate::Event;

pub trait EventStreamSource {
    fn len(&self) -> usize;

    fn for_each_event<F>(&self, f: F)
        where F: FnMut(&mut Event);
}

#[derive(Debug, Default)]
pub struct EventStream {
    events: Vec<Event>,
}

impl EventStream {
    /// Iterate through the events currently stored in the stream
    pub fn iter(&self) -> EventStreamIterator {
        EventStreamIterator {
            events: &self.events,
        }
    }

    pub fn refill<E: EventStreamSource>(&mut self, events: &E) {
        self.clear();
        self.events.reserve(events.len());
        events.for_each_event(|event| self.push(event.clone()));
    }

    /// Clears all events currently stored in the stream
    ///
    /// This does not impact the memory allocated for the events. That memory
    /// will be reused for the next set of events.
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Push an event into the event stream
    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }
}

impl Extend<Event> for EventStream {
    fn extend<T: IntoIterator<Item = Event>>(&mut self, iter: T) {
        self.events.extend(iter);
    }
}

impl<'a> IntoIterator for &'a EventStream {
    type IntoIter = EventStreamIterator<'a>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
pub struct EventStreamIterator<'a> {
    events: &'a [Event],
}

impl<'a> ExactSizeIterator for EventStreamIterator<'a> {
    fn len(&self) -> usize {
        self.events.len()
    }
}

impl<'a> Iterator for EventStreamIterator<'a> {
    type Item = &'a Event;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let event = self.events.get(0)?;
            self.events = &self.events[1..];

            if event.should_propagate() {
                break Some(event);
            }
        }
    }
}
