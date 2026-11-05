# world-rreport-presentation

## forward

cell escapes pipes and replaces newline/tab with spaces; tsv writes twenty columns, sanitizing answer/question text and some diagnostics. dev displays explicitness by the first at most two bytes.

## invariant

Formatting is deterministic for fixed rows. Identity/category fields in TSV are not cell-sanitized. Unicode explicitness can panic in dev when byte two is not a boundary; unsupported malformed metadata remains a declared gap.

## cost

N rows, G groups, V verdicts, total emitted bytes L: aggregation O(N log G + L), sampling O(N log N) with repeated hash-string work and seven attribute scans; manual O(N log V), formatting O(L). Answering and ROUGE dependency costs are separate. No measured speed claim.

## composition

main.rs reader evaluation calls reader::read_all on qs, then rows(own,&qs,ans), report/tsv and optional sample/manual. read_all maps qs in unchanged order. Native sample->read_verdicts->manual/report joins preserve (story,n) but bypass read_all and nonempty rows scoring. Ordered positional answer identity is a caller obligation.

## epoch-memory

First source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.

## checks

cargo test -p world --lib rreport::wave10_models -- --test-threads=1. See root ../model.md#evidence for raw evidence hashes, repeated profiles and compiled controls. Syntax is partial; macro bodies are opaque and semantic AST/name/type resolution is absent.

## behavior-complement

Subject/revision: bound source hash and this scope. Domain/observation/operator: Over metadata without separators and ASCII explicitness, exclude extra record lines from newline/tab characters in fields passed through cell. This is a conditional subset, not general TSV safety.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## reverse-reconstruction

Subject/revision: bound source hash and this scope. Domain/observation/operator: Printed text admits multiple antecedents: original spaces, tabs and newlines collapse; three-decimal scores merge intervals. TSV cannot uniquely reconstruct original Row or answer provenance.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## coupling

Subject/revision: bound source hash and this scope. Domain/observation/operator: Public dev/tsv presentation trusts arbitrary Row fields. Native identity join checks row count only for clean metadata; parser and downstream TSV roundtrip are not tested.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## observation-fiber

Subject/revision: bound source hash and this scope. Domain/observation/operator: None versus Some(empty) state distinction is erased by TSV text but retained in aggregate none count. Parent composition must keep Option state, not reconstruct it from a blank cell.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.
