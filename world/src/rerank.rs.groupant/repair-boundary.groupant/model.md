# world-rerank-repair-boundary

## forward

repairs proposes lexical changes first, otherwise matrix/category/tag-conditioned role changes. hand_pass_without applies at most two passes, changes relation and lexical UPOS/tag, and accumulates applications. repair_words_with joins this with shipped induced rules depending on mode/domain.

## invariant

Returned application count counts changes per pass, not distinct final word identities. Valid head indices and matrix score range are caller premises; arbitrary malformed supplied trees can panic.

## cost

C candidates and N sentences: pick O(C), report O(sum C + 1000N) time and O(N) bootstrap storage. Lambda sweep multiplies candidate cost by ten. repairs repeatedly scans nodes and matrix lookups; hand passes bounded by two. Parser, matrix initialization and induced-rule costs not bounded here. No timing gain claimed.

## composition

main.rs absurd-rerank calls run, which loads dev/test via items using annotator.annotate/annotate_kbest and absurd::judge. Scores pass through en::annotate::annotate_kbest from parser.parse_kbest (f32 promoted to f64), so negative infinity is admissible. run selects lambda on dev, report uses get for sentinel safety. Actual repair callers are tree/reader and evaluation paths; source inspection does not execute them.

## epoch-memory

First source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.

## checks

cargo test -p world --lib rerank::wave10_models -- --test-threads=1. See root ../model.md#evidence for raw evidence hashes, repeated profiles and compiled controls. Syntax is partial; macro bodies are opaque and semantic AST/name/type resolution is absent.

## behavior-complement

Subject/revision: bound source hash and this scope. Domain/observation/operator: Conditional source model excludes hand-only application of explicitly excluded Repair kinds and more than two hand passes. This fragment has source inspection only; no runtime repair inference is certified.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## reverse-reconstruction

Subject/revision: bound source hash and this scope. Domain/observation/operator: Observed repaired words admit every earlier word/tree and rule trace producing them; without previous labels, modes and matrix snapshot recovery is ambiguous.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## coupling

Subject/revision: bound source hash and this scope. Domain/observation/operator: reader tree and event pipelines call repair_words; run_repair and repair_eval join gold lengths and parser outputs by position. Zip truncation can hide missing parser words; exact aligned lengths remain unverified at these external boundaries.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## observation-fiber

Subject/revision: bound source hash and this scope. Domain/observation/operator: Mode induced runs lexical hand first then shipped rules; both runs full hand then shipped rules; other mode strings run full hand. These branches are not inferred from final labels alone.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.
