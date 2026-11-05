# modelcheck-library-registry-tree

## descriptant

### Registry schema, node clauses, path ownership, parent uniqueness and root reachability groupant

Every successful registry is one nonempty rooted tree, source nodes are childless, groups have children, and source strings are unique. relative uses platform Path components; unlike syntax admission it does not separately reject repeated slash spellings.

Useful prior epochs remain in the parent descriptant. This child uses exact byte anchors and the same whole-source hash: editing unrelated bytes also invalidates its binding. The fragment is a partial structural/behavior model, not a proof. No executable percentage surrogate is supplied: categorical gates have no defined useful loss metric; any future profile must use supported caps 1, 2, 5 or 10 percent with explicit domain and evidence.

#### Wave35 explicit registry composition

A base registry B must independently satisfy the existing tree validator. Overlay O may name B's root among its children. Composition C has schema 1, root O.root and nodes B.nodes concatenated with O.nodes in input order; O.schema must equal 1 and C must satisfy the same validator. No IDs or sources are silently replaced. Validation rejects duplicate ownership, unreachable nodes, unknown children and cycles. Cloning leaves both inputs intact. Its reverse relation consists of compatible ordered prefix/suffix decompositions whose first part independently validates and whose second part names C.root; output alone does not identify a unique split. The behavior complement excludes accepted unions violating these conditions, relative to schema-1 declarative registries. Cost is two tree validations plus cloning owned text: O(N log N + E) with storage O(N + E + text bytes).

Optional groupant_path applies only to group nodes and selects a normal repository-relative directory ending .groupant; omission retains models/<id>.groupant. Existing canonical file confinement checks manifest/component resolution, including symlink escape. Source nodes remain adjacent. Directory selection changes lookup, not registry ownership, source binding or mathematical validity. Missing directories retain existing explicit backlog semantics. Canonical aliases and concurrent filesystem changes are not a transaction guarantee.

A valid sidecar without the optional registry.node pointer can be inspected as unregistered evidence by a smaller registry. Composing ownership later fills its report model; IDs, source hashes, syntax paths and other manifest checks remain enforced. The strict discovery default is unchanged. No scoped-discovery flag is introduced.

## catalog

Wave33 historical scope and evidence follow. Current source hashes and refreshed ranges are in manifest.json; wave34 refinements and evidence are at the source root.

# Registry schema, node clauses, path ownership, parent uniqueness and root reachability typed models

Subject/revision: source SHA-256 in manifest; scope [596 , 3699). Domain is one completed scoped call on a fixed snapshot with adequate resources, unless narrowed below. Observation includes return/error and declared filesystem effects. Models are source-derived partial relational constraints; complements are inner exclusion hypotheses, converse relations retain compatible-input ambiguity. No universal proof or inverse executor is claimed. Evidence: tests::hierarchy_requires_both_directions_and_a_single_reachable_tree. Mutation evidence and limits are recorded in ../model.md#wave33-evidence.

## behavior_complement

U is registry/result pairs; exclude success with duplicate IDs/sources, missing clauses, unknown or multiply owned children, parented root or unreachable nodes. This is a source-derived inner exclusion model.

Oracle: tests::hierarchy_requires_both_directions_and_a_single_reachable_tree. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## reverse_reconstruction

For successful validation, compatible antecedents are all registries satisfying the tree predicate; success contains no unique labels, clauses or topology. Error text constrains the first failed ensure only.

Oracle: tests::hierarchy_requires_both_directions_and_a_single_reachable_tree. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## ownership_fiber

Hold each source string fixed; compatible ownership maps are injective source-to-node maps with zero file children. Groups may rearrange only while unique parent and reachability remain true.

Oracle: tests::hierarchy_requires_both_directions_and_a_single_reachable_tree. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## first_failure_partition

Node schema/clauses precede edge ownership; root-parent rejection precedes DFS reachability. A fixture with several defects belongs to the earliest enabled error partition.

Oracle: tests::hierarchy_requires_both_directions_and_a_single_reachable_tree. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## reachability_certificate

Given the declared root and edge list, reached=N plus unique incoming parent and unparented root excludes a disconnected cycle. This is a finite tree invariant, not semantic dependency completeness.

Oracle: tests::hierarchy_requires_both_directions_and_a_single_reachable_tree. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## path_spelling_constraint

Allowed Path components must all be Normal and suffix must be .rs; filesystem existence and normalized slash spelling are not established here. Git inventory provides the later actual-source constraint.

Oracle: tests::hierarchy_requires_both_directions_and_a_single_reachable_tree. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## tests

### Native correspondence

Exact named checks: tests::hierarchy_requires_both_directions_and_a_single_reachable_tree.

See ../model.md#wave33-evidence for current repeated profiles, actual production mutants and scope limits. AST generation alone is not behavioral evidence. Each reference test body was read before execution.

## forward

### Scoped forward model

Registry schema, node clauses, path ownership, parent uniqueness and root reachability

Build ID and source sets; reject malformed nodes before parent analysis. Insert each child edge only once, reject a parented root, then DFS from the root and require reached count = node count. Registry deserialization separately denies unknown fields.

## invariant-and-cost

### Invariant and cost

Every successful registry is one nonempty rooted tree, source nodes are childless, groups have children, and source strings are unique. relative uses platform Path components; unlike syntax admission it does not separately reject repeated slash spellings.

For N nodes, E edges and total clause/path bytes B: O(B+(N+E) log N) comparison operations, O(N+E) auxiliary entries. String comparison length and allocations add costs.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: materialized a scoped view after full-source review. Build ID and source sets; reject malformed nodes before parent analysis. Insert each child edge only once, reject a parented root, then DFS from the root and require reached count = node count. Registry deserialization separately denies unknown fields.
