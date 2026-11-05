# algorithm-longest-increasing-subsequence

## descriptant

### longest_increasing_subsequence mod groupant

Combiner: source identity, recurrence, invariant and checks constrain a
conditional model. Complement and converse answer different questions; no
additive inverse identity. Syntax is partial with opaque macros and no type
or name resolution. Finite evidence is not proof of all inputs or parent wiring.
Lawful Ord and successful execution are assumed; trait panics and allocation
failures are outside successful-call semantics. Preserve these observations
in future epochs. No runtime speedup claim.

## approximant

### Forward, invariant and cost

Maintain minimum tail indices via lower bound; store predecessors and reconstruct final chain.

Invariant: Tail values strictly increase; predecessors are earlier/smaller; frontier entries realize minimum tails.

Cost: O(n log n) time and O(n) space under ordinary allocation and constant comparisons.

## models

# Longest-increasing-subsequence typed models

Subject/revision: mod.rs at manifest SHA. Observe one normally terminating call
under lawful stable Ord and successful allocation. Models are source-derived
and partial. Local translate measure.rs moved() informed the lower-tail pattern;
this generic index-only implementation and fixtures are original.

## behavior_complement

U=(input x,proposed index vector I). Exclude out-of-range or nonincreasing
indices, non-strict value order, and length below the optimum over all increasing
subsequences. This is the contract complement and an inner bound on absent actual
outputs: alternative optimal vectors may satisfy the contract but differ from
the deterministic tie policy. Nonstrict-boundary mutant fails the brute oracle.

## reverse_reconstruction

Given I, compatible inputs make its indices/values strictly increase, admit no
longer increasing subsequence, and satisfy the tail-replacement/reconstruction
policy. Dropping the last condition yields only an outer bound on the preimage.
Many inputs share I; no unique inverse. Finite brute-length tests check optimality,
not complete reverse enumeration. Allocation/trait failure is outside this model.

## minimum_tail_invariant

Type: frontier_model. After each prefix, tails[l-1] indexes the minimum attainable
tail value of a length-l strictly increasing subsequence. Replace the first tail
>= current value. Previous[i] records the retained shorter tail at insertion,
an earlier index with smaller value. Following the final longest-tail chain
reconstructs an optimal witness. Internal states are reasoned about, not traced.

## equal_tail_policy

Type: deterministic_tie_model. Equal tail values replace their index with the
latest index, and capture the predecessor immediately. Thus [1,2,2,3] yields
[0,2,3]; [2,2,2] yields [2]. No earliest-index or lexicographically-minimal
promise. deterministic_equal_tail_policy executes these and competing tails.

## resource_bound

Type: frontier_cost. n lower-bound searches cost O(n log n) comparisons;
predecessor updates and reconstruction O(n), total storage O(n).
Expensive Ord multiplies comparison cost. Allocation exhaustion and enormous
zero-sized slices are outside bounded evidence. No runtime speedup is claimed.

## checks

### Evidence

Four tests for this algorithm passed twice with temporary local crate exports:
66 total tests, identical normalized outcomes. Strict all-target Clippy and
rustfmt passed. Temporary exports are not committed.

Coverage: 3280 ternary arrays length 0..7, 335923 subset masks; 1028 seeded/full-width i64 entries against quadratic DP and tie witnesses.

External mutant (replace strict lower-bound predicate with nonstrict comparison) compiled; exactly one filtered new exhaustive test
failed an assertion with exit 101. No mutant committed. One red control is not
mutation completeness. Parent owns final exports, registry and source bindings.

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29: prospective predictions saved outside repository before
implementation. Original std-only code/fixtures; four algorithm tests pass
twice. Compiled red control: replace strict lower-bound predicate with nonstrict comparison.

### 2026-10-29 · checks

Heading `### Evidence, 2026-10-29` stays in checks as `### Evidence`.

### 2026-10-30 · descriptant

#### Executable companion epoch, 2026-10-30

The opt-in Rust companion returns a greedy strict witness only when its length
L certifies the requested relative loss via L <= OPT <= n; otherwise it calls
this exact source. Four profiles cap objective loss at 1/2/5/10%. Exact witness
feasibility is retained; accepted index identities and tie choices may differ.
See rust_approximant.rs.groupant/model.md#descriptant for proof, prospective costs and
composition limits. Production remains unchanged. Central benchmarking and
registry/AST integration are required independently of local finite tests.
