use indexmap::IndexMap;

pub type Duration = u128;

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

#[derive(Debug)]
pub enum Commands {
    NOP,
    Sound,
    Sequence
}

type CommandArg = String;

#[derive(Debug)]
pub struct Command {
    pub name: Commands,
    pub args: Vec<CommandArg>
}

impl Command {

    pub fn nop () -> Command {
        Command { name: Commands::NOP, args: vec![] }
    }

    pub fn new (name: Commands, args: Vec<CommandArg>) -> Command {
        Command { name, args }
    }

}

pub fn command (command: &Command) {
    println!("[{:#?}]", &command);
}