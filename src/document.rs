use crate::types::{FrameTime, Frame};
use crate::media::SoundMap;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Event {
    src: String,
    slice_start: Option<FrameTime>,
    slice_end: Option<FrameTime>,
    duration: FrameTime,
}

#[derive(Debug)]
pub struct Document {
    media: SoundMap,
    pub length: FrameTime,
    pub events: BTreeMap<FrameTime, Vec<Event>>,
    longest: FrameTime,
}

impl Document {
    pub fn new () -> Document {
        Document {
            media: SoundMap::new(),
            length: 0,
            events: BTreeMap::new(),
            longest: 0
        }
    }
    pub fn add_event (
        &mut         self,
        at:          FrameTime,
        src:         &str,
        slice_start: Option<FrameTime>,
        slice_end:   Option<FrameTime>,
    ) -> FrameTime {
        // bump longest duration.
        // used for determining the range of frame times
        // that may contain events that matter in a frame
        //println!("{} {:?} {:?}", &src, &slice_start, &slice_end);
        let duration = match slice_end {
            Some(slice_end) => match slice_start {
                Some(slice_start) => slice_end - slice_start,
                None => slice_end
            },
            None => self.media.get_source_length(src)
        };
        if duration > self.longest {
            self.longest = duration
        }

        let event = Event {
            src: src.to_string(),
            slice_start,
            slice_end,
            duration
        };
        match self.events.get_mut(&at) {
            Some(events) => events.push(event),
            None => {
                let mut events = Vec::new();
                events.push(event);
                self.events.insert(at, events);
            }
        }

        duration
    }
    pub fn bounds (&self) -> (FrameTime, FrameTime, FrameTime) {
        let min = *self.events.keys().next().unwrap();
        let max = *self.events.keys().next_back().unwrap();
        let longest = self.longest;
        (min, max, longest)
    }
    pub fn get_frame (&self, frame_index: u128) -> Option<Frame> {
        // nothing if document is empty
        if self.events.len() == 0 { return None }

        // determine real bounds of document
        let (min, max, longest) = self.bounds();
        let end = max + longest;

        // nothing before the beginning or after the end
        if frame_index < min || frame_index > end { return None }
        // nothing if document is empty

        // maybe something in the middle?
        let mut event_frames = Vec::new();
        let start = if frame_index < longest { 0 } else { frame_index - longest };
        let event_range = self.events.range(start..frame_index+1);
        for (event_start, events) in event_range {
            let event_frame_index = frame_index - event_start;
            for event in events {
                let offset = match event.slice_start {
                    None => 0,
                    Some(offset) => offset
                };
                let index = (event_frame_index + offset) as i64;
                match event.slice_end {
                    Some(slice_end) => if index as u128 > slice_end { continue },
                    _=>{}
                }
                //println!("{}={:?}[{}]", &frame_index, &event.src, &index);
                match self.media.get_frame(&event.src, index) {
                    Some(frame) => event_frames.push(frame),
                    _ => {}
                }
            }
        }
        sum_subframes(event_frames)
    }
}

fn sum_subframes (event_frames: Vec<Frame>) -> Option<Frame> {
    let mut frame: Frame = Vec::new();
    for event_frame in event_frames.iter() {
        for (i, value) in event_frame.iter().enumerate() {
            match frame.get_mut(i) {
                Some(&mut current) => {frame[i] = current.saturating_add(*value);},
                None => frame.push(*value)
            }
        }
    }
    if frame.len() > 0 {
        Some(frame)
    } else {
        None
    }
}
