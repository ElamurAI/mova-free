# onto-judge

## descriptant

### onto-judge

Clean removes nested bracket spans then first900 Unicode chars. FNV1a wrapping hash mod3 selects train; fixed xorshift shuffles before/after items and assigns1000+i. Parse accepts four1..5 u8 scores and last valid duplicate wins. report joins IDs and computes means/tied ranks; run/architect invoke teacher.

## approximant

### Forward model: onto-judge

Clean removes nested bracket spans then first900 Unicode chars. FNV1a wrapping hash mod3 selects train; fixed xorshift shuffles before/after items and assigns1000+i. Parse accepts four1..5 u8 scores and last valid duplicate wins. report joins IDs and computes means/tied ranks; run/architect invoke teacher.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

parse(ans)[id]=last row whose id parses and all four scores∈{1,…,5}; invalid rows have no state transition. mean4(s)=Σsᵢ/4. is_train(x)⇔FNV1a(UTF8(x)) mod3≠0, using wrapping u64 multiplication. clean=take_chars(900,strip_nested_brackets(text)).

## invariant

# Invariant and cost

Invalid duplicate does not erase previous valid score. Parse IDs unconstrained until report joins. Report direct TSV parser does not enforce1..5. No held-out files/teacher used in tests.

## Cost

O(B+I log I) report sorting plus repeated category/subgroup scans; clean first allocates O(B) before truncation. Teacher costs external and unmeasured.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

describe files → deterministic item IDs → wdvmm::ask score responses → parser/report. No teacher or held-out input execution; pure parser success is not judge validity.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

parse cannot return out-of-range scores; report ingestion can. Unmatched opening bracket suppresses suffix; clean does not remove unmatched closing bracket.

## reverse_reconstruction

Map admits invalid ignored lines and earlier overwritten duplicates. Clean result admits arbitrary bracket insertions and suffix after900 retained chars. IDs require same ordered inputs to reconstruct shuffle.

## split_hash

UTF8 bytes xor then wrapping multiply; h%3!=0 train. Deterministic split does not guarantee finite-sample2/3 balance.

## score_mean

Equal rubric weighting /4; empty subgroup denominator max1 yields0 rather than missing.

## rank_ties

Equal means get average zero-based ranks; covariance denominator at least1e-9; constant vectors rho0.

## effect_resume

Existing main IDs resume; any alt row skips all alt work. Main errors returned, alt errors not checked after work; writes ignored. Live effects unexecuted.

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

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Invalid duplicate does not erase previous valid score. Parse IDs unconstrained until report joins. Report direct TSV parser does not enforce1..5. No held-out files/teacher used in tests.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
