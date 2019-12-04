pub type FrameTime = usize;
pub type Sample = i16;
pub type Frame = Vec<Sample>;
pub type Wave = Vec<Sample>;
pub type Chunk = Vec<Option<Frame>>;

pub enum SliceType {
    Full,
    Abs,
    Fwd,
    Rew
}
