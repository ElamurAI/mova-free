# onto-corevmm-risks

## descriptant

### onto-corevmm-risks

Allocate n None; valid one-based ID with>=2 fields creates Some(empty); none leaves slot; valid lowercase act/harm and f64 range append tuple.

## approximant

### Forward model: onto-corevmm-risks

Allocate n None; valid one-based ID with>=2 fields creates Some(empty); none leaves slot; valid lowercase act/harm and f64 range append tuple.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

For i∈1..n, v[i-1] initially None. On row with valid i and width≥2: v[i-1]:=Some(v[i-1].unwrap_or([])); if act∈ACTS∧harm∈HARMS∧p∈[0,1], append tuple. NaN∉[0,1]. none leaves accumulated list unchanged.

## invariant

# Invariant and cost

Scope bytes [6860,7861) in the parent source. No accepted p outside[0,1]; bad valid-ID row may still create empty slot.

## Cost

Linear traversed scope input/returned bytes except lookup logarithmic comparisons; allocations follow actual result buffers. Parent whole-pipeline cost is not assigned to this fragment.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

Scope output feeds its parent: wdvmm response transport supplies strings parsed here; pick and verdict key feed coresel. Live ask, append failure and worker races are explicit gaps.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

No accepted p outside[0,1]; bad valid-ID row may still create empty slot.

## reverse_reconstruction

Some([]) admits none, bad act,NaN; None admits no row or invalid ID; accepted order preserved.

## coordinates

1..=n maps to i-1; n0 accepts no row.

## float_gate

NaN/infinity rejected,0 and1 accepted.

## none_monotonicity

none does not clear earlier appended tuples.

## prompt_gap

Parser has no three-risk maximum or0.001 floor.

## tests

### Evidence

Selected command: cargo test --offline --locked -p onto --bin onto -- --exact --test-threads=1 corevmm::wave29::candidate_threshold_order_and_dedup corevmm::wave29::risk_slot_is_distinct_from_valid_risk corevmm::t::parse_risk_answer

Named outcomes:

- corevmm::wave29::candidate_threshold_order_and_dedup
- corevmm::wave29::risk_slot_is_distinct_from_valid_risk
- corevmm::t::parse_risk_answer

Two debug and two release/optimized executions returned zero and identical sorted nonempty named outcomes. Source-generated syntax pairs matched byte-for-byte. 1 compiled source mutation rejected by named test, exact source restored and positive rerun succeeded.

- Control: "(0.0..=1.0).contains(&p) -> (0.0..=0.9).contains(&p)" → "corevmm::wave29::risk_slot_is_distinct_from_valid_risk" (exit 101).

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Scope bytes [6860,7861) in the parent source. No accepted p outside[0,1]; bad valid-ID row may still create empty slot.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
