use std::collections::HashMap;
use std::time::Instant;
use crate::parser::Document;
use crate::model::Event;

#[derive(Debug)]
pub struct Player {
    grid_usec: u128,
    document: Document,
    started: Option<Instant>,
    last_step: Option<u128>
}

impl Player {

    pub fn new (document: Document) -> Player {
        Player {
            grid_usec: 234000,
            started:   None,
            last_step: None,
            document
        }
    }

    pub fn play (&mut self) {
        self.started = Some(Instant::now());
        loop {
            match self.started {
                Some(_) => self.tick(),
                None => break
            }
        }
    }

    pub fn stop (&mut self) {
        self.started = None
    }

    fn get_step (&self) -> (u128, u128) {
        let elapsed = self.started.unwrap().elapsed().as_micros();
        let step    = elapsed / self.grid_usec;
        let jitter  = elapsed % self.grid_usec;
        (step, jitter)
    }

    fn tick (&mut self) {
        let (step, jitter) = self.get_step();
        match self.last_step {
            None => self.step(step, jitter),
            Some(last_step) => if step > last_step {
                self.step(step, jitter)
            }
        }
    }

    fn step (&mut self, step: u128, jitter: u128) {
        self.last_step = Some(step);
        let main_sequence = self.document.sequences.get("<main>").unwrap();
        print!("{}+{}us", &step, &jitter);
        match main_sequence.get(step) {
            None => {},
            Some(s) => {
                for event in s {
                    print!("{:#?}", &event.name);
                }
            }
        }
        println!("");
    }

}
