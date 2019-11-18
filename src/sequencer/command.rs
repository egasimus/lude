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
