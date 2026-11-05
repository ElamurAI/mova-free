# prag-agree-tree-form

## forward

tree_form ignores comments, short rows, multiword and decimal IDs. Last punctuation ? or a raw trailing ? yields wh only when the first nonpunctuation row has a wh tag, otherwise q. A root VB with no direct subject/expletive, no earlier auxiliary, at most two permitted softeners yields imp before ! yields excl. Other forms yield decl.

## invariant

Subject/auxiliary relations refer to textual root ID; sparse numeric IDs are allowed. Question classification takes precedence over imperative, imperative over exclamation. This is an annotation heuristic, not proof of speech act.

## cost

O(B+N) parse and a bounded number of row scans, O(B+N) references/vector storage. No parser model, external process or semantic tree inference.

## composition

Public APIs table, tree_form, agree_one and run compose supplied strings by exact sent_id. Native run->tree_form->agree_one->table joins execute. prag CLI reads three files before run and writes two outputs; CLI filesystem behavior is inspected only.

## epoch-memory

Initial source-bound epoch. Old-code witnesses exposed CRLF sentence-block merging and missing act/form key panics in report accounting. Normalization repairs block identity and map defaults now agree with agree_one. Last-row-wins duplicate semantics are preserved, not silently relabeled as one observation per raw line. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.

## checks

cargo test -p prag --lib agree:: -- --test-threads=1; the wave12_models submodule contains new finite checks. File ../model.md#evidence binds old witnesses, repeated profiles and compiled production mutations. Syntax projections are partial and macro bodies opaque.

## behavior-complement

Domain: finite inputs satisfying this fragment's forward premises and the parent API tag/state constraints; output is its scoped observation. Let J_fragment(x,y) impose the concrete relations: tree_form ignores comments, short rows, multiword and decimal IDs. Last punctuation ? or a raw trailing ? yields wh only when the first nonpunctuation row has a wh tag, otherwise q. A root VB with no direct subject/expletive, no earlier auxiliary, at most two permitted softeners yields imp before ! yields excl. Other forms yield decl. Also require: Subject/auxiliary relations refer to textual root ID; sparse numeric IDs are allowed. Question classification takes precedence over imperative, imperative over exclamation. This is an annotation heuristic, not proof of speech act. The inner absent-behavior model is {(x,y): not J_fragment(x,y)}. This excludes wrong indices, selections or outputs, rather than treating a scope disclaimer as a behavioral predicate. Finite checks support selected cases; unknown external semantics remain outside this claim.

## reverse-reconstruction

Each of five forms has many compatible blocks, IDs and lexical contents. wh constrains a first wh-tagged word plus a question marker; q may arise from raw fallback. decl includes ordinary declarations and malformed/empty supplied blocks; none permits unique reconstruction.

Formally retain every parent-tagged input and compatible intermediate state satisfying the forward relation with the observed fragment output. Do not merge antecedents merely because their projected result agrees; no unique inverse or complete executed solver is claimed.

## intervention

Question-priority counterfactual: replacing final punctuation changes q/wh before the imperative branch can run. Twelve explicit punctuation/subject/wh cases check this priority with sparse IDs.

All typed records bind the exact source revision and scope in manifest.json. Domains, observations and operators are above. Stable dependencies and successful termination are premises. These partial relations retain ambiguous antecedents; no universal solver, unique inverse or actual causal inference is supplied.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. Old-code witnesses exposed CRLF sentence-block merging and missing act/form key panics in report accounting. Normalization repairs block identity and map defaults now agree with agree_one. Last-row-wins duplicate semantics are preserved, not silently relabeled as one observation per raw line. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.
