# prag-agree-report

## forward

run normalizes CRLF to LF, splits sentence blocks and joins by exact sent_id. Duplicate tree IDs keep the last form. Iterate unique label IDs in BTree order; witness-only/tree-only IDs add no output row. Missing act/form use the same empty defaults as agree_one. Emit one TSV row per retained label ID, count rule applications and form/act changes; move ties retain lexical order.

## invariant

Report n equals distinct retained label-table IDs (including an empty ID on a nonempty TSV row), not raw rows, witnesses or changed rows. Rule totals can exceed n across multiple rules. Output fields retain id association; CRLF must preserve block boundaries. Polarity/hedge changes are not included in move counts.

## cost

O(B) normalization allocation precedes table/tree maps O(N log N), followed by fixed rule checks per label and O(M log M) sorting M distinct move descriptions. No speedup claim; normalization deliberately adds one string allocation.

## composition

Public APIs table, tree_form, agree_one and run compose supplied strings by exact sent_id. Native run->tree_form->agree_one->table joins execute. prag CLI reads three files before run and writes two outputs; CLI filesystem behavior is inspected only.

## epoch-memory

Initial source-bound epoch. Old-code witnesses exposed CRLF sentence-block merging and missing act/form key panics in report accounting. Normalization repairs block identity and map defaults now agree with agree_one. Last-row-wins duplicate semantics are preserved, not silently relabeled as one observation per raw line. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.

## checks

cargo test -p prag --lib agree:: -- --test-threads=1; the wave12_models submodule contains new finite checks. File ../model.md#evidence binds old witnesses, repeated profiles and compiled production mutations. Syntax projections are partial and macro bodies opaque.

## behavior-complement

Domain: finite inputs satisfying this fragment's forward premises and the parent API tag/state constraints; output is its scoped observation. Let J_fragment(x,y) impose the concrete relations: run normalizes CRLF to LF, splits sentence blocks and joins by exact sent_id. Duplicate tree IDs keep the last form. Iterate unique label IDs in BTree order; witness-only/tree-only IDs add no output row. Missing act/form use the same empty defaults as agree_one. Emit one TSV row per retained label ID, count rule applications and form/act changes; move ties retain lexical order. Also require: Report n equals distinct retained label-table IDs (including an empty ID on a nonempty TSV row), not raw rows, witnesses or changed rows. Rule totals can exceed n across multiple rules. Output fields retain id association; CRLF must preserve block boundaries. Polarity/hedge changes are not included in move counts. The inner absent-behavior model is {(x,y): not J_fragment(x,y)}. This excludes wrong indices, selections or outputs, rather than treating a scope disclaimer as a behavioral predicate. Finite checks support selected cases; unknown external semantics remain outside this claim.

## reverse-reconstruction

Counts alone preserve all row sets yielding the same changes, losing which IDs changed. A final TSV row may combine labels, tree and witness rows; preserving all three supplied inputs is needed for provenance, and even they do not establish actual speech intent.

Formally retain every parent-tagged input and compatible intermediate state satisfying the forward relation with the observed fragment output. Do not merge antecedents merely because their projected result agrees; no unique inverse or complete executed solver is claimed.

## intervention

Line-ending intervention holds logical rows fixed while mapping LF to CRLF. The old implementation merged blocks and misjoined forms. The native run/table join checks output identity, two-sentence denominator and two P0 plus one P1 applications; duplicate label IDs intentionally reduce the denominator.

All typed records bind the exact source revision and scope in manifest.json. Domains, observations and operators are above. Stable dependencies and successful termination are premises. These partial relations retain ambiguous antecedents; no universal solver, unique inverse or actual causal inference is supplied.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. Old-code witnesses exposed CRLF sentence-block merging and missing act/form key panics in report accounting. Normalization repairs block identity and map defaults now agree with agree_one. Last-row-wins duplicate semantics are preserved, not silently relabeled as one observation per raw line. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.
