use std::time::Instant;
use std::cell::RefCell;
use crate::document::Document;
use crate::types::{FrameTime, Frame, Chunk, Wave};

/// Generates and returns the `Chunk` of `doc`
/// that is between `begin` and `end`.
/// ```
/// let doc = Document::new()
/// assert!(render(doc, 0, 999).len(), 1000)
/// ```
/// Some old musings cc rendering:
/// In a perfect world...
/// * samplers would be 1 process per voice or 1 process per instrument,
///   triggered via osc over udp or, even better, cosc over jack
/// * the sequencer would be separate, synced via jack transport which does not
///   seem to be supported by rust-jack atm
/// * some hands-on access/visibility via fuse
/// * if ipc becomes too heavy, merge everything into one address space
/// * or, migrate to a simpler OS ;)
/// * but for now let's overengineer it into an integrated sampler/sequencer
/// * even though this exists: https://github.com/RustAudio/sampler
/// * it's gonna be a nice exercise and the cuepoint juggling functionality
///   can be merged into an existing project
/// * what's unique about this approach: the semantic overlay (timeline + tracks)
///   that applies equally to a single sample, a sequence of samples, or a whole
///   composition (a sequence of sequences) and is described through a DSL

/// * The renderer may handle magic variables in a special way.
/// * For audio, magic variables are samplerate! and bpm!
/// * For audio, the time unit is the length of a bar, subdivided.
/// * Event grids are defined in terms of the time unit.
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

/// Converts a `Chunk` of optional multi-channel `Frame`s to an array of `Wave`s.
pub fn to_channels (chunk: Chunk) -> Vec<Wave> {
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
    let mut output = Vec::new();
    for channel in channels {
        output.push(channel.into_inner());
    }

    eprintln!("regrouped chunk of {} frames into {} channels in {} usec",
        &chunk.len(), &output.len(), start.elapsed().as_micros());

    output
}

/// Regroups an array of `Wave`s to an array of `Frame`s.
pub fn to_frames (channels: Vec<Wave>) -> Vec<Frame> {
    let start = Instant::now();

    let duration = channels.get(0).unwrap().len();
    let mut frames = Vec::with_capacity(duration);
    let channel_count = channels.len();
    for frame_index in 0..duration {
        let mut frame = Vec::with_capacity(channel_count);
        for channel in channels.iter() {
            frame.push(*channel.get(frame_index).unwrap())
        }
        frames.push(frame)
    }

    eprintln!("regrouped {} channels into {} frames in {} usec",
        &channels.len(), &frames.len(), start.elapsed().as_micros());

    frames
}
