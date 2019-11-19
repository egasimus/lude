In a perfect world...
* samplers would be 1 process per voice or 1 process per instrument,
  triggered via osc over udp or, even better, cosc over jack
* the sequencer would be separate, synced via jack transport which does not
  seem to be supported by rust-jack atm
* some hands-on access/visibility via fuse
* if ipc becomes too heavy, merge everything into one address space
* or, migrate to a simpler OS ;)
* but for now let's overengineer it into an integrated sampler/sequencer
* even though this exists: https://github.com/RustAudio/sampler
* it's gonna be a nice exercise and the cuepoint juggling functionality
  can be merged into an existing project
* what's unique about this approach: the semantic overlay (timeline + tracks)
  that applies equally to a single sample, a sequence of samples, or a whole
  composition (a sequence of sequences) and is described through a DSL
