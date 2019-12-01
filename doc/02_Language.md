# Language

## Overview
A **document** is a text string describing a temporal sequence of events.

It is **parsed** by [Pest](https://pest.rs) into **statements**
which are **evaluated** into a full, unambiguous description of
what is to be **rendered** to an **output**.

* `TODO` Render the results of the parse and evaluate stages in a GUI.
See [Iced](https://github.com/hecrj/iced).

## Statements
A document contains zero or more **statements**, separated by **whitespace**.

* `TODO` Make whitespace less significant.

### Comments
* `TODO` A statement can be a **comment**, which is ignored.
* `TODO` A comment is delimited by a start comment token
         and either an end comment token or a newline.

### Time
During evaluation, a **cursor** points to the current **time**.
Time is measured in **frames**, represented by an **unsigned integer**.
Frames correspond to the **output sample rate** (currently hardcoded 44100 Hz)

* `TODO` Measure time in [flicks](https://en.wikipedia.org/wiki/Flick_(time)).
* `TODO` Allow custom units of time to be defined.
* `TODO` Allow output sample rate to be set.

Stating one of the following commands moves the cursor:

* The **jump** command (`@NUMBER`) sets the cursor to `NUMBER`.
* The **skip** command (`@+NUMBER`) moves the cursor forward by `NUMBER`.
* The **back** command (`@-NUMBER`) mobes the cursor back by `NUMBER`.
* `TODO` Remove `@` prefix from skip and back.
* `TODO` The **mark** command gives a name to the current value of the cursor.

A document has a temporal **length**, equal to
the start point of the last event
plus the length of the last event.

### Source
Stating a **path** to a **source** makes that source **active**.

### Slice
Stating a **slice** writes a portion of the active source
to the current position of the cursor and advances the cursor
by the duration of the slice.

Following a slice with `&` prevents the cursor from advancing,
allowing multiple slices to be played simultaneously.

The following slices are available:

* The `[]` slice writes the full source.
* The `[START:]` or `[START...]` slice writes the part of the source
  from `START` frames into the source up to the end of the source.
* The `[:END]` or `[...END]` slice writes the part of the source
  from the start of the source up to `END` frames into the source.
* The `[START:END]` slice writes the part of the source
  between `START` and `END`.
* `TODO` `[START...]`, `[...END]`, `[` vs `(` (inclusive vs. non-inclusive)
* `TODO` Change `[START:END]` to `[START:LENGTH]` (length pos/neg)
         and use `..` for absolute frame positions.

### Name
Assignment is of the form `NAME = (CONTENT)`.
Afterwards, writing `NAME` is equivalent to writing `CONTENT`.
