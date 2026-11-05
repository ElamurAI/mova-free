# coref-eval-report

## descriptant

### coref-eval-report

System configs, cumulative contribution report and ordered optional effects.

## approximant

### Scoped forward relation

System configs, cumulative contribution report and ordered optional effects.

Parent transition model (restricted to this byte scope): gold_of groups first-seen eid and sets anaphoric iff mention key differs from entity minimum (sent,start,-end). visible excludes internal and singleton coordination absent coord-sgl substring override. sys_entities recomputes span heads; out_mentions preserves mention heads, numbers nonempty visible clusters from1 and notes last link/new/sgl. score_all adds score_doc Counts over zip, not average F1. run gold parses/self-checks then verifies document counts, appends baseline/cumulative reports, writes errors, checks strip invariance, optionally writes CoNLL-U.

## invariant

### Scoped invariant/cost

Scores are micro numerator/denominator totals. Empty counts yield scorer zero ratios. Gold ids and output ids are local ordinal labels, not cross-document identity. run mismatch fails before appending report; later failures can leave report and error file effects. Filtered clusters remain ordered.

Costs sum repeated resolver and scorer calls: two singleton policies × six systems plus FULL.len+1 cumulative runs plus diagnostics and strip-control reruns. Scoring delegates matching and assignment matrix costs. Error rows capped400 per diagnostic run; report/output buffers and per-document predictions retained.

Scope: System configs, cumulative contribution report and ordered optional effects.. Parent bounds are conservative; no separate timing measured.

## models

Scope: System configs, cumulative contribution report and ordered optional effects.. Relations below are restricted to this fragment's transitions and retained parent state.

# Typed models

## behavior_complement

Internal mentions cannot enter sys_entities/out_mentions even with coord-sgl enabled. A mismatched input document count cannot reach report generation after successful gold self-check. score_all alone does not exclude truncated unequal slices. Leak comparison covers span/eid/note, not head/etype.

## reverse_reconstruction

Given output entities, compatible Sys states may differ in filtered internal mentions, singleton groups, explanations and original heads (sys_entities recomputes them). A metric row admits many gold/pred partitions with identical aggregate counts. Serialized notes constrain last from-link but not complete history.

## visibility

visible uses full cluster length before filtering; a coordination mention clustered only with an internal mention can become visible. coord-sgl checks substring, unlike exact-token sieve switches.

## micro_transport

score_all(g,p)=sum_i score_doc(g_i,p_i) for i<min lengths; precision/recall/F are computed after summing Counts.

## failure_effect_order

gold parse -> scorer self-check -> length check -> report -> errors write -> strip leak check -> output write. No transaction or directory creation in run.

## diagnostic_precedence

link_stats counts from-spurious before to-spurious; same-chain only when both match. Wrong-chain error examples capped400; pronouns count matched third-person ppr only.

No executable percentage surrogate: categorical state/effect semantics currently lack a useful cheaper observation metric. Supported future caps are 1%, 2%, 5%, 10%; no profile is claimed here.

## tests

### Evidence

Safe synthetic command: env -u COREF_X -u COREF_MD -u PRAG_OFF cargo test -p coref tests:: -- --skip score::. Two debug and two release executions are retained externally with numeric exit status and sorted named outcomes. Selected checks:

- tests::write_read_round_trip
- tests::wave27_coordination_mentions_and_visibility
- tests::wave27_event_antecedent_and_empty_evaluation

See source-test-map.json and controls.json in the wave27 coref run receipts. The library declaration file has compilation evidence only. No corpus, teacher, GPU or speed measurement.

Partial scope: No real corpus, fault injection or exhaustive score checks; existing score source modeled elsewhere. Partial gold-leak equality does not compare head/etype.

#### Measured receipts

Final source state: 13 named tests (12 library, 1 binary) passed twice debug and twice release, 52 total executed test instances; all four command exits0 and sorted named outcomes identical. Five added tests are four library boundary tests plus one main load test. The pre-existing score tests and groupant_transfers were filtered out.

Compiled actual-source control: tests::wave27_coordination_mentions_and_visibility. Compile exit0, assertion-run exit101, exact bytes restored, positive rerun exit0. Retained logs: control-eval-compile.log, control-eval-red.log, control-eval-restored.log.

Nine source AST projections were generated twice identically. Shared ancestor declarations are deferred to root integration; local owned leaves have no structural diagnostics. No timings or general accuracy claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: scoped view of parent state; no independent completeness claim.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
