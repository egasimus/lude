/// A point in time.
pub type FrameTime = usize;

/// An amplitude value.
pub type Sample = i16;

/// One or more simultaneous samples.
pub type Frame = Vec<Sample>;

/// One or more subsequent samples.
pub type Wave = Vec<Sample>;

/// One or more subsequent frames or silences.
pub type Chunk = Vec<Option<Frame>>;

/// A directive to **write** some **samples** from a **source**
/// at the current **cursor** position.
///
/// **TODO** Following a slice with `&` prevents the cursor from advancing.
/// This allows multiple slices to be triggered simultaneously.
///
/// The following slices are available:
///
/// * The `||` slice writes the full source.
///   from `x` frames into the source up to the end of the source.
/// * The `|x:|` slice writes the part of the source
///   from `x` frames into the source up to the end of the source.
/// * The `|:y| slice writes the part of the source
///   from the start of the source up to `y` frames into the source.
/// * The `|x:y|` slice writes the part of the source
///   between `x` and `y`.
/// * The `|x+n|` slice writes the part of the source
///   between `x` and `x+n`.
/// * The `|x-n|` slice writes the part of the source
///   between `x` and `x-n`.
/// * **TODO** wrap slices around
/// * **TODO** source cursor for `|+n|` / `|-n|` (or are those jump/skip?)
/// * **TODO** The `|x|` slice writes one frame of the source.
/// * **TODO** The `|x,y,...| slice writes individual frames.
/// * **TODO** The `|x_n|` slice writes the `x`th frame `n` times
pub enum SliceType {
    Full,
    Abs,
    Fwd,
    Rew
}
