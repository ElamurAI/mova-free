# treebanks-render-decimal-grouping

## descriptant

### Descriptant

Convert u64 to decimal and insert a space before each nonfirst digit whose remaining suffix length is divisible by three.

## approximant

# Forward approximant

Convert u64 to decimal and insert a space before each nonfirst digit whose remaining suffix length is divisible by three.

## Composition

This fragment is the exact byte-scoped helper identified in the manifest; its relation is restricted to the operations above.

The containing source wires this scoped relation to its callers. Treeagree proposal→gate→commit and treebanks parse→match→collect→render are actual joins; parsecmp is an independent diagnostic path. The external group model distinguishes these tags.

## catalog

# Typed models for treebanks-render-decimal-grouping

## behavior_complement

Output cannot contain leading/trailing spaces or nondecimal characters other than internal grouping spaces.

Operator: inner exclusion on admitted domain. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## reverse_reconstruction

Removing spaces and parsing u64 uniquely recovers input within canonical num output language; arbitrary spaced decimal input is not an inverse domain.

Operator: compatible antecedent relation, retaining ambiguity. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## zero_case

Zero maps to exactly 0 with no separator; values below 1000 retain ordinary decimal spelling.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## threshold_boundary

999 has no separator, 1000 becomes 1 000; every additional three digits adds one separator.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## width_bound

Largest u64 has 20 decimal digits and six separators, total 26 output bytes; no arithmetic on numeric magnitude beyond formatting.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## right_alignment

Group boundaries count remaining digits from right, independently of locale; all separators are ASCII space.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## tests

### Scoped evidence

Parent native slice and exact names are in `../model.md#tests`.

Only named finite assertions cover selected inputs; other branches and invalid-domain behavior remain source-derived.

This fragment has no independent full-coverage claim. Its byte range and source hash bind the source-derived relation; parent test success is evidence only where that helper is actually reached.

## invariant-and-cost

### Invariant and cost

O(decimal digits), at most 20 input digits and 6 separator spaces for u64; one output String.

Output cannot contain leading/trailing spaces or nondecimal characters other than internal grouping spaces.

The complement and specialized catalog constrain preserved coordinates and effect boundaries. Empty/malformed domains are only guaranteed where explicitly admitted. No whole-program cost bound follows from a helper bound.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective source reading identified conditional input alignment, information loss and failure order before tests. Existing production code is retained; added tests use actual helpers. Finite results update this source-bound model, not all parser/corpus semantics.

No executable percentage surrogate has a justified observation metric for these categorical/effectful APIs. Future profiles require metric, domain and real-reference evidence at caps 1/2/5/10%; no higher cap. Syntax is partial lexical Rust projection without macro expansion or semantic resolution.

### Undated · document

Subject: source SHA-256 `1d5101cda9ced92075a30cb2fd999239fc136ec6c5d315230f18c4c0fa15403a`; manifest supplies exact source/byte scope. Domain: admitted states in forward model, nonoverflow arithmetic and successful allocations. Observation: one terminating invocation, except explicitly described file trace. Source-derived partial relations; named finite tests constrain selected cases only. Each entry holds arguments and environment fixed except its stated intervention.
