# world-rreport-aggregation

## forward

rows first requires equal question/answer cardinality, then pairs by position and calls old answer/baseline/ROUGE scoring. Agg independently sums four supplied metrics and None-state count. report groups known attributes and ordered explicitness/story/source/frame-prefix keys.

## invariant

For Agg from add, none<=n; denominators max(n,1) prevent empty NaN. Supplied Row scores are trusted, so state absence does not force r_state zero outside rows construction. Unknown attributes remain in all totals but have no attribute line.

## cost

N rows, G groups, V verdicts, total emitted bytes L: aggregation O(N log G + L), sampling O(N log N) with repeated hash-string work and seven attribute scans; manual O(N log V), formatting O(L). Answering and ROUGE dependency costs are separate. No measured speed claim.

## composition

main.rs reader evaluation calls reader::read_all on qs, then rows(own,&qs,ans), report/tsv and optional sample/manual. read_all maps qs in unchanged order. Native sample->read_verdicts->manual/report joins preserve (story,n) but bypass read_all and nonempty rows scoring. Ordered positional answer identity is a caller obligation.

## epoch-memory

First source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.

## checks

cargo test -p world --lib rreport::wave10_models -- --test-threads=1. See root ../model.md#evidence for raw evidence hashes, repeated profiles and compiled controls. Syntax is partial; macro bodies are opaque and semantic AST/name/type resolution is absent.

## behavior-complement

Subject/revision: bound source hash and this scope. Domain/observation/operator: Exclude successful rows return with unequal input lengths, silent truncated measurement and missing-state count determined by source rather than state Option. Valid callers retain positional identity.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## reverse-reconstruction

Subject/revision: bound source hash and this scope. Domain/observation/operator: Aggregate observations retain all row sequences matching sums and counts. The nine two-score assignments over {0,.5,1} produce five fibers sized 1,2,3,2,1. Rounded output loses further precision, order and identity.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## coupling

Subject/revision: bound source hash and this scope. Domain/observation/operator: reader::read_all maps the same qs in order; main.rs passes those qs and answers to rows. Equal lengths prove cardinality only, not semantic identity. Native mismatch/empty checks bypass answering; nonempty scoring pipeline is source-inspected only.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## observation-fiber

Subject/revision: bound source hash and this scope. Domain/observation/operator: 216 independent integer-sum fixtures cover all 27 three-score assignments and eight state-presence masks, with different r/r_state scales and fixed old/base scores. Four-column ordering is checked in printed lines.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.
