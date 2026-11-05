# en-ctx

## descriptant

### en-ctx

Document-context projections, feature codes, rule-derived type/entities, seeded cross-document shuffling and environment configuration.

## approximant

### Forward approximant

Gold scans comments before dropping tokenless sentences; newdoc resets prev/speaker but does not reset genre. Entity openings are pushed before observing deepest mention, closings remove the last matching ID after observation. Derived resets seen nouns at document change, returns error on annotation count mismatch, leaves empty contexts empty. stype_rule chooses first head-zero root, then question→interjection→fragment→modal/sub→inf/ger/imp→decl. No root returns Other before question check.

Domain/state: Optional context layers, sentence annotations, document IDs, open-mention stack, previous speaker/type, process OnceLocks and RNG u64.

Executable surrogate: unsupported. Categorical decisions, binary layouts and effects have no calibrated percentage metric here. No automatic substitution and no claimed 1/2/5/10 percent profile.

## invariant

### Invariant and cost

masked only removes layers; codes 0=no layer, entity 1=outside (including out-of-range with present layer), full entity=2+4*type+info. First previous sentence code=100. Gold output and docs lengths equal nonempty sentence count. shuffled preserves absent contexts and resizes donor entity vector to recipient length.

Cost: Mention closes O(stack depth); total O(bytes+mentions*depth). stype_rule O(n²) through nested xcomp/mark scan; derived O(tokens) expected hash work plus rule cost. shuffled donor scans O(sentences²), rejection loop has no fixed worst bound. O(tokens+context) storage.

Limits: GUM corpus tests gum_entities_balanced and stype_rule_on_gum_dev explicitly not run; Environment Setup branch combinations and OnceLock alternate modes source-derived only; Invalid docs/lens lengths can panic; no adversarial allocation tests

## composition

### Composition

Gold scans comments before dropping tokenless sentences; newdoc resets prev/speaker but does not reset genre. Entity openings are pushed before observing deepest mention, closings remove the last matching ID after observation. Derived resets seen nouns at document change, returns error on annotation count mismatch, leaves empty contexts empty. stype_rule chooses first head-zero root, then question→interjection→fragment→modal/sub→inf/ger/imp→decl. No root returns Other before question check.

Rng starts splitmix(seed)|1, xorshift13/7/17 with u64 wrapping mix. below(0)=0. seed invalid/unset=0; seeded(base) preserves base exactly at zero. Other seed states collide through OR1, so reverse seed recovery not unique.

Composition is branch-selected, not a claim that every function runs in sequence. Inputs and effects cross the English library module boundary; consumer checks cannot establish delegated training, parser, dictionary, registry or filesystem behavior. The main CLI has source-scoped children for dispatch, XPOS, evaluation/context, text/conversion and persistence/comparison. Child coverage is lexical and parent relations remain partial.

## models

## behavior_complement

For enabled group mask g, exclude output containing layers whose corresponding bit is absent. For a nonempty multi-document donor pool, exclude shuffled donor from recipient document; this says nothing about accidental equal context values.

## reverse_reconstruction

R(code,mode) includes all contexts with that code under the process-cached ablation. Full code >=2 recovers type/status, span mode merges every mention, type/info each erase one coordinate. Gold context cannot recover raw comments or entity identifiers; R(shuffle) includes multiple donor indices with equal values.

## mention_timing

All opens at a token are applied before its observation, all closes afterward, even when textual closings precede openings. Duplicate IDs close rposition; unmatched closings increment counter, remaining stack length reports open mentions.

## environment_precedence

Setup parses train, rejects Pred, parses eval, groups, source, validates pred dependencies, rejects context eval with no-context training, then train_pass1/rule dependency. EN_CTX_ENT and EN_CTX_WORD are first-call cached; later environment changes do not alter them.

## document_reset

newdoc increments document number only after started; empty sentences still change genre/reset state/started. Speaker updates only on annotated nonempty sentences; genre can persist across documents.

## seeded_selection

Rng starts splitmix(seed)|1, xorshift13/7/17 with u64 wrapping mix. below(0)=0. seed invalid/unset=0; seeded(base) preserves base exactly at zero. Other seed states collide through OR1, so reverse seed recovery not unique.

## tests

### Evidence

Checks: ctx::wave27::absence_modes_and_unmatched_entities; ctx::tests::entities_nested_and_single; ctx::tests::gold_context_from_comments_and_shuffle_control.

The selected suite contains 14 actual named Rust tests, each run twice debug and twice release with identical sorted named outcomes, all exit0. Build entrypoint additionally ran empty-directory, two-lemma synthetic dictionary and malformed-tag precedence cases twice per profile; outputs matched and malformed row exited101 without changing previous outputs. Compiled mutation control 1: compiler exit0, selected assertion run exit101, exact source restoration, positive rerun exit0. See external control-1.txt and compile/red/green logs.

Unexecuted: GUM corpus tests gum_entities_balanced and stype_rule_on_gum_dev explicitly not run; Environment Setup branch combinations and OnceLock alternate modes source-derived only; Invalid docs/lens lengths can panic; no adversarial allocation tests. No corpus, training, model mutation, held-out set, GPU, teacher or API was run.

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
