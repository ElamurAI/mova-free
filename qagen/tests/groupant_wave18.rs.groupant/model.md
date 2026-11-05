# qagen-groupant-wave18

## model

# Wave18 native joins

## forward

Wave18 question-generation integration reaches run retry resolution, report gathering and collection on synthetic Rule V1 supplied inputs. One join connects retry selection, accepted s1 answer and persisted ledger aggregates; another checks malformed-ledger and u64-overflow preflight errors preserve all three existing report artifacts; a Unix join propagates retry metadata ELOOP through gather despite a valid primary. Costs cover reached raw/ledger parsing, aggregation and output bytes, not transport or model inference. These are distinct supplied-input observations, with no live teacher, model, database or corpus operation.

## invariant

Synthetic Rule V1 only; one batch and sentence s1 remain aligned through raw resolution, gather and persisted QA. Valid retry takes precedence. Returned accounting equals serialized params for the valid fixture. Malformed JSONL and checked u64 overflow reject before modifying any of qa.tsv/rejects.tsv/params.json. Unix retry ELOOP propagates rather than silently selecting primary. No live acquisition or model is exercised.

## cost

Bounded temporary-directory creation/cleanup, raw/ledger reads, Rule V1 row parsing and report writes for one paragraph, one-row raw files and at most two call records. Work and storage scale with reached supplied bytes/records and output; error cases stop at failed preflight or metadata lookup. No inference, teacher transport, corpus traversal, whole-system timing or late-write atomicity claim.

## composition

Each named test is a separate tagged relation. Composition requires identical graph vertices, original edge IDs, byte offsets, exact integer domains, range endpoints and immutable version identities; Mova joins retain batch/source coordinates, raw-output preference and checked ledger observations. Distinct tests do not constitute one end-to-end pipeline. Independent finite or literal oracles remain distinct from cross-implementation equality.

## epoch-memory

# Wave18 independent integration harness prospective model

Graph join: canonical edge blocks partition input IDs; non-loop singleton blocks equal bridges. Convert a literal block forest into a DAG and count unique root paths. Parallel edge identities and singleton loops remain distinct. Ambiguity: many graphs share block sizes, so partition plus literal identity checks supplement counts. Tiny fixture costs are linear after block construction.

Text join: run encode/decode, feed decoded bytes to Two-Way search, independently enumerate slice equality, align original and decoded sequences. Needleman-Wunsch score is compared to existing Levenshtein distance under match=0,mismatch=-1,gap=-1. Independently reconstruct inputs and sum column costs. Empty inputs and zero bytes are included. No universal inference from finite fixtures.

Number join: segmented primes and Pollard factors checked against literal divisibility/product; floor_sum models counts of sorted wavelet values less than a threshold on a simple arithmetic progression. Persistent versions retain sums of snapshots and wavelet order statistics for each snapshot. Outputs do not uniquely reconstruct histories or all original permutations. Fixture costs bounded by small exhaustive loops, not performance measurements.

Qagen join: synthetic raw/retry and calls JSONL flow through public raw resolution, gather, calls_json and collect. Rule V1 avoids model loading. Distinct retry answers show precedence; malformed accounting and u64 overflow must preserve qa/rejects/params sentinels; retry ELOOP must propagate through gather with valid primary. No teacher, database, corpus or model. Runtime and memory linear in fixture bytes and calls. Reverse fiber: counts identify compatible accepted rows but cannot recover discarded text or call order. Concurrent writers and late-write failure atomicity remain untested.

Negative control: mutate alignment adapter mismatch score from -1 to 0, compile, and require edit_distance_join_alignment_score assertion failure on unequal literal texts. Restore exact harness bytes and repeat positive. This tests actual new API wiring, not an unconsumed proxy.

## Harness model specification for root sidecars

Mandatory behavior_complement first: the algorithm harness excludes biological interpretation, resource exhaustion, universal correctness, large-data performance and signed-to-unsigned conversion outside its nonnegative snapshot domain. Qagen harness excludes transport/model/database calls, filesystem races and rollback after an actual write failure. Assertions are only finite observations.

Mandatory reverse_reconstruction second: alignment columns reconstruct both byte inputs; decoded runs reconstruct original bytes only for canonical runs; factors reconstruct n via product; historical sums and wavelet multisets leave permutations and update histories ambiguous. Qagen retry identity plus row content identifies the chosen raw fixture, but aggregate accounting retains many compatible logs. Sentinel preservation reconstructs unchanged preflight outputs only for tested errors.

Specialized algorithm views: (1) edge partition plus singleton bridge correspondence with explicit self-loop exclusion; (2) block incidence orientation and source path count, including isolates; (3) byte codec/search offsets and alignment trace reconstruction; (4) primality/product factorization join; (5) floor-sum horizontal threshold-count identity; (6) persistent historical sums joined with wavelet kth/rank and independent sorted snapshots. Specialized qagen views: (1) retry file precedence joins accepted answer and persisted anchor ID; (2) call sums and retries join serialized params; (3) accounting errors preserve all three previous artifacts; (4) metadata failure is not silently reclassified as missing or primary fallback.

Named finite domains and costs: block_forest_join_path_counts uses 6 original vertices, 7 edges and 10 incidence nodes plus a 3-node parallel DAG. run_decode_join_byte_search_and_edit_alignment uses 6 original byte words, 36 ordered pairs, lengths <=7 and <=64 DP cells; sum O(nm+n+m+offsets) over this fixture. segmented_primes_join_factorization_and_literal_divisibility uses n=1..160 and interval [0,160], factors with budget 10,000, independent trial divisibility. floor_sum_join_wavelet_threshold_counts uses n=0..12,m=1..7,a,b=0..4 (2,275 combinations), maximum value 48, thresholds up to 60, wavelet construction O(64n) per combination. persistent_versions_join_wavelet_order_statistics uses 5 versions of length 5, all 21 half-open ranges per version and equality queries 0..10. Qagen uses one paragraph, one-row primary/retry, at most two calls per log, three sentinel outputs, two accounting error classes, and Unix ELOOP. No certified approximant metric is proposed; exact fixture comparisons are not approximants.


## checks

3 named tests pass twice in debug and twice in release. The compiled adapter control fails its named assertion, with exact restoration; repeated partial syntax agrees. model.md#evidence records final native and structural receipts. Macro bodies remain opaque; semantic AST and whole-system proof are absent.

## catalog

# Wave18 supplied-input relations

Source-derived specifications; bounded native evidence and repeated partial syntax are recorded in model.md#evidence. No general proof or complete inverse engine is claimed.

## behavior-complement

Type: behavior_complement. Named check: all three named synthetic run/report joins.

On the supplied synthetic Rule V1 directory states, let R relate run/report calls to their chosen raw path, accepted rows, ledger aggregates, errors and persisted bytes. Its complement includes choosing primary when a valid retry is present, losing the s1 sentence coordinate, serializing inconsistent counts or costs, replacing any sentinel artifact on the tested accounting preflight failures, or converting retry ELOOP into missing output or primary fallback. Observations stop at these named assertions. Teacher/model/database/corpus acquisition, concurrent directory mutation, crash consistency and rollback after a later output-write failure are outside this relation; preflight preservation is not a transaction guarantee.

## reverse-reconstruction

Type: reverse_reconstruction. Named check: all three named synthetic run/report joins.

A selected retry path plus accepted answer and persisted s1 coordinate retains compatible one-row Rule V1 raw fixtures; aggregates alone cannot recover discarded primary text, rejected rows, exact call order or unique call records. Totals 18 input tokens, 2.5 seconds and 0.25 estimated cost have many possible ledgers. A failed preflight with three unchanged sentinels retains directory states having those same initial artifact bytes and one of the tested accounting failures, not a unique full prior directory. Retry ELOOP identifies the error class in the supplied Unix fixture but does not reconstruct all filesystem links. These are compatible-antecedent fibers, not an inverse execution engine.

## retry-gather-ledger

Type: retry_selection_report_accounting_relation. Named check: retry_resolution_joins_gather_and_collect_accounting.

Domain is one supplied paragraph, batch 1 and sentence s1, one-row primary and retry files with distinct answers, Rule V1, and two call records for the same batch. try_final_raw must select retry_path, gather must accept exactly its retry answer, and collect must preserve that answer with the s1 coordinate in qa.tsv while omitting the primary answer. params must report one pair, two calls, one retry, 18 input tokens, 2.5 seconds and 0.25 estimated cost; on-disk params.json must equal the returned JSON. All row positions and batch/source coordinates are shared, and ledger values are exact supplied finite values. Cost is linear in these raw/ledger/output bytes plus bounded Rule V1 parsing and report aggregation. This invokes supplied-input public APIs, without a teacher or model.

## report-preflight-preservation

Type: accounting_preflight_artifact_preservation. Named check: accounting_failure_preflight_preserves_all_report_artifacts.

Domain is one valid primary raw row and preexisting qa.tsv, rejects.tsv and params.json sentinel bytes. In separate fixtures use malformed JSONL or two otherwise valid calls whose input token totals u64::MAX+1 overflow. collect must return an error and all three old artifacts must remain byte-for-byte identical. The first fixture checks parse failure; the second checks checked aggregation overflow. This relation requires accounting validation to precede report writes; it does not claim atomic multi-file replacement after output writing begins, filesystem race safety, or semantic QA quality. Work is bounded by reading/validating the supplied ledger and any preflight stages reached before failure; memory and output observations are limited to these fixture bytes.

## metadata-error-through-gather

Type: retry_metadata_error_propagation. Named check: retry_metadata_error_propagates_through_gather.

Unix-only domain is a valid primary raw row and a retry symlink referring to its own filename, causing ELOOP on metadata resolution. Both try_final_raw and report::gather must return errors. A retry metadata error must not be treated as absence, and the valid primary must not silently become fallback. Cost is bounded path/metadata attempts and the stages reached before this error; behavior on non-Unix platforms is outside this cfg-gated test. The fixture validates a particular error propagation path, not every permission error, symlink topology or concurrent filesystem state.

## evidence

### Wave18 question-generation evidence

Existing qagen/src/run.rs and report.rs gain file groupants and nine nested fragments. Three new supplied-input joins extend question-generation under school-support. Four qagen sources remain unregistered: fix, lemma, lib and main. Filename coverage is not full behavioral coverage.

Final root validation: 55 selected checks passed twice in debug and twice in release with identical normalized outcomes: 48 library checks, four wave17 joins and three wave18 joins. Every broad library run explicitly skips lemma::tests::doubt_with_en_model. Fixtures use synthetic raw/retry text, supplied calls and isolated temporary directories. No live teacher, parser model, database, corpus, training or school assessment ran.

Existing signatures are preserved and run::try_final_raw is added: strict metadata errors propagate through gathering and pending-work checks; final_raw remains an intentionally lossy compatibility wrapper. Token totals use checked u64 arithmetic; nonfinite floating accumulation errors; large finite values avoid decimal-scaling overflow. Retry metadata, call accounting, gathering, optional doubt computation and serialization now precede artifact writes. Malformed or overflowing ledgers leave prior qa.tsv, rejects.tsv and params.json intact. Later write failures and concurrent directory changes remain nontransactional.

Owner receipts record six old-code witnesses in both profiles and seven compiled production mutations followed by exact restoration. Root misrouted retry text to the primary filename; the named retry-path assertion failed, then restored execution passed. Independent review and a floating-point threshold probe found no further concrete defect in this scope. All-target qagen Clippy completes with dependency warnings; formatting passes. V1 fixtures test source/accounting consistency, not factual answer quality.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective design retained below; no speed or accuracy gain is claimed.

### 2026-10-30 · document

Date: 2026-10-30. Saved before harness implementation.

### 2026-10-30 · evidence

Heading `### Wave18 question-generation evidence — 2026-10-30` stays in evidence as `### Wave18 question-generation evidence`.

#### Wave18 integration scope — 2026-10-30

Base ae72362aaa65659aa444cb7d090a05636c0e0691. Inventory: 353/784 Rust files registered, 431 pending (348 public, 83 closed); 696 nodes, 16 groups, 327 fragments, 3,899 typed views, 1,196 labels and 343 partial syntax projections. Added 24 registered file entries: 20 new algorithm implementation/test files, two existing qagen sources and two new harnesses; 13 fragments and 223 views. The library export source also changes, giving 25 changed Rust files.

Declared groupants pass structural validation. Repeated syntax projections match byte-for-byte. Injected stale bindings and wrong child identities reject, then restore exactly. Root corrected two imported file-node child lists and 32 graph heading references found by integration validation. These are model-wiring repairs. Source binding and section validation do not prove semantics, expanded ASTs or a complete dependency closure.

Final receipts cover 550 algorithm checks and 55 focused qagen checks, each twice in debug and twice in release. Unrelated Mova subsystems and the whole school were not rerun. No numerical approximation tier is introduced. Complement models and compatible-antecedent models remain distinct, with explicit information loss.

Raw receipts: $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30; final native logs in native-final/. Prospective models and prior dated evidence remain. Question generation belongs to school-support; no inference-core or live acquisition pipeline is introduced.


##### Receipt bindings

External run: $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30. Native evidence index SHA-256: f85b3e074aa997321aff915acd6068610e33c9e50322525af7b6f529a7d7f422. Algorithm normalized outcomes: a307113f556e3e4dab5bf677df657899f439ffab39a1c8e1113ed23c97c1c733. Qagen normalized outcomes: 34efc5330be15215be0220412a9af2a517680394f975a7011f75a799a5210d21. Structural controls: a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Benchmark summary: 8ad77b5c224d73c393111ef5f6e14de3bb1d2b8108f5f01a89f90eee8d4a3c20.

##### Selected dependency snapshots

These reached production files are execution snapshots, not a complete semantic dependency closure.

- algorithms/src/bridges/mod.rs: 72204db4ba66993be2366219bc6aa00825548df331aa0dfd9d498729c3d4b6db
- algorithms/src/topological_sort/mod.rs: c0621a09327b1b3a07f8daadaa85d44a4e948f8b1013770b2a6be42d47787baf
- algorithms/src/miller_rabin/mod.rs: 0e6f4c3f59e549dc83106dbc98834b9eac28366202b094d60f67c288996f270b
- algorithms/src/gcd/mod.rs: abce5c9a9f0a2de26a6dbe392180e6a1c20eea179623fd5ede84434679b0d0e3
- algorithms/src/modular_power/mod.rs: be662de347e48f75c3a12b6238d7bca7d65114a1a79e4627723aab4e16299991
- algorithms/src/levenshtein/mod.rs: f762f7d12b5ee35141cf679db6723099c10e39b7a009c08e6b0337b743ccb724
- prag/src/opus.rs: 0169f72bddab55f9e493b85a86918b5f0adbcf5153c7c12725bb249f2ed5c136
- qagen/src/gate.rs: 09328f711abf4b6453f95bed3d873deb205c8801d939841974416fe74e3e5806
- qagen/src/schema.rs: 03e113198711324f029d8e9e4d9fc899416e3144ea856240b9d962775c78c2fe
- qagen/src/select.rs: 0c35caf4eda91c0a5d36bce728f3846b360b483a66fa4ca3c884964839cd2554
- qagen/src/run.rs: a34e297bdb69e178791983d8b9f6b35b1df34f223f15f1f13bc65d29153f2abd
- qagen/src/report.rs: 4abf85ffbd32f22802a89ef625780b288d92e92616c247a4d591e37fe10977c2

### Undated · evidence

#### Wave18 supplied-input native evidence

Source SHA-256 3ab5bd3bab4823dbf89e78ab1811e2286565e1899947ba24b247e3e8c0149813. Repeated partial syntax SHA-256 a9a074460b98277934e972dfdcf5bb1ac9d130dfb4d0fd46fff49f2c8345115b.
