use std::env;
use std::process::exit;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::time::Instant;

mod model;
use crate::model::{load, Sequence};

#[derive(Debug)]
struct Player {
    grid_usec: u128,
    sequences: HashMap<String, Sequence>,
    playing:   Vec<String>,
}

impl Player {
    pub fn play (&mut self) {
        let playback_started = Instant::now();
        let mut last_step: i128 = -1;
        loop {
            let elapsed_usec = playback_started.elapsed().as_micros();
            let grid_step = elapsed_usec / self.grid_usec;
            let grid_jitter = elapsed_usec % self.grid_usec;
            if grid_step as i128 > last_step {
                //println!("{:?}", frame);
                last_step = grid_step as i128;
                println!(
                    "{} {} {} {}",
                    elapsed_usec,
                    grid_step,
                    grid_jitter,
                    playback_started.elapsed().as_micros() - elapsed_usec
                );
                for name in &self.playing {
                    let sequence = self.sequences.get(name).unwrap();
                    let index = elapsed_usec % (sequence.length * 1000) / 1000;
                    print!(" {} {} ", &sequence.length * 1000, &index);
                    match sequence.events.get(&index) {
                        None    => print!(""),
                        Some(x) => print!("{}", &x.name)
                    }
                    println!(" {} ", &name);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let (grid, sequences, playing) = load(reader);
    let grid_usec = grid * 1000;
    let mut player = Player { grid_usec, sequences, playing };
    println!("{:?}", player);
    player.play();
}
