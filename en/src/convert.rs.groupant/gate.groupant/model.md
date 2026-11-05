# en-convert-gate

## descriptant

### Current wave25 model

This section supersedes implementation/cost claims in retained earlier epochs below. The current gate preserves ordered row parsing, range/root-label checks, nonempty root count, then least nonterminating start. For n<=64, ordered head walks remain. For n>64, color[0]=2; color 0 is unvisited, 1 is the current head path, 2 is certified terminating. An unseen path reaches 2 or revisits 1; only the latter errors. A successful path is recolored 2 by following its preserved heads. Earlier completed paths contain no color 1, so cycles are reported for the least original start, including tails. Reverse adjacency remains a mathematical equivalence, not a runtime dependency.

Each vertex is followed at most once while coloring 1 and once during cleanup. Cycle phase O(n), at most 2n head transitions and O(n) scratch. Parsing retains its O(n) heads Vec; valid large calls allocate exactly heads plus n+1 colors, with no row/queue allocation. Tiny calls allocate only heads (empty input none). Diagnostic String construction adds allocation on errors. A successful Vec<usize> allocation bounds n by isize::MAX/size_of::<usize>(), so n+1 and bounded tiny step increments cannot overflow; no saturating sentinel arithmetic. Allocation failure behavior remains allocator-defined and outside the normal-return relation.

Behavior complement excludes false success for a cycle, erroneous Cycle(k) on a previously completed suffix, and selecting a cycle member instead of its earlier tail. Reverse reconstruction retains all head functions with the returned first failure and arbitrary lexical fields; star and chain successes are distinct compatible preimages. Existing four typed implementation specializations below remain mathematical relations; resource-refinement now uses coloring rather than runtime BFS. Tests retain legacy and exact transfer-parent references, all 126,126 head maps through n=6, boundary 63/64/65/66/128/1024 shapes and completed-suffix/later-parse-error witnesses. No semantic bug was found and no old-code-failing correctness claim is made. Ignored benchmark pairs compare both references with uninstrumented production. Categorical exact semantics have no useful approximate percentage profile; no executable surrogate is claimed.


#### Retained earlier modeling epochs

### en-convert

Dialect conversion rule parsing, selected edits, differences and structural tree validation.

#### Mathematical combiner

Combine source-bound partial syntax, the forward relation, typed obernants, invariants and finite checks. Restrict the forward relation by stage guards; compose the guarded graph adapter with BFS and project the first unreachable token. Use observed disagreement to revise the adapter or model. The source is authoritative; missing macro expansion, name resolution and full legacy semantic coverage remain explicit.

#### Hybrid correction after exploratory timing

The initial unconditional-BFS implementation preserved semantics but root's baseline-first exploratory pilot observed converter star regressions of 1.95–4.21x and n=8/32 chain regressions of 3.33x/2.03x. Those observations motivated an amendment, not a final performance claim. The original epoch remains recorded. The amended hybrid retains old small and cheap walks, then applies BFS only after a linear work budget. Benchmark methodology now uses three warmup pairs and seven alternating-order samples; no amended timings are claimed here.

## approximant

### Current wave25 model

This section supersedes implementation/cost claims in retained earlier epochs below. The current gate preserves ordered row parsing, range/root-label checks, nonempty root count, then least nonterminating start. For n<=64, ordered head walks remain. For n>64, color[0]=2; color 0 is unvisited, 1 is the current head path, 2 is certified terminating. An unseen path reaches 2 or revisits 1; only the latter errors. A successful path is recolored 2 by following its preserved heads. Earlier completed paths contain no color 1, so cycles are reported for the least original start, including tails. Reverse adjacency remains a mathematical equivalence, not a runtime dependency.

Each vertex is followed at most once while coloring 1 and once during cleanup. Cycle phase O(n), at most 2n head transitions and O(n) scratch. Parsing retains its O(n) heads Vec; valid large calls allocate exactly heads plus n+1 colors, with no row/queue allocation. Tiny calls allocate only heads (empty input none). Diagnostic String construction adds allocation on errors. A successful Vec<usize> allocation bounds n by isize::MAX/size_of::<usize>(), so n+1 and bounded tiny step increments cannot overflow; no saturating sentinel arithmetic. Allocation failure behavior remains allocator-defined and outside the normal-return relation.

Behavior complement excludes false success for a cycle, erroneous Cycle(k) on a previously completed suffix, and selecting a cycle member instead of its earlier tail. Reverse reconstruction retains all head functions with the returned first failure and arbitrary lexical fields; star and chain successes are distinct compatible preimages. Existing four typed implementation specializations below remain mathematical relations; resource-refinement now uses coloring rather than runtime BFS. Tests retain legacy and exact transfer-parent references, all 126,126 head maps through n=6, boundary 63/64/65/66/128/1024 shapes and completed-suffix/later-parse-error witnesses. No semantic bug was found and no old-code-failing correctness claim is made. Ignored benchmark pairs compare both references with uninstrumented production. Categorical exact semantics have no useful approximate percentage profile; no executable surrogate is claimed.


#### Retained earlier modeling epochs

### Forward approximant

Rule parsing produces typed actions or source diagnostics. Pair selection preserves file order. apply builds binding edits, rejects conflicts, applies ordered rules and runs tree_gate; see source for detailed action semantics. tree_gate parses each head, rejects range then root-label mismatch in token order, checks root count only for nonempty input, then reports minimum nonterminating word. Detailed rule-language semantics remain a partial model, not verified by the BFS transfer.

For the transferred gate, after earlier predicates pass, let H(i) be the head and H(0)=0. Success iff every i in 1..=n reaches 0 under iteration of H. Otherwise return the least i whose orbit never reaches 0. Reversed adjacency satisfies G[h]={i:H(i)=h}. Reachability from 0 in G is equivalent to termination under H. Correct leaves alone do not establish the adapter; exhaustive differential checks test its wiring.

Scope remains partial for unmodified parsing/formatting/action details; no semantic AST or full application proof is claimed.


## invariant

### Current wave25 model

This section supersedes implementation/cost claims in retained earlier epochs below. The current gate preserves ordered row parsing, range/root-label checks, nonempty root count, then least nonterminating start. For n<=64, ordered head walks remain. For n>64, color[0]=2; color 0 is unvisited, 1 is the current head path, 2 is certified terminating. An unseen path reaches 2 or revisits 1; only the latter errors. A successful path is recolored 2 by following its preserved heads. Earlier completed paths contain no color 1, so cycles are reported for the least original start, including tails. Reverse adjacency remains a mathematical equivalence, not a runtime dependency.

Each vertex is followed at most once while coloring 1 and once during cleanup. Cycle phase O(n), at most 2n head transitions and O(n) scratch. Parsing retains its O(n) heads Vec; valid large calls allocate exactly heads plus n+1 colors, with no row/queue allocation. Tiny calls allocate only heads (empty input none). Diagnostic String construction adds allocation on errors. A successful Vec<usize> allocation bounds n by isize::MAX/size_of::<usize>(), so n+1 and bounded tiny step increments cannot overflow; no saturating sentinel arithmetic. Allocation failure behavior remains allocator-defined and outside the normal-return relation.

Behavior complement excludes false success for a cycle, erroneous Cycle(k) on a previously completed suffix, and selecting a cycle member instead of its earlier tail. Reverse reconstruction retains all head functions with the returned first failure and arbitrary lexical fields; star and chain successes are distinct compatible preimages. Existing four typed implementation specializations below remain mathematical relations; resource-refinement now uses coloring rather than runtime BFS. Tests retain legacy and exact transfer-parent references, all 126,126 head maps through n=6, boundary 63/64/65/66/128/1024 shapes and completed-suffix/later-parse-error witnesses. No semantic bug was found and no old-code-failing correctness claim is made. Ignored benchmark pairs compare both references with uninstrumented production. Categorical exact semantics have no useful approximate percentage profile; no executable surrogate is claimed.


#### Retained earlier modeling epochs

### Invariant and cost

All adapter endpoints lie in 0..=n before BFS is invoked. The artificial source exists even for n=0. Each word contributes exactly one edge, with unchanged source records. BFS reachability is projected by ascending original token index, preserving tails and first-error order. Annotation rejects empty input and self-heads before BFS; conversion accepts empty input and diagnoses self-heads through the cycle stage. Earlier local errors remain ordered and untouched.

Cycle-stage cost changes from O(n²) worst-case time/O(1) auxiliary space to O(n) time/O(n) auxiliary space. Converter already allocates its heads vector. Registry string/feature checks and converter head parsing keep their existing costs. Allocation failures remain Rust Vec behavior; no throughput claim follows from asymptotics.


## catalog

# Current wave25 model

This section supersedes implementation/cost claims in retained earlier epochs below. The current gate preserves ordered row parsing, range/root-label checks, nonempty root count, then least nonterminating start. For n<=64, ordered head walks remain. For n>64, color[0]=2; color 0 is unvisited, 1 is the current head path, 2 is certified terminating. An unseen path reaches 2 or revisits 1; only the latter errors. A successful path is recolored 2 by following its preserved heads. Earlier completed paths contain no color 1, so cycles are reported for the least original start, including tails. Reverse adjacency remains a mathematical equivalence, not a runtime dependency.

Each vertex is followed at most once while coloring 1 and once during cleanup. Cycle phase O(n), at most 2n head transitions and O(n) scratch. Parsing retains its O(n) heads Vec; valid large calls allocate exactly heads plus n+1 colors, with no row/queue allocation. Tiny calls allocate only heads (empty input none). Diagnostic String construction adds allocation on errors. A successful Vec<usize> allocation bounds n by isize::MAX/size_of::<usize>(), so n+1 and bounded tiny step increments cannot overflow; no saturating sentinel arithmetic. Allocation failure behavior remains allocator-defined and outside the normal-return relation.

Behavior complement excludes false success for a cycle, erroneous Cycle(k) on a previously completed suffix, and selecting a cycle member instead of its earlier tail. Reverse reconstruction retains all head functions with the returned first failure and arbitrary lexical fields; star and chain successes are distinct compatible preimages. Existing four typed implementation specializations below remain mathematical relations; resource-refinement now uses coloring rather than runtime BFS. Tests retain legacy and exact transfer-parent references, all 126,126 head maps through n=6, boundary 63/64/65/66/128/1024 shapes and completed-suffix/later-parse-error witnesses. No semantic bug was found and no old-code-failing correctness claim is made. Ignored benchmark pairs compare both references with uninstrumented production. Categorical exact semantics have no useful approximate percentage profile; no executable surrogate is claimed.


## Retained earlier modeling epochs

# Typed obernants

## behavior-complement
Fix the one-call universe U of well-typed finite inputs and tagged normal returns, with successful allocation and no environmental interruption. Let R be the forward relation, including first-error precedence and all retained source behavior. C=U\R excludes wrong successful output, false acceptance, incorrect error kind/index/message, and a later error masking an earlier one. This is a partial source-derived model of absent behavior; finite negative tests do not define the complement. Allocation aborts, arbitrary external I/O failures and timings are outside U.

## reverse-reconstruction
Pre(y)={x | R(x,y)} retains every compatible input. A successful gate accepts all inputs passing local predicates with every head chain reaching 0; token words/features and tree shape are not uniquely recoverable. A cycle-i fiber requires all earlier gates pass, all starts j<i terminate and i does not. Error fibers retain every input with that exact first failure, including arbitrary unexamined later fields. This is a relation, not an executable inverse.

## reachability-preimage
For a fixed unreachable set W, compatible validated head maps satisfy i in W iff its head is in W, with root 0 outside W; every node outside W has a finite chain to 0. Every finite nonempty closed W component contains a cycle, but W may include tails. This variant forgets distance and lexical fields.

## error-precedence
The first-failure index is conditional on all preceding stage predicates succeeding. Altering any earlier form/head/registry/root condition may change the returned diagnostic regardless of unchanged cycle structure. This is a precedence relation, distinct from a reachability fiber.

## representation-equivalence
Permuting adjacency children preserves BFS distances and the gate output; changing word numbering generally changes the first error and is not an unconditional equivalence. Both input records remain immutable during validation.

## resource-refinement
After early gates, V=n+1 and E=n. BFS is O(n) time with O(n) allocated adjacency, queue and distances; prior head chasing is O(n²) worst time with O(1) auxiliary cycle-phase storage. This relation models resource order only, not a universal measured speedup or error budget.


## tests

### Current wave25 model

This section supersedes implementation/cost claims in retained earlier epochs below. The current gate preserves ordered row parsing, range/root-label checks, nonempty root count, then least nonterminating start. For n<=64, ordered head walks remain. For n>64, color[0]=2; color 0 is unvisited, 1 is the current head path, 2 is certified terminating. An unseen path reaches 2 or revisits 1; only the latter errors. A successful path is recolored 2 by following its preserved heads. Earlier completed paths contain no color 1, so cycles are reported for the least original start, including tails. Reverse adjacency remains a mathematical equivalence, not a runtime dependency.

Each vertex is followed at most once while coloring 1 and once during cleanup. Cycle phase O(n), at most 2n head transitions and O(n) scratch. Parsing retains its O(n) heads Vec; valid large calls allocate exactly heads plus n+1 colors, with no row/queue allocation. Tiny calls allocate only heads (empty input none). Diagnostic String construction adds allocation on errors. A successful Vec<usize> allocation bounds n by isize::MAX/size_of::<usize>(), so n+1 and bounded tiny step increments cannot overflow; no saturating sentinel arithmetic. Allocation failure behavior remains allocator-defined and outside the normal-return relation.

Behavior complement excludes false success for a cycle, erroneous Cycle(k) on a previously completed suffix, and selecting a cycle member instead of its earlier tail. Reverse reconstruction retains all head functions with the returned first failure and arbitrary lexical fields; star and chain successes are distinct compatible preimages. Existing four typed implementation specializations below remain mathematical relations; resource-refinement now uses coloring rather than runtime BFS. Tests retain legacy and exact transfer-parent references, all 126,126 head maps through n=6, boundary 63/64/65/66/128/1024 shapes and completed-suffix/later-parse-error witnesses. No semantic bug was found and no old-code-failing correctness claim is made. Ignored benchmark pairs compare both references with uninstrumented production. Categorical exact semantics have no useful approximate percentage profile; no executable surrogate is claimed.


#### Retained earlier modeling epochs

### Checks

Manual benchmark only: cargo test -p annot --release bfs_transfer_benchmark -- --ignored --nocapture --test-threads=1, then equivalent -p en. It compares full gates on fixed synthetic chains/stars, with setup outside timing and black_box inputs/results. No model or corpus access. Timing remains deferred; no performance result is asserted.


#### Hybrid checks

A fourth active-fallback test per consumer covers the 64/65 threshold and a long rooted prefix exhausting the budget before a later cycle. Reverse-projection mutants run this test rather than the small precedence fixture, which now correctly takes the old-walk branch. Eight tests pass twice debug/release; original focused tests pass twice. The ignored benchmark uses 3 warmup pairs and 7 alternating-order samples at n=8,32,64,65,128,512,2048; setup is excluded. No timings run by this worker.

## History

### 2026-10-30 · descriptant

Heading `### Current wave25 model — 2026-10-30` stays in descriptant as `### Current wave25 model`.

#### Epoch 2026-10-30

Prospective prediction: preserve exact errors and reduce worst-case cycle-phase work, paying linear memory. Original code is retained externally and in the test-only baseline. Exhaustive head maps, named precedence fixtures, actual compiled semantic mutants, and paired syntax projections are used; see model.md#tests for completed evidence. Timings remain deferred to a serial run. No earlier file-model epoch existed; legacy behavior is summarized rather than claimed fully verified.

### 2026-10-30 · approximant

Heading `### Current wave25 model — 2026-10-30` stays in approximant as `### Current wave25 model`.

#### Hybrid amendment 2026-10-30

For n<=64 the original ordered cycle walks run unchanged. For n>64, ordered walks share a saturating 4*n step budget. A detected cycle returns the original diagnostic immediately. Completion returns Ok without BFS. Exhaustion discards partial reachability work and invokes the exact reverse-head BFS adapter over the whole input, preserving the least failing start even when exhaustion happened on an earlier rooted prefix. Prior input/head/root predicates are unchanged. Budget exhaustion is not an error and never implies a cycle.

Cost: tiny input work is bounded by 65*n; larger prefixes perform at most 4*n head steps before O(n) BFS. Thus worst-case cycle-phase time is O(n). Auxiliary space is constant for the walk-only branch and O(n) on fallback; converter's preexisting heads vector remains O(n). Saturation cannot increase work beyond the mathematical 4*n bound. Stars complete their walks without adjacency allocation. No universal throughput gain follows from this bound.

### 2026-10-30 · invariant

Heading `### Current wave25 model — 2026-10-30` stays in invariant as `### Current wave25 model`.

#### Hybrid amendment 2026-10-30

For n<=64 the original ordered cycle walks run unchanged. For n>64, ordered walks share a saturating 4*n step budget. A detected cycle returns the original diagnostic immediately. Completion returns Ok without BFS. Exhaustion discards partial reachability work and invokes the exact reverse-head BFS adapter over the whole input, preserving the least failing start even when exhaustion happened on an earlier rooted prefix. Prior input/head/root predicates are unchanged. Budget exhaustion is not an error and never implies a cycle.

Cost: tiny input work is bounded by 65*n; larger prefixes perform at most 4*n head steps before O(n) BFS. Thus worst-case cycle-phase time is O(n). Auxiliary space is constant for the walk-only branch and O(n) on fallback; converter's preexisting heads vector remains O(n). Saturation cannot increase work beyond the mathematical 4*n bound. Stars complete their walks without adjacency allocation. No universal throughput gain follows from this bound.

### 2026-10-30 · document

Heading `# Current wave25 model — 2026-10-30` stays in document as `# Current wave25 model`.

#### Hybrid amendment 2026-10-30

For n<=64 the original ordered cycle walks run unchanged. For n>64, ordered walks share a saturating 4*n step budget. A detected cycle returns the original diagnostic immediately. Completion returns Ok without BFS. Exhaustion discards partial reachability work and invokes the exact reverse-head BFS adapter over the whole input, preserving the least failing start even when exhaustion happened on an earlier rooted prefix. Prior input/head/root predicates are unchanged. Budget exhaustion is not an error and never implies a cycle.

Cost: tiny input work is bounded by 65*n; larger prefixes perform at most 4*n head steps before O(n) BFS. Thus worst-case cycle-phase time is O(n). Auxiliary space is constant for the walk-only branch and O(n) on fallback; converter's preexisting heads vector remains O(n). Saturation cannot increase work beyond the mathematical 4*n bound. Stars complete their walks without adjacency allocation. No universal throughput gain follows from this bound.

### 2026-10-30 · tests

Heading `### Current wave25 model — 2026-10-30` stays in tests as `### Current wave25 model`.

2026-10-30: three transfer tests per consumer: exhaustive 126126 head maps (n=0..6), precedence fixtures and long chain/star/cycle-tail shapes (n=8,64,1024). Compare exact production Result to the retained old implementation. Debug and release execute twice; existing ann and convert unit tests execute twice. Source mutant reverses the final scan, causing the tail-first fixture to report word/token 4 instead of 1; reject the compiled mutant and restore source before passing the named test again. Evidence hashes are recorded after execution. Paired syntax JSON outputs must match byte-for-byte. These finite checks are not a proof for arbitrary inputs.
