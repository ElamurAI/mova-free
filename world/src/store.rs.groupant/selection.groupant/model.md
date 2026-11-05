# world-store-selection

## descriptant

# Query and set transitions

## Forward

Start with IDs 0..len, apply each term in order, read absent feature counts as zero. Metadata book equality filters, sample selects and sorts, prefix intersects a reordered block prefix. Saved sets carry query and model/order stamps. Stable/current stamps return parsed rows; stale sampled/prefix sets fully reselect. Eligible pointwise ancestor queries merge old-minus-changed with current-intersect-changed; unrelated stamps reselect.

## Composition

The carry-policy child chooses whether saved membership remains admissible. The candidate-coupling child explains why sampled output depends on the entire current ordered candidate vector. The ordering child constrains the fold that computes that vector. These share select, version indexes and saved query text; they are overlapping viewpoints. Correct sampling and a correct set merge do not prove their composition preserves fresh membership. Parent reverse fibers retain branch, index and intermediate candidate state. Parent complement is not the union of child complements.

## Invariant

Pure filters only remove candidates; sampling/prefix can change their order. Stable/current loads preserve parsed duplicate IDs and row order; ancestor merges sort and deduplicate. Refreshed sets persist and return one selection result. Query text and names are not escaped for tabs/newlines/path components. Invalid numeric saved lines are ignored, while invalid query numbers return errors.

## Cost

Each selection loads the full index; feature predicates build posting maps and scan candidates. Sampling costs O(n log n). Carry constructs changed IDs but still selects across the whole corpus. Recompute selects once then serializes, including prefix/model invalidation. Bounded calibration is recorded separately in ../model.md#current-evidence; no general speedup follows.

## Epoch memory

## catalog

# world-store-selection: typed model catalog

Domain: finite valid byte files, bounded IDs and successful allocation; an operation's arguments plus its initial filesystem and scheduler state are inputs. Observations include returns/errors and written bytes. No simultaneous writer, path traversal, malformed offsets or crash is assumed unless explicitly modeled. Source-derived conditional relation, not a proof of all Rust executions. Evidence: ../model.md#tests distinguishes executed probes from proposed oracles.

## nonbehavior

Type: behavior_complement.

U is well-formed finite store/query x ID vectors. U minus R includes a pure conjunction introducing an ID absent from initial 0..len and unknown-feature>=1 accepting any candidate. Candidate inner bound; all absent postings count zero. Probe exercises both >=1 and =0.

## antecedents

Type: reverse_reconstruction.

Given selected S for feature>=c, compatible postings assign each surviving candidate at least c and excluded candidates below c, with unconstrained rows outside candidates. For c=0 all candidates survive and no positive-count information is recoverable. Probe finite count threshold fibers.

## stamp-partition

Type: invalidation_partition.

Stable or exact-model stamps return parsed IDs without reselecting. order: stamps additionally compare daily key and base. Stale sampled/prefix stamps recompute and overwrite before ancestry discovery. Eligible pointwise ancestor stamps use union changed IDs; unrelated stamps recompute. Missing query defaults empty; malformed ID lines are ignored. Probe stable, same-model, ancestor and unrelated paths plus repaired sampled regression. Pointwise carry still requires membership stability outside changed IDs.

## sample-domain

Type: ordered_selection.

For fixed seed and ordered candidates, xorshift keys depend on position and IDs; sorted(key,id), take(k), then sort(ids) defines output. k>=len returns all IDs sorted. Swapping sample/filter can change size and membership. Probe searches a two-book fixture for a seed witnessing noncommutation.

## History

### 2026-10-29 · Epoch memory

2026-10-29 baseline: sampled tree eligibility changes invalidate unchanged IDs' sampled membership. The original probe recorded [] versus [2]. Current regression requires repaired [2]. The original update also ignored lemma/tag-only differences. The repaired identity gate closes that projection gap; pointwise carry remains conditional on complete successful deltas and preserved inherited lemma meanings. Fixtures do not invoke the annotator.

2026-10-29 identity epoch: the delta-closure child supplies the missing bridge from stored semantic equality through lexical features to pointwise membership. Sample/prefix invalidation still follows its separate coupling model. Full selection cost is unchanged; no index or parser completeness follows from the gate repair.

### Undated · document

Every entry is a separate observation/constraint. Status: partial source-derived conditional model. Subject: world/src/store.rs at SHA-256 6801677c4f17273e615090a790f2e4018387ca5d006aab899bafda09f01b24d9. Reverse operators retain all compatible antecedents; complements are candidate inner bounds, not complements of an empirically fitted forward function. Proposed oracles are not executed evidence.
