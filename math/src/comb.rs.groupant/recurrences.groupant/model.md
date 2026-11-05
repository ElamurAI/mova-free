# math-comb-recurrences

## descriptant

### math-comb-recurrences

Stirling/Bell/partition/derangement/Fibonacci retain distinct zero seeds and update directions. Outputs have recurrence-specific nonunique index preimages. DP table/row costs differ; Cassini is not a complete sequence certificate. Child uses parent nt sieve/product joins.

## approximant

### Scoped forward and cost

Stirling/Bell/partition/derangement/Fibonacci retain distinct zero seeds and update directions. Outputs have recurrence-specific nonunique index preimages. DP table/row costs differ; Cassini is not a complete sequence certificate. Child uses parent nt sieve/product joins.

No executable percent surrogate claimed.

## invariant

### Scoped invariant

Stirling/Bell/partition/derangement/Fibonacci retain distinct zero seeds and update directions. Outputs have recurrence-specific nonunique index preimages. DP table/row costs differ; Cassini is not a complete sequence certificate. Child uses parent nt sieve/product joins.

Source range[4466,10809) bound by manifest digest. Parent domain applies, not arbitrary public states.

## models

# Source-scoped typed relations

## behavior_complement

stirling2(0,0) excludes0 and returns1; stirling2(n>3000,k>n) excludes zero shortcut because n limit wins. Empty Fibonacci seed excludes(1,1), returning(0,1).

## reverse_reconstruction

Partition count1 admits n=0 and n=1; derangement1 admits n=0 and n=2. Cassini acceptance constrains f1*(f1-f)-f²=(-1)^n but does not uniquely recover Fibonacci sequence provenance.

## stirling_state

S(i,j)=j*S(i-1,j)+S(i-1,j-1), descending j preserves previous row; reset row0=0 after each nonzero i. UnsignedI computes full row with multiplier i-1 and returns row sum.

## partition_direction

Distinct partitions update sum descending for each part; odd partitions update ascending. Euler pentagonal recurrence alternates pairs of signs at k(3k±1)/2. Their equality checks different counting paths.

## empty_and_limit

partitions0=1, partitions_k(0,0)=1, derangements0=1/1=0; Bell0=1. Each function has its own n gate; alternate Bell/partition/derangement checks cover smaller ranges.

## doubling_cost

F pair recursion halves n; c=a*(2b-a),d=a²+b² and parity chooses(c,d) or(d,c+d). O(log n) big arithmetic steps does not mean O(log n) bit time or bounded result storage.

## tests

### Evidence

See ../model.md#tests for actual checks and source mutant; this fragment adds no runtime test.

Stirling/Bell/partition/derangement/Fibonacci retain distinct zero seeds and update directions. Outputs have recurrence-specific nonunique index preimages. DP table/row costs differ; Cassini is not a complete sequence certificate. Child uses parent nt sieve/product joins.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read, parent AST reused; partial finite evidence only.
