# mova-whale-epoch-tests

## Descriptant

Tests of the epoch driver: a fixture paper (learn::tests::PAPER), one fixture neighbour, a pinned fixture prompt and a fake terminal (never a model); and the ignored measurement `measure_epoch` with a reading fake that answers from the files the prompt names. Source: `tools/whale/src/epoch/tests.rs`.

## Forward approximant

setup(tag, min) -> (temp dir, EpochConfig) with mova.words min..max(min, 2000)+1000 and structure_share_max 0.6 (a short fixture); each test runs epoch::run and asserts statuses, dossier rows, terminal questions and files. ReadingFake: per given source its sentences of >= 8 words, round-robin, each cited «[n]», up to `words`, then a reference list of ids.

## Invariant

No test reaches a model or a live terminal; every gate test has its control in the same test or in the negative-control list of tests.md.

## Cost

Under a second for all epoch tests with the model loaded; measure_epoch about a minute for five papers.

## Composition

Parent: mova-whale-epoch (child module `epoch::tests`). Uses learn::tests (model_ok, tmp, PAPER) and terminal::tests::profile.

## behavior_complement

Not tested here: the real MovaTerminal adapter (tests/terminal_adapter.rs covers it), the world CLI, the topic_corpus child step, the catalog document source. Evidence status: finite checks.

## reverse_reconstruction

Each test's temp directory keeps the run root and dossier it wrote until the next run of the same test.

## History

### 2026-11-05 · document

#### Epoch 2026-11-05 (created)

All 8 tests pass with the 0.4 model; see epoch.rs.groupant/tests.md for the negative controls.

#### Epoch 2026-11-05 (world tool)

`fake_world` (a shell script over a JSONL directory and fixed answers, never the real store) serves `dossier list|text|add` and `world neighbourhood|related`; `the_world_tool_gives_dossier_and_neighbourhood` checks the registered rows, Mova's citations of the neighbour and of the document's older description, and the staged inputs.
