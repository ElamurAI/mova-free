# world-rreport-sampling

## forward

sample groups known ATTRS, excludes exact (story,n), then stable-sorts salted wrapping FNV/finalizer keys and takes k per type. read_verdicts skips malformed records and overwrites duplicate identities. manual averages only rows with exact verdict identity matches.

## invariant

Sample size per known type <=k and eligible count; duplicate input identities can survive and hash collisions retain input order. Verdict floats are permissive, including NaN/out-of-range, and repeated rows count repeatedly.

## cost

N rows, G groups, V verdicts, total emitted bytes L: aggregation O(N log G + L), sampling O(N log N) with repeated hash-string work and seven attribute scans; manual O(N log V), formatting O(L). Answering and ROUGE dependency costs are separate. No measured speed claim.

## composition

main.rs reader evaluation calls reader::read_all on qs, then rows(own,&qs,ans), report/tsv and optional sample/manual. read_all maps qs in unchanged order. Native sample->read_verdicts->manual/report joins preserve (story,n) but bypass read_all and nonempty rows scoring. Ordered positional answer identity is a caller obligation.

## epoch-memory

First source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.

## checks

cargo test -p world --lib rreport::wave10_models -- --test-threads=1. See root ../model.md#evidence for raw evidence hashes, repeated profiles and compiled controls. Syntax is partial; macro bodies are opaque and semantic AST/name/type resolution is absent.

## behavior-complement

Subject/revision: bound source hash and this scope. Domain/observation/operator: Exclude sampled identities in the exclusion set and contribution by unmatched rows to manual denominator. Equal n in different stories is not the same identity.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## reverse-reconstruction

Subject/revision: bound source hash and this scope. Domain/observation/operator: Sample/aggregate output retains every input multiset containing chosen rows and any omitted ineligible/lower-ranked rows. Overwritten verdict histories and unselected rows are not recoverable.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## coupling

Subject/revision: bound source hash and this scope. Domain/observation/operator: Two stories with the same question number retain distinct manual verdicts; supplied fixture yields 50% per metric and sentence count 2. Exclusion removes only one story's identity.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## observation-fiber

Subject/revision: bound source hash and this scope. Domain/observation/operator: For 20 unique clean identities and k=0..22 native checks enforce cardinality, uniqueness and deterministic order; these do not prove hash distribution or uniqueness for duplicate input identities.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.
