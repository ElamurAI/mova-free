# dag_path_count: validation

Source hash and half-open byte scope are in manifest.json. Domain: reachable entry states from the parent function with successful allocation, trusted Rust compiler/library and fixed input. Observation: scoped exit state or early returned error.

## Forward
Given graph and source, first reject source >= graph.len(). Otherwise delegate full topological sorting. Success emits a full ordered vertex permutation; invalid endpoints precede cycle rejection.

## Invariant
No count allocation or arithmetic occurs before global graph validation succeeds. Every successful order contains each vertex once and all arcs point forward.

## Cost
O(E+V log(V+1)) time and O(V) auxiliary allocation inherited from topological_sort; the source failure is constant-time.

## behavior-complement
Type: behavior_complement. Within the declared reachable entry states and scoped return/state observations, let R be the forward relation above. Its complement consists of all incompatible outputs or state transitions: violations of the scoped invariant, incorrect early errors or absent transitions required by R. This is a specification complement, not merely a collection of negative fixtures. Given graph and source, first reject source >= graph.len(). Otherwise delegate full topological sorting. Success emits a full ordered vertex permutation; invalid endpoints precede cycle rejection.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## reverse-reconstruction
Type: reverse_reconstruction. A topological order admits many arc sets pointing forward; it does not reconstruct adjacency or parallel multiplicity. Each error admits all inputs having its first failed condition under the stated precedence.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## source-first
Type: source_error_priority. Even a malformed edge is not inspected when the source is invalid.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## endpoints-first
Type: endpoint_error_priority. An invalid target anywhere wins over a cycle, including in an earlier adjacency row.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## global-domain
Type: whole_graph_validation. Every component participates in ordering, including vertices unreachable from source.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## tie-order
Type: deterministic_order. Smallest available vertex is chosen; it establishes deterministic overflow witness order for the next stage.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## Composition
This fragment relies on its parent's entry preconditions and establishes only the exit conditions stated here. Parent composition wires traversal labels into canonicalization, or global ordering into checked counting. The next stage's preconditions require all preceding invariants, not merely local test success.

## Epoch memory
Prospective predictions and finite outcomes are retained in the parent model. The child scope clarifies interaction obligations; no independent execution or universal proof is claimed.

## Limits
Allocation failure excluded. Partial syntax has no semantic resolution or macro expansion. No useful certified approximation metric was established; exact finite checks are not approximate surrogates.
