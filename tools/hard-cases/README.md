# Hard cases: from reading failures to learning

Mova's animals and the Octopus, the large language model that develops Mova, are described in [the sea animals of Mova](../../docs/animals.md).

`mova-hard-cases` closes the loop between what Mova reads badly and what it learns. The corpus readers keep the
sentences they read badly on [one shared list](../hard-case-list/README.md); this tool sends them to the 🐟 Fish in
a priority lane, harvests the validated annotations, prepares them for the ⚡ Eel, re-checks the list with every new
gparse model and measures the before and after on a fixed sample. No step calls a teacher or trains by itself: the
Fish pagers and the Eel do, under their own controls and gates.

The Wikipedia corpus tool `wikicorpus` named below is internal and not in the public tree: in a public checkout
the Wikipedia source of the list stays empty, and the topic corpus (`tools/topic-corpus`) is the public source.

    cargo build --release --manifest-path tools/hard-cases/Cargo.toml --offline
    cargo test --manifest-path tools/hard-cases/Cargo.toml --offline

```text
 wikicorpus check (hourly) ──┐                          ┌── recheck (gparse view) ◄── new gparse weights
 topic corpora (Whales) ─────┴─► hard-cases list ───────┤
                                       │                └── measure (fixed sample, before/after)
                                       ▼
                      wave: Fish tasks in context (priority lane)
                                       │  Fish pagers (review-only)
                                       ▼
                      harvest: learn/<track>/<split>.conllu
                                       │  witness: Stanza, Trankit, spaCy trf
                                       ▼
                      eel: one Eel round per track (tree agreement, dev gates, guarded install)
```

## Commands

Every command takes one configuration (`mova-hard-cases-v1`, absolute paths, unknown fields refused).

| command | does |
|---|---|
| `list CONFIG` | the list by source, licence track, `en` and `gparse` view, lane kinds |
| `wave CONFIG [--dry]` | the next lane wave: tasks, pages, ready manifests, pager configurations, ledger |
| `lane CONFIG` | the waves and their pagers' progress (status snapshots, not liveness) |
| `guard CONFIG` | the lane's priority (generic pagers yield while lane pagers work) and its share of the limit guard |
| `harvest CONFIG` | validated Fish replies → `learn/<track>/<split>.conllu` and `-prag.tsv` |
| `witness CONFIG TRACK` | Stanza, Trankit, spaCy trf on the track's training trees, under the GPU lease |
| `eel CONFIG TRACK` | one Eel round on the track's data, when ready |
| `sample CONFIG [NAME]` | the fixed measurement sample (built once, never changed) |
| `measure CONFIG MODEL_DIR [LABEL] [NAME]` | pass rate, silver LAS, EWT LAS of a gparse model on the sample |
| `recheck CONFIG [--force]` | the list re-judged with the deployed gparse |
| `watch CONFIG [SECONDS] [NAME]` | measure and recheck whenever the deployed gparse weights change; `STOP` in the lane root stops it |

A deployment keeps its configuration outside this repository.

## What a case may become (`policy`)

**Track, by the licence of the text** (`licence/DATA.md`):

| licence class | track | meaning |
|---|---|---|
| CC BY, CC0, public domain, ours | `shipped` | may train the model that ships |
| CC BY-SA (Wikipedia) | `internal` | trains only a separate internal model, installed in its own directory, never shipped |
| NC, ND, research-only, no-reuse, unknown | `measure` | never trained and, by default, not sent to the teacher |

Why CC BY-SA is internal: the shipped `en.bin` and `gparse` are already CC BY-SA 4.0, because they learn from UD English
EWT, so a model trained on Wikipedia would not change their licence. But the data rules say CC BY-SA «as little as
possible», never carried or modified into a release, so Wikipedia's hard cases train a **separate internal
track** by default; the track is one line of the configuration (`policy.tracks."cc-by-sa"`). The
internal track's annotations and model stay internal, like everything derived from the Wikipedia corpus.

**Split, by document** (no document is in two splits): `train` 70 %, `dev` 15 % (the Eel's development gate for
the track), `measure` 15 % (the fixed before/after sample's first choice, never used to select a checkpoint).

**Kinds.** The lane takes cases flagged by syntax judges (`parse`, `tree`, `frames`, `grammar`, `nonprojective`,
`witness`, `roots`, `head_outside`, `no_words`, `no_frames`). `no_skeleton` (what the Whales' reader could not
restate: mostly table cells and headings) stays on the list for the readers but is not annotated. Text that is markup
the reader let through (wiki tables, templates, links, tags) is left out too: that is a reader bug to fix, not a
sentence to teach.

## The lane (`wave`)

A wave selects the cases open for gparse that the policy admits, not already in the ledger, not held out (exact or
whitespace-normalized text against the pinned held-out sets), one per normalized text, hardest first (most judges,
then lowest witness agreement), up to `fish.max_cases`. Each document is fetched once as a bundle
(`mova-hard-bundle-v1`, below) and pinned in the wave. Each target becomes a single-target Fish task:

- context: up to `fish.context` (8) preceding sentences of its document, then the target; ids are
  `hard:<doc16>:<n>`, with `doc16` the first 16 hex of SHA-256(source, NUL, document id) (Fish ids admit only ASCII);
- two candidate producers: `mova-en` (the source's own tree) and `mova-gparse` (the deployed gparse on the same
  tokens, MISC kept), both with the pragmatics SLM's labels;
- no open pragmatic witness, declared absent (the Fish record `missing_witness`);
- `dataset_use`: `teaching` for the train and dev splits, `unknown` for the measure split.

The public packer (`mova-fish-batch`) packs them page by page under the 100 000-byte request bound, up to
`fish.max_targets` targets per task, never across documents. Every target of a page is queued, deferred or
rejected, with its reason (a registry violation in a candidate tree is a rejection). The wave writes intents, task
queues and ready manifests exactly as `mova-fish-pages` consumes them, deals the pages round-robin to
`fish.pagers` plans, writes one pager configuration each, `selection.jsonl`, `wave.json` last, then appends every
selected case to the ledger (`queued.jsonl`): a case is sent once. An incomplete wave (no `wave.json`) stops the next
one until inspected.

**Priority.** The lane's pagers are ordinary `mova-fish-pages` processes with their configured profile. `guard` gives them
priority: while a lane pager works (fresh heartbeat, non-terminal state), the generic pagers named in
`control.yield_roots` hold an owned `PAUSE` (they stop at their next job boundary), so the hard cases take those
places first and the pool keeps its size. When no lane pager works, only the exact owned bytes are removed; an
operator's `PAUSE` is never touched. A lane pager that turns `uncertain` stops every lane pager and every
`control.stop_roots` root; a generic root stopped by its own limit guard stops the lane.

## Bundles

A source's command prints one document as `mova-hard-bundle-v1`: `schema`, `source`, `document`, `title`,
`version`, `licence`, `model` and `sentences` in order, each `{id, n, text, conllu, prag}`. `prag` is the
pragmatics SLM's row (header ending in `conf`) or empty; empty rows are labelled by the lane with the SLM trained on
the configured silver run, the whole document in order. The Wikipedia corpus: `wikicorpus bundle <title>`; the topic
corpora: `mova-topic-corpus bundle CONFIG CORPUS DOC`. The case's `where` fields named by `sources.<name>.pointer`
are the command's arguments.

## Harvest, witnesses, the Eel

`harvest` reads every lane pager's jobs; a reply counts when the pager's receipt and the Fish executor's receipt both
say `validated_review_only`, the reply binds the job's task and each target tree keeps the target's id and text
(first validated reply per target; held-out texts dropped again). It writes `learn/annotations.jsonl` and, per track
and split, `learn/<track>/<split>.conllu` (a `# hard_case = <key> <source> <wave>` line after the text) and
`<split>-prag.tsv`. These are review-only silver: the teacher's agreement is not gold.

`witness` runs the existing neural runner (`neural-offline.py`) for Stanza, Trankit and spaCy trf on the track's
training trees under the shared GPU lease, attaches `# text` only where id and every token form agree, and writes
`agreed-input.conllu` (the fully witnessed records) and `parsers/out/*.conllu`; the others are listed as missing. It
is bound to the training file's hash.

`eel` runs one round of the public Eel engine (`mova_eel::trainer::run`, once) on the track's data: the engine's own
tree agreement with the three parsers, held-out exclusions, the snapshot development set (the track's `dev` split: a
candidate may not lose LAS on hard cases it never trained on), the EWT LAS gate and absolute floor, the reload check
and the guarded installation. The track's engine configuration (`eel.<track>`) sets
`fresh_train_prefixes: ["hard:"]` (the engine admits only `school:` ids otherwise) and its own run root; the
`internal` track must name a model directory other than the deployed one. A round is ready only when train, dev and
current witnesses exist; otherwise it reports why and does nothing.

## Recheck and measurement (`judge`)

The judge of one tree, without gold: `roots`, `head_outside`, `grammar` (`world::induce::violations`),
`nonprojective` (never for gparse, whose decoder is projective) and `witness` (Stanza on the same tokens attaches
fewer than 80 % of the non-punctuation words to the same heads). It measures disagreement and broken constraints,
not correctness.

`recheck` judges every case open for gparse (and every case it learned, so regressions show) that has a draft tree,
with the deployed gparse: read well → `learned` for gparse, a learned case failing again → `open`. Cases without
tokens (the topic corpora's) stay as they are. It runs once per model (`recheck.json`). Run it before a wave: cases
gparse already reads well are not sent to the teacher.

`sample` builds the fixed sample once (`measure/<name>.jsonl`): cases with a draft tree, the `measure` split first,
then by key; Stanza's heads stored. `measure` runs gparse (CPU) on it and reports the pass rate, the drafts' own rate
(0 by construction: a red control), UAS/LAS against the harvested Fish trees of sample cases (the never-trained
`measure` split apart) and LAS on the pinned EWT dev; one file per run in `measure/runs/`, one line in
`measure/log.tsv`. `watch` does both whenever the deployed weights change, so every Eel round gets an after.

## Measured (2026-11-04)

On a copy of the list after `wikicorpus hard-sync` (2 068 cases: 1 937 topic-corpus cases, all `no_skeleton` and
without licence, and 131 Wikipedia cases from 8 articles), branch `hard-loop`:

- **Recheck with the deployed gparse:** 66 of the 131 Wikipedia cases read well by gparse (learned for gparse), 65
  stay open. The `en` view is unchanged.
- **Wave** (after the recheck): 61 cases selected (51 train, 8 dev, 2 measure; all `internal`), 4 left out as markup,
  1 937 outside the lane's kinds. A wave over all 131 (before the recheck) took 4 s: 125 queued in 4 pages, 2
  rejected (UD registry violations in a candidate tree); the largest prepared request was 37 263 bytes for three
  targets. The real `fish-pages` binary accepted the plan, intents and ready manifests (checked with a failing fake
  Fish: it reserved and launched the first job, then stopped uncertain as it must).
- **Fixed sample** (131 cases, Stanza heads for all, 27 s): pass rate of gparse models, CPU, two runs identical:

| model | what | hard-case pass | EWT dev LAS |
|---|---|---|---|
| `gparse:4924991fef908b4d` | frozen copy of 2026-11-01 | 68 / 131 (51.9 %) | 85.46 |
| `gparse:bdcbb8637c347e62` | before Eel round 2683 | 66 / 131 (50.4 %) | 85.41 |
| `gparse:6e9564693e9307b6` | after round 2683 and one more | 66 / 131 (50.4 %) | 85.40 |
| `gparse:458cffe10655ec46` | after round 2699 (deployed) | 66 / 131 (50.4 %) | 85.40 |

The three ordinary rounds since round 2683 left every per-case verdict identical: ordinary Eel rounds on school
data do not reach these sentences. The drafts (`en.bin`) pass 0 of 131 (red by construction). The silver LAS column
fills once harvested annotations exist. These are first numbers on 131 sentences from 8 articles; the split by
document is coarse at this size.

- **Witness step:** three training trees, all three parsers aligned, 38 s under the GPU lease; `treeagree agree`
  accepted the output.

## Limits

- The judge measures disagreement with Stanza and broken constraints, not correctness; a case can be "learned" by
  agreeing with Stanza's error.
- Topic-corpus cases have no draft tree in the list, so `recheck` and `sample` skip them; they are annotated from their
  bundles only.
- The `en` view and the `gparse` view are separate: a case gparse learned stays open for the corpora until `en.bin`
  reads it well.
- A wave's cases are sent once; a deferred or rejected case returns only by an explicit new decision (the ledger keeps
  it).
- The two candidate producers are correlated (same tokens, same labels); the Fish see that.
- `guard` is cooperative: a generic pager finishes its accepted attempt before pausing, and a stale status snapshot
  does not prove a pager is dead.
