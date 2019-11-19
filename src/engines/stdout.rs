use std::io::stdout;
use crate::Document;

pub fn start_stdout_engine (mut document: Document) {
    let output = stdout();
    let frame_size = 1024;
    let sample_index = 0;
    loop {
        let (frame, stop) = document.get_frame(sample_index, frame_size);
        output.write_all(frame);
        output.flush();
        if stop break;
    }
}
