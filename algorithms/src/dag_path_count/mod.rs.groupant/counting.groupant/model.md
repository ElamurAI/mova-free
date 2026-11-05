# dag_path_count: counting

Source hash and half-open byte scope are in manifest.json. Domain: reachable entry states from the parent function with successful allocation, trusted Rust compiler/library and fixed input. Observation: scoped exit state or early returned error.

## Forward
Given a valid full topological order, initialize the source to one and other counts to zero. For each outgoing arc add its source count with checked_add, returning the first overflow endpoints or the complete count vector.

## Invariant
Before processing vertex v its count is the exact sum over all incoming path contributions, plus the empty path when v is source. All terms are nonnegative.

## Cost
O(V+E) time, O(V) result allocation. Fixed-width checked arithmetic never wraps. Early overflow may stop before all arcs are scanned.

## behavior-complement
Type: behavior_complement. Within the declared reachable entry states and scoped return/state observations, let R be the forward relation above. Its complement consists of all incompatible outputs or state transitions: violations of the scoped invariant, incorrect early errors or absent transitions required by R. This is a specification complement, not merely a collection of negative fixtures. Given a valid full topological order, initialize the source to one and other counts to zero. For each outgoing arc add its source count with checked_add, returning the first overflow endpoints or the complete count vector.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## reverse-reconstruction
Type: reverse_reconstruction. Successful count vector retains all acyclic predecessor multiplicities satisfying the recurrence and order. Overflow retains all prefix sums whose next incoming contribution exceeds u128::MAX; counts of unvisited vertices are not determined.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## empty-path
Type: initial_condition. The source's length-zero path contributes one, even with no outgoing arcs.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## multiplicity
Type: distinct_arc_choices. Two equal adjacency targets cause two additions, expressing distinct arc paths.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## zero-flow
Type: unreachable_zero. An arc from a zero-count vertex contributes zero and cannot itself overflow.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## boundary
Type: monotone_overflow. An exact sum of u128::MAX is allowed. Since terms are nonnegative, an overflowed partial sum cannot later return to range.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## Composition
This fragment relies on its parent's entry preconditions and establishes only the exit conditions stated here. Parent composition wires traversal labels into canonicalization, or global ordering into checked counting. The next stage's preconditions require all preceding invariants, not merely local test success.

## Epoch memory
Prospective predictions and finite outcomes are retained in the parent model. The child scope clarifies interaction obligations; no independent execution or universal proof is claimed.

## Limits
Allocation failure excluded. Partial syntax has no semantic resolution or macro expansion. No useful certified approximation metric was established; exact finite checks are not approximate surrogates.
