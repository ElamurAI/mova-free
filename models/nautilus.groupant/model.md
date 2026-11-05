# nautilus

## model

# nautilus

## Descriptant

Nautilus (mova-nautilus): the dictionary maker, one of Mova's animals. Native dictionary layers from courses and corpora, English through the native deputy, a bounded standing loop with review-only proposals, and its teacher through its own mova-terminal behind a narrow interface (no model call in the tests).

## Forward

sources -> annotated units -> layer::build -> render -> layer file; deputy prompt -> Hands(draft|stats) -> English; work: Job -> native or teacher::submit through a PrivateTerminal -> validate -> proposal -> journal.

## Invariant

Native work is deterministic; every teacher question needs the operator's word, is journaled before and after, and is never asked twice; nothing is admitted by the animal.

## Cost

Drafts O(tokens); one question per loop step at most.

## Composition

Leaves are file groupants under `tools/nautilus/src/*.rs.groupant` and `tools/nautilus/tests/*.rs.groupant`. Correct leaves do not prove the wiring: the interaction checks are the English tests, which run the deputy, the handler and the real work together.

## behavior_complement

No model calls in its tests; no admission; no slots; drafts only for configured layers; the Wikipedia layer stays outside the repository.

## reverse_reconstruction

Layer files, the deputy ledger, the teacher journal and work.jsonl recover what was made, asked, answered, proposed and rejected; not unchosen units or question texts. Evidence status: source-derived with finite checks.

## tests

### Checks

No teacher or model call; the private terminal is a fake in the tests.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (pool)

Two children joined: `mova-nautilus-improve` (the `layer` source: weak rows of an existing layer asked in tens,
proposals in the builder's formats, the layer file read only) and `mova-nautilus-pool` (a Crab pool member: the lease
read between steps, a job's shared ledger, report and backlog files). Interaction: animal::Config::job builds a
LayerJob or a DictJob for part of parts; work::Loop with a ledger makes a job movable between members without
repeated questions; the counterpart is `crab-pool`, which writes the leases and reads the reports and backlogs.
Licence classes are enforced by the configuration (shipped layers never take CC BY-SA, NC or research-only inputs)
and by the data (a CC BY-SA header in a lesson or layer file). Finite checks only; red controls listed in the
children's tests.

#### Epoch 2026-11-04 (senses)

Two children joined: `mova-nautilus-senses` (lemma matching of words in examples, junk and boilerplate, the pupils'
numbered senses, native picks by lemma overlap, restatements, absent answers, the shared sense store) and
`mova-nautilus-replay` (the loop measured on copies with earlier real answers). Interaction: DictJob and LayerJob plan
every word through senses::plan; the loop writes Mova's picks next to the teacher's proposal and both feed the store
(Job::learn), which every later layer and instance reads (a fixed sense is asked only as `same`); the configuration
carries the store, the batch and the pacing. Measured 2026-11-04 by replay on a snapshot of 129 questions: 47
questions instead of 129, 23.4 words per question instead of 9.9, 578 question bytes per word instead of 771, 126 words
settled natively, 107 of 1 030 teacher senses restated a numbered sense. Prospective: on live runs about one word in
ten settled natively and about one teacher sense in ten marked as a restatement; to be corrected from the first
live day.

### 2026-11-04 · tests

cargo test --manifest-path tools/nautilus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (15 unit + 2 English tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): no operator-word check; no terminal-id check; a limit not stopping the loop; stats drafting instead of reading.

### 2026-11-05 · document

#### Epoch 2026-11-05 (sampled Wikipedia)

A child joined: `mova-nautilus-sampling` (the `wikipedia-examples` source). Interaction: animal::make orders tier 1's
words and calls the private `wikicorpus` binary (`examples-batch`) as a separate process, so the public crate does
not depend on private code; its rows enter work::DictJob like any draft, with `native_picks: false` putting the layer
in `ask_all`. The licence boundary stays in animal::LayerConfig::check: CC BY-SA forced, never shipped, and never
written inside a Git work tree. `nautilus dry` measures a job's items with nothing sent or written.
