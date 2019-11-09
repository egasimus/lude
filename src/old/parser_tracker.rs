

#[derive(Debug)]
struct Phrase {
    tracks: IndexMap<String, Track>,
    empty: bool
}

impl Phrase {

    pub fn new () -> Phrase {
        Phrase { tracks: IndexMap::new(), empty: true }
    }

    pub fn append (&mut self, line: &str) {
        if self.empty {
            let headers: Vec<&str> = line.split(" ").collect();
            let name = headers[0];
            for header in &headers[1..] {
                self.tracks.insert(header.to_string(), Track::new());
            }
        } else {
            if !line.starts_with("       ") { panic!("wat") }
            let mut line = &line[8..];
            let step = self.step_get();
            let phrase_name = self.current_phrase.as_ref().expect("wat");
            let phrase = self.phrases.get_mut(&phrase_name.to_string()).expect("wat");
            println!(">{} {}", &line, &line.len());
            for (name, track) in phrase.tracks.iter_mut() {
                let len = min(name.len(), line.len()-1);
                let value = &line[..len].trim();
                print!(" {}={}", name, value);
                track.events.insert(
                    step,
                    value.to_string().trim().to_string()
                );
                line = &line[len+1..];
            }
            self.current_step += 1;
        }
    }

}

fn load (reader: BufReader<File>, sequencer: &mut Sequencer) {
    for (_index, result) in reader.lines().enumerate() {
        let line = result.expect("wat");
        println!("{}", &line);
        match sequencer.current_phrase {
            None => {
                if line.starts_with("grid ") {
                    sequencer.grid_set(&line);
                }
                else if line.starts_with("phrase ") {
                    sequencer.phrase_start(&line);
                }
            },
            Some(_) => {
                sequencer.phrase_append(&line);
            }
        }
    }
}
#[derive(Debug)]
struct Sequencer {
    grid: u128,
    phrases: HashMap<String, Phrase>,

    pub current_phrase: Option<String>,
    current_step: u128
}

impl Sequencer {
    pub fn new () -> Sequencer {
        Sequencer {
            grid: 100,
            phrases: HashMap::new(),
            current_phrase: None,
            current_step: 0
        }
    }

    pub fn play (&mut self) {
        let playback_started = Instant::now();
        loop {
            let elapsed_usec = playback_started.elapsed().as_micros();
            let grid_usec = self.grid as u128 * 1000;
            let on_grid = elapsed_usec % grid_usec == 0;
            if on_grid {
                let i = elapsed_usec / grid_usec;
                let _frame = self.frame_get(elapsed_usec % (grid_usec * 8) / 1000);
                //println!("{:?}", frame);
                println!(
                    "{} {}",
                    i,
                    playback_started.elapsed().as_micros() - elapsed_usec
                );
            }
        }
    }

    fn frame_get (&self, t: u128) -> HashMap<String, String> {
        let phrase_name = self.current_phrase.as_ref().expect("wat");
        let phrase = self.phrases.get(&phrase_name.to_string()).expect("wat");
        let mut frame = HashMap::new();
        for (name, track) in phrase.tracks.iter() {
            frame.insert(
                name.to_string(),
                track.events.get(&t).expect("wat t").to_string()
            );
        }
        frame
    }

    fn grid_set (&mut self, line: &str) {
        let grid = &line[5..].parse().expect("wat");
        println!("set grid {}", &grid);
        self.grid = *grid;
    }

    fn phrase_start (&mut self, line: &str) {
        let headers: Vec<&str> = line.split(" ").collect();
        let name = headers[0];
        println!("start phrase: {} {:?}", &name, &headers[1..]);
        let mut tracks = IndexMap::new();
        for header in &headers[1..] {
            tracks.insert(header.to_string(), Track::new());
        }
        let phrase = Phrase { tracks };
        self.phrases.insert(name.to_string(), phrase);
        self.current_phrase = Some(name.to_string())
    }

    fn phrase_append (&mut self, line: &str) {
        if !line.starts_with("        ") { panic!("wat") }
        let mut line = &line[8..];
        let step = self.step_get();
        let phrase_name = self.current_phrase.as_ref().expect("wat");
        let phrase = self.phrases.get_mut(&phrase_name.to_string()).expect("wat");
        println!(">{} {}", &line, &line.len());
        for (name, track) in phrase.tracks.iter_mut() {
            let len = min(name.len(), line.len()-1);
            let value = &line[..len].trim();
            print!(" {}={}", name, value);
            track.events.insert(
                step,
                value.to_string().trim().to_string()
            );
            line = &line[len+1..];
        }
        self.current_step += 1;
    }

    fn step_get (&mut self) -> u128 {
        self.current_step * self.grid
    }
}
