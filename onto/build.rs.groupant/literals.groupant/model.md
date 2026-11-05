# onto-build-script-literals

## descriptant

### onto-build-script-literals

camel concatenates separator-delimited words with ASCII first-letter uppercase; lit Debug-escapes string; fnum parses f64 then Debug-formats.

## approximant

### Forward model: onto-build-script-literals

camel concatenates separator-delimited words with ASCII first-letter uppercase; lit Debug-escapes string; fnum parses f64 then Debug-formats.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

camel(s)=concat(upper_ascii(first(w)) ++ tail(w)) over nonempty split(s,{'_',' ','-'}); lit(s)=Debug(s); fnum(s)=Debug(parse_f64(s)) if parse succeeds, otherwise panic.

## invariant

# Invariant and cost

Scope bytes [1144,1595) in the parent source. Invalid number cannot return normally; camel cannot preserve separators.

## Cost

Linear traversed scope input/returned bytes except lookup logarithmic comparisons; allocations follow actual result buffers. Parent whole-pipeline cost is not assigned to this fragment.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

Scope output feeds its parent: coregate::parse → generated CoreId/CORE included by core.rs. A syntactically complete TSV may still fail literal emission/compiler numeric or identifier constraints.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

Invalid number cannot return normally; camel cannot preserve separators.

## reverse_reconstruction

RedFox admits red_fox and red-fox; 1.25 admits alternative parsed numeric spellings.

## unicode_case

Non-ASCII initial chars are unchanged by ASCII uppercase.

## escape_transport

Quotes/newlines are escaped into Rust literal text.

## numeric_domain

NaN/infinity parse without rejection; compiler acceptance separate.

## failure_order

fnum parse precedes formatting and reports supplied field context.

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

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Scope bytes [1144,1595) in the parent source. Invalid number cannot return normally; camel cannot preserve separators.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
