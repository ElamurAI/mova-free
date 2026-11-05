# world-absurd-matrix

## forward

File parsing decodes BTreeMap<String,[u8;2]>, splits keys at the first pipe, rejects scores outside 0..4, and builds verb rows and a noun union. Empty components and extra pipes remain accepted; duplicate JSON keys follow serde replacement. Global lookup prefers domain-rated cells. Recognized OBJ idiom/light-verb/collocation returns the rated value capped at one, or None when unrated (bypassing the known layer). Other lookups use the known layer only when no rated cell exists. Unsupported roles return None before indexing.

## invariant

Accepted file entries retain supplied role scores. Global known-layer TSV parses u8 without validating 0..4, so compiled layer validity remains a premise.

## cost

C cells: O(C log C) map operations plus strings; file lookup is two map searches. Compiled lexical lookup and one-time layer initialization costs are external.

## composition

Actual callers: domains_with projects Word to Node then consumes judge indices without reordering; rerank::items sums judgment penalties per candidate, and induce calls judge on supplied vectors. Only the domains join executes in these native tests.

## epoch-memory

Initial source-bound epoch. Old native witnesses exposed role-index panic and spurious formal activation with empty overrides. Guards/default resolution repair these; explicit finite values and sparse-cell gap semantics remain. No optimization, surrogate or percentage bound is claimed.

## checks

cargo test -p world --lib absurd::wave11_models -- --test-threads=1. Root ../model.md#evidence records repeated profiles and mutants. Partial syntax leaves macro bodies opaque; no semantic AST, parser, teacher or corpus execution.

## behavior-complement

Universe: terminating scoped calls and their outputs. Inner absent-behavior model excludes outputs contradicting: Accepted file entries retain supplied role scores. Global known-layer TSV parses u8 without validating 0..4, so compiled layer validity remains a premise. Unknown external behavior remains outside this exclusion.

## reverse-reconstruction

Score s retains every matrix assigning s to the queried cell/role, with arbitrary unobserved cells. None includes invalid roles, absent rows/cells and absent known-layer roles; it cannot identify a cause.

## coupling

Domain counterfactual holds verb/noun/role fixed and switches real to tale; rated overrides can change scores, otherwise global lookup inherits real. Configured lexical judgments are not universal truth.

All typed entries bind this scope and manifest source revision. Domain and observation/operator are stated above. Status: partial source-derived relations with finite evidence named in checks, assuming stable dependencies and bounded successful execution. Reverse relations preserve ambiguity, with no complete-search or causal claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. Old native witnesses exposed role-index panic and spurious formal activation with empty overrides. Guards/default resolution repair these; explicit finite values and sparse-cell gap semantics remain. No optimization, surrogate or percentage bound is claimed.
