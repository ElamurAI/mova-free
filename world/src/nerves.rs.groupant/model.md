# world-nerves

## descriptant

### world-nerves

norm filters Unicode alphanumerics and lowercases, drops stops/numbers before suffix singularization. build creates symmetric 1000-weight edges. spread seeds unique known words at 1e6 and repeats three steps: next[v]+=floor(floor(a*w/1000)/max(degree,1)), remove zeros, sort descending activation/id ascending, retain 4000; accumulate retained values with seeds.

## approximant

norm filters Unicode alphanumerics and lowercases, drops stops/numbers before suffix singularization. build creates symmetric 1000-weight edges. spread seeds unique known words at 1e6 and repeats three steps: next[v]+=floor(floor(a*w/1000)/max(degree,1)), remove zeros, sort descending activation/id ascending, retain 4000; accumulate retained values with seeds.

Exact categorical/effect semantics: no useful executable percent surrogate supplied. Future admissible caps 1/2/5/10% require defined metric/domain; no error bound claimed.

## invariant

Unknown/empty query seeds nothing. Seed multiplicity does not increase activation. Equal activation sorts by id. sentences sorts coverage, integer density, activation then lexical text, not length despite comment.

Cost: Build O(B log V+E log V); three sparse edge traversals plus O(R log R) sorting per frontier; top frontier 4000, total can exceed 4000. u64 products/sums and u32 ids require representability; allocation failures unmodeled.

## composition

Stops precede plural reduction, ASCII-only digit rejection, suffix byte lengths; many raw words collapse to one concept.

lib exposes modules; main activates state and dispatches. ideas uses nerves::norm and talk::now, store::write_atomic; talk uses babi splitting/prag Act. VMM uses lang::parse_all and world executor summaries before teacher continuation. These adapters preserve different coordinates (line number versus sentence anchor); splice acceptance does not establish execution validity. Synthetic two-node graph and normalization/manual boundaries only; no compiled manual-feature corpus, no production graph scale guarantee.

## models

## behavior_complement

spread cannot seed unknown concepts or count duplicate normalized query words twice. Each retained positive frontier is capped at 4000, while accumulated totals are not subject to that cap. Equal activation is ordered by node ID. Sentence ranking excludes a pure shortest-length tie rule: coverage, density, activation and lexical text decide in that order.

## reverse_reconstruction

A normalized word admits punctuation/case/plural variants; activation admits repeated query words and unknown additions. A graph hash is a lossy fingerprint, not unique reconstruction.

## normalization_projection

Stops precede plural reduction, ASCII-only digit rejection, suffix byte lengths; many raw words collapse to one concept.

## failure_precedence

Unknown/empty query seeds nothing. Seed multiplicity does not increase activation. Equal activation sorts by id. sentences sorts coverage, integer density, activation then lexical text, not length despite comment.

## resource_domain

Build O(B log V+E log V); three sparse edge traversals plus O(R log R) sorting per frontier; top frontier 4000, total can exceed 4000. u64 products/sums and u32 ids require representability; allocation failures unmodeled.

## effect_boundary

Synthetic two-node graph and normalization/manual boundaries only; no compiled manual-feature corpus, no production graph scale guarantee.


## tests

### Bounded evidence

Synthetic graph only; production manuals excluded

Names: ["nerves::wave28_dialogue::normalization_and_manual_boundaries","nerves::wave28_dialogue::synthetic_spread_units_and_dedup"]

Command: CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=ISOLATED_TARGET MOVA_HOT_ROOT=FRESH_TEMP_DIRECTORY cargo test -p world --lib --bin world [--release] wave28_dialogue -- --test-threads=1

The fresh temporary directory basename must start with wave28-dialogue-. Never point this fixture at a real hot root. The binary test calls private helpers; it never invokes main or state activation.

Nine selected cases passed twice debug and twice release with identical sorted named outcomes. Only the named source behaviors are exercised.

Control: mutant-nerves: seed units. All four source mutants compiled and failed with exit 101; each was restored byte-for-byte and nine positive cases passed again. lib and main have no runtime mutant claim.

Six parent syntax projections were independently generated twice and compared. Child fragments reference parent syntax with exact byte ranges and hashes. Strict whole-world Clippy remains subject to existing warnings in untouched code. No live teacher, corpus, DB, model, GPU or school execution. No semantic proof or executable percent surrogate follows.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source-derived partial model; prospective hypotheses retained in external receipt. Synthetic two-node graph and normalization/manual boundaries only; no compiled manual-feature corpus, no production graph scale guarantee.

### 2026-10-31 · tests

Heading `### Bounded evidence — 2026-10-31` stays in tests as `### Bounded evidence`.

### 2026-11-04 · descriptant

#### Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.

### 2026-11-05 · public export

The test time_words_reach_the_world_of_time is gated by cfg(mova_legacy_worlds) instead of feature worlds, matching CENTRAL_MANUALS (world/build.rs).
