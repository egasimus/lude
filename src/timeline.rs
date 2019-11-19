use std::collections::BTreeMap;

pub type Moment = u128;
pub type EventMap<E> = BTreeMap<Moment, Vec<E>>;

#[derive(Debug)]
pub struct Timeline<E> {
    events: EventMap<E>,
    pub start: Moment,
    pub end: Moment,
}

impl<E> Timeline<E> {
    pub fn new () -> Timeline<E> {
        Timeline { events: BTreeMap::new(), start: 0, end: 0 }
    }
    pub fn add (&mut self, index: u128, event: E) {
        match self.events.get_mut(&index) {
            Some(events) => {
                events.push(event)
            }
            None => {
                let mut events = Vec::new();
                events.push(event);
                self.events.insert(index, events);
            }
        }
    }
    pub fn get (&self, step: Moment) -> Option<&Vec<E>> {
        self.events.get(&step)
    }
}
