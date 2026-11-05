# world-rerank-measurement

## forward

LAS=100 sum(correct)/max(sum(n),1), not mean sentence LAS. report compares selected correctness with top1; oracle maximizes correctness independently of selection. Bootstrap draws sentence differences with replacement 1000 times under fixed xorshift seed, counting sums<=0. run scans configured lambda grid, retaining earliest improvement exceeding 1e-9.

## invariant

For bounded counts correct<=n, LAS lies in [0,100]. Bootstrap lies in {0,.001,...,1}; empty and all-zero differences give 1. Empty LAS gives zero, not evidence of accuracy.

## cost

C candidates and N sentences: pick O(C), report O(sum C + 1000N) time and O(N) bootstrap storage. Lambda sweep multiplies candidate cost by ten. repairs repeatedly scans nodes and matrix lookups; hand passes bounded by two. Parser, matrix initialization and induced-rule costs not bounded here. No timing gain claimed.

## composition

main.rs absurd-rerank calls run, which loads dev/test via items using annotator.annotate/annotate_kbest and absurd::judge. Scores pass through en::annotate::annotate_kbest from parser.parse_kbest (f32 promoted to f64), so negative infinity is admissible. run selects lambda on dev, report uses get for sentinel safety. Actual repair callers are tree/reader and evaluation paths; source inspection does not execute them.

## epoch-memory

First source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.

## checks

cargo test -p world --lib rerank::wave10_models -- --test-threads=1. See root ../model.md#evidence for raw evidence hashes, repeated profiles and compiled controls. Syntax is partial; macro bodies are opaque and semantic AST/name/type resolution is absent.

## behavior-complement

Subject/revision: bound source hash and this scope. Domain/observation/operator: Exclude token-weighted denominator replaced by sentence count, changed count on empty candidates and positive-improvement bootstrap returning one on nonempty constant-positive differences.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## reverse-reconstruction

Subject/revision: bound source hash and this scope. Domain/observation/operator: A displayed LAS or bootstrap value retains all count/difference vectors producing that rounded observation. Neither recovers original tokens or candidate trees; sentence order is latent for fixed pseudorandom index draws.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## coupling

Subject/revision: bound source hash and this scope. Domain/observation/operator: Paired comparisons use identical sampled indices for baseline/system via their difference; adding a common per-sentence offset preserves p within integer bounds. Native translation and constant-sign cases exercise this coupling.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## observation-fiber

Subject/revision: bound source hash and this scope. Domain/observation/operator: run chooses lambda on dev only then reuses it for test report. Native tests cover supplied candidates and grid objectives, not file loading or heldout selection; empty dev selects first grid entry.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.
