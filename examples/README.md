* Language for describing a sequence of events.
* Three stages: parse, evaluate, render.
* A document is parsed by PEG.
* A parsed document is evaluated statement by statement.
* An evaluated document can be rendered to file or realtime output.
* An evaluated document with its parse metadata can be passed to a GUI editor.

* A document may contain zero or more statements.
* A document has a temporal length.
* A document has a playhead, starting at 0.
* Events of non-zero length advance the playhead.

* A statement can be (1) a comment.
* A statement can end with a comment.
* Comments are ignored by evaluation, and thus always have zero length.

* A statement can be (2) a jump command, denoted by `@`.
* A jump command moves the playhead to an absolute or relative temporal location.

* A statement can be (3) an event. Events may have non-zero length.
* Evaluating each event advances the parent document's playhead by the length of that event.
* Thus, top-level events are written to the timeline end-to-end.
* The document's length is equal to the end time of the last event.

* A statement can be (4) multiple simultaneous events, joined by `&`.
* The length of such a statement is equal to the length of the longest event

* A statement can be (5) an event grid.
* Events in an event grid are spaced at a regular interval instead of end-to-end.
* Events in an event grid can be marked with offsets to play between the gridlines.

* A statement can be (6) an assignment.
* An assignment binds an alias to a value.
* A value can be a number, a string, or a statement.
* After an assignment, writing the alias is equivalent to writing its value.
* Assignments never move the playhead.
* Assignments in super-documents have precedence over assignments in sub-documents.
* Assigning a sub-document to an alias allows you to refer to the assignments
  contained in the sub-document using the `.` operator.

* A statement can be (7) a sub-document.
* A sub-document can be written in place, between `()`.
* A sub-document can be a path to a file.

* The renderer may handle magic variables in a special way.
* For audio, magic variables are samplerate! and bpm!
* For audio, the time unit is the length of a bar, subdivided.
* Event grids are defined in terms of the time unit.
