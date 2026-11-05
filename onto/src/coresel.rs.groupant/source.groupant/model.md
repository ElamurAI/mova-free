# onto-coresel-source

## descriptant

### onto-coresel-source

Scan boundary Q/P tokens and cast integer widths; match rank substrings by fixed priority; parse steps from first ' steps)' and last opening parenthesis.

## approximant

### Forward model: onto-coresel-source

Scan boundary Q/P tokens and cast integer widths; match rank substrings by fixed priority; parse steps from first ' steps)' and last opening parenthesis.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

q=first_boundary_Q_u64 mod2^32; p=first_boundary_P_u64 mod2^16; absent/unparseable→0. rank=first matching member of ordered[preferred,normal,deprecated], else '-'. steps=parse_u8(parenthesized text before first ' steps)') or0.

## invariant

# Invariant and cost

Scope bytes [1882,2838) in the parent source. Embedded xQ12 cannot become Q12. Missing source returns zeros/dash; narrowing overflow not rejected.

## Cost

Linear traversed scope input/returned bytes except lookup logarithmic comparisons; allocations follow actual result buffers. Parent whole-pipeline cost is not assigned to this fragment.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

Scope output feeds its parent: wdwalk::read_core and corevmm::{candidates,verdicts,key_of} feed selection; q_synsets aligns accepted/also nouns; output consumed by coregate/build script. Helpers tested; full selection file pipeline not executed.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

Embedded xQ12 cannot become Q12. Missing source returns zeros/dash; narrowing overflow not rejected.

## reverse_reconstruction

Zeros admit absent/invalid tokens or exact narrowing modulus; nonzero tuples admit arbitrary ignored words.

## token_boundary

Unicode alphanumeric predecessor rejects token, punctuation permits.

## rank_priority

preferred wins over normal regardless textual order.

## step_grammar

Steps parse u8 or default0, including malformed/overflow.

## width_loss

P70000 becomes4464; Q narrowing similarly modulo2^32.

## tests

### Evidence

Selected command: cargo test --offline --locked -p onto --bin onto -- --exact --test-threads=1 coresel::wave29::applicability_and_provenance

Named outcomes:

- coresel::wave29::applicability_and_provenance

Two debug and two release/optimized executions returned zero and identical sorted nonempty named outcomes. Source-generated syntax pairs matched byte-for-byte. 1 compiled source mutation rejected by named test, exact source restored and positive rerun succeeded.

- Control: "\"magical\" => field == \"risk\" -> \"magical\" => field != \"risk\"" → "coresel::wave29::applicability_and_provenance" (exit 101).

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Scope bytes [1882,2838) in the parent source. Embedded xQ12 cannot become Q12. Missing source returns zeros/dash; narrowing overflow not rejected.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
