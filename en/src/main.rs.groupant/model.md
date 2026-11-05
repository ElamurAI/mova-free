# en-main

## descriptant

### en-main

English CLI dispatch and adapters for evaluation, context wiring, file transforms, conversion and saved annotators. Most model/training paths remain explicitly unexecuted.

## approximant

### Forward approximant

Dispatch matches command and selected arity guards; train gates licenses before requiring output. Pure split uses first global -- then slices from index (panic if separator precedes from). xpos_of maps UPOS/features in ordered cases. folds rejects k<2 before read, assigns newdoc groups d%k, creates output dir and writes fold/rest per f. test_ctx None returns before file read; remaining branches read doc then validate shapes, optional pass1 then derive/shuffle then mask.

Domain/state: Arguments/environment, file paths, sentence/context vectors, delegated models and output streams/files.

Executable surrogate: unsupported. Categorical decisions, binary layouts and effects have no calibrated percentage metric here. No automatic substitution and no claimed 1/2/5/10 percent profile.

## invariant

### Invariant and cost

xpos_fill replaces only integer-ID ten-column rows with unknown/_ XPOS, keeps existing recognized tags, normalizes LF line endings and appends final LF. folds preserves selected sentence bodies but forces blank_before=0 and final_newline=true; not byte-lossless over arbitrary input framing. Conversion may emit earlier files before later failure; train save may persist even if reload verification fails.

Cost: Pure helpers O(args/features/input bytes). folds clones/writes O(k*document bytes), retains one partition at a time. Context validation O(tokens), generation/shuffle as ctx model. Evaluation/training cost delegated; output accumulation can be full-file or full-report. No cost cap on user counts/epochs/beam.

Limits: Training, model load/save/annotation, real corpus evaluation, teachers and APIs not executed; Missing args on expert-check can index-panic; split invalid from/separator can panic; Environment fraction accepts negative/NaN rather than validating claimed 0..1 domain; Report/schema/data adapters are source-derived outside bounded helpers

## composition

### Composition

Dispatch matches command and selected arity guards; train gates licenses before requiring output. Pure split uses first global -- then slices from index (panic if separator precedes from). xpos_of maps UPOS/features in ordered cases. folds rejects k<2 before read, assigns newdoc groups d%k, creates output dir and writes fold/rest per f. test_ctx None returns before file read; remaining branches read doc then validate shapes, optional pass1 then derive/shuffle then mask.

compare maps reference IDs last wins, only identical-form full sentences enter Scores; per-token classes still zip partial matching forms. clean counts no UPOS mismatch on compared subset, not full agreement. frac uses floor step changes; f>=1 returns all, zero returns none, NaN comparison can retain all.

Composition is branch-selected, not a claim that every function runs in sequence. Inputs and effects cross the English library module boundary; consumer checks cannot establish delegated training, parser, dictionary, registry or filesystem behavior. The main CLI has source-scoped children for dispatch, XPOS, evaluation/context, text/conversion and persistence/comparison. Child coverage is lexical and parent relations remain partial.

## models

## behavior_complement

For folds k<2 exclude all reads/mkdir/writes; for test_ctx Mode::None exclude any path I/O. For train denied license exclude entry into train_model regardless of output argument. These are scoped prefix-effect exclusions, not transactional guarantees.

## reverse_reconstruction

R(xpos VB) includes VERB/AUX with Inf even if Past also set; R(split) admits original prefix command, same train/rest and first separator, but paths lose original OS encoding through String args. Fold/rest pair recovers selected sentence multiset/order under known k/document assignment but not leading blanks/final-newline framing.

## xpos_precedence

VERB/AUX: Inf before Part+Pres before Part before Ger before Past/Imp before third-sing before VBP. PRON/DET possessive beats interrogative/relative. Unknown UPOS→FW; punctuation maps by spelling with colon fallback.

## context_join

train_ctx maps pass1 by ID last duplicate wins, matches token counts, filters tagged sentences, offsets document IDs between files, checks total count, shuffles before masking. test_ctx rule+Gold uses provided trees; other rule modes load basename.none.conllu and require sentence IDs/counts, not forms.

## effect_order

ud_eval validates Setup then creates prediction directory before reading/training. convert loads/parses rules before pair check and file reads. train_model output argument check precedes train_files; save→metadata→optional reload→comparison, no rollback. folds writes fold/rest sequentially.

## evaluation_selection

compare maps reference IDs last wins, only identical-form full sentences enter Scores; per-token classes still zip partial matching forms. clean counts no UPOS mismatch on compared subset, not full agreement. frac uses floor step changes; f>=1 returns all, zero returns none, NaN comparison can retain all.

## tests

### Evidence

Checks: main::wave27::argument_and_xpos_precedence; main::wave27::synthetic_folds_and_xpos_fill.

The selected suite contains 14 actual named Rust tests, each run twice debug and twice release with identical sorted named outcomes, all exit0. Build entrypoint additionally ran empty-directory, two-lemma synthetic dictionary and malformed-tag precedence cases twice per profile; outputs matched and malformed row exited101 without changing previous outputs. Compiled mutation control 6: compiler exit0, selected assertion run exit101, exact source restoration, positive rerun exit0. See external control-6.txt and compile/red/green logs.

Unexecuted: Training, model load/save/annotation, real corpus evaluation, teachers and APIs not executed; Missing args on expert-check can index-panic; split invalid from/separator can panic; Environment fraction accepts negative/NaN rather than validating claimed 0..1 domain; Report/schema/data adapters are source-derived outside bounded helpers. No corpus, training, model mutation, held-out set, GPU, teacher or API was run.

## prospective

### Prospective source-bound modeling

- build.rs assembles dictionary blobs and constants. index has power-of-two capacity >=2n and linear probing, zero sentinel, 1-based stored indices; src_bits unions four known source bits. main reads missing/unreadable dictionaries as empty, parses tags before lemma/count, merges UD lower-lemma paradigms and writes three outputs sequentially after OUT_DIR lookup. Test actual helpers and invoke compiled build main only in isolated synthetic directories. Overflow and large-id casts remain gaps.
- ctx.rs preserves absent layers, emits stable group codes, stacks entity openings before token observation and closes afterward; gold carries genre across newdoc while resetting previous type/speaker, skips empty tokens after state updates. derived checks sentence count then noun lemma recurrence by document. Setup rejects training pred before eval/groups/source/dependency checks. Rng wrapping mix and bounded modulo; shuffled resizes entities and rejects same-document donors, with unbounded rejection-loop worst case. Existing two GUM tests must not run. Safe existing entity and gold/shuffle fixtures plus added empty/cycle/precedence cases.
- ctxeval.rs load reads each file then checks sentence count, id and token count (not forms), rejects no runs last. report assumes nonempty systems and runs; bootstrap with zero resamples panics, floating means of empty samples can be NaN. Metrics percent units, sample standard deviation, quantile indices floor(.025B), ceil(.975B)-1. Add bounded pure helper tests plus temporary System::load and report fixtures. Invalid arbitrary public System shapes are not certified.
- diag.rs class partitions relations; depths cycles return n+1 despite comment saying n; invalid heads panic. nonprojective walks intervening words; report delegates parser work and constrained failures fall back to original parse. Synthetic depth/crossing tests only, no model training. Report ceiling prose is not a mathematical guarantee.
- emode.rs first matching closed entry wins force, tagged finds alternate thine reading; contextual guards precede ending dictionary resolution; fix filters feats through loaded embedded registry, only be keeps cop. postverbal_not skips modern continuations, auxiliaries, already-attached and newly-created cycle cases; preexisting cycles bounded to n+1 steps, invalid aligned arrays/heads can panic. Execute safe existing closed-word and negation tests; dictionary-sensitive early-return tests excluded.
- lib.rs solely declares 26 public modules: compile/name visibility is the only source-owned boundary, no independent runtime semantics or control claimed.
- license.rs sequentially strips ordered suffixes; empty paths yield empty source. Table is embedded and collects duplicate keys last; gate first denied/unknown path fails, empty train succeeds without opening files. Existing safe path-only gate test plus empty and failure-precedence fixtures.
- main.rs dispatches arguments and effectful delegated work. split searches separator globally and may panic when separator precedes from. xpos_of has ordered morphology precedence. folds rejects k<2 before read, preserves per-sentence raw content but resets document blank_before/final_newline; creates directory then writes fold/rest sequentially without rollback. test_ctx None returns before any read. train license gate precedes missing output validation; model save precedes reload verification. Nested fragments will cover dispatch, pure/file helpers, context/evaluation, conversion, persistence. Add bounded helper/folds/xpos/temp tests; training/model/teacher execution excluded.

Expected costs: context O(bytes + mentions*stack depth), tree rules O(n²) worst case; diagnostics O(n²) tree traversal plus delegated decode; report O(B*n*seeds) bootstrap plus token metrics; build linear probing O(n²) worst case and binary/strings space; CLI costs sum delegated stages with sequential partial effects. Compiled mutation controls target index sentinel, context mode, bootstrap statistics, cycle bound, archaic force, license denial and XPOS precedence. Exact restoration verified byte-for-byte. No useful percentage surrogate for categorical decisions, effects or visibility; executable approximants explicitly unsupported.

## History

### 2026-10-31 · descriptant

2026-10-31 modeling epoch: inspected complete existing source before changes; added bounded inline tests where missing. Production behavior unchanged. The model records existing failure boundaries, including panics, lossy projections and partial effects. A registered leaf is partial coverage, not a proof.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.

Receipts: $MOVA_DATA/runs/octopus-wave27-control-2026-10-31/. verify.rs and controls.rs preserve exact commands; *.exit files preserve numeric statuses. Paired syntax projections were identical. Strict Clippy exits101 on pre-existing en/build.rs:51 manual_is_multiple_of before library checking; no lint-clean claim. Only appended test modules formatted; existing source formatting retained.

### 2026-10-31 · prospective

Heading `### Prospective source-bound modeling — 2026-10-31` stays in prospective as `### Prospective source-bound modeling`.

Read all eight sources at base e90d56b1d87092a22ce7a05ce8352fe70cf151d8 before edits. Scope is English orchestration under inference-core. No training, corpus or external services will execute.

### 2026-11-04 · descriptant

Epoch 2026-11-04: `en lexicon-redact [<data-dir>]` dispatches to `lexgen::redact` (default `data`) and prints the counts of redacted lemmas and dropped form rows.

2026-11-04 epoch (en-morph): adds the commands derive (analyses with reasons) and lexicon-cover (unknown words before and after derivation per file, EN_COVER_LIST lists). Source 9895d057b5ea; earlier receipts and evidence refer to the previous source hash and are kept as history.

2026-11-04 epoch (lexicon growth): new command lexgrow (dispatch fragment grew); derive prints the extension's rows with their sources, an acronym's expansions and the derivation of an extension word.

### 2026-11-05 · descriptant

Epoch 2026-11-05: `en licence` (also `license`) prints `dict::LICENCE`, the compiled data/LICENSE.md: the dictionary's CC BY-SA 4.0 licence and its citations; it reads no file and needs no model.
