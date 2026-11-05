# prag-agree-labels

## forward

table maps first-column IDs to header/value zip maps; short rows omit keys, duplicate header names and IDs keep last values, blank lines disappear. agree_one retains supplied labels unless ordered rules change a field. P0 respects character voice and recognized original forms. P1 precedes P2/P3; P2 excludes IDs containing :books: and requires both assert probabilities >=0.2. P3-P5 are enabled only by PRAG_AGREE_ALL=1. Missing/bad numeric cells become zero, NaN comparisons are false; positive infinity remains accepted.

## invariant

Default-mode disagreement or subthreshold evidence cannot trigger P2. Character voice prevents P0 but does not disable P1. No label vocabulary validation, probability range validation or general witness truth guarantee is provided.

## cost

Table O(B + N log N + N H log H) including string bytes B, rows N, columns H. A fixed number of rule probes gives bounded map lookups and copied label strings per row; no witness inference occurs here.

## composition

Public APIs table, tree_form, agree_one and run compose supplied strings by exact sent_id. Native run->tree_form->agree_one->table joins execute. prag CLI reads three files before run and writes two outputs; CLI filesystem behavior is inspected only.

## epoch-memory

Initial source-bound epoch. Old-code witnesses exposed CRLF sentence-block merging and missing act/form key panics in report accounting. Normalization repairs block identity and map defaults now agree with agree_one. Last-row-wins duplicate semantics are preserved, not silently relabeled as one observation per raw line. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.

## checks

cargo test -p prag --lib agree:: -- --test-threads=1; the wave12_models submodule contains new finite checks. File ../model.md#evidence binds old witnesses, repeated profiles and compiled production mutations. Syntax projections are partial and macro bodies opaque.

## behavior-complement

Domain: finite inputs satisfying this fragment's forward premises and the parent API tag/state constraints; output is its scoped observation. Let J_fragment(x,y) impose the concrete relations: table maps first-column IDs to header/value zip maps; short rows omit keys, duplicate header names and IDs keep last values, blank lines disappear. agree_one retains supplied labels unless ordered rules change a field. P0 respects character voice and recognized original forms. P1 precedes P2/P3; P2 excludes IDs containing :books: and requires both assert probabilities >=0.2. P3-P5 are enabled only by PRAG_AGREE_ALL=1. Missing/bad numeric cells become zero, NaN comparisons are false; positive infinity remains accepted. Also require: Default-mode disagreement or subthreshold evidence cannot trigger P2. Character voice prevents P0 but does not disable P1. No label vocabulary validation, probability range validation or general witness truth guarantee is provided. The inner absent-behavior model is {(x,y): not J_fragment(x,y)}. This excludes wrong indices, selections or outputs, rather than treating a scope disclaimer as a behavioral predicate. Finite checks support selected cases; unknown external semantics remain outside this claim.

## reverse-reconstruction

An unchanged act admits absent, disagreeing or low-confidence witnesses and protected book narration. Changed assert admits all agreeing pairs >=0.2 outside books with the same reachable precedence. Last-write maps cannot recover overwritten rows or column multiplicity.

Formally retain every parent-tagged input and compatible intermediate state satisfying the forward relation with the observed fragment output. Do not merge antecedents merely because their projected result agrees; no unique inverse or complete executed solver is claimed.

## intervention

Threshold intervention: hold labels/id fixed and raise either assert threshold from 0.2 to 0.3; boundary witnesses at 0.2 must cease qualifying. Native finite grid observes 32 probability/book cases and character voice. Optional P3-P5 remain source-inspected only.

All typed records bind the exact source revision and scope in manifest.json. Domains, observations and operators are above. Stable dependencies and successful termination are premises. These partial relations retain ambiguous antecedents; no universal solver, unique inverse or actual causal inference is supplied.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. Old-code witnesses exposed CRLF sentence-block merging and missing act/form key panics in report accounting. Normalization repairs block identity and map defaults now agree with agree_one. Last-row-wins duplicate semantics are preserved, not silently relabeled as one observation per raw line. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.
