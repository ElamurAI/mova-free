# coref-sieve-output

## descriptant

### coref-sieve-output

Sorted cluster and last-link projections plus formatted feature observations.

## approximant

### Scoped forward relation

Sorted cluster and last-link projections plus formatted feature observations.

Parent transition model (restricted to this byte scope): new detects mentions, classifies it, internalizes pleonastic mentions and optionally inserts event mentions before sorting/dedup. Start singleton clusters. run dispatches configured passes in order. merge(from,to) moves whole source cluster, rewrites of, sorts members, ORs masks and appends link (possibly redirecting internal target to first earlier visible member). clusters sorts surviving groups by first member. Ordinary compatibility rejects same cluster, number, gender, animacy, person, then disjoint I voices, in that order.

## invariant

### Scoped invariant/cost

On valid input, nonempty clusters partition all mention indexes and of agrees; each real merge decreases cluster count by1. Therefore links <= initial mention count-1 even with repeated run. Mask unions are not intersections. Speaker identity links and nearest bypass some/all cluster agreement; notional plural fallback intentionally bypasses number agreement.

Let S passes,M mentions,n maximum sentence length,L retained mention text/features. Conservative O(S M³ L + S M² n + S M² log M), plus mention extraction: candidate sorting, repeated cluster materialization and ancestor/binding scans. Storage O(ML+N+M) plus per-candidate temporaries. Hash lookups expected; no throughput claim.

Scope: Sorted cluster and last-link projections plus formatted feature observations.. Parent bounds are conservative; no separate timing measured.

## models

Scope: Sorted cluster and last-link projections plus formatted feature observations.. Relations below are restricted to this fragment's transitions and retained parent state.

# Typed models

## behavior_complement

Configured default resolution excludes event/deixis linking (Deixis not in DEFAULT), blocks nonanaphoric it in pronoun pass and blocks personal co-argument cluster links by principle B there. These exclusions do not apply uniformly to Nearest or other passes. Merges never split clusters or create mentions after construction.

## reverse_reconstruction

Given clusters and links, compatible pre-states include singleton mention features/configs whose ordered transitions produce those observations. OR masks lose provenance and final partition alone loses merge order. A link's explanation constrains branch/rank but does not reconstruct all rejected candidates or original tree.

## partition_transition

merge maps every member of ra to rb, empties ra, unions sorted members and masks. Internal to may be rewritten only in observable Link, not in destination root selection.

## candidate_order

Candidates preceding m exclude nested/same-start-later mentions and singular-to-group. Same sentence ranks own clause then depth/start/-end; previous pronouns rank left first, nominals right first; sentence recency remains outer order.

## binding_preference

Pronoun filters quantified/interrogative antecedents, principle B across entire cluster, then compatibility. Animate preference is two passes; collective notional fallback occurs only after no strict candidate.

## pass_exceptions

Speaker may merge I/you/name despite person mismatch. Nearest ignores reading/window/cluster compatibility and uses mention masks. Exact/relaxed/head require first-in-cluster, but appos/pred/speaker differ. No universal first-in-cluster claim.

No executable percentage surrogate: categorical state/effect semantics currently lack a useful cheaper observation metric. Supported future caps are 1%, 2%, 5%, 10%; no profile is claimed here.

## tests

### Evidence

Safe synthetic command: env -u COREF_X -u COREF_MD -u PRAG_OFF cargo test -p coref tests:: -- --skip score::. Two debug and two release executions are retained externally with numeric exit status and sorted named outcomes. Selected checks:

- tests::story_chains_and_reasons
- tests::principle_b_is_what_blocks_who
- tests::speakers
- tests::notional_they_only_without_a_plural
- tests::resolver_ignores_gold_entities

See source-test-map.json and controls.json in the wave27 coref run receipts. The library declaration file has compilation evidence only. No corpus, teacher, GPU or speed measurement.

Partial scope: No full branch or language-accuracy proof; environmental ablations and arbitrary malformed graphs untested. HashSet iteration is not used to select semantic winners; explanations sort set words.

#### Measured receipts

Final source state: 13 named tests (12 library, 1 binary) passed twice debug and twice release, 52 total executed test instances; all four command exits0 and sorted named outcomes identical. Five added tests are four library boundary tests plus one main load test. The pre-existing score tests and groupant_transfers were filtered out.

Compiled actual-source control: tests::principle_b_is_what_blocks_who. Compile exit0, assertion-run exit101, exact bytes restored, positive rerun exit0. Retained logs: control-sieve-compile.log, control-sieve-red.log, control-sieve-restored.log.

Nine source AST projections were generated twice identically. Shared ancestor declarations are deferred to root integration; local owned leaves have no structural diagnostics. No timings or general accuracy claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: scoped view of parent state; no independent completeness claim.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
