# en-lib

## descriptant

### en-lib

English library compilation surface: public module declarations only; no standalone execution or reexport transformation.

## approximant

### Forward approximant

Rust resolves the declared public module names to their source modules at compile time. Consumers en::ctx, en::ctxeval, en::diag, en::emode and en::license share these exact definitions. Runtime semantics reside in modules, not this declaration file.

Domain/state: Rust module namespace and compiled dependent modules.

Executable surrogate: unsupported. Categorical decisions, binary layouts and effects have no calibrated percentage metric here. No automatic substitution and no claimed 1/2/5/10 percent profile.

## invariant

### Invariant and cost

A declaration adds module visibility but executes no module function. No initialization/effects are attributed to lib.rs itself. Compile success is representation evidence only.

Cost: Declaration scan O(source bytes); compilation/linking cost delegated to Rust and modules, no independent runtime cost asserted.

Limits: No independent runtime tests/control for a pure module-declaration file; Semantic behavior delegated to module groupants; No meaningful percentage surrogate

## composition

### Composition

Rust resolves the declared public module names to their source modules at compile time. Consumers en::ctx, en::ctxeval, en::diag, en::emode and en::license share these exact definitions. Runtime semantics reside in modules, not this declaration file.

There is no source-local function, static or algorithm; initialization state belongs to modules such as ctx/emode OnceLocks.

Composition is branch-selected, not a claim that every function runs in sequence. Inputs and effects cross the English library module boundary; consumer checks cannot establish delegated training, parser, dictionary, registry or filesystem behavior. The main CLI has source-scoped children for dispatch, XPOS, evaluation/context, text/conversion and persistence/comparison. Child coverage is lexical and parent relations remain partial.

## models

## behavior_complement

For the module-declaration source alone exclude any claimed train/read/write effect: those require invoking linked functions. Reject the inference that visibility proves the behavior of every public module.

## reverse_reconstruction

Given an exported module-name set, compatible source declarations include arbitrary comments/order and equivalent visibility declarations. The compiled API set does not recover module implementation text.

## visibility_boundary

pub mod makes a namespace visible to downstream crates; visibility of members still depends on their own declarations.

## compile_representation

Successful use of en::ctx::Mode and en::license::commercial_gate verifies resolution and signatures, not totality, effects or semantic equivalence.

## dependency_join

main imports en namespace, while ctxeval uses crate::ctx and diag uses crate::parse: all resolve to the same library module graph.

## no_runtime_state

There is no source-local function, static or algorithm; initialization state belongs to modules such as ctx/emode OnceLocks.

## tests

### Evidence

Checks: cargo test -p en --lib selected fixtures: library compile/name visibility only.

The selected suite contains 14 actual named Rust tests, each run twice debug and twice release with identical sorted named outcomes, all exit0. Build entrypoint additionally ran empty-directory, two-lemma synthetic dictionary and malformed-tag precedence cases twice per profile; outputs matched and malformed row exited101 without changing previous outputs. Compile/name-resolution evidence only. No source-local runtime test or negative control is claimed.

Unexecuted: No independent runtime tests/control for a pure module-declaration file; Semantic behavior delegated to module groupants; No meaningful percentage surrogate. No corpus, training, model mutation, held-out set, GPU, teacher or API was run.

## prospective

### Prospective source-bound modeling

- build.rs assembles dictionary blobs and constants. index has power-of-two capacity >=2n and linear probing, zero sentinel, 1-based stored indices; src_bits unions four known source bits. main reads missing/unreadable dictionaries as empty, parses tags before lemma/count, merges UD lower-lemma paradigms and writes three outputs sequentially after OUT_DIR lookup. Test actual helpers and invoke compiled build main only in isolated synthetic directories. Overflow and large-id casts remain gaps.
- ctx.rs preserves absent layers, emits stable group codes, stacks entity openings before token observation and closes afterward; gold carries genre across newdoc while resetting previous type/speaker, skips empty tokens after state updates. derived checks sentence count then noun lemma recurrence by document. Setup rejects training pred before eval/groups/source/dependency checks. Rng wrapping mix and bounded modulo; shuffled resizes entities and rejects same-document donors, with unbounded rejection-loop worst case. Existing two GUM tests must not run. Safe existing entity and gold/shuffle fixtures plus added empty/cycle/precedence cases.
- ctxeval.rs load reads each file then checks sentence count, id and token count (not forms), rejects no runs last. report assumes nonempty systems and runs; bootstrap with zero resamples panics, floating means of empty samples can be NaN. Metrics percent units, sample standard deviation, quantile indices floor(.025B), ceil(.975B)-1. Add bounded pure helper tests plus temporary System::load and report fixtures. Invalid arbitrary public System shapes are not certified.
- diag.rs class partitions relations; depths cycles return n+1 despite comment saying n; invalid heads panic. nonprojective walks intervening words; report delegates parser work and constrained failures fall back to original parse. Synthetic depth/crossing tests only, no model training. Report ceiling prose is not a mathematical guarantee.
- emode.rs first matching closed entry wins force, tagged finds alternate thine reading; contextual guards precede ending dictionary resolution; fix filters feats through loaded embedded registry, only be keeps cop. postverbal_not skips modern continuations, auxiliaries, already-attached and newly-created cycle cases; preexisting cycles bounded to n+1 steps, invalid aligned arrays/heads can panic. Execute safe existing closed-word and negation tests; dictionary-sensitive early-return tests excluded.
- lib.rs solely declares its public modules (30 at the time of writing): compile/name visibility is the only source-owned boundary, no independent runtime semantics or control claimed.
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

2026-11-04 epoch (en-morph): declares the new modules derive and tagrule. Source 37225479b7da; earlier receipts and evidence refer to the previous source hash and are kept as history.

2026-11-04 epoch (lexicon growth): modules lexext and lexgrow exported.
