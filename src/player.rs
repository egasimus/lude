use std::collections::HashMap;
use std::time::Instant;
use crate::model::Sequence;

#[derive(Debug)]
pub struct Player {
    pub grid_usec: u128,
    pub sequences: HashMap<String, Sequence>,
    pub playing:   Vec<String>,
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
