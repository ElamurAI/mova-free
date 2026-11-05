# world-store

## descriptant

# Tree-store groupant

## Forward

This partial model covers store.rs at the bound source hash. A store consists of fixed-width metadata, text bytes, version-parent rows, dictionary offsets, token records, feature postings, named sets and a mutable branch reference. Calls consume filesystem state and explicit arguments; build/update additionally consume annotator output, scheduling and time. Model the return, write trace and resulting bytes, including propagated errors. Malformed files may panic; parent cycles may fail to terminate.

## Composition

Publication supplies captured references to lookup; lookup resolves the nearest dictionary along parent links. Selection reads materialized full-version indexes, not records directly. Set invalidation fully reselects stale sampled/prefix queries and persists the same vector it returns. Eligible pointwise carry uses version dictionaries to restrict replacement membership but still calls full selection. Order reads metadata, cached order files and UTC day; prefix selection can mutate this cache. These are coupled through one directory and version names. Child source envelopes overlap and include selected index/words/identity functions; build and many other internals still lack detailed child models. These are views, not disjoint partitions. Their costs must not be blindly added. The root complement is relative to whole-call traces and is not the union of child complements.

## Invariant

Under admitted well-formed fixtures, dictionary IDs are sorted, offsets and lengths fit files, token lengths are multiples of eight, enum indices fit, versions are acyclic and safe names are single path components. These are assumptions required by readers, not validated storage invariants. Cached lookup returns the nearest explicit record; absence falls through parents. Branch-reference publication follows index generation in successful update, while versions.tsv publication precedes index generation. open's separate version-row and reference reads can capture old rows with new main during append-only publication; a coherent chain is an assumption. Pointwise set carry matches full reselection exactly when membership outside changed IDs stays fixed. Stale sampled tree queries reselect because candidate coupling need not satisfy that condition. Single-file rename does not make the group of files transactional or durable.

## Cost

Symbolic, unmeasured: build parses all admitted sentences; pending books and lemma maps can dominate memory. Lookup retains dictionaries and file handles; cold read cost includes all dictionary bytes visited. Parent search scans version rows per miss. Selection loads postings and scans current candidates per term; sampling sorts them. Set carry still performs full selection. blocks is O(n); shuffle O(blocks); prefix may allocate requested capacity even when the corpus is smaller. Bounded calibration is recorded separately in model.md#current-evidence; no general speedup follows.

## Epoch memory

The Rust probe extracts actual syntax-node source ranges, supplies a scalar decoder stand-in only for uncalled Store::tree, and calls real filesystem/query/set/order functions on temporary fixtures. It does not execute build, update, actual enums, model_hash, feature extraction, tokenizer or production corpora. Partial syntax has no macro expansion/name resolution/type inference. Counts are viewpoints, not proofs.

## catalog

# world-store: typed model catalog

Domain: finite valid byte files, bounded IDs and successful allocation; an operation's arguments plus its initial filesystem and scheduler state are inputs. Observations include returns/errors and written bytes. No simultaneous writer, path traversal, malformed offsets or crash is assumed unless explicitly modeled. Source-derived conditional relation, not a proof of all Rust executions. Evidence: model.md#tests distinguishes executed probes from proposed oracles.

## nonbehavior

Type: behavior_complement.

Let U be finite call/input/filesystem x return/final-filesystem observations; R is the source transition relation with dependencies fixed. A candidate inner bound of U minus R includes a successful records lookup returning ancestor payload bytes different from the nearest child's payload when that child dictionary contains the ID; a metadata-only stable set rewriting itself on unchanged load; and a successful ref_get returning leading/trailing whitespace. This bound excludes unknown I/O/crash schedules. Oracle: conflicting child/base fixture, byte-preserved stable load, whitespace reference.

## antecedents

Type: reverse_reconstruction.

R^-1(y)={x: R(x,y)} retains all input files, args, unobserved ancestors and prior overwritten reference bytes compatible with y. A returned tree constrains only the first matching dictionary on the version chain; a reference string determines trimmed content, not original whitespace. Oracle: distinct whitespace refs and distinct unused parents have the same observation.

## publication

Type: publication_order.

For successful update the actual write order is trees, dict, lemmas, append versions row, index_full, branch ref. build likewise appends versions before index. Contrary to nearby comments, a versions reader may see an unindexed version; only branch-ref publication is last. Oracle proposed: fail index creation and inspect the earlier versions row; not executed because update requires annotator.

## atomicity

Type: failure_boundary.

write_atomic writes path.with_extension(tmp-PID) then renames it. On a single-writer same-filesystem POSIX success the pathname switches to completed bytes; neither file nor parent directory is fsynced. No power-loss durability or multi-file transaction follows. Same-process same-stem extensions share a temporary path. Oracle: replacement and rename failure fixture; concurrent collision remains untested.

## paths

Type: admission_constraint.

ref name and set name are joined without normalization or confinement. Empty, slash-containing, absolute or parent-relative strings are not validated here. Model domain restricts names to one safe component; this is a caller obligation, not an enforced protection. Oracle proposed: inspect join results without writing outside scratch.

## layout

Type: serialization_projection.

Meta uses 24 bytes, dictionary 16, token 8 and posting 5; integers are little-endian. len floors meta bytes/24; dictionary loops floor bytes/16, but nonmultiple token lengths can panic. Casts truncate sentence/head/lemma widths (lemma24). Decoder enum indexes can panic; tag alone falls back to NN. Oracle: records bytes and meta-tail fixture; enum limits remain inspected.

## snapshot

Type: state_projection.

Store::open captures meta, versions and main once; dictionaries/file handles populate a mutex cache lazily. latest prefers any nonempty main ref even if absent from versions. New refs do not update an existing handle. open reads version rows before main: an append-only concurrent update can interleave these reads so old rows coexist with the new main, preventing parent fallback for an inherited ID. Lazy file opens introduce further snapshot limitations under overwrite. Oracle: old/new handle ref comparison executed; interleaved-open failure source-derived, not executed.

## delta-identity

Type: information_loss.

The repaired identity gate compares token count, UPOS, relation, head, tag and parent-resolved normalized lemma. Forms/feats remain outside the stored projection. Historical structural-only equality missed lemma/tag differences; that information loss is a rejected baseline. Native helper/decoder fixtures with independent test packing establish bounded correspondence; full update still depends on successful reads, table extension, annotation and publication.

## set-carry

Type: membership_transfer.

Current/stable stamps retain parsed rows. Stale sampled or prefix queries recompute full selection; order stamps refresh on order/base mismatch. Only eligible pointwise ancestor queries merge (old minus C) union (select(new) intersect C), sorted and unique. Merge equality with fresh membership holds iff old minus C equals fresh minus C. The historical sampled mismatch is now a regression expecting [2]. The identity repair now detects normalized lemma/tag differences; carry completeness additionally requires successful reads, complete deltas and inherited lemma-ID meaning preservation. Native synthetic fixtures and abstract closure laws do not verify whole update.

## query-order

Type: noncommutative_composition.

Terms apply left to right. Feature and book filters retain candidate order; sample chooses from the current candidates then sorts; prefix replaces order with global block order intersected with current candidates. sample and filter generally do not commute. The documented book<N has no metadata implementation and a bare < term fails operator parsing. Oracle: sample=1 versus later book filter.

## ordering-key

Type: cache_identity.

order key hashes sentence count, books.tsv length, first version and UTC day; size is absent. A valid cached order ignores the new size argument; equal key does not imply equal corpus content. Prefix takes whole blocks until ceil(n*pct/100) is reached, so may overshoot. Oracle: size1 then size400 and prefix-one-percent fixture.

## parallel-build

Type: schedule_dependence.

Book output is drained in book index order, but lemma IDs are allocated at first arrival under a mutex and limit admission checks a shared count once per book. Parallel scheduling can alter lemma bytes and admitted books; limit can overshoot by a whole book or concurrent books. Worker flush errors are discarded. No unconditional deterministic-build or strict cap claim. Oracle proposed: controlled scheduler and injected sink failure.

## tests

### Bounded store evidence

#### Reproduction

From this sidecar, use an isolated CARGO_TARGET_DIR and run:

    cargo run --offline --bin generate
    cargo run --offline --bin extend
    cargo run --offline --bin finite
    cargo run --offline --bin ordering-oracle
    cargo run --offline --bin probe
    cargo run --offline --bin probe
    cargo run --offline --bin probe -- --negative-control

The last command must fail. Also run probe -- --negative-sample-control, finite -- --negative-control and ordering-oracle -- --negative-control; each must fail. extend adds the three nested selection manifests after generate. finite checks exact finite algebra; ordering-oracle models sample/filter mathematics without calling production. generate reads the sibling authoritative store.rs and syntax.json, copies exact selected item ranges into probe.txt and writes model manifests/catalogs. oracle.txt supplies the independently stated expectations. Run modelcheck syntax twice with fresh output paths and compare before regenerating on source changes. The generator intentionally leaves human descriptants/tests untouched.

Probe creates a fresh PID-named temporary directory with exclusive create_dir, writes only there, and removes it on unwind/exit. It never opens active store, cache, model or configuration paths. Store::tree has an explicit scalar decoder stand-in and is uncalled; all called store methods and functions are extracted without edits. Dependencies are anyhow and serde_json, not the production parser.

#### Evidence scope

No full world tests, parser/training, build/update, crash recovery, power-loss durability, live concurrency, corruption fuzzing or performance measurement is claimed. Proposed catalog oracles remain prospective. The false model asserts prefix output equals the exact one-sentence budget, which the actual whole-block implementation must reject.

#### Repaired selection epoch

The finite four-ID oracle passes 4096 old/fresh/changed triples, 65536 candidate-output relation/complement/converse comparisons. Exactly 2800 triples violate the false universal legacy-carry equality. Two summaries are identical; the false-equivalence control fails with 2800 versus 0.

The independently authored standalone ordering oracle models the scalar sampling formula; it does not call production source. All 64 old and 64 new eligibility masks over six IDs, k=0..7 and seed=0..7 give 262144 cases. Sample-before-feature has zero legacy carry failures; feature-before-sample has 40928. Two summaries are identical. The false order-independent carry control fails with 40928 versus 0. This bounded evidence supports conservative invalidation without proving a minimal syntactic policy.

Two fresh syntax generations are byte-identical, with 5394 nodes over 45564 source bytes. All local scopes/manifests were regenerated. This sidecar now holds eight nodes and 41 typed models: the pre-existing five nodes/28 models plus carry policy (five), candidate coupling (four) and operator ordering (four). Local structural validation reports no store-sidecar diagnostics; the two parent source-binding diagnostics are delegated to integration. This is partial syntax without semantic analysis, not a proof of runtime behavior.

#### Abstract Lean law

selection.groupant/carry.groupant/SampleCarry.lean proves carry equals fresh iff predecessor and fresh agree outside changed IDs, equivalently their symmetric difference is a subset of changed IDs. It also proves empty/full changed-set cases, the full compatible-predecessor relation, and a concrete refutation of unconditional equality. Reverse recovery leaves predecessor membership inside changed IDs free.

Executed with Lean 4.35.0-rc3 using the existing Mathlib environment: lake env lean PATH_TO_SIDECAR/selection.groupant/carry.groupant/SampleCarry.lean. Both runs passed with identical axiom logs. The three audited principal theorems use propext, Classical.choice and Quot.sound; no sorryAx. NegativeControl.lean is intentionally failing and must be compiled separately; its false unconditional equality failed with exit 1. No dependency download or full proof-suite run was performed. This verifies abstract set mathematics only; Rust correspondence, filesystem validity, delta completeness and sampling semantics remain separate obligations.

#### Integrated source and timing evidence

The inference/root bindings have been refreshed and the integrated structural report has no diagnostics. Native world tests, source-mutant results and a separate CPU timing experiment are recorded in [current-evidence](model.md#current-evidence). The finite oracle retains only substantive merge/complement/converse comparisons; a tautological recomputation assertion was removed during integration and the revised oracle was rerun.

#### Semantic identity and feature closure epoch

The older extracted probe still has a scalar decoder stand-in and does not exercise actual tag decoding, same_stored_tree, annotation or update. Its success cannot serve as evidence for those paths. Native source tests and their bounded fixtures are reported separately; mathematical closure does not establish full-update correspondence.

selection.groupant/closure.groupant/SemanticDelta.lean proves that complete semantic-projection deltas suffice for every pointwise-query carry, and conversely that universal pointwise carry implies completeness. Its structural fiber leaves tags and resolved normalized lemmas unconstrained; concrete tag-only, lemma-only and carried-membership counterexamples reject the coarser model. The premise is the full stored projection, not arbitrary Word equality. The proof also does not claim that every production feature query distinguishes every semantic field: actual feature projection is coarser.

To reproduce Lean checks, copy carry.groupant/SampleCarry.lean and closure.groupant/SemanticDelta.lean into fresh scratch. In the existing Mathlib environment prepend scratch to LEAN_PATH and compile SampleCarry first with lean -R SCRATCH -o SCRATCH/SampleCarry.olean SCRATCH/SampleCarry.lean, then SemanticDelta likewise. Compile WeakenedNegative.lean separately and require failure. Two positive runs produced identical axiom logs; the weakened premise was rejected because structural completeness does not discharge full completeness. This establishes abstract laws only, with no dependency download or Rust correspondence proof.

Two fresh syntax projections are byte-identical: 6106 nodes over 52547 bytes. Eleven local manifests and 53 typed entries bind the repaired source, including expanded parent scopes for new nested children. Groupant validation has no store-sidecar diagnostic; only two external parent hash bindings await integration. Changed-source registry check reports world/src/store.rs with no missing entry. The older extracted probe again passes 62 assertions twice with identical output and unchanged extraction; its decoder limitation remains explicit.

Final integration refreshes inference/root bindings and registers the downstream derivation source. [Semantic delta evidence and timing](model.md#delta-evidence) records native tests, test-only cleanup and the actual predicate comparison against a correct decoded-vector reference. This bounded wall-time experiment does not measure full update, annotation or school performance.

## current-evidence

### Current sampled-set evidence

#### Behavior and source tests

For pred>=1 sample=1/2, eligibility [0,1,2] becomes [0,2], with exactly ID 1 changed. The saved sample [1] must become [2]. The baseline carry returns []; the repaired source reselects once and returns/persists [2]. Recompute uses one select call for coupled, unrelated and invalidated order-stamp paths. Stable/current shortcuts remain; pointwise ancestor carry still assumes deltas cover all feature changes.

Native gate: cargo test --offline -p world --lib set_carry_tests, using an isolated CARGO_TARGET_DIR. Baseline had two failures and one pass; final source has four passes. Disabling the new coupled-query guard causes three failures; restored source passes four again. Matrix coverage is four predicates by four cardinalities by four seeds by four term patterns: 256 sampled queries, each compared with full selection and persisted reload, plus five pointwise queries. Direct/transitive/unrelated versions, unchanged snapshot bytes and legacy-prefix output order have dedicated controls. Two serial native runs have the same test order/status; harness times differ.

The extracted-source probe passes 62 assertions twice with identical diagnostics. model.md#tests records its scalar decoder stand-in and excludes uncalled parser paths. The four-ID relation oracle checks 4096 triples and 65536 output/complement/converse pairs, rejecting 2800 false universal-equality cases. The independent six-ID sampling model checks 262144 cases; 40928 expose feature-before-sample failure of historical carry. Abstract Lean laws and a failing proof control are retained in the carry fragment. These evidence layers do not constitute a proof of all production executions.

#### Prospective cost and bounded runtime check

Before timing: T_old = selection + ancestry/delta reads + set merge + persistence; T_new = selection + persistence for an outdated sampled set. The old carry already ran full selection. Removing its subsequent merge and ancestry reads can save work; no percentage was predicted. Selection remains corpus-sized, with sample sorting O(M log M).

Median elapsed wall-clock milliseconds for 60 set_load calls per cell, three rounds. The workload runs on CPU; these measurements include filesystem work and are not process CPU time:

| N | changed IDs | before ms | after ms | delta |
|---:|---:|---:|---:|---:|
| 128 | 0 | 2.425 | 2.253 | -7.1% |
| 128 | 1 | 1.390 | 0.984 | -29.2% |
| 128 | 128 | 1.630 | 0.987 | -39.5% |
| 2048 | 0 | 6.474 | 6.345 | -2.0% |
| 2048 | 1 | 6.473 | 5.925 | -8.5% |
| 2048 | 2048 | 13.475 | 5.887 | -56.3% |

Three alternating rounds per executable, eight warmups and 60 measured calls per cell. Each iteration restores the old saved set outside its timer. Full ID vectors agree between executables and persisted bytes equal the expected target-model file on every call. Synthetic unchanged postings make the old implementation correct on these timing fixtures; the separate regression checks the defect. Small timings include shared-host and filesystem noise; no confidence interval or general speedup is established. The selection function, file I/O, ancestry reads and set merge are included; the parser is absent. No school, GPU, teacher, concurrency or crash experiment was run.

## delta-evidence

### Semantic delta evidence

#### Observed defect and repair

The old gate compared only UPOS, relation and head. A new lemma or XPOS tag could be discarded before indexing, leaving both fresh selection and carried sets on the old annotation. The new helper compares raw records with candidate words using all stored semantic fields and immutable parent lemma resolution. Candidate lemmas use the same Rust lowercase operation as the writer. Missing parent IDs conservatively force a delta. Forms and FEATS remain outside the stored format.

This changes future updates. Existing snapshots are not rewritten, and same-model early return still bypasses annotation. Stable dense unique parent lemma mappings, valid enum bytes and encoding-width bounds remain assumptions. Read-error skipping, writer truncation, publication ordering and concurrent snapshots are not repaired here.

#### Rust and model checks

Eight native store tests passed twice: four existing sampled-set tests plus four new identity tests. The structural-only baseline failed all four new tests. Removing tag equality caused two failures; removing lemma equality caused three. The restored source passes. Final fixture ownership cleanup passed the four new tests and acquires cleanup ownership only after successful exclusive directory creation.

The integration fixture invokes the actual identity helper, inherited raw-record lookup, production index_full, select and set_load. Its byte packing is independently implemented; it does not call the production encoder, annotator or full update loop. It demonstrates existing/new lemma changes, tag retention and lexical query transitions. Separate helper cases cover normalization, missing IDs, token lengths and structural fields.

The extracted store probe still passes 62 assertions; its decoder stand-in is confined to that older probe and supplies no evidence for the new identity helper. delta-finite models 64 semantic pairs, 262144 three-version chains and 524288 pointwise cases; a weakened structural-only delta premise gives 63488 failures. Lean proves abstract completeness implies pointwise query agreement and the existing carry law, plus a necessary condition for all pointwise queries. The intentionally weakened premise is rejected. Neither formal proof nor finite model certifies production serialization or feature extraction.

#### Bounded predicate timing

Prospective model: fused raw-record comparison avoids a decoded tuple vector; tag and normalized lemma checks add work relative to the incorrect old gate. Retaining parent lemma strings also adds one map-construction clone, outside this timing. No speedup percentage was predicted.

Actual helper plus decode/decode_tag were extracted from source-bound syntax and compiled at opt-level3 against the same existing real en dependency artifact. This is a pure helper experiment, not a fresh release-world build. Eight/128-token ASCII, Unicode and late-mismatch fixtures use 1000 warmups and 20000 measured calls; literal Boolean expectations are checked for both implementations. black_box prevents dropping the measured calls. Three rounds alternate reference/candidate order.

Median elapsed wall-clock milliseconds for 20000 calls:

| tokens | case | reference ms | fused ms | delta |
|---:|---|---:|---:|---:|
| 8 | ascii | 3.559 | 3.369 | -5.3% |
| 8 | unicode | 5.815 | 5.424 | -6.7% |
| 8 | late-change | 2.195 | 1.911 | -13.0% |
| 128 | ascii | 30.047 | 28.738 | -4.4% |
| 128 | unicode | 86.424 | 85.957 | -0.5% |
| 128 | late-change | 30.433 | 28.969 | -4.8% |

The comparison is against a correct all-field reference, not the former structure-only predicate. Medians show 0.5–13.0% less elapsed time in these six fixtures; the 0.5% difference is especially sensitive to noise. This is not a full update or school speed measurement. It excludes parsing, disk reads, map construction and extra parent-table memory. On the shared host three rounds do not establish a calibrated confidence interval. Final test-only cleanup did not change any timed function; extracted bodies were compared byte-for-byte.

## History

### 2026-10-29 · Epoch memory

2026-10-29: source inspection corrected the stronger publication comment: versions rows precede both base and delta index generation. It also identified a weaker update equality (upos/rel/head only), cache key omission of requested block size, and schedule-sensitive lemma allocation/limit admission. Production code is unchanged. These limitations qualify the top-of-file nonblocking/deterministic descriptions.

2026-10-29 repair epoch: the historical sampled-carry counterexample is retained in model.md#tests, while the active oracle requires full [2]. Selection now has three nested child views: carry policy, candidate coupling and ordered query composition. Abstract finite relation checks keep feature/index correspondence conditional. Source syntax and scopes are regenerated against the repaired implementation; semantic ASTs and automatic combining remain absent.

2026-10-29 identity repair epoch: lookup now has nested stored-identity and lexical-feature projections; selection adds delta completeness across ancestry. The historical structural-only gate is repaired to compare parent-resolved normalized lemma and tag as well. The updated model distinguishes omitted Word forms/feats, stored semantic fields and the coarser feature map. Historical observations above remain dated; this repair does not verify full update. Worst-case identity work grows from O(T) to O(T+B) normalized lexical bytes with short circuiting; full index rebuild and full-selection carry remain unchanged.

### 2026-10-29 · tests

2026-10-29: executed results are recorded below. Expectations cover reference trim/error fibers, rename replacement/failure, temporary-name aliasing, metadata floor length, captured main, child precedence and parent inheritance, empty/missing records, positional text/place, threshold selection, missing postings, unsupported book< syntax, sample/filter noncommutation, tree-dependency classification, stable/current/ancestor/unrelated sets, block conservation and boundaries, cached size omission, prefix overshoot and sparse lemma overwrite.

#### Executed results

2026-10-29, unchanged production source at baseline 1e63aaf0a: two probe runs each passed 56 assertions with identical diagnostic/summary text. The negative-control run failed with observed prefix length 3 versus the false expected 1. Both syntax generations compared byte-identically. Groupant structural validation returned valid=true with no diagnostics (26 file sidecars, seven group sidecars in this worktree). These totals include pre-existing models; this sidecar contributes one root with 12 typed entries and four fragments with four entries each, 28 total. Source-derived views beyond the executed probes remain partial.

Review refinement, 2026-10-29: the extended probe passed 59 assertions twice, with byte-identical combined output. Synthetic postings remove only ID 1 from eligible [0,1,2], and the delta dictionary identifies exactly ID 1. For `pred>=1 sample=1/2`, the saved sample contains ID 1; full reselection returns [2], but real set_load carry returns []. This demonstrates candidate coupling even with exact eligibility deltas; no feature-extraction or annotator correspondence is claimed. The existing false prefix-budget control still fails (3 versus 1), and structural groupants validation still passes. The append-only open interleaving is source-derived/proposed, not a concurrency experiment. Lookup complements now distinguish unequal payloads rather than unobservable ancestor provenance.

2026-10-29, source commit 60ac7363c, SHA-256 aa516b096f4e83e75e8996b0632e0c162d9b0e4610438719b63a3efebe6903c0: the extracted actual-source probe passes 62 assertions twice, with identical combined output. The old sample witness now requires query pred>=1 sample=1/2 and returned [2], checks a repeated current-stamp load, and retains the historical [] result only as a rejected baseline. The prefix-budget control still fails (3 versus 1); the new historical-sample control fails ([2] versus []).

Final test-fixture refinement, 2026-10-29: source commit 467f3ceaa removes a midnight rollover assumption from the native legacy-prefix test without changing production selection. Current source SHA-256 is 23b6c720b97cb535aca1eaaf4086b0aa690c5f291e3f3b72fa5f97bb8d517c1a. Two fresh syntax projections agree byte-for-byte: 5421 nodes over 45860 bytes. All eight local bindings/scopes are regenerated. The extracted source probe again passes 62 assertions; its production extraction is unchanged. The coupling converse now explicitly includes the final numeric ID sort. Earlier source-bound measurements above remain historical observations; current bounded timing is recorded separately in model.md#current-evidence.

2026-10-29: three nested views add stored identity, lexical feature projection and complete pointwise deltas. Run delta-extend after generate and extend, then delta-finite twice; delta-finite -- --negative-control must fail. The finite oracle is an independent semantic model, not byte encoding or production execution. It enumerates 64 token pairs (24 structural-only blind pairs; 16 unequal full records with equal features), 262144 two-ID three-version chains and 524288 pointwise membership/carry cases. It also checks four compatible predecessor sets for each case. Both runs printed identical summaries. The rejected structural-only completeness model fails in 63488 cases, and its negative control fails with 63488 versus zero.

Source repair 4cd54fb75, SHA-256 4745446cb22eeb0d31d095e7b1452eac9bd8a26f5f1796da9d48ee66c3b06fd3: native store scope passes eight tests, including four new identity tests. Original structural-only decision fails all four new tests. Omitting tag comparison fails two of four; omitting lemma comparison fails three of four. Fixtures exercise the real identity helper, decoder/tag decoder, inherited lookup, full feature index, selection and carry, using an independent test record writer. They do not execute the production encoder, parser or whole update. This distinction prevents a decoder test from being reported as an encoder round trip.

Final fixture ownership refinement, 2026-10-29: commit fe025c938 creates its temporary directory before acquiring cleanup ownership, preventing deletion of a preexisting directory if exclusive creation fails. The four identity tests pass after this test-only change. Final source SHA-256 6801677c4f17273e615090a790f2e4018387ca5d006aab899bafda09f01b24d9; two fresh syntax generations agree byte-for-byte with 6108 nodes over 52570 bytes. All eleven local manifests/scopes and 53 typed model bindings are refreshed. Structural validation again reports only the two external parent bindings awaiting integration. Production identity and the extracted probe remain unchanged.

### 2026-10-29 · current-evidence

Heading `### Current sampled-set evidence — 2026-10-29` stays in current-evidence as `### Current sampled-set evidence`.

Baseline 7d744cbbe; source change 60ac7363c, integrated as 266c286b0. Timed store.rs SHA-256 aa516b096f4e83e75e8996b0632e0c162d9b0e4610438719b63a3efebe6903c0. A subsequent test-only daily-order rollover guard (467f3ceaa, integrated as 364e27c4f) gives final source SHA-256 23b6c720b97cb535aca1eaaf4086b0aa690c5f291e3f3b72fa5f97bb8d517c1a; production functions are unchanged.

Artifacts live under $MOVA_DATA/runs/octopus-sample-integration-2026-10-29. benchmark.rs.txt contains the isolated fixture, benchmark-build.rs.txt builds each extracted source version, and benchmark-run.rs.txt alternates and summarizes the executables. Compile the build driver with rustc, then pass RUN_DIR, before/after, the corresponding probe.txt and the same anyhow rlib path. Copy benchmark.rs.txt into RUN_DIR under that name first. Compile/run the runner with RUN_DIR after building both versions. Historical probe.txt requires a baseline checkout. Both timed sources used rustc -C opt-level=3 and the same existing anyhow dependency artifact; this is not a fresh release-world build. Exact selected production function bodies come from each source-bound syntax projection; the unused decoder is stubbed.

### 2026-10-29 · delta-evidence

Heading `### Semantic delta evidence — 2026-10-29` stays in delta-evidence as `### Semantic delta evidence`.

Baseline 47188b0a2. Production change 4cd54fb75, integrated as ab10c8bbd; test cleanup fe025c938 integrated as 0b003d7b9. Final store.rs SHA-256 6801677c4f17273e615090a790f2e4018387ca5d006aab899bafda09f01b24d9. The timed source preceded only the fixture ownership change; its helper and decoders are unchanged.

Raw artifacts: $MOVA_DATA/runs/octopus-delta-integration-2026-10-29. Retained delta-benchmark.rs.txt is the fixture/reference; delta-benchmark-driver.rs.txt extracts, builds and runs it. Compile the driver with serde_json, then provide RUN_DIR, REPO and EN_RLIB_PATH. Put the fixture in RUN_DIR under its retained name and generate RUN_DIR/store-syntax.json from the same source with mova-modelcheck syntax first. Use an isolated run directory. No parser models or live store are accessed.

### Undated · document

Every entry is a separate observation/constraint. Status: partial source-derived conditional model. Subject: world/src/store.rs at SHA-256 6801677c4f17273e615090a790f2e4018387ca5d006aab899bafda09f01b24d9. Reverse operators retain all compatible antecedents; complements are candidate inner bounds, not complements of an empirically fitted forward function. Proposed oracles are not executed evidence.
