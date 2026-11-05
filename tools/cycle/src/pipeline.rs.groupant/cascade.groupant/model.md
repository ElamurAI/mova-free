# cycle-pipeline-cascade

## descriptant

### cycle-pipeline-cascade

#### Descriptant

Prospective inspection identified this relation before additions: Compare zipped head/relation rows by id; choose at most floor(n/2) positive-change blocks ordered by descending count then seeded FNV hash; fill from remaining hash-ranked blocks; restore original index order.

The production implementation is unchanged; only selected inline tests are appended. Source-bound syntax is generated twice and compared. Native checks exercise bounded helpers; entry orchestration, teacher calls, models and corpora remain source-derived. Partial syntax is not semantic proof. Preserve failure prefixes and ambiguity when refining this model.

## approximant

### Forward relation

Compare zipped head/relation rows by id; choose at most floor(n/2) positive-change blocks ordered by descending count then seeded FNV hash; fill from remaining hash-ranked blocks; restore original index order.

#### Composition

This fragment isolates the stated helper relation; parent composition retains surrounding effects and failures.

## catalog

# Typed observations: cycle-pipeline-cascade

## behavior_complement

n=0 yields no block; n≥N yields original recognized-id blocks in input order. A selected index cannot repeat.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## reverse_reconstruction

Selected block sequence admits arbitrary omitted blocks and compatible change/hash rankings. No unique draft or seed recovery; hash collisions preserve ambiguity.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## hard_quota

Only positive changes qualify for initial n/2. Fewer hard blocks shift remaining quota to hash sample.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## zip_loss

Missing draft id has zero changes; extra rows past zip length are ignored, so differing lengths need not rank hard.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## hash_ties

Wrapping FNV seeded with k gives deterministic scores; stable sorts preserve index order on exact equal keys. This is not uniform statistical sampling proof.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## output_order

Final numeric index sort restores below-file order regardless of selection rank; reverse relation must preserve this subsequence constraint.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## tests

### Checks and limits

pipeline::wave33_tests::cascade_count_order_and_zero covers zero/oversize, hard inclusion, order and determinism.

Exact native outcomes and command metadata are retained in the external wave33 cycle receipt bundle. Selected names run twice in debug and release; syntax is independently regenerated twice. Compiled mutations and source restoration receipts distinguish behavior changes from assertion-only controls. These claims are finalized in ../model.md#evidence. No live teacher, real model/corpus, benchmark CLI or lesson-learning CLI is run.

## invariant-and-cost

### Invariant and cost

Output contains min(n,N) input blocks, subject to allocation/usize feasibility; identity is block index, not unique id. Unmatched row suffixes do not count changed.

For B input bytes and maximum sentence-id length L_id, O(B+N log(N+1)*(1+L_id)+N min(n,N)): sort_by_key recomputes the id hash, and rest membership uses linear out.contains. IDs are not assumed to have unit byte length.

No executable percentage surrogate is claimed: effects, categorical acceptance and report text have no justified approximate metric here. Future profiles require a useful metric, zero case, reference and evidence at caps 1/2/5/10%.

## History

### 2026-10-31 · descriptant

Heading `#### Epoch 2026-10-31` stays in descriptant as `#### Descriptant`.

### Undated · document

Subject: tools/cycle/src/pipeline.rs, SHA-256 a24611a430e98b630e7f086c437036a1099978243ea80521ebad17ed44f9f8e4. Domain: bounded source inputs, declared filesystem/environment state and normal allocations. Each view is a partial source-derived relation, not a universal proof. Observation is one helper call or the explicitly described trace. Checks are in model.md#tests; unexecuted effects remain gaps.
