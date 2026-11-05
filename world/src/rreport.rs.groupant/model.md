# world-rreport

## model

# world-rreport

## forward

Question/answer alignment, metric aggregation, presentation, deterministic sampling and manual identity joins. Pure supplied rows are tested; nonempty answer generation is outside the executed slice.

## invariant

Only declared bounded valid domains have modeled invariants. Successful allocations and stable dependency snapshots are assumed; arbitrary public states and external effects are not certified.

## cost

N rows, G groups, V verdicts, total emitted bytes L: aggregation O(N log G + L), sampling O(N log N) with repeated hash-string work and seven attribute scans; manual O(N log V), formatting O(L). Answering and ROUGE dependency costs are separate. No measured speed claim.

## composition

main.rs reader evaluation calls reader::read_all on qs, then rows(own,&qs,ans), report/tsv and optional sample/manual. read_all maps qs in unchanged order. Native sample->read_verdicts->manual/report joins preserve (story,n) but bypass read_all and nonempty rows scoring. Ordered positional answer identity is a caller obligation.

## epoch-memory

First source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.

## checks

cargo test -p world --lib rreport::wave10_models -- --test-threads=1. See root model.md#evidence for raw evidence hashes, repeated profiles and compiled controls. Syntax is partial; macro bodies are opaque and semantic AST/name/type resolution is absent.

## behavior-complement

Subject/revision: bound source hash and this scope. Domain/observation/operator: Universe: bounded supplied candidate or Row inputs and one terminating call. Exclude observations forbidden by the scoped child invariants, conditional on each child's explicit premises. This is an inner absent-behavior model, not the complement of all Rust traces.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## reverse-reconstruction

Subject/revision: bound source hash and this scope. Domain/observation/operator: Keep every compatible scoped input and intermediate observation satisfying forward constraints. Aggregate scores, selected candidates and sampled records discard information; no unique inverse or causal explanation follows.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## coupling

Subject/revision: bound source hash and this scope. Domain/observation/operator: main.rs reader evaluation calls reader::read_all on qs, then rows(own,&qs,ans), report/tsv and optional sample/manual. read_all maps qs in unchanged order. Native sample->read_verdicts->manual/report joins preserve (story,n) but bypass read_all and nonempty rows scoring. Ordered positional answer identity is a caller obligation.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## observation-fiber

Subject/revision: bound source hash and this scope. Domain/observation/operator: Native fixtures and caller inspection are separate evidence. A checked leaf does not prove parser, filesystem, gold alignment or answer-generation joins. The scope deliberately excludes full assessment and performance claims.

Status: partial source-derived relation; finite evidence only as named in checks. Assumptions: bounded counts without integer or floating arithmetic overflow, successful allocations and fixed dependencies. Unknown external behavior is not excluded. Reverse relations retain ambiguity; finite witnesses do not establish universal completeness.

## evidence

### Evidence

Bounds: 2916 candidate selections, 216 aggregation fixtures, nine aggregate-fiber pairs, 23 sampling bounds and nine bootstrap sizes plus translation checks. No measured speed or percentage approximation claim.

Structural validation: both source syntax projections generated twice with identical bytes. Full original registry plus an external scaffold and the two new entries validates twice identically (200 file sidecars,179 fragments, no diagnostics); changed-source gate reports both sources and no missing entries. The scaffold parent is deliberately unmaterialized; central parent integration is separate. The earlier two-entry-only registry failed on unrelated existing sidecar ownership and was replaced by the full registry scaffold. No central registry or parent was edited here. git diff --check passes.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first source-bound epoch. Before repair, native invalid-score and cardinality assertions failed on old production behavior. Narrow input checks now prevent undefined NaN ordering and silent report undercount. Infinite parser scores remain supported. Empty conventions and malformed-metadata/repair limits are retained explicitly. No optimization or percentage-error surrogate is proposed.

### 2026-10-30 · evidence

2026-10-30, synthetic native inputs only. Four rerank and four rreport tests pass twice per debug/release profile; normalized identities/results match. Old production code failed two regression assertions (6 passed, 2 failed); no parser/gold data were loaded. Selection control reverses the primary comparison; aggregation control subtracts the merge score. Both compiled and failed their named independent-oracle assertions; exact source bytes restored and all eight checks passed again.

Raw logs: $MOVA_DATA/runs/octopus-wave10-observation-2026-10-30/. Hashes below bind the recorded bytes; sidecar source hashes bind production plus tests.

- before.log: `a36df3771f5026a26611ceab683f8963d5be83d04a098fe88b76ba656cc486f0`

- debug1.log: `03c1c4c9b65c9febb88ca7af82370b4c0680c14eeec0254db205b63408da5767`

- debug2.log: `a75666665c9ca24cb96d60905077a60ee8b0bed55e764be194b332248abd2716`

- release1.log: `1c7402f41297ed9e7abc53dc6d51fa1ce379aa18c3ae4b7d0d8522e027550a22`

- release2.log: `ebe4c6c3dc5ebb9f6a8b1542efdd6dce3b4f593dda6b48c6fccc68e244cd7d0f`

- debug-normalized.txt: `7caab49871ee32d5168e0f198a4808e623248f70c3f814b41512dd4529cf8410`

- release-normalized.txt: `7caab49871ee32d5168e0f198a4808e623248f70c3f814b41512dd4529cf8410`

- mutant-rerank.log: `1ce1e752636ad2b450c178808c21083daaf4d8249dc2789e94b640fca51f5e01`

- mutant-rreport.log: `70fe2d027578c2226dba324e1ffa494564e7ce453fa9bd978e833a497dd9244f`

- restored.log: `87b391d3a572a7f84f690a8df80bcadd6349e7f5f2dad4bbfee5be656e5fe97f`
