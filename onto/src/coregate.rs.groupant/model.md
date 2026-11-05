# onto-coregate

## descriptant

### onto-coregate

read_core trims columns and validates domain. Property rows check width20→membership→field→status→synset consistency, group into maps, then check completeness by core and FIELDS order, homogeneous status, none-only-risk, singleton known number.

## approximant

### Forward model: onto-coregate

read_core trims columns and validates domain. Property rows check width20→membership→field→status→synset consistency, group into maps, then check completeness by core and FIELDS order, homogeneous status, none-only-risk, singleton known number.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

Success ⇔ read_core succeeds ∧ every property row passes ordered(width20, member, field, status, consistent_synset) ∧ ∀k∈core,f∈FIELDS: cell(k,f)≠∅ ∧ uniform_status(cell) ∧ (status=none⇒f=risk) ∧ (status=known∧f∈NUMBER⇒|cell|=1). First failing row/predicate determines error before completeness traversal.

## invariant

# Invariant and cost

Success has every core×11 field and one synset per key; duplicate core entries retained. Empty core/props succeeds; empty unknown reason, numeric values and units unvalidated.

## Cost

O(B+RC+11C+R) expected map work, with BTree field lookup factors. O(B+R+C) storage; no allocation-failure Result.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

coresel writes 20-column rows consumed by build.rs through this parser. Parsing validates categorical completeness, not f64 finiteness or provenance literals.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

Success excludes width/member/field/status/synset/cell violations, but not NaN or empty unknown reason.

## reverse_reconstruction

Parsed rows retain within-cell order; comments, blank lines and interleavings between cells are unrecoverable. Core whitespace and extra columns lost.

## failure_precedence

Core domain errors precede all property checks. Row-local errors precede completeness even for earlier missing concept.

## numeric_cardinality

Only known numeric fields require singleton; unknown can have multiple same-status rows.

## risk_state

none is risk-only; known risk action and probability grammar not checked.

## whitespace_domain

Core fields trim; property fields do not. Leading-space # is not a comment; comparisons case-sensitive.

## tests

### Evidence

Selected command: cargo test --offline --locked -p onto --bin onto -- --exact --test-threads=1 coregate::wave29::ordered_gate_and_empty_domain

Named outcomes:

- coregate::wave29::ordered_gate_and_empty_domain

Two debug and two release/optimized executions returned zero and identical sorted nonempty named outcomes. Source-generated syntax pairs matched byte-for-byte. 1 compiled source mutation rejected by named test, exact source restored and positive rerun succeeded.

- Control: "f.len() != 20 -> f.len() != 21" → "coregate::wave29::ordered_gate_and_empty_domain" (exit 101).

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Success has every core×11 field and one synset per key; duplicate core entries retained. Empty core/props succeeds; empty unknown reason, numeric values and units unvalidated.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
