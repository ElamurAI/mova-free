# world-ideas

## descriptant

### world-ideas

read ignores comments and rows with fewer than six tab fields; invalid time becomes zero. record updates first equal kind/clean(text), evidence/context/time and preserves source, else appends; atomic rewrite errors discarded. related scores sum 1e6/df for normalized shared words, sorts score descending, held/principle/derived/other, newest, text. carried_over keeps held nonempty differing contexts.

## approximant

read ignores comments and rows with fewer than six tab fields; invalid time becomes zero. record updates first equal kind/clean(text), evidence/context/time and preserves source, else appends; atomic rewrite errors discarded. related scores sum 1e6/df for normalized shared words, sorts score descending, held/principle/derived/other, newest, text. carried_over keeps held nonempty differing contexts.

Exact categorical/effect semantics: no useful executable percent surrogate supplied. Future admissible caps 1/2/5/10% require defined metric/domain; no error bound claimed.

## invariant

Missing/unreadable file becomes empty. Zero overlap drops an idea; k=0 returns none. Text/evidence/source tab/newline cleaning is asymmetric: kind/context remain raw and can break TSV. seed_principles uses snapshot membership so return count need not equal new unique records.

Cost: read/write O(B); record O(B+N). related builds df O(W log V), scores intersections and sorts O(N log N), retains entire file. No locking, write contention/durability guarantee or overflow certificate for u64 score sums.

## composition

Normalization is nerves::norm, shared query words count once. Kind priority precedes time; ctx comparison is exact text.

lib exposes modules; main activates state and dispatches. ideas uses nerves::norm and talk::now, store::write_atomic; talk uses babi splitting/prag Act. VMM uses lang::parse_all and world executor summaries before teacher continuation. These adapters preserve different coordinates (line number versus sentence anchor); splice acceptance does not establish execution validity. Isolated tempfile read/update/ranking and carried-over tests; no world seeding or production idea store access.

## models

## behavior_complement

record updates an existing matching idea's evidence/context/time but excludes replacing its original source field. related excludes zero-score ideas and uses kind before time for equal score. carried_over excludes empty/current contexts. Missing/unreadable storage can yield empty read results; a successful return from record does not exclude an ignored atomic-write error.

## reverse_reconstruction

TSV records admit cleaned raw strings differing in tabs/newlines; overwritten evidence/time are unrecoverable, and source stays first. Ranking admits unrelated idea insertions and query normalization collisions; total ranking is not invertible.

## rarity_and_carry

Normalization is nerves::norm, shared query words count once. Kind priority precedes time; ctx comparison is exact text.

## failure_precedence

Missing/unreadable file becomes empty. Zero overlap drops an idea; k=0 returns none. Text/evidence/source tab/newline cleaning is asymmetric: kind/context remain raw and can break TSV. seed_principles uses snapshot membership so return count need not equal new unique records.

## resource_domain

read/write O(B); record O(B+N). related builds df O(W log V), scores intersections and sorts O(N log N), retains entire file. No locking, write contention/durability guarantee or overflow certificate for u64 score sums.

## effect_boundary

Isolated tempfile read/update/ranking and carried-over tests; no world seeding or production idea store access.


## tests

### Bounded evidence

Isolated tempfile actual record/read/rank; no real store

Names: ["ideas::wave28_dialogue::isolated_tsv_update_and_rank"]

Command: CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=ISOLATED_TARGET MOVA_HOT_ROOT=FRESH_TEMP_DIRECTORY cargo test -p world --lib --bin world [--release] wave28_dialogue -- --test-threads=1

The fresh temporary directory basename must start with wave28-dialogue-. Never point this fixture at a real hot root. The binary test calls private helpers; it never invokes main or state activation.

Nine selected cases passed twice debug and twice release with identical sorted named outcomes. Only the named source behaviors are exercised.

Control: mutant-ideas: overwrite preserved source. All four source mutants compiled and failed with exit 101; each was restored byte-for-byte and nine positive cases passed again. lib and main have no runtime mutant claim.

Six parent syntax projections were independently generated twice and compared. Child fragments reference parent syntax with exact byte ranges and hashes. Strict whole-world Clippy remains subject to existing warnings in untouched code. No live teacher, corpus, DB, model, GPU or school execution. No semantic proof or executable percent surrogate follows.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source-derived partial model; prospective hypotheses retained in external receipt. Isolated tempfile read/update/ranking and carried-over tests; no world seeding or production idea store access.

### 2026-10-31 · tests

Heading `### Bounded evidence — 2026-10-31` stays in tests as `### Bounded evidence`.

### 2026-11-05 · tests

2026-11-05: without MOVA_HOT_ROOT the test now prints a skip line and returns
instead of panicking, so `cargo test` without the isolated root does not touch
the shared hot root and does not fail. With MOVA_HOT_ROOT set the checks are
unchanged. evidence.json stays bound to the 2026-10-31 source hash (history).
