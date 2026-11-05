# search-indexes

## model

# search-indexes

## forward

Tagged exact search interfaces: nearest point, metric-radius word matches, closed interval overlap, explicit-priority set updates and stateful splay access. They share query/result identity obligations, not one interchangeable metric or one automatic pipeline. Define Q(A,x): KD argmin_i(||A_i-x||²,i); BK sorted {(i,d_byte(A_i,x)):d<=r}; intervals sorted {i:a_i<=q_hi && q_lo<=b_i}. Trees map (state,key) to (new state,membership).

## invariant

KD/BK/interval indexes retain original occurrence indexes; ordered trees retain unique keys. Tie order, duplicate policy and state mutation follow the selected API. Validation errors and allocator failure never become empty successful answers.

## cost

Build/query costs stay tagged: KD median build O(n log²n), worst query O(n); BK worst quadratic number of metric calls at build and linear at query; balanced interval queries may inspect O(n), plus output sorting. Treap height may be n with explicit priorities; splay operations may be linear individually and only sequence amortization gives logarithmic bounds. Sum only calls actually executed; reused indexes amortize build, not correctness checks.

## composition

Potential adapter boundaries are candidate-index lookup followed by exact source projection and optional ordered-set deduplication. Original indexes and key values require an explicit mapping; deduplication loses occurrence multiplicity. KD squared Euclidean radius is not BK byte-edit radius. Interval boundaries are closed. Splay access changes future shape while Treap contains does not.

## limitations

Successful allocations and each leaf's finite domain/error precedence. These are tagged relations, not automatic production adoption. Floating residuals are not solution-error guarantees. No generic 1/2/5/10% composition-wide profile; finite evidence remains separate from analytical claims.

## epoch-memory

Wave26. Tagged library tools; no automatic training adoption, source-volume quality claim or generic numerical profile.

## checks

["cargo test -p mova-algorithms kd_tree::tests","cargo test -p mova-algorithms bk_tree::tests","cargo test -p mova-algorithms interval_tree::tests","cargo test -p mova-algorithms treap::tests","cargo test -p mova-algorithms splay_tree::tests"]

## wave26-executed-checks

# search-indexes: wave26 checks

13 source-local test declarations across five modules. Every module has a compiled failing production mutation followed by exact restoration. The final whole library passed 877 checks twice debug and twice release. See [integrated evidence](../algorithm-library.groupant/model.md#wave26-evidence) for scopes and limitations. These tagged APIs are not an invented serial pipeline.

## catalog

# Typed family relations

## behavior-complement

Type: behavior_complement.

Exclude y!=Q(A,x), invalid inputs paired with Ok, missing duplicate indexes, larger-index KD ties, or wrong splay resulting root. Metric/radius units and mutation behavior remain tagged.

## reverse-reconstruction

Type: reverse_reconstruction.

Q^-1(y)={(A,x):Q(A,x)=y}. Nearest constrains one distance and competitor inequalities, leaving farther coordinates ambiguous. Empty matches admit empty or nonempty disjoint/far data. Key sets forget insertion order/priorities; membership forgets tree shape.

## metric_units

Type: metric_units.

Squared distances, byte edits and endpoint order cannot be substituted; selected indexes require explicit identity mapping.

## occurrence_to_set

Type: occurrence_to_set.

Set insertion can collapse duplicate occurrences; reconstruct all original matches only if a reverse multiplicity map is retained.

## stateful_access

Type: stateful_access.

Splay misses change root to last visited; caching by returned boolean alone loses state. Treap observation is shape-neutral.

## height_resources

Type: height_resources.

Explicit priorities have no random balancing guarantee; splay amortization includes initial potential and excludes output traversal.

## wave26-evidence

### search-indexes: wave26 checks

13 source-local test declarations across five modules. Every module has a compiled failing production mutation followed by exact restoration. The final whole library passed 877 checks twice debug and twice release. See [integrated evidence](../algorithm-library.groupant/model.md#wave26-evidence) for scopes and limitations. These tagged APIs are not an invented serial pipeline.

## History

### 2026-10-30 · epoch-memory

Wave26, 2026-10-30. Tagged library tools; no automatic training adoption, source-volume quality claim or generic numerical profile.
