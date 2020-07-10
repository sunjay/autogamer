use crate::Event;

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
            let item = self.events.get(0)?;
            self.events = &self.events[1..];

            if item.should_propagate() {
                break Some(item);
            }
        }
    }
}
