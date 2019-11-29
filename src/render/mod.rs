use super::eval::Document;

pub mod sound;

pub fn render (doc: &Document, start: u128, end: u128) -> Vec<Vec<f32>> {
    let mut channels = Vec::new();
    for frame in start..(end+1) {
        eprintln!("rendering frame {}", &frame);
        channels.push(doc.get_frame(frame));
    }
    channels
}
