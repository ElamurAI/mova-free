# Disjoint-set groupant

## forward

State S=(p,s,c) has a fixed universe V=0..n, parent forest p, root sizes s
(non-root sizes are historical), and component count c. Pi(S) partitions V by
root equality. new(n) returns singleton roots with size one and c=n.
Validation precedes all mutations; for binary calls the first invalid argument
in argument order supplies IndexError { index, len: n }. Invalid calls return
the exact original S. Valid find(i) returns its root and rewrites all ancestors
on i's path to that root. connected(a,b) compresses a then b and returns root
equality. union(a,b) compresses a then b; equal roots return false. Otherwise
the larger root size wins, with smaller root index breaking ties, its size gains
the losing root's size, the losing root points to it, c decreases by one, and
the result is true. Allocation failure and process interruption are outside R.

## invariant

Every parent is in V; iteration reaches a self-parent within n steps.
For each root r, s[r]=|{i:root(i)=r}|. The number of self-parents is c.
0<=c<=n, with c=0 iff n=0. Every union coarsens the partition, and find and
connected preserve it. A failed call preserves both vectors and the count,
including paths that otherwise could have been compressed. Root sizes cannot
overflow usize because disjoint merged classes contain at most n members.

## cost

Prospective model: construction uses O(n) time and two O(n) vectors.
Union by size gives depth at most floor(log2(n)) before compression; each move
under a different root at least doubles that component's size. Full compression
never increases depths. A single operation therefore costs O(log(n+1)) worst
case, invalid calls O(1); classical union-by-size plus compression gives
O(n + m alpha(n)) amortized total for m valid operations. No runtime benchmark
or empirical asymptotic fit is claimed. Operations allocate no extra heap
storage, and iterative root traversal uses O(1) stack space.

## behavior-complement

Type behavior_complement; single-call raw-state observation. Fix n.
U contains tuples (S,op,result,T) where S is an API-reachable state, op is
find/connected/union with usize arguments, result has the operation's result
type, and T is arbitrary vectors and count with length n.
R is precisely the transition rule in forward. The exact target is U minus R.
The useful inner approximant B is the union of: invalid calls with T!=S or
the wrong first invalid index; valid calls whose reported representative or
Boolean disagrees with Pi(S); calls that split a class; queries changing Pi;
a union identifying any classes beyond its two input classes; and union counts
other than c-1 for distinct classes or c otherwise. Each condition excludes a
legal forward transition. B need not exclude every wrong parent arrangement,
so B is not asserted equal to U minus R. Independent finite partition and
atomic-error tests support the stated inner bound, not all-machine proof.

## reverse-reconstruction

Type reverse_reconstruction; observation is the final partition Q, not all
parent pointers. With fixed n and operation op define
Pre(Q,op)={S reachable at n : Pi(T)=Q for some result with (S,op,result,T) in R}.
For a valid query or invalid call this is every reachable S with Pi(S)=Q.
For valid union(a,b), compatible antecedent partitions include Q itself when
a,b are together, and every partition obtained by splitting their Q block into
exactly two nonempty blocks separating a and b; all other blocks stay fixed.
If Q separates a,b there is no antecedent. For each candidate partition retain
every reachable parent forest and historical non-root size array realizing it.
This is a declarative set-valued preimage, not an enumerator or unique inverse.
Projection loses compression history, merge order, and representative choices.
The executable ambiguity witness uses reachable depth-two and compressed
forests with identical partitions. It proves nonuniqueness for one fiber;
completeness of unrestricted reachable-forest enumeration remains untested.

## partition

Type partition_transition; hold n, input partition and arguments fixed and
observe the output partition, result and c. For valid union(a,b), Q is the
least equivalence relation containing P and (a,b); changed iff a and b lie in
different P blocks, and c'=c-[changed]. Queries preserve P. Independent label
relabeling implements this quotient rule without using parent traversal to
compute expected merges. Exhaustive traces cover every find, connected, union
argument in 0..=n through length three at n=0..3. This is exact for the quotient
specification and bounded empirical correspondence to Rust.

## frame

Type mutation_frame; fixed raw S and operation. An invalid operation has
T=S exactly, even if a valid first argument has a nontrivial ancestor path.
Valid find changes only parent entries on its original ancestor path; sizes
and c are untouched. connected composes two such path frames. A valid distinct
union additionally changes the losing root parent, winning root size and c;
other sizes are historical and unchanged. These source-derived frame bounds
are conservative, since writing an already-correct parent produces no change.
Tests compare full state on errors and query size/count/unrelated storage.
The frame identifies a bug hidden by partition-only observation: compressing
the first argument before validating the second.

## tie

Type deterministic_representative; fixed pre-union roots ra,rb and root sizes.
If sizes differ choose argmax size. If equal choose min(ra,rb).
An unequal-size union may select a root larger than some component member;
a global-minimum representative specification is therefore false. Compression
does not alter the root. The finite tie test checks a descending singleton
pair and a later merge of smaller index zero into that larger component.
The rule is exact on the specified state domain; different union histories
may still yield different roots for the same final partition.

## tests

Epoch 1: standalone rustc --test suite passed all seven original tests.
Epoch 2: tests moved unchanged to sibling tests.rs and three larger tests added.
All ten focused Cargo tests pass twice; rustfmt and Clippy -D warnings pass.
Production logic is unchanged; only the test module declaration moved.
New evidence covers 5,400 mixed calls at n=4,17,64; a depth-six balanced tree
with full compression and post-compression invalid-call atomicity; and three
permuted/oriented/repeated edge histories at n=32. A compiled scratch mutant
that retains the next parent instead of compressing to the root is rejected by
the new balanced-tree test (node 63 parent 62 rather than 0; exit 101).
See tests.rs.groupant for bounded test-oracle models and coverage limits.
The exhaustive depth-three mixed-operation trace oracle includes n=0 and
one invalid index per n. Separate cases include usize::MAX, repeated unions,
self unions, second-index atomicity, actual compression and root tie behavior.
The negative control mutates the chosen merge and separately the component
count; the independent oracle rejects both. It checks parent bounds, cycles,
root sizes, all pairwise memberships and component count. This is bounded
testing, not proof. The source syntax projection omits macro expansion and
semantic resolution; structural model checking does not validate mathematics.

## descriptant

Composition: source/partial AST supplies the operational transition, forward
and invariants constrain legal states, the complement rejects forbidden
transitions, reverse reconstructs compatible hidden forests, and partition,
frame and tie observations distinguish separate failure modes. None alone
proves the full implementation. Initial epoch observation: partition equality
misses path-compression side effects on an invalid second argument, so tests
must preserve and compare the complete state. The reverse witness shows why
a partition cannot reconstruct a unique forest. Performance bounds are
prospective, with allocation and process-failure semantics explicitly omitted.

## History

### 2026-10-29 · document

Original implementation; no external exercise text or solutions. Revision 2,
2026-10-29. The adjacent manifest binds every model to the formatted source hash.

### 2026-10-29 · tests

2026-10-29 epoch 1: standalone rustc --test suite passed all seven original tests.
Epoch 2: tests moved unchanged to sibling tests.rs and three larger tests added.
All ten focused Cargo tests pass twice; rustfmt and Clippy -D warnings pass.
Production logic is unchanged; only the test module declaration moved.
New evidence covers 5,400 mixed calls at n=4,17,64; a depth-six balanced tree
with full compression and post-compression invalid-call atomicity; and three
permuted/oriented/repeated edge histories at n=32. A compiled scratch mutant
that retains the next parent instead of compressing to the root is rejected by
the new balanced-tree test (node 63 parent 62 rather than 0; exit 101).
See tests.rs.groupant for bounded test-oracle models and coverage limits.
The exhaustive depth-three mixed-operation trace oracle includes n=0 and
one invalid index per n. Separate cases include usize::MAX, repeated unions,
self unions, second-index atomicity, actual compression and root tie behavior.
The negative control mutates the chosen merge and separately the component
count; the independent oracle rejects both. It checks parent bounds, cycles,
root sizes, all pairwise memberships and component count. This is bounded
testing, not proof. The source syntax projection omits macro expansion and
semantic resolution; structural model checking does not validate mathematics.
