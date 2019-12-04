# Language

**Lude** (a.k.a. **ludelang**) is a language for painting with **time**.

## Example

**TODO**

## Renderer arguments

* Input file
* **TODO** Source paths (merged)
* **TODO** Output format and destination path

## Realtime control

**TODO**

## Language overview
A **document** is a text string describing a temporal sequence of events.

It is **parsed** by [Pest](https://pest.rs) into **statements**
which are **evaluated** into a full, unambiguous description of
the **slices** that are to be **rendered** to an **output**.

* **TODO** Render the results of the parse and evaluate stages in a GUI,
which allows the source file to be manipulated in a simple, semantics-aware
manner. See [Iced](https://github.com/hecrj/iced).

A document contains zero or more **statements**, separated by **whitespace**.

A document has a temporal **length**, equal to
the start point of the last slice
plus the length of the last slice.

* **TODO** Make whitespace less significant.

### Writing comments.
Things between `(` and `)` are ignored.
You can use this to describe things for humans.

### Time
During evaluation, a **cursor** points to the current **time**.
Time is measured in **frames**, represented by an **unsigned integer**.
Frames correspond to the **output sample rate** (currently hardcoded
at 44100 Hz)

* **TODO** Measure time in [flicks](https://en.wikipedia.org/wiki/Flick_(time)).
* **TODO** Index time from 1 instead of 0
* **TODO** Allow custom units of time to be defined.
* **TODO** Allow output sample rate to be set.

Stating one of the following commands moves the cursor:

* The **jump** command (`@NUMBER`) sets the cursor to `NUMBER`.
* The **skip** command (`@+NUMBER`) moves the cursor forward by `NUMBER`.
* The **back** command (`@-NUMBER`) mobes the cursor back by `NUMBER`.
* The **sync** command gives a **name** to the current value of the cursor,
  so that you can reference a point in time by a name rather than a number.
  It is equivalent ot an alias (see below)
* **TODO** use `/` and `*` for speeding up/slowing down
* **TODO** add something for repetition

### Source
Stating a **path** to a **source** makes that source **active**.

* **TODO** Paths are evaluated relative to the location of the source file.

### Slice
To **write** a portion of the active source, use a **slice**.
This puts some samples at the cursor and advances the cursor
by the number of samples.

**TODO** Following a slice with `&` prevents the cursor from advancing.
This allows multiple slices to be triggered simultaneously.

The following slices are available:

* The `||` slice writes the full source.
  from `x` frames into the source up to the end of the source.
* The `|x:|` slice writes the part of the source
  from `x` frames into the source up to the end of the source.
* The `|:y| slice writes the part of the source
  from the start of the source up to `y` frames into the source.
* The `|x:y|` slice writes the part of the source
  between `x` and `y`.
* The `|x+n|` slice writes the part of the source
  between `x` and `x+n`.
* The `|x-n|` slice writes the part of the source
  between `x` and `x-n`.
* **TODO** wrap slices around
* **TODO** source cursor for `|+n|` / `|-n|` (or are those jump/skip?)
* **TODO** The `|x|` slice writes one frame of the source.
* **TODO** The `|x,y,...| slice writes individual frames.
* **TODO** The `|x_n|` slice writes the `x`th frame `n` times

### Name
Assignment is of the form `NAME = [CONTENT]`.
Afterwards, writing `NAME` is equivalent to writing `CONTENT`.

### Command
**TODO** use `!` for commands to the renderer (such as setting sample rate,
mixing algorithm, etc)

### Alter
**TODO**

