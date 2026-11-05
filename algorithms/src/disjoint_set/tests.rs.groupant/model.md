# Disjoint-set executable checks

## forward

Given the compiled DisjointSet implementation I, execute ten deterministic
tests. merge maintains independent class labels by whole-class relabeling.
agrees inspects parent bounds, termination, root cardinalities, all pairwise
memberships and component count without mutating I. Assertions compare
operation returns, abstract partitions and explicit raw-state frames.
Harness output is pass or an assertion/panic report; no random external input.

## invariant

Expected classes are computed from the operation history and labels, never
from production roots. Observation traversals are bounded, so cycles reject
rather than hang inside agrees. Error snapshots precede calls, allowing checks
of compression side effects. Permutation assertions compare partition and
count, not raw representatives, which may depend on history.

## cost

Let n be universe size and L trace length. Label merge is O(n), agrees is
O(n squared) on valid size-balanced forests (its O(n log n) traversals are
dominated); defensive malformed-forest traversal may also cost O(n squared).
Seeded traces cost O(L n squared), O(n) live state plus assertion strings.
The exhaustive domain branches b(n)=(n+1)+2(n+1)^2 to depth three at n<=3.
These are test-harness bounds, not measured production performance.

## behavior-complement

Type behavior_complement. U consists of pairs (compiled implementation I,
harness verdict v) for this fixed suite and successful normal execution.
R assigns each I its actual deterministic verdict. Target U minus R.
Inner approximation B contains (I,Pass) whenever an executed assertion is
known false, and (I,Fail) when every executed assertion and call terminates
normally with matching observations. Operational correspondence is conditional
on trusted Rust assertion semantics and execution environment. The useful
witness is a compiled source mutation disabling path rewrites: the balanced
test sees parent[63]=62, excludes Pass, and the harness returns failure.
No claim is made that Pass excludes all incorrect implementations.

## reverse-reconstruction

Type reverse_reconstruction. For observed verdict v, retain all compiled
implementations and histories compatible with the finite observations:
Pre(v)={I : Harness(I)=v}. Pass leaves unsampled sizes, histories and resource
behavior unconstrained. A specific failing assertion narrows the antecedents
to executions reaching that assertion with unequal compared values, but does
not uniquely identify a source edit. For example parent 62 after querying 63
can arise from omitted compression or wrong assignment; neither follows
uniquely from the failure. The existing reachable-forest ambiguity test
additionally exhibits distinct raw input forests with one partition.
This is a declarative preimage, not complete implementation enumeration.

## state-oracle

Type independent_state_oracle. Hold labels l and candidate raw state S fixed.
A(S,l) accepts iff vector lengths agree, every parent chain stays in range
and terminates, root sizes equal observed membership cardinalities, c equals
the number of distinct labels, and root equality iff label equality for every
ordered pair. The expected merge replaces all occurrences of l[b] by l[a].
This representation differs from parent forests and independently specifies
the quotient partition. A does not require particular historical non-root
sizes or tie-selected representatives; dedicated tests cover those omissions.
Original wrong-merge and wrong-count controls reject corrupted states.

## history-obligations

Type operation_history_relation. For fixed call history H, each successful
union coarsens exactly its two classes; queries preserve partition; invalid
calls preserve the entire before-state and return the first invalid argument.
The larger seeded test checks after every step. In the balanced history,
all six ancestors of leaf 63 must become root zero after find, root sizes
and count stay fixed, and later invalid first/second/both arguments preserve
the compressed forest including still-deep branches 31 and 47.
For a fixed undirected edge set, permuting edges and reversing arguments
preserves the final connected partition and count. Repeated edges return
false; raw forest equivalence is deliberately not an obligation.
These are bounded empirical correspondences, not all-history proofs.

## bounded-coverage

Type bounded_coverage_cost. The exact larger sampled domain is nine traces:
n in {4,17,64}, seed in {1,0x51a7,0xdead_beef}, each 600 calls, hence 5,400.
A wrapping u64 linear-congruential generator selects operations and arguments
from 0..n+2 using upper bits. A 64-element balanced union history creates
63 successful unions and a six-edge ancestor path. Invalid probes use
n and usize::MAX in first/second/both positions after compression. Three
seeded permutations of 28 edges at n=32 preserve four eight-member classes.
This is deterministic sampling, not exhaustive coverage of larger domains
or a statistical confidence estimate. The small exhaustive depth-three
domain is retained. No timings are used as a performance claim.

## tests

Cargo test -p mova-algorithms disjoint_set::tests, one harness
thread, passes 10/10 twice. rustfmt --check and cargo clippy -p mova-algorithms
--all-targets -- -D warnings pass. Compiled external scratch source mutation
replaces self.parent[index] = root with self.parent[index] = next; running
only the NEW balanced_tree_compression_and_errors_preserve_full_state test
fails at uncompressed node 63 (62 != 0), exit 101. No mutant is retained
in repository source. Existing state-corruption controls are separate evidence.
Modelcheck syntax is partial Rust syntax, with opaque macro bodies; structural
validation does not certify these mathematical claims.

## descriptant

Composition requires the production API and test oracle to be compiled
together. The test leaf is a sibling of the production leaf under the library,
with model references to production semantics; tests are not runtime behavior.
Epoch 1 retains seven original tests and adds three targeted coverage axes.
Observation: a no-compression mutant can preserve every partition answer,
so a successful membership oracle alone cannot validate compression.
Likewise invalid calls can silently alter shape while preserving all answers;
full raw-state snapshots are required for their frame guarantee.

## History

### 2026-10-29 · document

Revision 1, 2026-10-29. Source identity is the manifest hash. Tests own the
finite experimental domain; the sibling production groupant owns API behavior.
Seven original checks are retained and three larger checks added.

### 2026-10-29 · tests

2026-10-29: cargo test -p mova-algorithms disjoint_set::tests, one harness
thread, passes 10/10 twice. rustfmt --check and cargo clippy -p mova-algorithms
--all-targets -- -D warnings pass. Compiled external scratch source mutation
replaces self.parent[index] = root with self.parent[index] = next; running
only the NEW balanced_tree_compression_and_errors_preserve_full_state test
fails at uncompressed node 63 (62 != 0), exit 101. No mutant is retained
in repository source. Existing state-corruption controls are separate evidence.
Modelcheck syntax is partial Rust syntax, with opaque macro bodies; structural
validation does not certify these mathematical claims.
