# en-build

## descriptant

### en-build

Compile-time dictionary encoder: lemma heap, open-addressed indexes, analyses/paradigms and generated word constants. Empty/unreadable input files become empty text; outputs are three sequential files.

## approximant

### Forward approximant

index(keys) allocates m=max(2,next_power_of_two(2n)), inserts i+1 at the first empty cyclic slot starting hash(key)&(m-1). main parses tag→lemma ID→count→source bits, builds analyses and UD-lowercase/AGID paradigms, then little-endian blob sections padded to four-byte boundaries. Word names first take punctuation, then first 5000 lowercase ASCII lemmas, then explicit constants; duplicate names retain the first assignment. Outputs lexicon.bin, lexicon_layout.rs, words.rs in that order.

Domain/state: Input TSV bytes, lemma/form IDs, source bits, hash slots, byte offsets, OUT_DIR and filesystem.

Executable surrogate: unsupported. Categorical decisions, binary layouts and effects have no calibrated percentage metric here. No automatic substitution and no claimed 1/2/5/10 percent profile.

## invariant

### Invariant and cost

Within representable usize/u32 sizes, slots contain each ordinal 1..n once and zeros elsewhere, capacity >=2n; section offsets are bytes, analysis/paradigm records are 12 bytes. Duplicate lemma text is allowed: lid uses last ID, index records every occurrence; no unique-ID inverse is implied.

Cost: Hash insertion expected O(n) with hash assumptions, worst O(n²); paradigm merge linear scan can make O(records²). Storage O(input bytes+records+slots+blob). next_power_of_two, usize multiplication and u32 narrowing have no overflow certificate.

Limits: Only helper semantics and isolated synthetic build execution; no real dictionary quality validation; Output writes are not atomic, allocation/cast/overflow failures untested; No useful approximate semantics for binary compatibility

## composition

### Composition

index(keys) allocates m=max(2,next_power_of_two(2n)), inserts i+1 at the first empty cyclic slot starting hash(key)&(m-1). main parses tag→lemma ID→count→source bits, builds analyses and UD-lowercase/AGID paradigms, then little-endian blob sections padded to four-byte boundaries. Word names first take punctuation, then first 5000 lowercase ASCII lemmas, then explicit constants; duplicate names retain the first assignment. Outputs lexicon.bin, lexicon_layout.rs, words.rs in that order.

Name collisions choose first insertion; names listing is sorted (id,name), independent of HashMap traversal. explicit consts must be existing ASCII lowercase words; absent/invalid names panic.

Composition is branch-selected, not a claim that every function runs in sequence. Inputs and effects cross the English library module boundary; consumer checks cannot establish delegated training, parser, dictionary, registry or filesystem behavior. The main CLI has source-scoped children for dispatch, XPOS, evaluation/context, text/conversion and persistence/comparison. Child coverage is lexical and parent relations remain partial.

## models

## behavior_complement

For index keys of representable length, exclude any slot result whose nonzero multiset differs from {1,..,n}; exclude output filesystem writes before OUT_DIR is obtained, but do not exclude partial files after a later write failure.

## reverse_reconstruction

R(slots)={ordered key lists K: index(K)=slots}; hash collisions and probing make R non-singleton. Blob plus layout recovers stored bytes and records within valid offsets, not skipped TSV columns, original source letter order or unknown source characters.

## source_bit_union

src_bits(s)=OR({1 if u,2 if a,4 if r,8 if b}); repeated known letters and arbitrary unknown letters are equivalent, empty string maps zero.

## failure_precedence

Missing/invalid-UTF8/read-failed dictionaries map empty. A >=5-column row checks tag before parsing lemma, then frequency; accessing lower/par may panic on invalid lemma. Invalid constants fail before OUT_DIR lookup and any output writes.

## binary_coordinates

L_OFF offsets are relative to lemma byte heap; F_OFF offsets are relative to form byte heap; A/P_OFF count records, not bytes; every emitted numeric word is little-endian.

## constant_selection

Name collisions choose first insertion; names listing is sorted (id,name), independent of HashMap traversal. explicit consts must be existing ASCII lowercase words; absent/invalid names panic.

## tests

### Evidence

Checks: build::wave27::index_and_record_layout.

The selected suite contains 14 actual named Rust tests, each run twice debug and twice release with identical sorted named outcomes, all exit0. Build entrypoint additionally ran empty-directory, two-lemma synthetic dictionary and malformed-tag precedence cases twice per profile; outputs matched and malformed row exited101 without changing previous outputs. Compiled mutation control 0: compiler exit0, selected assertion run exit101, exact source restoration, positive rerun exit0. See external control-0.txt and compile/red/green logs.

Unexecuted: Only helper semantics and isolated synthetic build execution; no real dictionary quality validation; Output writes are not atomic, allocation/cast/overflow failures untested; No useful approximate semantics for binary compatibility. No corpus, training, model mutation, held-out set, GPU, teacher or API was run.

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

2026-11-04 epoch (lexicon growth): extension() compiles data/lexicon-ext.tsv and data/acronyms.tsv into lexext.bin with its own layout (strings, open-addressing indexes, 8-byte records); the dictionary blob and its fingerprint are unchanged.
