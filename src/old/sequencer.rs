

#[derive(Debug)]
struct Player {
    grid: u128,
    patterns: HashMap<String, Pattern>,

    current_pattern: Option<String>,
    current_step: u128
}

impl Player {
    pub fn new () -> Player {
        Player {
            grid: 100,
            patterns: HashMap::new(),
            current_pattern: None,
            current_step: 0
        }
    }

    pub fn load (&mut self, reader: BufReader<File>) {
        for (_index, result) in reader.lines().enumerate() {
            let line = result.expect("wat");
            println!("{}", &line);
            match self.current_pattern {
                None => {
                    if line.starts_with("grid ") {
                        self.grid_set(&line);
                    }
                    else if line.starts_with("pattern ") {
                        self.pattern_start(&line);
                    }
                },
                Some(_) => {
                    self.pattern_append(&line);
                }
            }
        }
    }

    pub fn play (&mut self) {
        let t = Instant::now();
        loop {
            let e = t.elapsed().as_micros();
            let g = self.grid as u128 * 1000;
            let m = e % g;
            if m == 0 {
                let i = e / g;
                let frame = self.frame_get(e % (g * 8) / 1000);
                println!("{:?}", frame);
                println!("{} {}", i, t.elapsed().as_micros() - e);
            }
        }
    }

    fn frame_get (&self, t: u128) -> HashMap<String, String> {
        let pattern_name = self.current_pattern.as_ref().expect("wat");
        let pattern = self.patterns.get(&pattern_name.to_string()).expect("wat");
        let mut frame = HashMap::new();
        for (name, track) in pattern.tracks.iter() {
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

    fn pattern_start (&mut self, line: &str) {
        let headers: Vec<&str> = line.split(" ").collect();
        let name = headers[0];
        println!("start pattern: {} {:?}", &name, &headers[1..]);
        let mut tracks = IndexMap::new();
        for header in &headers[1..] {
            tracks.insert(header.to_string(), Track::new());
        }
        let pattern = Pattern { tracks };
        self.patterns.insert(name.to_string(), pattern);
        self.current_pattern = Some(name.to_string())
    }

    fn pattern_append (&mut self, line: &str) {
        if !line.starts_with("        ") { panic!("wat") }
        let mut line = &line[8..];
        let step = self.step_get();
        let pattern_name = self.current_pattern.as_ref().expect("wat");
        let pattern = self.patterns.get_mut(&pattern_name.to_string()).expect("wat");
        println!("line {} {}", &line, &line.len());
        for (name, track) in pattern.tracks.iter_mut() {
            let len = min(name.len(), line.len()-1);
            let value = &line[..len].trim();
            println!("{} {}", name, value);
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

#[derive(Debug)]
struct Pattern {
    tracks: IndexMap<String, Track>
}

#[derive(Debug)]
struct Track {
    events: BTreeMap<u128, String>
}

impl Track {
    pub fn new () -> Track {
        Track { events: BTreeMap::new() }
    }
}
