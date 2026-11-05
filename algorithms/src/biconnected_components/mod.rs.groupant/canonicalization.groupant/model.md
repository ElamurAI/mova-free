# biconnected_components: canonicalization

Source hash and half-open byte scope are in manifest.json. Domain: reachable entry states from the parent function with successful allocation, trusted Rust compiler/library and fixed input. Observation: scoped exit state or early returned error.

## Forward
Input is a total label vector indexed by edge identity, with labels in 0..blocks. Scan identities ascending; allocate a result block on first encounter of a label and append each identity to its mapped position.

## Invariant
positions[label] is unset exactly before the first encountered identity of that class. At every prefix, output partitions that prefix and blocks are ordered by their first identity.

## Cost
O(E+B) time and space including B output vectors; B <= E. No sort or pairwise comparisons.

## behavior-complement
Type: behavior_complement. Within the declared reachable entry states and scoped return/state observations, let R be the forward relation above. Its complement consists of all incompatible outputs or state transitions: violations of the scoped invariant, incorrect early errors or absent transitions required by R. This is a specification complement, not merely a collection of negative fixtures. Input is a total label vector indexed by edge identity, with labels in 0..blocks. Scan identities ascending; allocate a result block on first encounter of a label and append each identity to its mapped position.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## reverse-reconstruction
Type: reverse_reconstruction. The canonical output determines the equality relation on input labels, but any injective renaming of used labels and any unused allocated label slots is compatible. Original endpoints and traversal order cannot be recovered.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## within-order
Type: monotone_identity. Appending the current scanned ID makes each block strictly increasing.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## block-order
Type: first_occurrence. New output blocks are created at their minimum identity; their minima strictly increase.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## label-renaming
Type: internal_name_invariance. Any bijection on internal labels preserves the exact output.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## totality
Type: label_precondition. Missing labels or labels outside positions are excluded entry states; the preceding traversal must establish totality.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## Composition
This fragment relies on its parent's entry preconditions and establishes only the exit conditions stated here. Parent composition wires traversal labels into canonicalization, or global ordering into checked counting. The next stage's preconditions require all preceding invariants, not merely local test success.

## Epoch memory
Prospective predictions and finite outcomes are retained in the parent model. The child scope clarifies interaction obligations; no independent execution or universal proof is claimed.

## Limits
Allocation failure excluded. Partial syntax has no semantic resolution or macro expansion. No useful certified approximation metric was established; exact finite checks are not approximate surrogates.

## History

### 2026-11-01 · Epoch memory

2026-11-01 review: once traversal supplies a total label vector, equality of labels, rather than their numeric values, determines the output. The public API's fresh-bridge probe in ../model.md#evidence checks the composition, not arbitrary synthetic label vectors. Permuting input edge identities does not preserve the literal output: the induced identity permutation must be applied before recanonicalizing.
