# world-rerank-selection

## forward

For finite nonnegative lambda choose the first maximum parser_score-lambda*penalty. Positive infinity uses lexicographic (-penalty,score). Empty candidates return sentinel zero. Reject NaN score/lambda, negative lambda and negative/nonfinite penalties before ordering; infinite parser scores remain admissible.

## invariant

Nonempty admissible candidates return an in-bounds first optimum. Empty sentinel is not a valid candidate; only get/first callers are safe. Floating overflow combinations producing NaN keys remain outside the modeled domain.

## cost

C candidates and N sentences: pick O(C), report O(sum C + 1000N) time and O(N) bootstrap storage. Lambda sweep multiplies candidate cost by ten. repairs repeatedly scans nodes and matrix lookups; hand passes bounded by two. Parser, matrix initialization and induced-rule costs not bounded here. No timing gain claimed.

## composition

main.rs absurd-rerank calls run, which loads dev/test via items using annotator.annotate/annotate_kbest and absurd::judge. Scores pass through en::annotate::annotate_kbest from parser.parse_kbest (f32 promoted to f64), so negative infinity is admissible. run selects lambda on dev, report uses get for sentinel safety. Actual repair callers are tree/reader and evaluation paths; source inspection does not execute them.

## epoch-memory

First source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.

## checks

cargo test -p world --lib rerank::wave10_models -- --test-threads=1. See root ../model.md#evidence for raw evidence hashes, repeated profiles and compiled controls. Syntax is partial; macro bodies are opaque and semantic AST/name/type resolution is absent.

## behavior-complement

Subject/revision: bound source hash and this scope. Domain/observation/operator: Over candidates with ordered computed keys, exclude a returned index strictly dominated by another, later equal-key winners and accepted NaN scores.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## reverse-reconstruction

Subject/revision: bound source hash and this scope. Domain/observation/operator: For an observed selected index retain every candidate sequence satisfying its dominance inequalities and earlier strict tie inequalities. Scores, penalties and unselected candidates are not recoverable.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## coupling

Subject/revision: bound source hash and this scope. Domain/observation/operator: At finite lambda each candidate pair switches only at score_difference/penalty_difference when denominator is nonzero; the infinity branch is a different lexicographic objective. Native 2916 choices cover four lambda values.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## observation-fiber

Subject/revision: bound source hash and this scope. Domain/observation/operator: Adding the same finite score offset to all candidates preserves selection absent rounding changes; selected index alone admits many common offsets. Native oracle enumerates first unbeaten candidates.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.
