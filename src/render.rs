use std::time::Instant;
use std::cell::RefCell;
use crate::document::Document;
use crate::types::{FrameTime, Chunk, Wave, Output};

pub fn render (doc: &Document, begin: FrameTime, end: FrameTime) -> Chunk {
    let start = Instant::now();

    let mut frames = Vec::new();

    for index in begin..(end+1) {
        let frame = doc.get_frame(index);
        frames.push(frame);
    }

    eprintln!("rendered {}..{} in {}usec ",
        &begin, &end, start.elapsed().as_micros());

    frames
}

pub fn flatten (chunk: Chunk) -> Output {
    let start = Instant::now();

    let mut channels: Vec<RefCell<Wave>> = Vec::new();
    for (frame_index, frame) in chunk.iter().enumerate() {
        match frame {
            Some(frame) => {
                for (
                    channel_index,
                    channel_value
                ) in frame.iter().enumerate() {
                    match channels.get(channel_index) {
                        Some(channel) => {
                            let mut channel = channel.borrow_mut();
                            channel.push(*channel_value);
                        },
                        None => {
                            let mut new_channel =
                                Wave::with_capacity(chunk.len());
                            for _ in 0..frame_index {
                                new_channel.push(0);
                            }
                            new_channel.push(*channel_value);
                            channels.push(RefCell::new(new_channel));
                        }
                    }
                }
            },
            None => {
                for channel in channels.iter_mut() {
                    channel.borrow_mut().push(0);
                }
            },
        }
    }
    let mut output = Output::new();
    for channel in channels {
        output.push(channel.into_inner());
    }

    eprintln!("flattened {} frames in {}usec ",
        &chunk.len(), start.elapsed().as_micros());

    output
}
