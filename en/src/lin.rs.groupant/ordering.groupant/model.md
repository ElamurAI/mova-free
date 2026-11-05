# en-lin-ordering

## descriptant

### lin: ordering

learn counts five backoff side levels and three chain levels. Left needs count>=3 and strict left majority. Emit recursively orders left/head/right, with depth>200 cutoff. DP degree<=10 and greedy degree>10 have different optimality contracts.

## approximant

### ordering forward scope

learn counts five backoff side levels and three chain levels. Left needs count>=3 and strict left majority. Emit recursively orders left/head/right, with depth>200 cutoff. DP degree<=10 and greedy degree>10 have different optimality contracts.

Composition rejoins the parent at its explicit state/representation boundaries; guarantees apply only to this byte range, not unrelated parent branches.

## invariant

### State and cost

Linearizer stores side counts and interpolated bigram/context counts. PunctTable stores ordered signatures and a process-global first-set OnceLock.

Tied side counts cannot choose left. PunctTable total<3 or best*2<=total cannot override rules. OnceLock cannot be replaced by later set calls. order does not guarantee permutation for invalid cyclic/disconnected inputs or depth>200.

Child grouping O(n); repeated construction/heavy scans can be quadratic at high degree. Each side degree k<=10 costs O(2^k*k^2) time/O(2^k*k) memory; k>10 greedy O(k^2), not exact optimum. Emission depth bounded201. u32 count arithmetic is unchecked.

## models

## behavior_complement

Tied side counts cannot choose left. PunctTable total<3 or best*2<=total cannot override rules. OnceLock cannot be replaced by later set calls. order does not guarantee permutation for invalid cyclic/disconnected inputs or depth>200.

## reverse_reconstruction

An output ordering admits all tree/side/chain table states whose local choices emit it; many weights preserve the same order. PunctTable write/read recovers represented map state but malformed lines are skipped and duplicate keys overwrite. Order alone cannot recover original sentence order or counts.

## side_ties

MIN3 chooses highest-specificity sufficient key, equality goes right. explain reports None/(0,0) on complete miss, levels4..0 otherwise.

## chain_ties

DP updates only on strict score increase, final max_by prefers later equal endpoint; greedy max_by prefers later equal candidate. Uniform floor0.05/64 keeps log finite for valid nonnegative finite counts.

## punctuation_majority

Some(empty) is a learned no-mark override, distinct from None fallback. Predict requires strict majority and at least3 observations.

## tree_domain

Heads are1-based root0; invalid high heads clamp to n for order/learn, but punctuation key directly indexes head tag and may panic. Depth cutoff emits head without descendants.

## tests

### Evidence

Compiled source mutants cover construct rounded-floor boundary, parse root legality and store checksum gate. Other sources have no compiled source mutation claim. Preflight empty-model nonempty tagging failed in ptag; synthetic fixtures adjusted without production changes. Existing corpus/seeds tests not selected.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31. Exact byte range/hash in manifest. Partial lexical AST is shared with parent; no macro expansion, semantic proof or executable percent surrogate. Parent tests exercise selected reachable paths, not every statement in this fragment.

### 2026-10-31 · tests

Actual source test: lin::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.
