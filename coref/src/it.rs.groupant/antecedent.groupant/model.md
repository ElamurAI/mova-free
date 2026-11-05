# coref-it-antecedent

## descriptant

### coref-it-antecedent

Preceding clause candidate ordering and punctuation bounds.

## approximant

### Scoped forward relation

Preceding clause candidate ordering and punctuation bounds.

Parent transition model (restricted to this byte scope): Other forms and missing parents return Anaphoric. For it, enabled weather, time/copula, extraposition and small-clause rules return Pleonastic first. it-event off returns Anaphoric next; event verb/copula/because and bare demonstrative rules follow. antecedent computes disjoint left predicates ranked (depth,-position), but returns root-left preceding-conj slice first, then ranked candidate, then previous sentence's first predicate root; trim edge punctuation and trailing cc.

## invariant

### Scoped invariant/cost

Returned antecedent references existing sentence/predicate and inclusive bounds on a valid forest. Classification is categorical and environment-dependent. A previous root must be VERB or have cop child; no earlier-than-previous search occurs.

reading O(n) child scans plus lowercase bytes; antecedent O(n²) conservative for n candidate ancestor walks, O(n) trimming; O(1) auxiliary besides temporary strings. Public invalid indexes panic.

Scope: Preceding clause candidate ordering and punctuation bounds.. Parent bounds are conservative; no separate timing measured.

## models

Scope: Preceding clause candidate ordering and punctuation bounds.. Relations below are restricted to this fragment's transitions and retained parent state.

# Typed models

## behavior_complement

With all switches unset, a subject it attached to rain is not Event/Anaphoric even if later event conditions might apply. Missing antecedent excludes inventing an event mention; a bare demonstrative adjacent to NOUN/PROPN/ADJ is excluded from that final event rule, not from earlier event predicates.

## reverse_reconstruction

For reading r, preimage includes all valid sentence/index/environment triples satisfying the first enabled applicable branch. Event result alone cannot recover its predicate, antecedent or lexical form. For antecedent tuple a, all forests preserving winning candidate and exclusion/rank relations are compatible.

## priority

Pleonastic precedes it-event switch; event branches precede bare demonstrative adjacency. PRAG_OFF uses exact comma tokens without trimming.

## candidate_rank

root_left preempts left even though both are computed; left uses minimum depth then greatest position. Previous-sentence fallback uses first head=0 only.

## span_trim

Trim trailing punctuation or cc while b>a; trim leading punctuation while a<b; a one-token punctuation span remains possible.

## effect_scope

No filesystem writes; repeated environment reads mean a concurrently changed environment is outside deterministic snapshot assumptions. Closed lexical classes are heuristic, not semantic truth.

No executable percentage surrogate: categorical state/effect semantics currently lack a useful cheaper observation metric. Supported future caps are 1%, 2%, 5%, 10%; no profile is claimed here.

## tests

### Evidence

Safe synthetic command: env -u COREF_X -u COREF_MD -u PRAG_OFF cargo test -p coref tests:: -- --skip score::. Two debug and two release executions are retained externally with numeric exit status and sorted named outcomes. Selected checks:

- tests::it_readings_keep_it_from_nouns
- tests::wave27_event_antecedent_and_empty_evaluation

See source-test-map.json and controls.json in the wave27 coref run receipts. The library declaration file has compilation evidence only. No corpus, teacher, GPU or speed measurement.

Partial scope: No exhaustive lexical/ablation coverage or arbitrary invalid public Sent tests; percentage surrogate unsupported.

#### Measured receipts

Final source state: 13 named tests (12 library, 1 binary) passed twice debug and twice release, 52 total executed test instances; all four command exits0 and sorted named outcomes identical. Five added tests are four library boundary tests plus one main load test. The pre-existing score tests and groupant_transfers were filtered out.

Compiled actual-source control: tests::it_readings_keep_it_from_nouns. Compile exit0, assertion-run exit101, exact bytes restored, positive rerun exit0. Retained logs: control-it-compile.log, control-it-red.log, control-it-restored.log.

Nine source AST projections were generated twice identically. Shared ancestor declarations are deferred to root integration; local owned leaves have no structural diagnostics. No timings or general accuracy claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: scoped view of parent state; no independent completeness claim.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
