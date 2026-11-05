# world-absurd-judgment-report

## forward

judge scans verbs in node order and calls sense::arguments with shared dense positions. Unknown verb yields a gap before noun checks; idiomatic subjects are suppressed. Unknown noun yields noun gap; known sparse missing cell abstains silently. Judgments retain verb/noun indices, role and opposite-role score. run aligns gold by nonempty ID, length and forms, then checks noun head/relation. Events scores TSV arguments independently.

## invariant

First eligible noun child wins; AuxPass controls active/passive interpretation. Judgments are verb ordered then SUBJ/OBJ ordered. Gaps lack cell/role provenance. Gold duplicate IDs overwrite; equal forms do not guarantee identity.

## cost

N nodes,V verbs: O(VN) scans plus matrix lookups; report maps/sorts add O(J log J+G log G). Event collection/sort O(E log E). No timing claim.

## composition

Actual callers: domains_with projects Word to Node then consumes judge indices without reordering; rerank::items sums judgment penalties per candidate, and induce calls judge on supplied vectors. Only the domains join executes in these native tests.

## epoch-memory

Initial source-bound epoch. Old native witnesses exposed role-index panic and spurious formal activation with empty overrides. Guards/default resolution repair these; explicit finite values and sparse-cell gap semantics remain. No optimization, surrogate or percentage bound is claimed.

## checks

cargo test -p world --lib absurd::wave11_models -- --test-threads=1. Root ../model.md#evidence records repeated profiles and mutants. Partial syntax leaves macro bodies opaque; no semantic AST, parser, teacher or corpus execution.

## behavior-complement

Universe: terminating scoped calls and their outputs. Inner absent-behavior model excludes outputs contradicting: First eligible noun child wins; AuxPass controls active/passive interpretation. Judgments are verb ordered then SUBJ/OBJ ordered. Gaps lack cell/role provenance. Gold duplicate IDs overwrite; equal forms do not guarantee identity. Unknown external behavior remains outside this exclusion.

## reverse-reconstruction

A tuple constrains indexed argument and cell, leaving unrelated tree/matrix state free. Empty judgments include missing verbs, gaps, sparse cells and idiom suppression; retain these separate antecedents.

## coupling

Gap partition oracle covers unknown verb, unknown noun and sparse-cell abstention. No gap does not imply coverage. Fifty score/voice fixtures preserve role/index identity; report I/O remains unexecuted.

All typed entries bind this scope and manifest source revision. Domain and observation/operator are stated above. Status: partial source-derived relations with finite evidence named in checks, assuming stable dependencies and bounded successful execution. Reverse relations preserve ambiguity, with no complete-search or causal claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. Old native witnesses exposed role-index panic and spurious formal activation with empty overrides. Guards/default resolution repair these; explicit finite values and sparse-cell gap semantics remain. No optimization, surrogate or percentage bound is claimed.
