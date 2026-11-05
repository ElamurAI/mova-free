# onto-judge-scores

## descriptant

### onto-judge-scores

Trim >=5-column lines; parse usize after leading id removal and four u8 within1..5; last complete valid row overwrites BTreeMap; mean4 sums/4.

## approximant

### Forward model: onto-judge-scores

Trim >=5-column lines; parse usize after leading id removal and four u8 within1..5; last complete valid row overwrites BTreeMap; mean4 sums/4.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

For valid row r: M'=M[id↦([C,F,U,E],error_or_dash)], else M'=M. Valid(r)⇔width≥5∧usize(id)∧∀j∈1..4:parse_u8(r[j])∈[1,5]. mean4([a,b,c,d])=(a+b+c+d)/4 in f64.

## invariant

# Invariant and cost

Scope bytes [4665,5365) in the parent source. Any invalid score excludes entire row; invalid duplicate leaves previous entry. Missing error defaults dash.

## Cost

Linear traversed scope input/returned bytes except lookup logarithmic comparisons; allocations follow actual result buffers. Parent whole-pipeline cost is not assigned to this fragment.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

Scope output feeds its parent: describe files → deterministic item IDs → wdvmm::ask score responses → parser/report. No teacher or held-out input execution; pure parser success is not judge validity.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

Any invalid score excludes entire row; invalid duplicate leaves previous entry. Missing error defaults dash.

## reverse_reconstruction

Map admits earlier valid duplicates and arbitrary invalid later rows; row order not reconstructible.

## identifier_domain

Any usize accepted; no submitted-ID membership check.

## range_gate

1 and5 accepted;0,6 and fractions rejected.

## duplicate_policy

Last valid duplicate wins; iteration numeric key order.

## mean_units

Each rubric score weighted1/4, finite u8 arithmetic cast to f64.

## tests

### Evidence

Selected command: cargo test --offline --locked -p onto --bin onto -- --exact --test-threads=1 judge::wave29::score_overwrite_and_cleaning

Named outcomes:

- judge::wave29::score_overwrite_and_cleaning

Two debug and two release/optimized executions returned zero and identical sorted nonempty named outcomes. Source-generated syntax pairs matched byte-for-byte. 1 compiled source mutation rejected by named test, exact source restored and positive rerun succeeded.

- Control: "(1..=5).contains(x) -> (1..=4).contains(x)" → "judge::wave29::score_overwrite_and_cleaning" (exit 101).

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Scope bytes [4665,5365) in the parent source. Any invalid score excludes entire row; invalid duplicate leaves previous entry. Missing error defaults dash.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
