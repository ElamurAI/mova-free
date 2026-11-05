# Training Mova

This page describes how Mova gets better: who builds it and who teaches it, the teacher transport, the training pipeline that turns raw text into agreed trees and a retrained parser, how reviewed annotation enters the parse database and how it is rolled back, how Mova improves its own rules (`world induce`, `world selfplay`, `world tune`), how changes are measured, and the gates that guard all of it. It is written for the next builder, human or AI. How school courses are written, run and diagnosed is in [the school](school.md).

## Who builds and who teaches

Mova is a small, deterministic program in Rust with a Combinatorial Small Language Model (SLM) at its core ([the hybrid system](hybrid-system.md)). It parses English into Universal Dependencies trees, reads the trees into frames and a world, and answers from that world with a reason. A large model in the project has one of two narrow roles:

- **the builder**: an AI working in a session with the human operator. It writes code, runs measurements, writes documents. It builds Mova; it is never Mova;
- **the teacher**: a large model called in batches through Codex CLI. It corrects parses and pragmatic labels, writes lessons and dictionary records, scores absurdity.

The teacher never answers at test time. Every score is Mova's own. Giving the uncertain questions to the teacher while the student answers is cheating and makes every number meaningless.

The working mode for an AI builder is [Gali](mysticism/02-gali.md): hold a position when you have evidence, separate confidence from certainty, prefer reality over elegance, keep answers proportionate. Read it before you start.

## Current teacher transport

Every teacher call goes through the shared Rust crate `mova-teacher` ([its README](../teacher/README.md)): **Codex CLI with ChatGPT authentication**. The default model is `gpt-6.1-terra` at medium effort. There is no Claude API route and no fallback to one.

Setup: install Codex CLI, run `codex login --device-auth`, check with `codex login status`.

A call runs in a fresh directory with a read-only sandbox, without API-key variables, user tool configuration or tool features; the prompt goes through stdin and the answer and token usage are decoded from JSONL events. A failed, incomplete or tool-using turn is rejected. Every answer still passes the task's own Rust gates.

| variable | meaning |
|---|---|
| `MOVA_CODEX_BIN` | the Codex executable (default `codex`) |
| `MOVA_TEACHER_MODEL` | overrides every component's model; otherwise the component's variable (`ANNOT_MODEL`, `CYCLE_MODEL`, `ENRICH_MODEL`, …), then the default |
| `MOVA_TEACHER_TIMEOUT_SECS` | per-call deadline (default 1800); a timeout kills the process group and is not retried |

Prepared batches freeze their model and effort. `codex-calls.jsonl` records provider, model, effort, time and tokens. Subscription dollar cost is not reported; the old numeric cost fields stay zero for compatibility and are not a measurement.

**Batches.** `annot batch-prepare` writes the request and state files. `annot batch-run` runs them through Codex; `ANNOT_NO_WAIT=1` starts or polls a local worker. Partial replies stay in `codex-results.partial.jsonl`; only complete success publishes `batch-results.jsonl`. A snapshot prevents resuming against changed prompts. After a failure, fix the cause and clear `codex-error.txt` explicitly to resume; successful requests are not repeated. Inspect a stale worker lock before removing it. `annot direct-run` uses the same queue synchronously.

**Runs.** Use a fresh run directory. Legacy Claude requests and unfinished API batches are rejected; finished historical results stay readable. The cycle runs only with `CYCLE_TEACHER=codex` (the default) and `CYCLE_TIERS=0`; a cascade of several teacher models is not used. Give the pool and corpus paths explicitly for a new run.

**Fish annotation profiles.** The Fish workers ([the sea animals](animals.md#models)) prepare syntax and pragmatics tasks through an explicit CLI/terminal profile: Sonnet by default, Terra recommended, Sol and Haiku configurable. A rendered request, wrappers included, is at most 100000 UTF-8 bytes. Concrete deployments keep their overrides outside public source. Preparing a request or a GPU-admission candidate does not run a teacher and does not authorize training.

Checks: `cargo test -p mova-teacher`, `cargo test -p annot --test codex_batches`. The opt-in live test `cargo test -p mova-teacher --test live -- --ignored --nocapture` makes one real call. Large training runs need the operator's explicit word.

## The loop

```text
          the operator: a direction, in a sentence
                      │
                      ▼
   ┌───────────► a task (a brief, a switch, a control)
   │                  │
   │                  ▼
   │         data: corpora, lessons, the dictionary ◄── the teacher (batches, never at test time)
   │                  │
   │                  ▼
   │         Mova reads: tokens → tree → frames → world → answer
   │                  │                         │
   │                  ▼                         ▼
   │         the student's score          the autoencoder's residual
   │         (two halves, controls red)   (bits by layer and class)
   │                  │                         │
   │                  └────────────┬────────────┘
   │                               ▼
   │                  a diagnosis: where it fails, and why
   │                               │
   └──── kept only if both halves gain and the controls stay red
```

Two numbers drive everything: **the student's score** on the school courses ([the school](school.md)) and **the residual** of the round trip, the bits each layer must store to give the text back exactly ([the autoencoder](english.md)). A change that neither raises the score nor lowers the residual has not helped, however good it looks.

## Setting a task

A training task states four things:

1. **What changes.** One mechanism, named. Not «improve the student» but «take the anchors from the lesson just read first».
2. **A switch.** Every new step sits behind `PRAG_OFF=<name>` (a diagnostic that is not a step has its own `MOVA_…` variable), so the old behaviour is one environment variable away and every A/B test is the same binary run twice.
3. **The measure, decided before the run**: which courses, which halves, which baseline run, what counts as a gain.
4. **A control that can go red.** If the change cannot show a loss, the measurement cannot show a gain.

Contributor workflow (worktrees, briefs, reviews, what needs the operator's word) is in [AGENTS.md](../AGENTS.md).

## Growing a corpus

Mova learns from corpora: school lessons, tales, public-domain books, the Hebrew Bible and its translations, open treebanks, the site's own pages.

**Licences first.** Before a byte is downloaded, its licence decides what it may become, by [the data and model licence rules](../licence/DATA.md); the licence is recorded beside the data when it comes in. The saved-model gate accepts only registered training-source identifiers, and its acceptance does not prove that a source's licence metadata is right.

**Data lives outside the repo**, under `$MOVA_DATA` ([data](data.md)). A corpus is a folder with a manifest: source, licence, size, date. Open, tale, corporate and other corpora stay in separate folders and databases; data flows only from the open corpus to closed ones, never back.

How a corpus grows:

1. A source is requested (a book, a treebank, a set of articles).
2. A reader in Rust turns it into sentences with ids. The reader stays as a tool with a test, so the next source of the same kind costs nothing.
3. The sentences go through Mova's parsers and the open witness parsers.
4. `treeagree` agrees the trees by written rules; the UD gates keep them valid.
5. The sentences join a pool that the training pipeline takes from.

**Big inputs are streamed, never loaded.** A large compressed dump is read block by block with a bounded window. Heavy jobs run through `tools/jobs` with a memory limit and a time limit: the job dies, not the machine.

## Lessons and questions

School lessons and questions are written by teacher helpers on shared briefs and checked by the `world knowledge` gates (`check`, `lint`, `balance`); the right option is moved to a drawn position so its place gives nothing away. Only `explain:` and `fact:` lines go into the parsers' training pools; test and exam sentences never do, and lessons held out for the parser's own test are left out whole. The full method, format, tools, the student and its diagnosis are in [the school](school.md).

Use the `world knowledge` subcommands, not scripts, for school data. A need they do not cover becomes a new subcommand in Rust with a test.

## Building the dictionary

Mova has its own explanatory dictionary of English. A record has six tab-separated fields: the word, its part of speech, its meaning for a pupil, its sense in the lessons, examples from the lessons, and a note for the reader of a maths problem. It feeds the student (what a word means, what kind of thing it is), the lessons (every word a lesson uses must have a record) and the site. The builder that writes records is not part of the public source; its output and the runtime store are.

- **One gate for every record**, whoever wrote it: six fields, valid UTF-8, the word in lower case, and no date anywhere in a field (a date means a writer leaked its context). The gate refuses ISO and dotted day-month dates and lets decimals («3.14») and section numbers («3.12.05») through; any `NN.NN` with a real day and month («12.05») is a known false refusal.
- **Closed under reading.** A school's dictionary explains every word its lessons use, every word those explanations use, and so on: a worklist over records. Lesson words without a record are the **gaps**, the teacher's next batch; words records use without a record are listed by `dictionary missing`.
- **Derived words** are explained from their base where the base has a record («unkind» is «not kind (kind: …)»), by a short affix table, longer affixes first. False splits («remain» is not «again main») are a known limit; the explanation names the base.
- **Growing from new texts.** New words get a whole record; known words at most one new sense and one example per pass. The example must be a corpus sentence the teacher was shown, word for word, and the record must still pass the gate. Touched words are listed so their dictionary lessons can be rewritten.
- **Layers**: senses in Jewish tradition, in Early Modern English and in the project's mystic school; a word's page merges them. Hebrew and Aramaic dictionaries are built from the texts, verse by verse.
- **The audit** checks the dictionary against the language manual: a missing sense is a gap; a contradiction is a bug in one of them.

**The runtime store** (`world/src/dictstore.rs`, `world dict`) is what Mova loads in learning and in work: layers with provenance (layer, state, licence, scope), hash-pinned when they are files, in manifest order (`MOVA_DICT`; otherwise the built-in layers: the pupils' dictionary and `$MOVA_DATA/school/dictionaries/commands.tsv`). A CC BY-SA layer such as the Wikipedia dictionary never ships: a manifest may load it from the data directory for internal runs (`scope: internal`, `MOVA_DICT_USE=internal`), a shipped use refuses it, and a built-in CC BY-SA text fails `shippable` (test `cc_by_sa_never_ships`). Only admitted layers are knowledge; drafts count only with `allow_unreviewed`; a teacher's proposal becomes a layer only through `world dict admit PROPOSAL REVIEW OUT` with a complete review record (the proposal's SHA-256, a reviewer, a date, every word accepted or rejected). A retained school state records the store's fingerprint and is examined only with the same store.

    world dict stats [MANIFEST]   world dict lookup WORD…   world dict check MANIFEST [--release]   world dict hash FILE

**The Wiktionary module in the pupil** (CPU, 2026-11-04). The add-on module `wiktionary-en` ([data.md](data.md),
«Wiktionary»; [modules.md](modules.md)) loads as a draft layer after the pupils' dictionary (a `MOVA_DICT` manifest
with `allow_unreviewed` and a `module:wiktionary-en@20260901-1/wiktionary-en` layer). Two readings were measured on
the gates of the dictionary-kind experiment: the ten courses (commands and nine primary), the twelve other primary
courses (everyday, euphemisms, religion-ethics, space, physics, chemistry, biology, myths, ai, money, arts, medicine)
and all four controls.

- **nyms** (`PRAG_VN=student-wikt`): for a question's thing the lessons never named, the synonyms of its Wiktionary
  noun entries and the lemmas they are a form of («brekkie» → breakfast), kept when the lessons name them;
  `student-wikt-lack` only for nouns WordNet lacks; `student-wikt-mutual` only a synonym whose own entries name the
  thing back. Prediction: slang and regional nouns of the scenario questions reach the lessons' words through fewer
  generic heads than the genus reading below.
- **genus** (`PRAG_VN=student-dict`, unchanged): with the module loaded, the first meaning of a noun the pupils'
  dictionary lacks comes from Wiktionary.

| variant | ten courses (7 283) | paired | exam (2 556) | controls: no lesson / shuffled / keys removed / another lesson |
|---|---:|---:|---:|---|
| base | 3 259 | | 1 000 | 1 883 / 2 375 / 2 309 / 2 175 |
| module loaded, switches off | 3 259 | rows identical | 1 000 | identical |
| nyms | 3 261 | +5 / −3 | 1 003 | 1 883 / 2 375 / 2 318 / 2 179 |
| nyms, nouns WordNet lacks | 3 259 | +1 / −1 | 1 000 | 1 883 / 2 375 / 2 309 / 2 176 |
| nyms, mutual only | 3 262 | +3 / −0 | 1 000 | 1 883 / 2 374 / 2 314 / 2 176 |
| genus, pupils' dictionary only | 3 273 | +17 / −3 | 1 002 | 1 883 / 2 372 / 2 312 / 2 181 |
| genus, pupils' then Wiktionary | 3 273 | +2 / −2 against the line above | 1 002 | 1 883 / 2 373 / 2 312 / 2 184 |

| variant | twelve courses (17 260) | paired | exam (5 450) | controls: no lesson / shuffled / keys removed / another lesson |
|---|---:|---:|---:|---|
| base | 8 723 | | 2 466 | 4 484 / 6 039 / 6 023 / 5 594 |
| module loaded, switches off | 8 723 | rows identical | 2 466 | identical |
| nyms | 8 733 | +15 / −5 (z 2.2) | 2 472 | 4 484 / 6 048 / 6 037 / 5 605 |
| nyms, nouns WordNet lacks | 8 724 | +1 / −0 | 2 466 | 4 484 / 6 042 / 6 022 / 5 593 |
| nyms, mutual only | 8 728 | +8 / −3 | 2 469 | 4 484 / 6 048 / 6 028 / 5 600 |
| genus, pupils' dictionary only | 8 743 | +43 / −23 | 2 468 | 4 484 / 6 065 / 6 034 / 5 636 |
| genus, pupils' then Wiktionary | 8 747 | +13 / −9 against the line above | 2 462 | 4 484 / 6 067 / 6 047 / 5 641 |

The nyms reading does what it was built for: «Dad, laughing on the phone: "Gaz! Twenty years, mate…"» and «Mum's mate
goes…» now read «mate» as the lessons' friend. But on both sets the shuffled, keys-removed and cross-lesson controls
rise about as much as the pupil (twelve courses: +10 against +9, +14 and +11): a synonym list is as generic as a genus
(«friend», «set»), so a control's question finds support through it as readily as the real one. Restricted to nouns
WordNet lacks it has almost nothing to act on; restricted to mutual synonyms it is smaller and the keys-removed control
still rises. Wiktionary's first meanings for the nouns the pupils' dictionary lacks move a few decisions each way and
the controls with them. **Nothing is kept on**: the three switches stay opt-in, and off (with or without the module
loaded) every trace is byte-identical to the base build.
Loading the module with its switches off and enabling its readings are separate checks; a school comparison binds
the executable, the module's content hash, the course inputs and the switch settings.

## The training pipeline

One pipeline for every corpus: school lessons, tales, the Torah, the Talmud, the site's pages. A corpus is only a **pool**: a file of sentences with ids (`book:pg10891:17`), in reading order, one document per book or lesson. Its sentences go round by round through the same steps (`cycle`, [its README](../tools/cycle/README.md)).

```text
pool of sentences
   │  a round takes the next N sentences
   ▼
1. drafts            Mova's perceptron (tokens, lemmas, features) + Mova's GPU parser (heads, relations)
2. witnesses         open parsers (Stanza, Trankit, spaCy) parse the same sentences on the GPU
3. treeagree         written rules agree the draft with the witnesses → the agreed tree
4. pragmatics        Mova's pragmatic reader + open witnesses → prag agree → the agreed pragmatics
5. the world         Mova reads the document so far → a short context line before each sentence
6. the teacher       a few sentences per request: tree + PRAG + WORLD; it answers with patches only
7. gates             every patched tree must be valid UD; a patch that breaks one is refused
8. the database      every version of every tree kept; the tree for training is a weighted vote
9. training          Mova's GPU parser is fine-tuned on the fresh trees; the next round drafts with it
   │
   └─► the next round
```

**1–4. Agreed before the teacher sees it.** `treeagree` merges Mova's drafts with the open parsers by written rules (which witness wins a head or relation, and when); the UD gates keep the result valid. Because the witnesses take part, agreement with them is not a fair score; the fair test is a held-out gold treebank ([parsers](parsers.md)). Pragmatics (form, act, indirectness, hedging, polarity, voice) is agreed the same way by `prag agree`, which keeps Mova's labels unless agreeing witnesses and rules say otherwise ([pragmatics](pragmatics.md)).

**5. Context without pages.** The teacher never gets the book. Mova reads the document up to each sentence and writes a **world line**: salient entities and what is known of them, the last speaker, recent events.

**6. Prepared input, patches out.** After a fixed instruction (the UD guidelines, the closed pragmatic sets, the answer form), each sentence comes with its agreed tree, PRAG line and WORLD line. The teacher answers with **patches only** («token 4, head 2, relation obl»), never a whole tree, so it cannot silently rewrite what it did not look at. Checking a prepared draft is a task a cheaper model does well; every request is built to fit the model's window. `treeagree` after the teacher is mandatory.

**7–8. Gates and the database.** A patched tree that breaks a UD gate (a cycle, two roots, a relation the tag does not allow) is refused and logged. The parse database keeps every version of every tree; `cycle db-agree` builds the training tree by a weighted vote per word. A teacher version gets its round's ordinary weight; authority is never raised by model name.

**9. Learning every round.** The GPU parser is fine-tuned from where it was on the round's trees plus a fresh sample of everything else, close to training from scratch at a fraction of the time. One model is trained on all corpora together.

A round reports how much of Mova's draft the teacher kept (UAS, LAS), how much `treeagree` still changed afterwards, and how many patches the gates refused. The first number should rise round after round.

**Running it.**

- **Asynchronous.** `cycle pipeline` feeds and collects. While fewer than `CYCLE_INFLIGHT` batches are with the teacher, it prepares the next round on local hardware (drafts, witnesses, agreement, world lines). A returned batch is applied at once and the parser fine-tuned on it while others are still out. Several corpora can run side by side.
- **Slowly by default.** A new corpus starts with one batch in flight and lowered priority; more only on the operator's word.
- **Restartable.** A round with a batch id and no results is in flight; a round with a row in the report is done. A crash loses only the round being prepared.
- **A clean start.** Every run script first unsets every `CYCLE_*` setting, so a run never inherits another corpus's teacher settings.
- **Held-out stays out.** A test lesson's sentence is never put in a round. A book pool is built with `--exclude` over every held-out set that could share its sentences.

After the pipeline, the fresh trees feed everything else: the student reads with the new parser, the autoencoder measures the new residual, the site shows the new trees. The pipeline's gain counts only if those numbers move.

## Annotation and storage contract

Mova exchanges syntax as source-bound CoNLL-U and pragmatics as schema-checked TSV. Every artifact keeps sentence identity, source text and revision, producer, and a durable receipt. Structural validity does not establish semantic correctness.

- **Storage, learning selection and publication are separate decisions.** A stored review version can stay ineligible, with no training split.
- **Exclusions and pins are explicit inputs**: held-out exclusions and the current source, teacher and witness pins.
- **Missing witnesses and disagreements stay evidence**; agreement is never fabricated.
- **Import is idempotent** for identical bound input. An imported revision does not imply a learner update. Database transactions and model activation have separate boundaries; a failure keeps earlier effects and unresolved uncertainty.
- Public interfaces need no private actor or deployment configuration. The worker side is in [the sea animals](animals.md).

## Reviewed annotation promotion

`cycle promote-reviewed` moves one independently filtered, operator-authorized batch of complete tree + pragmatics revisions into an existing parse database; `cycle rollback-reviewed` undoes it. Structural validity and teacher agreement are silver evidence; neither proves linguistic correctness. The command trusts operator-owned provenance; it is not a provenance authenticator.

```sh
CYCLE_DB=SNAPSHOT_DB cycle promote-reviewed TREES.conllu PRAG.tsv DECISIONS.jsonl
CYCLE_DB=SNAPSHOT_DB cycle rollback-reviewed TRANSACTION_HASH
```

Set `MOVA_BIG` as usual and `CYCLE_DB` to the intended database. Stop every writer first. Snapshot and lock checks coordinate cooperating callers; they are not a cross-process transaction against arbitrary old programs. Old frozen executables do not enforce the policy and must not write a promoted database.

### What is admitted

The three inputs have identical, nonempty, unique sentence-id sets. Only current, verified `school:` teaching explanations and facts are admitted; question, test, exam, held-out, joined-source and ambiguous revisions are excluded before promotion.

- **Trees**: one exact id/text header, ten columns, contiguous integer tokens, one acyclic in-range root, unchanged source text and token forms. Multiword and empty-node corrections are refused. Headerless fragments and malformed ids are rejected. Joined legacy ids are split; contiguous newdoc/version/comment preambles move with the following id.
- **Pragmatics**: header `sent_id form act indirect hedge polarity voice teacher` (tab-separated); the six categorical fields use the closed pragmatic schema; the last field is bounded free text. There is no hash of the previous pragmatic row, so the operator checks the prior database against its frozen backup.
- **Decisions** (JSONL), one per id, every record an allow (a deny rejects the whole batch before any change). The current implementation accepts only its pinned `authorization` and `provenance_state` values: it is a snapshot-specific import, not a general authorization API, and preparing a record grants no permission to promote it.
  - `sent_id` (a `school:` id), `decision: allow`, `authorization` (the pinned value);
  - `phase: teaching_text_explain_or_fact`, `provenance_state` (the pinned value), `heldout: false`, `joined_source: false`;
  - `training_split`: `train` or `dev` (an optional `split` must equal it);
  - `source_text_sha256`, `previous_tree_sha256`, `tree_sha256`, `prag_sha256`, `source_phase_sha256`;
  - `source_phase_record`: the original phase-index JSON line, without its newline. Its id, phase, state and text hash must agree; `training_allowed` must stay false and `review_only`/`heldout` must not be true.

Hashes are lowercase SHA-256. A tree hash covers the normalized sentence block with trailing whitespace removed and exactly two final newlines, all headers kept; a pragmatic hash covers the exact row; a text hash the exact `# text = ` payload; a phase hash the raw embedded record. The exact schema and normalization live in `tools/cycle/src/reviewed.rs` and its groupant.

### Effect on the database and training

A reviewed overlay overrides the weighted vote with the whole selected tree, tags, lemmas and features included; `db-agree` validates the selection and reapplies full trees after voting. A `terra-reviewed` provenance event is appended; ordinary model-name weights stay. The persistent `reviewed.conllu`, `reviewed-prag.tsv` and `reviewed-decisions.jsonl` hold the selection. Later conflicting tree or pragmatic updates fail; a stale or incomplete policy blocks writes and training. Old history and model weights are not changed.

`training_file` and the trainer's fresh selection admit only ids with `training_split=train`, minus held-out exclusions; `dev` revisions stay in the database but cannot train through these paths. A strict primary refuses an unreviewed secondary training database before GPU training (the fresh cache may already have been written), so keep `CYCLE_TRAIN_DBS` empty for an isolated refresh. Without a policy, the allowlist does not apply. Every `gparse train` input of the cycle (the database export `train-db.conllu`, the trainer's fresh trees, the round's `GP_ALWAYS` trees and the fine-tune base files) passes one held-out filter, `db::Heldout`: the ids and normalized texts (lowercase letters and digits) of EWT dev/test, GUM dev/test, ESLSpok dev/test, the school dev set, the school test lessons (`CYCLE_TEST`) and every Wikipedia-school article (cv-a, cv-b, test); `CYCLE_HELDOUT` adds more (colon-separated, each must exist). The log names the sources and the dropped count. `cycle db-export <out>` writes the export alone. The annotator's training files (`CYCLE_BASE` and the cycle's agreed corpus, for `annot drafts` and `annot batch-prepare`) pass the same filter into `<round>/base-inputs/`, and `cycle heldout-filter <out-dir> <file>...` filters any other training file the same way (file names kept for `en`'s licence gate). Safe replay selection stays the caller's responsibility. Policy cannot undo exposure already present in historical checkpoints.

One reviewed batch per database: an identical repeat validates without appending history or log; a different batch needs a fresh snapshot database. A multi-batch extension would need its own precedence design.

### Transaction and rollback

Promotion snapshots every database file (refusing symlinks and special files), records a SHA-256 inventory, stages the seven managed files (`current.conllu`, `history.conllu`, `log.tsv`, `prag.tsv`, `reviewed.conllu`, `reviewed-prag.tsv`, `reviewed-decisions.jsonl`; absent ones are created) and journals `PREPARED → APPLYING → COMMITTED` under `.reviewed-transactions/<SHA-256 of decisions>`. Each rename is atomic; the set is not. A failure after `APPLYING` keeps `.reviewed.lock`, which ordinary callers refuse; a failure before it can leave an incomplete transaction directory with unchanged data. Inspect the directory before removing a lock; never overwrite a transaction directory blindly.

Rollback verifies the before-inventory, restores every snapshotted file, removes later artifacts and keeps a receipt. It runs with all callers stopped and never through `update()`; an interrupted rollback keeps its lock and can be repeated. It restores file content, not ownership, permissions, model checkpoints or worker state: keep the deployed model backup separately.

### Retraining after promotion

A refresh continues the deployed GPU parser on the reviewed `train` trees plus a separately vetted replay set (gold treebank and current teaching trees, with every reviewed id, development lesson and protected held-out text excluded). The split is by whole lessons, so train and development share no id, lesson or text. Before activation the candidate must show finite CUDA training, a valid reload, eligible training data, an EWT LAS loss of no more than 0.5 points and teaching-development agreement at least the baseline. The best development checkpoint is kept, not the last epoch. Activation is a synced temporary-file rename at the runtime's parser path with compatible vocabulary and configuration files; reload from that path must reproduce the saved development scores. This trains the syntactic parser only; pragmatic revisions are stored but do not train a new pragmatic classifier.

## Hard cases: from reading failures to learning

The corpus readers keep the sentences Mova reads badly on one shared list (`tools/hard-case-list`, format
`mova-hard-case-v2`): the Wikipedia corpus's hourly check (pipeline failures, UD grammar constraints,
non-projective trees, Stanza disagreement) and the Whales' topic corpora (no root, heads outside, no frame, nothing to
restate). Each case carries its text, licence and a pointer to its document, and one status per parser (`en`, the
corpora's `en.bin`; `gparse`, the parser the Eel trains). `tools/hard-cases` closes the loop:

1. **recheck** with the deployed gparse: cases it already reads well (no broken constraint, Stanza agreement ≥ 80 %)
   leave the lane;
2. **wave**: the open cases become Fish tasks in their documents' context (up to eight preceding sentences, the
   source's tree and gparse's as candidates), packed under the 100 000-byte bound, in a priority lane: lane pagers
   with their configured profile, while generic pagers yield their places (`guard`);
3. **harvest**: validated, review-only replies become silver trees per licence track and split by document
   (`train`, `dev`, `measure`);
4. **witness** and **eel**: the three open parsers on the training trees, then one Eel round through the unchanged
   engine (tree agreement, held-out exclusions, the track's `dev` split and the EWT gates, guarded install);
5. **measure**: a fixed sample, before and after every new gparse model (pass rate, silver LAS on the never-trained
   split, EWT dev LAS), and the school's quick bench on both models.

Licences decide the track: CC BY, CC0, public-domain and our own texts may train the shipped model; CC BY-SA
(Wikipedia) trains only a separate internal model by default, because the data rules keep CC BY-SA out of what ships; NC, ND, research-only and unknown texts are only measured.
The commands and policy are in [tools/hard-cases](../tools/hard-cases/README.md).

## Re-annotating the retained parse snapshot

The retained school parse snapshot can be re-annotated by the Terra teacher (`gpt-6.1-terra`, medium effort). Here an AST is a sentence dependency tree in CoNLL-U.

- **Route.** Use `cycle pipeline`: GPU draft → witnesses → `treeagree` → pragmatic labels and `prag agree` → world context → Terra patches → gates. The legacy `round`/`run` pre-flow lacks pragmatic and world inputs and equivalent held-out exclusions.
- **Isolation.** For probes redirect `CYCLE_ROOT`, `CYCLE_DB`, `CYCLE_AGREED`, `CYCLE_CORPUS` and `CYCLE_MODELS`. Annotation state freezes its corpus destination (changing `ANNOT_CORPUS` later does not redirect application), so apply from an isolated state copy. A probe replaces no canonical corpus or deployed model.
- **Request size.** `annot` enforces no prompt-byte cap, and the Codex route does not enforce a legacy `max_tokens`. Preflight every request and shrink whole-source batches rather than truncating annotations.
- **The snapshot.** A derived copy separates the original's joined sentence pairs. Every tree passes the basic Rust structure checks (ids, text, columns, token indices, root, head bounds, cycles), which do not validate full UD inventories, pragmatics or semantics. Some lesson trees lack pragmatic rows. A version-bound pending-review queue covers every tree; it is a queue, not a running executor.
- **Phases.** Historical `tN` ids merge non-exam kinds (`qid(kind, n)` is not injective): recover exercise/test identity from the original lesson kind, index, text and revision, and never overwrite an ambiguous identity. `eN` identifies exams. Reviewing question text does not authorize gold labels in training. The queue excludes the configured parse test; SVAMP train-challenge items and unknown phases need separate controls; every record has `training_allowed=false` until provenance, phase and review gates pass. Existing checkpoints may already have historical exposure that filtering cannot undo.

Before scaling up: recover phases and revisions, fill missing pragmatic rows, store new teacher versions separately, and select revisions by a measured policy promoted through [reviewed annotation promotion](#reviewed-annotation-promotion), since a Terra round has only ordinary vote weight. Structural coverage, version retention, semantic review and learning are separate states; a full semantic review of the snapshot has not been run.

## Corpus enrichment

`mova-enrich` ([its README](../tools/enrich/README.md)) is an experimental mode that turns one agreed narrative round (`pre/agreed.conllu`, `pre/prag-agreed.tsv`, `world.tsv`) into reviewed teaching drafts. Mova prepares bounded lemma/role proposals and excerpts from distinct primary courses (lexical overlap is a retrieval aid, not proof of a missing topic); the teacher reviews text, annotations, context, dictionary entries and proposals through `mova-teacher`. The output is a draft dictionary supplement, a books lesson (twelve exercises, eight tests, four exams), optional course-gap candidates and explicit corrections. Known meanings are kept; fiction never becomes fact; course gaps wait for independent evidence.

```sh
mova-enrich run ROUND DICTIONARY.tsv SCHOOL_DIR NEW_RUN 32     # prepare + review + stage
mova-enrich prepare|review|stage|status|check-source …         # the same in steps
mova-enrich refine REJECTED_RUN NEW_REPAIR_RUN                 # one targeted repair
mova-enrich restage OLD_RUN NEW_RUN                            # restage without a new call
```

- **Sampling.** One job takes 8–64 sentences (default 32) from the largest eligible train-routed document: a sample, not coverage. All inputs are fingerprinted; `check-source` rejects changed inputs before a resumed job calls the teacher.
- **Cycle hook.** `CYCLE_ENRICH=1` (off by default) runs one job synchronously after annotation collection; `mova-cycle enrich K` backfills a round. Settings: `CYCLE_ENRICH_BIN`, `_DICTIONARY`, `_SCHOOL`, `_SENTENCES`, `_BUDGET`.
- **Budget.** A durable attempted-call budget per cycle root (default one call; a review plus one repair needs two), with a lock and synced reservation before each attempt. `ENRICH_PROMPT_MAX_BYTES` (default 150000) fails oversized prompts; annotations are never truncated.
- **Gates.** Rust rejects mismatched ids, bad trees, incomplete joins, stale review hashes, fabricated examples, erased meanings, duplicate questions, and answers outside a verbatim source span or over twelve words. A failed span/length gate may get one typed, hash-bound patch for exactly the failed questions; every other field is preserved. Transport failures are not retried.
- **Publication boundary.** Staged files (`dictionary-records.tsv`, `books.lesson`, `questions.jsonl`, `source-map.json`, `cross-course-review.json`, `teacher-corrections.json`) come with a receipt saying `published=false`, `complete=false`. Nothing canonical is overwritten; dictionary gates, curriculum review, independent evidence, duplicate checks and downstream evaluation come first. No seen cursor advances because drafting passed. `Enrichment.lean` proves completion needs every artifact gate.
- `cargo run --release -p mova-enrich --example lesson_smoke -- STAGED_RUN` reads one staged lesson with the three school controls.

The Mova-driven loop with retained learning and isolated exams is `mova-enrich improve` ([the school](school.md)).

## Self-directed learning

Mova's repair rules and knobs are data, so Mova can propose changes to them, test each on a copy, keep only what is significant and safe, and never change the shipped configuration in place.

### Inducing tree rules: `world induce`

`world induce <train.conllu>... --dev <dev.conllu> --test <test.conllu>... --out <dir> [--iters N] [--min N]` (`world/src/induce.rs`) learns label rules on top of the parser by transformation-based, error-driven learning (Brill 1995).

- **Features are knowledge, not words**: parts of speech of the word and its head, finiteness, position, the head's subject or other object, intransitivity, speech/motion verbs, light verbs, [absurdity matrix](knowledge.md) scores, idiom kind, measure/time nouns, archaic adverbs, animacy, surrounding punctuation. `INDUCE_WITHOUT=<group>` hides one group for ablation; syntax and the matrix carry the most.
- **Candidates** are conjunctions of 1–3 atoms built from the parser's label errors. The greedy loop takes the best `fixed − broken` with precision ≥ 0.85, applies it and repeats; every rule is printed as a sentence with its counts.
- **Scopes**: **domain rules** are induced on the training domain; **general rules** also do not hurt a development set of another domain. The shipped rules are `world/data/rules-induced.tsv`; self-play changes them only through review.
- **Where it helps**: only on errors the parser does not already know, so on unseen text with gold or teacher trees; on the parser's own training data it finds almost nothing.
- **GPU**: with `--features gpu` and `MOVA_EXEC=gpu`, break counting is an exact CUDA kernel; rules and reports are identical. `MOVA_TIMING=1` prints phases.
- **Stability**: `world rediscover <book-list> [--sentences N] [--iters K]` hides one shipped rule, parses with and without it, and the inducer must recover it with no gold.

Limits: label rules only (no head changes); the induced sets do not yet replace the hand-written repairs in the reader.

### Self-play on the repair rules: `world selfplay`

`world selfplay --dev <conllu>... --held <conllu>... [--gens N] [--threads T] [--adopt] [--out <dir>]`. A delta removes a rule, relaxes it (drops a condition), narrows it (adds a condition) or adds a rule induced from Mova's own errors on dev. Every delta runs on a copy, in parallel threads, on cached parses, with a paired bootstrap on dev. With `--adopt`, the best delta that is significant on dev (p < 0.05) and does not hurt the held-out set enters the next generation's working set, in memory only; the result goes to `rules-proposed.tsv` for review, never straight into `rules-induced.tsv`. `selfplay.jsonl` logs every delta.

**Experiment memory.** `$MOVA_DATA/runs/selfplay-journal.jsonl` keeps every tested delta (kind, rule features, dev and held-out gain, p, verdict: positive, negative or neutral), keyed by code version, base rule set and data. A repeated experiment is reused instead of rerun. Entries of another code version or older than 7 days are dropped when the journal opens. `INDUCE_DEBUG=1` prints every non-add delta. Relaxing a condition almost always hurts; useful additions are rare and precise.

Limit: the held-out set takes part in adoption, so a third untouched set gives the final number.

### Any component: `world tune`

`world tune <component> --dev <set> --held <set> [--gens N] [--threads T] [--adopt]` (`world/src/tune.rs`) runs the same loop for any part that exposes its knobs as an abstract configuration (name → value, values worth trying) and scores itself per item. A delta moves one knob to a neighbouring value; results go to the same journal; proposals are ordered by what helped before. Components: `domains` (`world/src/domains.rs`: lexicon and clash thresholds, content-word filters, main-sense rule, idioms, humour level) and `expressions` (`world/src/mwe.rs`: contiguous matching, dependency-arc pairs, minimum content words). The objective «right − wrong» favours precision.

### States, brain and letters

Mova is a skeleton plus a mutable layer; the skeleton works without any of the following.

- **`world state`** (`world/src/state.rs`): the frozen original (the cold layer in the binary) and a tree of states, each a line delta over its parent (rules, domain knobs, absurdity cells, idioms). `rollback`, `guard` (measure against the parent, roll back on a drop), `diff`, `tree`, `story`, `stable`; every transition is logged. `world state las <id> <gold> <domain>` scores a state.
- **Reproducible runs.** A request carries a state hash (`--state <id>` or `MOVA_STATE`), unpacked into `<hot>/cache/<id>/`; without one Mova runs in the last stable state and reports `[state: processing …, last stable …]`.
- **The brain** (`<hot>/brain/<id>/`) is the one memory cell per state, in plain English: working memory, pragmatics, stage progress, and rules Mova writes for itself (`stop-gain: G`). A child copies its parent's brain. Storage is a base plus an append-only, checksummed change log, compacted atomically; a crash cuts at most the last entry. The base brain is `world/data/brain-base.md`, opening with `world/data/self.md`. The brain steers training of new states, never the processing of a request.
- **Letters** (`world letters`): at the end of a stage Mova writes where to go next and why; the last letter of a kind of stage steers the next one.
- `world self`, `world brain probe|serve|say` show the self-description and how a brain reacts, over a Unix socket in English. «I am Mova» sets the speaker's name once; a name never changes.

### The family: training as a conversation

`world family serve|say|log` (`world/src/family.rs`) runs the training loop as one logged Unix-socket chat. **Mova Dev** decides how the state changes; **Mova Current** speaks for the current state and objects to a move worse by more than 0.1 LAS; **Mova State <id>** members live in other states and say where they are better; random commenters hint from the history and the journal.

- **Explore and pool.** Each member runs `induce::explore_changes` from its own rules (add, remove, relax, narrow). An idea enters `<hot>/family/pool.tsv` only with dev gain, p < 0.1 and held-out gain > 0; ideas already tried do not come back. Dev picks up to 3 pooled ideas into one child → guard → stable, or rollback and the best single idea alone. A run stops after a whole rotation brings nothing.
- **Splits and scopes.** The tales test never enters the family (`is_test_set` refuses it); explorers rotate over development sets; rules keep their domain or general scope, and the guard wants a gain in the idea's own domain and no loss beyond a small tolerance in the other. Domain knobs are explored the same way (negative control `knob_explorer_finds_the_way_back`).
- **Brains steer where to look** by a Laplace estimate of where each member's ideas worked; `FAMILY_BRAINS=0` is the ablation.
- **Live control**: `status`, `progress`, `stop`, `focus`, `reject`, `pool`, `try`, `propose` (a client's change, tried first), `negative`; `rollback` during training is refused.
- **Talk** (`world/src/talk.rs`): every client sentence gets a speech act from the closed set (ISO 24617-2 dimensions, Bunt et al. 2020); social acts are answered and remembered in a memory stream ranked by importance, recency and overlap (Park et al. 2023). Test: `tests03::dialogue_acts_of_the_family`.

### Learning on big texts

`world/src/big.rs` samples the tree store into large unlabelled sets (`big-tale`, never the tale-test books; `big-real`), each with an exploring and a disjoint guard bucket: `world big-explore <tale|real>`, `world big-judge <state> <tale|real>`. A delta (including adds from matrix pseudo-gold: an absurd argument sensible in the other role is relabelled `nsubj ↔ obj`) must pass four checks:

- **absurdity**: fewer absurd arcs, at least half coming back sensible, paired bootstrap p < 0.01 on exploring and p < 0.05 on the guard bucket, with own gold not below the parent or the original beyond small fixed tolerances;
- **grammar**: `induce::violations` (two subjects or objects on a head, case-marked arguments, vocatives as arguments) may not grow;
- **teacher**: `big::judge_changes` shows relabelled words with old and new labels in alternating order; the new label must clearly win; a failed call rejects. The matrix alone passes harmful children; grammar and teacher checks turn them red;
- **negative list**: `<hot>/family/negative.tsv` bans proven nonsense and its narrower variants; a listed rule returns for review after 30 days, then 60, 120…

**Limits.** With single-rule changes over the current atoms the parser is at a local optimum on the development sets; the next lever is new features, not more search. Some gains on UD training sets are label-convention differences between treebank versions, not parsing gains. Matrix-judged repairs of big texts have not shown up on the gold tests.

**Beliefs over stable rules (design, not implemented).** A candidate rule would carry its evidence as counts and a Beta posterior over its precision; it is promoted when the credible interval's lower bound clears the bar on untouched data, demoted to the negative list when the upper bound falls below it, and stays «believed», usable with its confidence stated, in between. The pool, the negative list and the guards exist.

### Tables: `world tabular`

`world tabular` (`world/src/tabular.rs`) reads records into Mova's world (self-chosen quartile bins, short categories, a name column read with English honorifics, identifier-like columns linking records that travel together), turns what linked records did into a relation attribute, learns a CN2-style decision list and explains every answer. Pseudo-labels from a hand-written teacher made it worse, so only Mova's untaught answers are submitted to competitions.

## Finding ideas

1. **Name the failure first.** «Most of the questions not understood are advice and situation questions», not «the student is weak».
2. **Search the library** of papers (`scisearch`) for the failure, not the solution.
3. **Read the card**, where each paper's ideas are numbered (I1, I2, …); an idea is a sentence you can implement or reject.
4. **Write the idea into the ideas register**, with the paper, where it might apply, its status, its result, its date and the state of the system it was tried on. A failed idea keeps **why it failed** and **what would have to change** for another try; when that changes, retry it against its old result.
5. **Cite it in the code**: `(brill-1995 I1)`.

Ideas also come from the residual (which ranks changes by the bits they would remove), from `world knowledge diagnose`, from reviews by other agents (check each point yourself), and from the operator. Ideas are about language, logic and code; plans, prices and private matters never go into them.

## Changing the source code

- **Grep before you write.** The thing you need is often there under another name. Compose from what exists.
- **Mechanics in Rust.** Code chews the data, not your tokens: write a subcommand instead of pasting thousands of lines into context. The Python bridges (to the neural parsers and pragmatic witnesses) are the exception.
- **Words become numbers.** Strings that rules compare become enums or interned ids; dictionaries and constants are built in at compile time.
- **Deterministic, always.** `BTreeMap`/`BTreeSet` wherever order decides anything. Run twice before claiming a gain; the runs must be byte-identical.
- **Every new step behind a switch**, which stays after the step is kept:

  ```rust
  // the lesson just read first (PRAG_OFF=student-anchor-first)
  if !prag::frames::off("student-anchor-first") && scored.iter().any(|x| Some(self.frames[x.0].src.0) == last) {
      scored.retain(|x| Some(self.frames[x.0].src.0) == last);
  }
  ```

- **A failed step leaves a comment, not code**: what was tried and what it gave, at the place where the next agent will have the same idea.
- **Worktrees for parallel work.** Crates depend on `en` by path, so an edit in a shared checkout breaks other builds. Each parallel agent works in its own `git worktree`; reuse shared build folders rather than one per task. The main instance merges.
- **Copy the binary for a long run**, so a rebuild in the middle does not change the program under measurement.
- **Per-file models.** Every owned Rust file has a groupant with its models; register changed files and run `mova-modelcheck` ([groupants](groupants.md), [mathematical modeling](mathematical-modeling.md)).
- **Commits in logical steps**, each after `./secret-guard.sh`; before merging a branch run it over the whole range, since a merge commit does not pass through it.

## Measuring

Most of what a builder believes about its own changes is wrong; only measurement says which part. The full school method (traces, the four controls, guessers, diagnosis) is in [the school](school.md); the rules:

- **Two halves.** Decide on the odd and the even lessons separately; keep a change only if it gains on both. For changes about memory or reading order the halves are not independent, so the change must also gain course by course.
- **Controls that must go red.** Every course run carries four controls (no lesson read, options moved under the next test's question, key lines removed, options under a question from a lesson half the course away), each scoring below the real student ([controls](school.md#controls-in-every-course-report)). Once, break the system on purpose and watch them turn red.
- **Guessers beside the score**: the longest option and the first option shown. A student near a guesser has learned the guesser.
- **Held-out stays held out.** The SVAMP test part is opened once per release, by the release script, never for a decision. Decisions use the two-fold check (`cv`) on the train part.
- **Every number has a date** and the state it was measured on (commit, mode, data).
- **Two modes.** CPU only is exact and the reference. GPU enhanced has exact stages (gated byte for byte) and approximants whose agreement is measured and reported; small measured deltas are accepted for speed, and a reference switch stays for comparison ([GPU execution](gpu-exec.md)).

### Fast probes and full runs

`world knowledge bench COURSE` is the iterative measurement. It keeps an automatic round counter per course and seed:

| tier | rounds | lessons per difficulty stratum | report-before pass | controls | parses |
|---|---|---|---|---|---|
| quick | other rounds | ceil(n/8), at least one | skipped | skipped, marked | cached when valid |
| medium | every fifth, except twentieth | ceil(n/3), at least one | skipped | all three | cached when valid |
| full | every twentieth | all | included | all three | recomputed |

Lessons are ranked by SHA-256 within each difficulty stratum (default seed 42); quick is a subset of medium, and lessons keep their order, ids and question indices. Missing earlier lessons change the learning state, so a probe score approximates, and can miss a small regression that the full score shows: confirm with periodic full runs. Keep the seed fixed for A/B comparisons. This is a fixed multi-fidelity cadence (after Li et al. 2018, Hyperband) without adaptive elimination.

```sh
MOVA_EXEC=gpu world knowledge bench mysticism                 # scheduled tier
world knowledge bench mysticism --round 5 --plan              # show the selection, load nothing
MOVA_EXEC=gpu world knowledge bench biology --tier quick --seed 7
```

`--round N` and `--tier` select explicitly without advancing the counter; `--no-cache` forces parsing. Artifacts go to `MOVA_BENCH_ROOT` (default `$MOVA_DATA/runs/school-bench`) and never replace the canonical course results; a parser failure leaves the counter unchanged. The parse cache is keyed by executable, model bytes, vocabularies, parser settings, GPU driver and hot rule files; corrupt entries are reparsed; only parses are reused, never learning or answers.

**Training probes.** Parser development cadence and checkpoint selection (`GP_DEV_EVERY`, `GP_DEV_QUICK`) are in
[the gparse README](../gparse/README.md#periodic-development-evaluation).

## Debugging by looking

Measurement says how much is wrong, not what. Most changes that moved Mova started with someone reading a hundred failures.

- **The autoencoder** ([english](english.md)) turns every loss into an item: layer, class, sentence, what had to be stored. Read the largest class of the weakest layer, fifty items: they are usually three problems. Compare exact mode (keeps every residual, gives the bytes back) with normal mode (says the text from the layer alone): the difference is what the layer does not know. Drop negation on purpose and only the negation class must grow. A change counts only if the residual falls on the development blocks with the gate still exact and two runs identical.
- **One text alone.** `roundtrip probe "<text>"` prints a text at every layer, both ways; `world knowledge trace <course> <lesson> <k>` prints one failed test (question reading, matched frames, option readings, choice). Ask whether the tree says who did what to whom, whether the frame keeps negation, tense, quantity and place, whether «he» points to the right entity. One text gives the hypothesis; the corpus decides.
- **Parallel versions.** Translations of one verse should give one world: a contradiction is a reading error or a real translation difference, and only reading tells which. Change one thing on purpose (negate, swap roles, change a number) and check the meta-world notices, after subtracting the background rate between unchanged translations. Rules between versions that gain as much on a wrong-verse control are style. The Hebrew original against its translations is the hardest test. `parsecmp agree` clusters Mova's disagreements with the open parsers by construction.
- **The architecture.** Signs the design is wrong: an error returns in a new form after each fix; removing a part improves the system; a layer's residual stays large whatever you add; controls stop going red. Then write the finding down with evidence, test the new design with the smallest switched change (if it does not gain, the big one will not), plan the real change in the docs, change formats with every reader and writer, re-measure everything downstream, and bring a large change to the operator first.

## Changing the skeleton

The skeleton is what everything hangs on: the parser (`en`), the frames reader, the world, the formats between them. Understanding mechanisms and hard parameters change chiefly through deliberate code and model updates and retraining ([knowledge](knowledge.md)).

- The parser learns in [the training pipeline](#the-training-pipeline); watch how much of Mova's draft the teacher keeps.
- Frames and the world change through the residual; a change is kept only if the residual falls with the round trip still exact.
- Formats change freely (nothing is frozen), but every reader and writer changes in the same commit.
- During a dictionary or lesson wave the skeleton's operations stay frozen, so the wave's numbers mean something.
- After a skeleton change everything downstream is re-measured: treebank scores, the student, the residual.
- A change that helps one course and hurts five is not kept for the one.

## Tests and gates

A **test** says a function does what its name says. A **gate** stops a build, merge or release when something is wrong. Both need a negative control: a case that must fail, and does. A gate that cannot show red is not a gate.

| gate | stops | red case (must stop) | green case (must pass) |
|---|---|---|---|
| the round trip | a layer that cannot give its input back byte for byte | a residual item dropped on purpose | every block of the training mix |
| the course controls | a course that does not measure reading | no lesson, shuffled questions, key lines removed, a far question | the real student, above all four |
| `treeagree` UD gates | an invalid tree in the agreed corpus | a cycle, two roots | the agreed school treebank |
| the record gate | a bad dictionary record | a date anywhere in a field; five fields | decimals, section numbers, hyphenated numbers; the whole dictionary |
| lesson gates (`check`, `lint`, `balance`) | a lesson that parses wrong or gives its answer away | a duplicate option; the right one always longest | every lesson of the courses |
| the site build | broken links or structured data, inline styles or scripts, internal strings on a public page | a broken link, an unparsable JSON-LD block, an inline script | the built site |
| the secret guard | a key, private string or private path in a commit | a staged fake key and private path (`tests/secret-guard-control.sh`) | the day's commits |
| determinism | a run that differs from itself | a `HashMap` iterated in a decision | the same run twice |

When you add a gate, write its red case first, then its green one, then make it pass.

## The cost twin

`mova-twin` (`tools/twin`, [its README](../tools/twin/README.md)) predicts a workload's wall time, runs it, records the prospective error and updates its coefficients. It models cost only, not correctness or learning.

For features x, `time = xᵀθ`. With `e = observed − predicted` and `h = xᵀPx`, recursive least squares from an explicit prior with `P = I/ridge` (Sherman & Morrison 1950; Kalman 1960) updates `θ' = θ + P x e / (1 + h)` and `P' = P − (P x)(P x)ᵀ / (1 + h)`. Cholesky checks positive definiteness. There is no forgetting factor and no transfer across changed binaries, data or hardware; P is not a calibrated interval; the observed feature box only flags extrapolation. `formal/Twin.lean` proves the scalar residual identity, not forecast accuracy. Features come from mechanisms (`gparse plan MODEL CORPUS TOKEN_BUDGET` counts batches, padded tokens and attention cells without loading weights); their choice is a stated hypothesis.

```sh
mova-twin predict experiment.json                      # from tools/twin/example.json, paths resolved
mova-twin run experiment.json "$MOVA_DATA/runs/twin-001"
mova-twin run candidate.json "$MOVA_DATA/runs/twin-002" "$MOVA_DATA/runs/twin-001"
mova-twin report "$MOVA_DATA/runs/twin-001" "$MOVA_DATA/runs/twin-002"
```

A run directory holds a copied executable, a `forecast.json` written before launch, outputs, `observation.json`, and `updated.json` only after a validated success; nothing is overwritten, and parent hashes are checked. The command runs without a shell in an explicit environment (only `PATH` and `LANG` default); declared inputs are hashed before and after. Predictions are hypotheses, never grounds for automatic promotion.

## Making Mova your own

Mova is a skeleton others build domain versions on (a field of knowledge, documents, another language).

**Keep**: the pipeline and its formats (tokens, UD trees, frames, the world); determinism, switches, two halves, controls; the licence discipline; the round trip as the first gate on any new text.

**Make your own**: courses (a folder of lessons in the same format; the student, gates and site work on it unchanged), [seeds](seeds/) (small hand-written rules from which bigger rules grow), dictionary layers (through the same record gate), worlds (dense manuals of central domains), and the teacher's prompts (it corrects Mova's draft; it never answers in Mova's place).

Change one thing at a time and measure against unchanged Mova on the same tests. A domain version much worse on primary school has broken the skeleton, not specialised it.

## Limits

- Odd and even lessons are not independent for changes about memory or reading order.
- The student's weakest questions need knowledge from many lessons at once ([the school](school.md)).
- The teacher's corrections are trusted only after the open parsers agree; where they all share a blind spot, so does Mova.
- Structural validity, teacher agreement and promotion are silver evidence, not proof of correctness; historical checkpoints may have seen data that current filters exclude.
- Most measurements are on primary-school English. Other languages have manuals and corpora, not yet students.
- Formats are not frozen and will change; the rules on code, measuring and gates should not.

Release-to-release results are in the release notes: [0.3](../releases/0.3.md), [0.4](../releases/0.4.md), [0.5](../releases/0.5.md).

— the Elamur team
