# onto-build-script

## descriptant

### onto-build-script

Read core first; absent props produce empty core only with bootstrap=1. Otherwise coregate parses then enum indices and CORE records follow core order. Status dispatch: n/a→NotApplicable, unknown→Unknown(note), none risk→Known([]), known number→Dist, risk→Risk array, object→Val array.

## approximant

### Forward model: onto-build-script

Read core first; absent props produce empty core only with bootstrap=1. Otherwise coregate parses then enum indices and CORE records follow core order. Status dispatch: n/a→NotApplicable, unknown→Unknown(note), none risk→Known([]), known number→Dist, risk→Risk array, object→Val array.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

For core sequence K=(k₀,…,kₙ₋₁), CoreId(kᵢ)=i and CORE[i]=emit(cells[kᵢ]). emit(n/a)=NotApplicable; emit(unknown,r)=Unknown(r[19]); emit(none,risk)=Known([]). For a referenced synset s, core_ref(s)=last{i:synset(kᵢ)=s}, or None.

## invariant

# Invariant and cost

Index order follows core, not HashMap iteration. Generation builds the full string before write; invalid confirmation/action/number may panic after gate succeeds. Camel names can collide and u16 enum capacity is unchecked.

## Cost

O(B+RC+11C+V) expected hash work and formatting; R rows,C core concepts,V values. Core membership scans C per row; all text/rows retained.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

coregate::parse → generated CoreId/CORE included by core.rs. A syntactically complete TSV may still fail literal emission/compiler numeric or identifier constraints.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

Missing props without bootstrap cannot successfully emit empty core; present invalid props cannot bypass gate. Gate success does not exclude invalid numeric tokens.

## reverse_reconstruction

Generated records admit varying comments, ignored columns, numeric spellings and invalid integer provenance mapped to zero. Camel loses separators and original capitalization; no unique TSV inverse.

## status_dispatch

Unknown/n/a never call fnum; none allowed only for risk. Known numeric field requires one row.

## provenance_lowering

q/p/steps parse to u32/u16/u8 with zero fallback; empty confidence becomes 0.0. Four confirmation strings accepted, others panic.

## identifier_collision

red_fox and red-fox both become RedFox; duplicate raw lemmas append domain but camel collisions across distinct lemmas remain.

## numeric_emission

fnum accepts parseable nonfinite f64; output may be invalid Rust. No probability, unit or min≤mean≤max validation.

## tests

### Evidence

Selected command: rustc --edition=2024 --test onto/build.rs; run generated test binary (debug and -O)

Named outcomes:

- wave29::generated_literal_helpers
- coregate::wave29::ordered_gate_and_empty_domain

Two debug and two release/optimized executions returned zero and identical sorted nonempty named outcomes. Source-generated syntax pairs matched byte-for-byte. 1 compiled source mutation rejected by named test, exact source restored and positive rerun succeeded.

- Control: "uppercase -> lowercase" → "wave29::generated_literal_helpers" (exit 101).

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Index order follows core, not HashMap iteration. Generation builds the full string before write; invalid confirmation/action/number may panic after gate succeeds. Camel names can collide and u16 enum capacity is unchecked.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
