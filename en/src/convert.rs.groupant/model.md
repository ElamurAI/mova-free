# en-convert

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


## composition

### Current wave25 model

This section supersedes implementation/cost claims in retained earlier epochs below. The current gate preserves ordered row parsing, range/root-label checks, nonempty root count, then least nonterminating start. For n<=64, ordered head walks remain. For n>64, color[0]=2; color 0 is unvisited, 1 is the current head path, 2 is certified terminating. An unseen path reaches 2 or revisits 1; only the latter errors. A successful path is recolored 2 by following its preserved heads. Earlier completed paths contain no color 1, so cycles are reported for the least original start, including tails. Reverse adjacency remains a mathematical equivalence, not a runtime dependency.

Each vertex is followed at most once while coloring 1 and once during cleanup. Cycle phase O(n), at most 2n head transitions and O(n) scratch. Parsing retains its O(n) heads Vec; valid large calls allocate exactly heads plus n+1 colors, with no row/queue allocation. Tiny calls allocate only heads (empty input none). Diagnostic String construction adds allocation on errors. A successful Vec<usize> allocation bounds n by isize::MAX/size_of::<usize>(), so n+1 and bounded tiny step increments cannot overflow; no saturating sentinel arithmetic. Allocation failure behavior remains allocator-defined and outside the normal-return relation.

Behavior complement excludes false success for a cycle, erroneous Cycle(k) on a previously completed suffix, and selecting a cycle member instead of its earlier tail. Reverse reconstruction retains all head functions with the returned first failure and arbitrary lexical fields; star and chain successes are distinct compatible preimages. Existing four typed implementation specializations below remain mathematical relations; resource-refinement now uses coloring rather than runtime BFS. Tests retain legacy and exact transfer-parent references, all 126,126 head maps through n=6, boundary 63/64/65/66/128/1024 shapes and completed-suffix/later-parse-error witnesses. No semantic bug was found and no old-code-failing correctness claim is made. Ignored benchmark pairs compare both references with uninstrumented production. Categorical exact semantics have no useful approximate percentage profile; no executable surrogate is claimed.


#### Retained earlier modeling epochs

### File composition

The gate fragment models the whole ordered validation pipeline, not only BFS. Earlier predicates prevent invalid adapter endpoints. The parent retains parsing, serialization or conversion context and explicitly partial legacy semantics. Annotation parsing invokes check only after constructing typed tokens; conversion invokes tree_gate after rule edits. No child result alone proves parser/action correctness or caller lifecycle semantics. The child uses the parent's whole-file syntax projection with an explicit gate byte range.

## catalog

# Current wave25 model

This section supersedes implementation/cost claims in retained earlier epochs below. The current gate preserves ordered row parsing, range/root-label checks, nonempty root count, then least nonterminating start. For n<=64, ordered head walks remain. For n>64, color[0]=2; color 0 is unvisited, 1 is the current head path, 2 is certified terminating. An unseen path reaches 2 or revisits 1; only the latter errors. A successful path is recolored 2 by following its preserved heads. Earlier completed paths contain no color 1, so cycles are reported for the least original start, including tails. Reverse adjacency remains a mathematical equivalence, not a runtime dependency.

Each vertex is followed at most once while coloring 1 and once during cleanup. Cycle phase O(n), at most 2n head transitions and O(n) scratch. Parsing retains its O(n) heads Vec; valid large calls allocate exactly heads plus n+1 colors, with no row/queue allocation. Tiny calls allocate only heads (empty input none). Diagnostic String construction adds allocation on errors. A successful Vec<usize> allocation bounds n by isize::MAX/size_of::<usize>(), so n+1 and bounded tiny step increments cannot overflow; no saturating sentinel arithmetic. Allocation failure behavior remains allocator-defined and outside the normal-return relation.

Behavior complement excludes false success for a cycle, erroneous Cycle(k) on a previously completed suffix, and selecting a cycle member instead of its earlier tail. Reverse reconstruction retains all head functions with the returned first failure and arbitrary lexical fields; star and chain successes are distinct compatible preimages. Existing four typed implementation specializations below remain mathematical relations; resource-refinement now uses coloring rather than runtime BFS. Tests retain legacy and exact transfer-parent references, all 126,126 head maps through n=6, boundary 63/64/65/66/128/1024 shapes and completed-suffix/later-parse-error witnesses. No semantic bug was found and no old-code-failing correctness claim is made. Ignored benchmark pairs compare both references with uninstrumented production. Categorical exact semantics have no useful approximate percentage profile; no executable surrogate is claimed.


## Retained earlier modeling epochs

# Typed obernants

## behavior-complement
Fix the one-call universe U of well-typed finite inputs and tagged normal returns, with successful allocation and no environmental interruption. Let R be the forward relation, including first-error precedence and all retained source behavior. C=U\R excludes wrong successful output, false acceptance, incorrect error kind/index/message, and a later error masking an earlier one. This is a partial source-derived model of absent behavior; finite negative tests do not define the complement. Allocation aborts, arbitrary external I/O failures and timings are outside U.

For tree_gate, the inner excluded set includes Roots(0) on the admitted empty sentence, a Head-style annotation error substituted for a converter self-cycle diagnostic, and Cycle(3) for heads [3,0,4,3] whose first failing word is 1. A cycle cannot mask earlier head parsing/range/root-label errors. Rule selection/edit semantics remain a separate partial model.

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

## evidence

### Current wave25 model

This section supersedes implementation/cost claims in retained earlier epochs below. The current gate preserves ordered row parsing, range/root-label checks, nonempty root count, then least nonterminating start. For n<=64, ordered head walks remain. For n>64, color[0]=2; color 0 is unvisited, 1 is the current head path, 2 is certified terminating. An unseen path reaches 2 or revisits 1; only the latter errors. A successful path is recolored 2 by following its preserved heads. Earlier completed paths contain no color 1, so cycles are reported for the least original start, including tails. Reverse adjacency remains a mathematical equivalence, not a runtime dependency.

Each vertex is followed at most once while coloring 1 and once during cleanup. Cycle phase O(n), at most 2n head transitions and O(n) scratch. Parsing retains its O(n) heads Vec; valid large calls allocate exactly heads plus n+1 colors, with no row/queue allocation. Tiny calls allocate only heads (empty input none). Diagnostic String construction adds allocation on errors. A successful Vec<usize> allocation bounds n by isize::MAX/size_of::<usize>(), so n+1 and bounded tiny step increments cannot overflow; no saturating sentinel arithmetic. Allocation failure behavior remains allocator-defined and outside the normal-return relation.

Behavior complement excludes false success for a cycle, erroneous Cycle(k) on a previously completed suffix, and selecting a cycle member instead of its earlier tail. Reverse reconstruction retains all head functions with the returned first failure and arbitrary lexical fields; star and chain successes are distinct compatible preimages. Existing four typed implementation specializations below remain mathematical relations; resource-refinement now uses coloring rather than runtime BFS. Tests retain legacy and exact transfer-parent references, all 126,126 head maps through n=6, boundary 63/64/65/66/128/1024 shapes and completed-suffix/later-parse-error witnesses. No semantic bug was found and no old-code-failing correctness claim is made. Ignored benchmark pairs compare both references with uninstrumented production. Categorical exact semantics have no useful approximate percentage profile; no executable surrogate is claimed.


#### Retained earlier modeling epochs

## transfer

### Current wave25 model

This section supersedes implementation/cost claims in retained earlier epochs below. The current gate preserves ordered row parsing, range/root-label checks, nonempty root count, then least nonterminating start. For n<=64, ordered head walks remain. For n>64, color[0]=2; color 0 is unvisited, 1 is the current head path, 2 is certified terminating. An unseen path reaches 2 or revisits 1; only the latter errors. A successful path is recolored 2 by following its preserved heads. Earlier completed paths contain no color 1, so cycles are reported for the least original start, including tails. Reverse adjacency remains a mathematical equivalence, not a runtime dependency.

Each vertex is followed at most once while coloring 1 and once during cleanup. Cycle phase O(n), at most 2n head transitions and O(n) scratch. Parsing retains its O(n) heads Vec; valid large calls allocate exactly heads plus n+1 colors, with no row/queue allocation. Tiny calls allocate only heads (empty input none). Diagnostic String construction adds allocation on errors. A successful Vec<usize> allocation bounds n by isize::MAX/size_of::<usize>(), so n+1 and bounded tiny step increments cannot overflow; no saturating sentinel arithmetic. Allocation failure behavior remains allocator-defined and outside the normal-return relation.

Behavior complement excludes false success for a cycle, erroneous Cycle(k) on a previously completed suffix, and selecting a cycle member instead of its earlier tail. Reverse reconstruction retains all head functions with the returned first failure and arbitrary lexical fields; star and chain successes are distinct compatible preimages. Existing four typed implementation specializations below remain mathematical relations; resource-refinement now uses coloring rather than runtime BFS. Tests retain legacy and exact transfer-parent references, all 126,126 head maps through n=6, boundary 63/64/65/66/128/1024 shapes and completed-suffix/later-parse-error witnesses. No semantic bug was found and no old-code-failing correctness claim is made. Ignored benchmark pairs compare both references with uninstrumented production. Categorical exact semantics have no useful approximate percentage profile; no executable surrogate is claimed.


#### Retained earlier modeling epochs

### Exact BFS transfer

Algorithm API: mova_algorithms::bfs::bfs(&[Vec<usize>], usize) -> Result<Vec<Option<usize>>, BfsError>.

Alpha, after original local/root gates: map H on 1..=n to G on 0..=n by pushing i into G[H(i)], source=0. Alpha ensures valid source/endpoints, so BfsError is unreachable without an adapter defect; the expect documents this invariant. Beta maps successful BFS distances to Ok iff every real token has Some(distance), otherwise the original cycle diagnostic at min{i>=1:d[i]=None}. Distances themselves are discarded; BFS queue ordering does not decide error ties. Root/head/form/registry checks and exact diagnostic text precede the transfer unchanged.

For every admitted H, H^k(i)=0 for some k iff i is reachable from 0 in alpha(H). The backward direction reverses the path; the forward direction reverses the head chain. Finite functional graphs force every nonterminating orbit into a cycle; reporting the least unreachable start preserves tails, not only cycle membership. This yields the original gate relation under successful allocation. No approximate numerical semantics, live training, or global composition bound is introduced.

Annotation empty input is Roots(0), self-head is Head(i). Conversion empty input succeeds, self-head is a cycle; its root-label condition and nonempty root-count condition remain separate. O(n) memory is a deliberate tradeoff; small stars may be slower and must be measured, not assumed.


## wave25-evidence

### Wave25 bounded evidence

Focused suite: five passing tests, two ignored timing tests. Two debug and two release executions have equal sorted named outcomes. The exhaustive test covers 126,126 head vectors through n=6; boundary/long shapes and the new shared-completed-suffix, earliest tail-cycle, and later malformed-head witnesses cover the new branch. Disabling the actual production color-cycle condition fails `wave25_color_paths_and_error_precedence` at equality; exact source restoration passes. No mutant is being presented as an actual baseline bug.

Temporary thread-local GlobalAlloc instrumentation counted every alloc, alloc_zeroed and realloc call during each query, excluding fixture setup. Requested bytes sum allocation requests including full new reallocation size; they are not peak live bytes, allocator overhead or RSS. The wrapper and observation tests were restored before timing. Debug observations:

| Input | Parent calls / bytes | Current calls / bytes |
| --- | ---: | ---: |
| empty | 0 / 0 | 0 / 0 |
| n64 chain | 1 / 512 | 1 / 512 |
| n65 chain | 69 / 5272 | 2 / 586 |
| n128 chain | 132 / 10312 | 2 / 1153 |
| n512 chain | 516 / 41032 | 2 / 4609 |
| n2048 chain | 2052 / 163912 | 2 / 18433 |
| n65 star | 1 / 520 | 2 / 586 |
| n2048 star | 1 / 16384 | 2 / 18433 |

Large stars therefore add one color allocation; exact latency remains to be measured. All allocation-run outputs agree with both references. Four owned source/test syntax projections are paired identical; source SHA and fragment byte scopes are refreshed. Changed-source modelcheck has no missing entries in a temporary registry carrying the four updated leaf records. Global groupant diagnostics are only stale ancestor bindings owned by integration; partial syntax and registration do not prove correctness.

#### Serial confirmation completed

The exclusive-slot measurements are now recorded in [wave25-performance](model.md#wave25-performance). They supersede the pending-timing status above. Both baselines and every observed regression are retained; no production edits followed timing.

## wave25-performance

### Conversion refinement timing

The targeted n65 chain improves 9.01–9.04x against the transfer parent (6.18–6.63x against legacy). Chains n128/512/2048 improve 8.38–9.11x against the parent, retaining and extending long-chain gains. Tradeoffs remain: n2048 star is 0.745–0.750x versus parent (about 33–34% more time), n64 chain 0.803–0.837x, and several other tiny/star cases are slower. The additional color allocation on large stars is real; no broad speed dominance is claimed. The source-identical tiny loop still varies across compiled callers, so the timing evidence is a kernel comparison, not isolation of every instruction-level cause.


Before/after hashes of both binaries and all four source/test files match exactly. World asserts equality of checksums on every pair; conversion asserts full-result equality before each workload and every measured call unwraps successfully. Across two baselines, two executions, 14 conversion plus 16 world shapes, there are 840 measured pairs. Tiny timings are sensitive to code layout, cache and timer effects; unchanged source branches may still differ in compiled context. There is no representative workload distribution, end-to-end school or Mova throughput result, or universal latency guarantee.

| Baseline | Suite | Shape | Median run 1 | Median run 2 | All-pair range |
| --- | --- | --- | ---: | ---: | ---: |
| legacy | en | n=128 chain | 16.650x | 16.839x | 16.255–16.860x |
| legacy | en | n=128 star | 0.900x | 0.913x | 0.893–0.923x |
| legacy | en | n=2048 chain | 230.686x | 231.054x | 223.487–231.505x |
| legacy | en | n=2048 star | 0.706x | 0.749x | 0.695–0.907x |
| legacy | en | n=32 chain | 0.871x | 1.078x | 0.855–1.085x |
| legacy | en | n=32 star | 0.978x | 0.900x | 0.895–1.064x |
| legacy | en | n=512 chain | 61.928x | 61.769x | 61.239–62.378x |
| legacy | en | n=512 star | 0.927x | 0.924x | 0.905–0.932x |
| legacy | en | n=64 chain | 0.727x | 0.777x | 0.713–0.814x |
| legacy | en | n=64 star | 0.976x | 0.978x | 0.953–1.028x |
| legacy | en | n=65 chain | 6.177x | 6.627x | 5.798–6.847x |
| legacy | en | n=65 star | 0.883x | 0.876x | 0.869–0.900x |
| legacy | en | n=8 chain | 0.933x | 0.934x | 0.931–1.133x |
| legacy | en | n=8 star | 0.986x | 0.948x | 0.928–2.228x |
| parent | en | n=128 chain | 9.113x | 8.839x | 8.686–9.249x |
| parent | en | n=128 star | 0.971x | 0.975x | 0.961–0.984x |
| parent | en | n=2048 chain | 8.400x | 8.377x | 8.218–8.471x |
| parent | en | n=2048 star | 0.745x | 0.750x | 0.732–0.784x |
| parent | en | n=32 chain | 0.866x | 0.948x | 0.822–1.030x |
| parent | en | n=32 star | 0.976x | 0.976x | 0.960–1.006x |
| parent | en | n=512 chain | 8.461x | 8.531x | 8.340–8.817x |
| parent | en | n=512 star | 0.963x | 0.987x | 0.955–0.989x |
| parent | en | n=64 chain | 0.837x | 0.803x | 0.764–0.892x |
| parent | en | n=64 star | 1.019x | 1.015x | 0.965–1.031x |
| parent | en | n=65 chain | 9.012x | 9.042x | 8.706–9.083x |
| parent | en | n=65 star | 0.962x | 0.955x | 0.943–0.993x |
| parent | en | n=8 chain | 0.929x | 0.873x | 0.804–2.077x |
| parent | en | n=8 star | 0.954x | 1.003x | 0.932–1.044x |

## wave25-prospective

### Current wave25 model

This section supersedes implementation/cost claims in retained earlier epochs below. The current gate preserves ordered row parsing, range/root-label checks, nonempty root count, then least nonterminating start. For n<=64, ordered head walks remain. For n>64, color[0]=2; color 0 is unvisited, 1 is the current head path, 2 is certified terminating. An unseen path reaches 2 or revisits 1; only the latter errors. A successful path is recolored 2 by following its preserved heads. Earlier completed paths contain no color 1, so cycles are reported for the least original start, including tails. Reverse adjacency remains a mathematical equivalence, not a runtime dependency.

Each vertex is followed at most once while coloring 1 and once during cleanup. Cycle phase O(n), at most 2n head transitions and O(n) scratch. Parsing retains its O(n) heads Vec; valid large calls allocate exactly heads plus n+1 colors, with no row/queue allocation. Tiny calls allocate only heads (empty input none). Diagnostic String construction adds allocation on errors. A successful Vec<usize> allocation bounds n by isize::MAX/size_of::<usize>(), so n+1 and bounded tiny step increments cannot overflow; no saturating sentinel arithmetic. Allocation failure behavior remains allocator-defined and outside the normal-return relation.

Behavior complement excludes false success for a cycle, erroneous Cycle(k) on a previously completed suffix, and selecting a cycle member instead of its earlier tail. Reverse reconstruction retains all head functions with the returned first failure and arbitrary lexical fields; star and chain successes are distinct compatible preimages. Existing four typed implementation specializations below remain mathematical relations; resource-refinement now uses coloring rather than runtime BFS. Tests retain legacy and exact transfer-parent references, all 126,126 head maps through n=6, boundary 63/64/65/66/128/1024 shapes and completed-suffix/later-parse-error witnesses. No semantic bug was found and no old-code-failing correctness claim is made. Ignored benchmark pairs compare both references with uninstrumented production. Categorical exact semantics have no useful approximate percentage profile; no executable surrogate is claimed.


#### Retained earlier modeling epochs

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

### 2026-10-30 · composition

Heading `### Current wave25 model — 2026-10-30` stays in composition as `### Current wave25 model`.

### 2026-10-30 · document

Heading `# Current wave25 model — 2026-10-30` stays in document as `# Current wave25 model`.

#### Hybrid amendment 2026-10-30

For n<=64 the original ordered cycle walks run unchanged. For n>64, ordered walks share a saturating 4*n step budget. A detected cycle returns the original diagnostic immediately. Completion returns Ok without BFS. Exhaustion discards partial reachability work and invokes the exact reverse-head BFS adapter over the whole input, preserving the least failing start even when exhaustion happened on an earlier rooted prefix. Prior input/head/root predicates are unchanged. Budget exhaustion is not an error and never implies a cycle.

Cost: tiny input work is bounded by 65*n; larger prefixes perform at most 4*n head steps before O(n) BFS. Thus worst-case cycle-phase time is O(n). Auxiliary space is constant for the walk-only branch and O(n) on fallback; converter's preexisting heads vector remains O(n). Saturation cannot increase work beyond the mathematical 4*n bound. Stars complete their walks without adjacency allocation. No universal throughput gain follows from this bound.

### 2026-10-30 · tests

Heading `### Current wave25 model — 2026-10-30` stays in tests as `### Current wave25 model`.

2026-10-30: three transfer tests per consumer: exhaustive 126126 head maps (n=0..6), precedence fixtures and long chain/star/cycle-tail shapes (n=8,64,1024). Compare exact production Result to the retained old implementation. Debug and release execute twice; existing ann and convert unit tests execute twice. Source mutant reverses the final scan, causing the tail-first fixture to report word/token 4 instead of 1; reject the compiled mutant and restore source before passing the named test again. Evidence hashes are recorded after execution. Paired syntax JSON outputs must match byte-for-byte. These finite checks are not a proof for arbitrary inputs.

### 2026-10-30 · evidence

Heading `### Current wave25 model — 2026-10-30` stays in evidence as `### Current wave25 model`.

#### Evidence 2026-10-30

Six new tests pass twice in debug and release: each consumer exhausts 126126 head maps, checks exact precedence, and covers nine long-shape fixtures. Two manual benchmarks remain ignored and unmeasured. Existing converter tests: 14 pass twice; existing annotation tests: 1 passes twice. Normalized named test outcomes match across repeated runs (elapsed time and compiler output excluded). Each reverse-scan production mutant compiles, fails the tail-first assertion with exit 101 (4 versus 1), and restored source passes. Groupant validation reported valid=true and no diagnostics, with legacy missing coverage retained. No teacher/model/database/corpus/GPU actions.

Source and baseline hashes:

- annot/src/ann.rs: 045f8efecd2114094439ec08054e3f62be6d8aacfe22fd18b2c4ceb1c25fa210
- en/src/convert.rs: b848de0ce16bf6f621bda9ac9a294730a3412f3ae227b30d47b3e961ca646634
- annot/src/ann.rs.groupant/checks.rs: 01a31faa266ba2f53f2bb4b6391d70c5ac5ecd65b1e9b2b0dc042cdeda8fddec
- en/src/convert.rs.groupant/checks.rs: 2e3bbf17bae35f3922c395785a2f2dfc5e0caa0d35fec196df8ddf1a5e01eb45
- External ann.baseline.rs: e6d149639113b39b087805b32ca7d5b433483dc3a36e6b0a473930a37b46d727
- External convert.baseline.rs: b3e808371759f739f1bf319f63e410d337505b89463ce5d86886abab241a36f5
- External prospective.md: 059ec85fe6b9bef93351c46a6aab285be7b5fb52c878c96ad7aef7e74470c1f0
- External debug-final1.log: d9950f0bff6ca94b7a5449f0ee9cf139d5da89fb8a7e7d79eb79ccc211d6a5d9
- External debug-final2.log: edebb38010a510d66198ea417c9bbd246a574b53ea9b08a3f84a4837f27622c5
- External release-final1.log: 42bea56b01434ab26b9f2772e0346ee9ceefdf418f88066b7eddccb644691619
- External release-final2.log: ba2520448351c36dfde8616c1240f41f5a9a2d38c904857448b4db1d805acffd
- External en-existing1.log: c9924f7db9b5d09fabbdb0dab186d7e6f7be70352dcd5b0e2edec9ffcc5c87b3
- External en-existing2.log: 80f979bc8ad0edaa93eb653adca772f611f5aeb2adef9e8bd492a2f2da9e916d
- External annot-existing1.log: d68d763012771a7e1ac6c55556889cc81550d262733986a7deb1219e2541eec4
- External annot-existing2.log: 962069dc2681c5ef2c70665f9323578a78d9121b5f6b09ff4a619321c0e13f04
- External annot-mutant.log: 535fdb38b65429426d79fb21fcbb8473c9316492557e977a0dbd763873653a49
- External annot-restored.log: 2a8d1dc4533601ad43e1290437ae5d66e23f4543ec544ff2539edd6fe97ea975
- External en-mutant.log: 45cb400480d3296d5d5274c72a669781ef09cf5a1caed93d06ac735308773763
- External en-restored.log: e4318a6db10457d646bad262fc0abc377a2179202856b3ab08dcacbb8fe5642e

Runtime logs and original source snapshots remain outside the repository. Hashes identify the evidence, not an analytical proof. Larger safe follow-up candidates are all en library unit tests and annot ann/repair unit tests; model-dependent or live integration suites were not run.


##### Hybrid evidence 2026-10-30

Eight transfer tests pass twice in debug/release (four per consumer, plus two ignored manual benchmarks). The old exhaustive 126126 vectors per consumer exercise the small-input branch. New cases exercise 64/65 boundaries, rooted chains and stars through n=129, and a 129-word rooted prefix followed by a tail entering a cycle: the budget expires before the first failing token 100. Both compiled reverse-projection mutants are rejected on this active BFS branch (exit 101); restored tests pass. Existing 14 converter and one annotation tests pass twice. Normalized test names/outcomes match in both repetitions. No timing was executed for this amendment.

Current source/evidence hashes:

- annot/src/ann.rs: 459a17bfd13ed4a6ba41719dac5dd97bac86bda51ad0b1ab2e176855c57aee2b
- en/src/convert.rs: b1076c61d8bc50673f47d8040b59c306edc3a934eba4582ab9a67ef9cddd86a6
- annot/src/ann.rs.groupant/checks.rs: 9cd22a2863d0e448e24362a6fc0f64ca97f499663bac1efd4a22544c5f16113a
- en/src/convert.rs.groupant/checks.rs: 773cfb5d98bdab8b3fd45646b404e8c75e1d46ddc9e19ba582f69812a67f0ae2
- hybrid-prospective.md: 2d1324336d1e6fa0f47e8200a8a42dac6c74975031393984bff3577e3f108516
- External hybrid/debug-final1.log: 008ddfbba5a67c8610aee8a15e406a61e8ccf0e662f7f52d39f3ab480dd09c55
- External hybrid/debug-final2.log: d1ec976696aa9a11c33924f6db6d8e3044142c20832a03dbdf25698819e7c308
- External hybrid/release-final1.log: a1ff670b57401e7d02e8b894540878fbd93c76e1b6152cf5a1a17ab1cf18febc
- External hybrid/release-final2.log: c97a901f41b0597ba441b7edc43f07c4fa356d5b1664fe4715b9bb0c2cc0fe89
- External hybrid/annot-mutant.log: 4eeaa4046853ee2b65ae7b9b8cf4b6d702b6124b52990647d517b9f54000339f
- External hybrid/annot-restored.log: 7bb2651295e729140a3b7243df85d3e1eb9cd16a2563f6ec8844b7024ef9fbd8
- External hybrid/en-mutant.log: 6ae354103c923ad4b855d66134a2ab000b585e5d24796aa677e7e7e30ecb6509
- External hybrid/en-restored.log: 8c8293a045400432a8857530acc89616b8119446aeb867b0092693a3c15cd663

### 2026-10-30 · transfer

Heading `### Current wave25 model — 2026-10-30` stays in transfer as `### Current wave25 model`.

Source algorithms/src/bfs/mod.rs SHA-256 bdaab64e535076072d6c65af132c68a3bc0529d0321aed22c6d9d55998246add.

Forward model algorithms/src/bfs/mod.rs.groupant/model.md#approximant SHA-256 9944f3c4228568a106703bd10eb0dffe19871724b5c8a47e494fbbdde7f6cbdd.

Invariant algorithms/src/bfs/mod.rs.groupant/model.md#invariant SHA-256 82f181256a3dc9b90ccad91e8f1163de89ff15e189ce13a9c4ce399d4828c688.

Consumer en/src/convert.rs SHA-256 b848de0ce16bf6f621bda9ac9a294730a3412f3ae227b30d47b3e961ca646634.

#### Current hybrid consumer

Consumer en/src/convert.rs SHA-256 b1076c61d8bc50673f47d8040b59c306edc3a934eba4582ab9a67ef9cddd86a6. The earlier consumer binding above belongs to the preserved plain-BFS epoch. Algorithm source/model hashes remain unchanged. Alpha and beta are now invoked only after n>64 ordered walks exhaust their shared budget. Early successful completion or observed cycle returns the old result. Exhaustion does not alter input state; alpha runs on the entire original head map, so the original reachability proof and earliest-index projection still apply.

#### Hybrid amendment 2026-10-30

For n<=64 the original ordered cycle walks run unchanged. For n>64, ordered walks share a saturating 4*n step budget. A detected cycle returns the original diagnostic immediately. Completion returns Ok without BFS. Exhaustion discards partial reachability work and invokes the exact reverse-head BFS adapter over the whole input, preserving the least failing start even when exhaustion happened on an earlier rooted prefix. Prior input/head/root predicates are unchanged. Budget exhaustion is not an error and never implies a cycle.

Cost: tiny input work is bounded by 65*n; larger prefixes perform at most 4*n head steps before O(n) BFS. Thus worst-case cycle-phase time is O(n). Auxiliary space is constant for the walk-only branch and O(n) on fallback; converter's preexisting heads vector remains O(n). Saturation cannot increase work beyond the mathematical 4*n bound. Stars complete their walks without adjacency allocation. No universal throughput gain follows from this bound.

### 2026-10-30 · wave25-evidence

Heading `### Wave25 bounded evidence — 2026-10-30` stays in wave25-evidence as `### Wave25 bounded evidence`.

Production now colors functional head graphs above n=64; it preserves the tiny ordered walk. APIs and diagnostics are unchanged. No correctness defect was found; this is a cost refinement with differential semantic witnesses. Parent fd540f37badaad97b53d56ba81684bbbc2ca4b9b and the legacy ordered-walk function remain test-only references.

Receipts: `$MOVA_DATA/runs/octopus-wave25-mova-2026-10-30/`: `en-{debug,release}-{1,2}.log`, `repeats.log`, `mutant-en-cycle.log`, `positive-en-cycle.log`, `en-alloc.log`, `en-postalloc.log`, `changed.json`, `groupants.json`. No live teacher, corpus, database, model, GPU or school operation occurred. Serial timing is explicitly pending an exclusive slot; no speedup claim is made here.

### 2026-10-30 · wave25-performance

Heading `### Conversion refinement timing — 2026-10-30` stays in wave25-performance as `### Conversion refinement timing`.

Measurements ran serially in the integration-granted CPU slot, using existing release test binaries, without compilation, allocator instrumentation, fixture setup, external models or IO inside timed calls. CPU Intel Core i5-13500; Rust 1.99.0, thin LTO workspace release profile. Two executions for each suite/baseline pair: eight invocations total. Every invocation passed. Each shape has three warmup pairs, then seven alternating baseline-first/production-first measured pairs; seven has a 4/3 split. Median is the median of paired baseline/production ratios; above one favors current production. All-pair ranges contain fourteen samples and are not confidence intervals. Both legacy and exact transfer-parent fd540f37badaad97b53d56ba81684bbbc2ca4b9b are compiled test-only references. No threshold changes or second tuning round followed these measurements.

Raw receipts: `$MOVA_DATA/runs/octopus-wave25-mova-2026-10-30/timing-{en,world}-{legacy,parent}-{1,2}.log`, `timing-before.sha256`, `timing-after.sha256`, `timing-status.log`, `timing-samples.tsv`, `timing-summary.md`. Allocation observations remain separately recorded in model.md#wave25-evidence; requested bytes are not peak RSS. The benchmark retains regressions rather than choosing only improving shapes.

### 2026-10-30 · wave25-prospective

Heading `### Current wave25 model — 2026-10-30` stays in wave25-prospective as `### Current wave25 model`.

#### Prospective wave25 refinement — 2026-10-30

Preserve validation and complete error strings. For the validated head function h:{1..n}-> {0..n}, termination is reaching 0. Three colors encode unseen, current ordered path, and certified root-reaching. Each unseen vertex enters a path once; revisiting current color detects a cycle, and processing starts in ascending order preserves the smallest offending start. Cleanup walks each certified path once. This costs at most 2n head transitions and n color writes per phase, O(n) time after parsing; heads allocate n usize values and colors n+1 bytes (two nonempty allocations). No adjacency rows, queue, per-row allocations, recursion, or overflow-dependent sentinels. Tiny n<=64 retains the old direct loop to avoid a second allocation. Above the existing boundary, replace both the budgeted replay and reverse BFS. Expected improvement: n65 chain eliminates 65 row allocations and replay; long-chain linear gains retained. No timings yet; no semantic bug claimed. Empty input and parse/range/root precedence stay unchanged. Differential ordered-walk oracle and boundary cycles check semantics; compiled cycle-check mutant must fail.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
