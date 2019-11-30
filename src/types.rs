pub type FrameTime = u128;
pub type Sample = i16;
pub type Frame = Vec<Sample>;
pub type Wave = Vec<Sample>;
pub type Chunk = Vec<Option<Frame>>;
pub type FlatChunk = Vec<Frame>;
pub type Output = Vec<Wave>;
