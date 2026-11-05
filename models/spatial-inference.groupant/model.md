# spatial-inference

## descriptant

# spatial-inference

## epoch-memory

The transfer reuses indexed adjacency while retaining a richer state than vertex-only reachability. It must preserve every reachable qualification state because an early short path may not establish Far and a later path may. Small scenes and absent-start queries justify a scan/preflight hybrid; index construction is charged per actual query. The source-local test module supplies an observation seam for private methods, while the story-reading boundary remains an explicit gap.

## model

# spatial-inference

## forward

Supplied immutable Scene queries first retain irreflexive and block-inference priorities, then inspect direction facts. Small fact sets (E<=64) keep scan-based traversal. Larger sets perform an O(E) outgoing-edge preflight for the requested start/relation; an absent start returns the exact no-path observation, while present starts build direction rows for traversal. reach computes boolean reachability; dir_path explores (object, far_seen, steps_capped_at_2, all_edges_qualified) and ORs evidence from every nonempty path to the target. near/far and tv compose these observations with direct facts and block state under their existing precedence.

## invariant

Rows contain precisely the edges admitted by the requested relation and preserve the scan relation. Missing outgoing edges cannot contribute a nonempty path. dir_path returns None iff no nonempty directional path exists, Some(false) iff paths exist but none witnesses far_seen or at least two fully qualified steps, and Some(true) iff at least one such witness exists. A shorter unqualified path cannot suppress a longer qualifying path. Holds remains irreflexive; dir_path may observe a nonempty cycle.

## cost

Let E count all facts, V_all all distinct fact endpoints, and V_reached reached vertices. For E<=64, scanning retains old per-state fact scans. Larger missing-start queries pay O(E) preflight without rows. Present-start queries add O(E log(V_all+1)) ordered-row construction, row lookups O(log(V_all+1)), visited-state operations O(log(V_reached+1)) and qualifier checks O(log(E+1)). At most 12 states per object are explored; per-query rows are rebuilt and far can invoke four direction searches. Reader/block/non-inference work remains separate.

## composition

The actual joins are directional fact projection -> reach/product-state traversal -> near/far/holds -> three-valued query projection, with direct-fact and block priority retained at the callers. The inference_tests source calls private scene methods and compares against a retained scan reference and bounded fixed-point/product-state observations. These supplied-scene tests do not run the story reader or establish prose-to-scene correctness. No generic shortest-path result substitutes for qualified all-path evidence.

## checks

["Source-local inference differential and bounded product-state checks","Threshold, missing-start, nonempty-cycle and longer-qualified-path fixtures","Block-priority and holds-irreflexive observations","Repeated focused checks, negative controls and paired syntax: see model.md#evidence"]

## limitations

Hybrid thresholds and outgoing preflight must match merged sources. Scene reader, descriptions, query parsing and other legacy module behavior remain partially modeled. Finite differential/fixed-point checks and partial syntax are not a general semantic proof. Contradictory direct facts and block observations follow existing priority rather than a newly imposed consistency axiom.

## catalog

# Typed transfer relations

Source-derived models; native evidence pending.

## excluded-path-observations

Type: behavior_complement.

Within supplied lawful scenes, excluded observations include a directional path absent from projected facts, reporting None when a nonempty path exists, returning Some(false) after a longer Far/fully-qualified witness is reachable, and treating the zero-edge x-to-x walk as a dir_path witness. At the caller, holds(x,r,x)=true and an object-fact answer overriding an earlier applicable block inference are excluded under the existing precedence.

## compatible-scenes

Type: reverse_reconstruction.

For a tagged query result retain all scenes, facts, object/block attributes and reachable product-state witnesses producing it. Some(true) from dir_path can arise from an explicit Far edge or a sufficiently long fully-qualified path; these witnesses are not uniquely recoverable. None constrains directional reachability, not unrelated facts. tv erases witness provenance and can merge different direct, block and inferred causes.

## product-state

Type: product_state_obligation.

The visited key is (object, far_seen, min(steps,2), all_edges_qualified). Merging visits by object alone loses distinctions needed for later witnesses. Qualification is conjoined along edges; far_seen is disjoined. State transitions use exact fact membership in either orientation for Near/Far/Touch qualification, and termination follows from the finite product-state space.

## all-path-aggregation

Type: all_path_projection.

The target observation ORs all qualifying witnesses instead of stopping at first arrival. A direct unqualified edge gives a path but may produce Some(false); a later longer Far or fully-qualified route upgrades the result to Some(true). Shortest-path length alone cannot determine this observation.

## dispatch-preflight

Type: resource_fallback.

E<=64 keeps the scan traversal. For larger E the outgoing preflight examines the requested start and relation before allocating indexed rows; absent outgoing edges return the operation's exact no-path result. Present starts pay index construction even if little else is reached. This dispatch is a resource policy over one relation, not a new inference rule.

## nonempty-and-irreflexive

Type: boundary_non_equivalence.

dir_path(x,r,x) may return a value when a nonempty cycle exists, while holds rejects x==y before inference. A zero-length reachability convention from a generic graph library cannot be applied blindly to either API. Tests must preserve operation tags rather than equating all path predicates.

## block-before-fact

Type: stage_priority.

holds and its query callers retain the established ordering of block-derived inference, object facts and inverse checks. A correct local adjacency result does not authorize bypassing an earlier block decision. Three-valued None is unknown under these rules, not proof that the opposite relation holds.

## evidence

# spatial-inference evidence

## checks (evidence)

See models/mova.groupant/model.md#transfer-evidence, transfer-benchmark.md and transfer-validation.md for the combined receipts. Source revisions are bound by this manifest and paired partial syntax. Finite checks do not establish universal equivalence, floating-point optimality, parser quality or whole-system speed.


## Restored wave25 checks

The lazy-index candidate was rejected on measured regressions. The exact fd540 production and test bytes were restored, then all 12 spatial checks passed twice in debug and twice in release with identical normalized observations. Candidate 13-check and performance receipts remain separate in model.md#wave25-rejected-performance and the external worker run. No candidate speed claim applies to this retained implementation.

## wave25-rejected-performance

### Rejected world candidate — decision

The 34-test candidate native evidence is preserved as candidate evidence. The final retained 33-test configuration requires its own validation; it is pending at this decision. See ../mova.groupant/model.md#transfer-wave25 for current scope. The original measurements below are preserved verbatim and describe the candidate, even where their original wording says current production.

#### Preserved candidate report

## History

### 2026-10-30 · epoch-memory

2026-10-30 finite transfer integration evidence.

### 2026-10-30 · checks (evidence)

2026-10-30: 12 focused checks passed twice in debug and twice in release with identical normalized observations. Twelve spatial checks include an independent fixed-point oracle over 1,024 small labeled graphs and 64 block masks, 64/65/96-fact boundaries and intermediate absent object-ID cases. Active-index inverse-edge and qualifier controls failed and were restored.

### 2026-10-30 · wave25-rejected-performance

Heading `### Rejected world candidate — decision 2026-10-30` stays in wave25-rejected-performance as `### Rejected world candidate — decision`.

Rejected candidate commit: 09a7e88594352fc02e0c24e8f57fe62bba8c2af0. Candidate world/src/spartqa.rs SHA-256: 0eaf4ae70113af3ab4ee10f2edf726acadb70821c2d31e7921e93310176cfa9c. The four-frontier lazy-index implementation is not retained in production. The exact fd540f37badaad97b53d56ba81684bbbc2ca4b9b world source, test source and sidecars are restored. Boundary65 product traversal was 0.466–0.473x parent speed (about 2.11–2.15 times latency); broader regressions outweighed selected small-query gains. No threshold was retuned after measurement. The retained implementation does not inherit the rejected candidate's timing or allocation claims.

#### Spatial traversal refinement timing — 2026-10-30

The small chain3 reachability target improves 1.16–1.21x against the transfer parent. The broader lazy-index prediction fails: boundary65 product traversal falls to 0.466–0.473x (about 2.11–2.15 times parent latency), while long chain256 reachability/product falls to about 0.858–0.896x. Boundary65 boolean is near parity/slower (0.966–0.994x); boundary64 remains slower. Large direct product traversal and missing-source product also regress. Against legacy, long-chain gains remain 9.45–9.47x boolean and 4.33–4.39x product, but this does not erase regressions against the already improved parent. Saving three allocations on the short boundary query did not compensate for repeated full-fact scans. This experiment does not justify unqualified adoption of the world replacement; its deployment decision should account for these results or retain the parent. No thresholds were retuned after seeing the timings.


Measurements ran serially in the integration-granted CPU slot, using existing release test binaries, without compilation, allocator instrumentation, fixture setup, external models or IO inside timed calls. CPU Intel Core i5-13500; Rust 1.99.0, thin LTO workspace release profile. Two executions for each suite/baseline pair: eight invocations total. Every invocation passed. Each shape has three warmup pairs, then seven alternating baseline-first/production-first measured pairs; seven has a 4/3 split. Median is the median of paired baseline/production ratios; above one favors current production. All-pair ranges contain fourteen samples and are not confidence intervals. Both legacy and exact transfer-parent fd540f37badaad97b53d56ba81684bbbc2ca4b9b are compiled test-only references. No threshold changes or second tuning round followed these measurements.

Before/after hashes of both binaries and all four source/test files match exactly. World asserts equality of checksums on every pair; conversion asserts full-result equality before each workload and every measured call unwraps successfully. Across two baselines, two executions, 14 conversion plus 16 world shapes, there are 840 measured pairs. Tiny timings are sensitive to code layout, cache and timer effects; unchanged source branches may still differ in compiled context. There is no representative workload distribution, end-to-end school or Mova throughput result, or universal latency guarantee.

Raw receipts: `$MOVA_DATA/runs/octopus-wave25-mova-2026-10-30/timing-{en,world}-{legacy,parent}-{1,2}.log`, `timing-before.sha256`, `timing-after.sha256`, `timing-status.log`, `timing-samples.tsv`, `timing-summary.md`. Allocation observations remain separately recorded in wave25-evidence.md; requested bytes are not peak RSS. The benchmark retains regressions rather than choosing only improving shapes.

| Baseline | Suite | Shape | Median run 1 | Median run 2 | All-pair range |
| --- | --- | --- | ---: | ---: | ---: |
| legacy | world | boundary-64 product=false | 0.901x | 0.912x | 0.881–0.957x |
| legacy | world | boundary-64 product=true | 0.806x | 0.832x | 0.781–0.928x |
| legacy | world | boundary-65 product=false | 0.819x | 0.822x | 0.807–0.915x |
| legacy | world | boundary-65 product=true | 0.848x | 0.863x | 0.823–0.905x |
| legacy | world | chain-256 product=false | 9.446x | 9.474x | 9.177–9.632x |
| legacy | world | chain-256 product=true | 4.390x | 4.329x | 4.223–4.643x |
| legacy | world | chain-3 product=false | 0.974x | 0.966x | 0.937–0.980x |
| legacy | world | chain-3 product=true | 0.927x | 0.931x | 0.922–1.002x |
| legacy | world | chain-32 product=false | 0.978x | 1.023x | 0.873–1.047x |
| legacy | world | chain-32 product=true | 0.835x | 0.832x | 0.797–0.935x |
| legacy | world | dense-direct product=false | 30.707x | 31.801x | 30.344–33.449x |
| legacy | world | dense-direct product=true | 1.834x | 1.841x | 1.823–1.870x |
| legacy | world | large-direct product=false | 2.278x | 2.266x | 2.197–2.376x |
| legacy | world | large-direct product=true | 3.225x | 3.279x | 3.132–3.408x |
| legacy | world | missing-source product=false | 0.948x | 0.866x | 0.791–1.076x |
| legacy | world | missing-source product=true | 0.862x | 0.970x | 0.829–0.986x |
| parent | world | boundary-64 product=false | 0.909x | 0.918x | 0.867–0.978x |
| parent | world | boundary-64 product=true | 0.874x | 0.967x | 0.843–0.994x |
| parent | world | boundary-65 product=false | 0.994x | 0.966x | 0.936–1.075x |
| parent | world | boundary-65 product=true | 0.473x | 0.466x | 0.450–0.496x |
| parent | world | chain-256 product=false | 0.871x | 0.858x | 0.791–0.975x |
| parent | world | chain-256 product=true | 0.867x | 0.896x | 0.775–0.959x |
| parent | world | chain-3 product=false | 1.161x | 1.210x | 1.125–1.303x |
| parent | world | chain-3 product=true | 0.985x | 0.956x | 0.929–1.025x |
| parent | world | chain-32 product=false | 0.991x | 0.958x | 0.890–1.027x |
| parent | world | chain-32 product=true | 0.966x | 0.984x | 0.914–1.043x |
| parent | world | dense-direct product=false | 0.982x | 0.991x | 0.938–1.053x |
| parent | world | dense-direct product=true | 0.964x | 0.966x | 0.946–0.992x |
| parent | world | large-direct product=false | 0.948x | 0.947x | 0.933–0.975x |
| parent | world | large-direct product=true | 0.884x | 0.844x | 0.799–0.910x |
| parent | world | missing-source product=false | 1.507x | 1.202x | 1.146–1.516x |
| parent | world | missing-source product=true | 0.859x | 0.839x | 0.732–0.899x |
