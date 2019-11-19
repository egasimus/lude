mod sound;
pub use sound::{Sampler, Sound};

mod sequence;
pub use sequence::{Sequencer, Sequence};

pub type Identifier = String;

#[derive(Debug)]
enum Schema {
    Sound,
    Sequence
}

type Path = String;

#[derive(Debug)]
pub struct Resource {
    schema: Schema,
    path:   Path
}

type Resources = HashMap<Identifier, Resource>;

type Definitions = HashMap<Identifier, Command>;

type Sequences = HashMap<Identifier, Sequence>;

/*
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
    eprintln!("[{:#?}]", &command);
}
*/
