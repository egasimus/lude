use std::collections::BTreeMap;

pub type Moment = u128;
pub type Duration = u128;
pub type EventMap<E> = BTreeMap<Moment, Vec<E>>;

#[derive(Debug)]
pub struct Timeline<E> {
    pub events: EventMap<E>,
    pub duration: Duration
}

impl<E> Timeline<E> {
    pub fn new () -> Timeline<E> {
        Timeline { events: BTreeMap::new(), duration: 0 }
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
}

/*
impl<E> Timeline<_> {
    pub fn new () -> Timeline<E> {
        Timeline { events: BTreeMap::new(), duration: 0 }
    }
}

#[derive(Debug)]
pub struct Event {
    pub name: String
}

impl Event {
    pub fn new (name: String) -> Event {
        Event { name }
    }
}

#[derive(Debug)]
pub struct Sequence {
    pub events: IndexMap<u128, Vec<Event>>,
    pub length: u128
}

impl Sequence {
    pub fn new () -> Sequence {
        Sequence { events: IndexMap::new(), length: 0 }
    }

    pub fn add (&mut self, index: u128, event: &str) {
        match self.events.get_mut(&index) {
            Some(events) => {
                events.push(Event::new(event.to_string()))
            }
            None => {
                let mut events = Vec::new();
                events.push(Event::new(event.to_string()));
                self.events.insert(index, events);
            }
        }
    }

    pub fn get (&self, step: u128) -> Option<&Vec<Event>> {
        let step = step % self.length;
        self.events.get(&step)
    }
}
*/
