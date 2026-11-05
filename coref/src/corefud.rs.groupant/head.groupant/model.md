# coref-corefud-head

## descriptant

### coref-corefud-head

Tree candidate set -> minimum depth -> punctuation exclusion -> first index, with start fallback.

## approximant

### Scoped forward relation

Tree candidate set -> minimum depth -> punctuation exclusion -> first index, with start fallback.

Parent transition model (restricted to this byte scope): Scan values left to right. '(' emits Open(eid, first-hyphen remainder), ')' immediately emits Close for singletons; other text through ')' emits Close. Open appends (sent,start,MAX); Close removes the rightmost equal eid and sets inclusive end, rejecting no-open before cross-sentence. Final nonempty stack errors. Writer opens descending end then eid, closes descending start then eid; head field=head-start+1. span_head chooses external-parent candidates, minimum depth, nonpunctuation, lowest index; no candidates returns start.

## invariant

### Scoped invariant/cost

For successful read, all emitted ends are assigned in the opening sentence. Writer valid-domain spans have start<=head<=end<sentence length and file_idx maps to the same raw words. strip removes only Entity/Bridge/SplitAnte fields across all rows. Output ordering does not establish mention disjointness.

B annotation bytes, M mentions, N words: reading O(B+M²) worst case for rightmost-search/remove; writing filters all mentions per sentence and per word, conservative O(B+NM log M), O(B+M) temporary/output storage; span_head O(span width).

Scope: Tree candidate set -> minimum depth -> punctuation exclusion -> first index, with start fallback.. Parent bounds are conservative; no separate timing measured.

## models

Scope: Tree candidate set -> minimum depth -> punctuation exclusion -> first index, with start fallback.. Relations below are restricted to this fragment's transitions and retained parent state.

# Typed models

## behavior_complement

For well-formed Document, successful read excludes an unmatched close, final open or completed cross-sentence mention. write excludes old Entity/Bridge/SplitAnte on processed words; it does not exclude stale annotations in documents omitted by zip truncation. strip excludes these fields globally but preserves other raw fields.

## reverse_reconstruction

R(y)={documents whose event fold produces ordered spans/eids/etype y}. Additional non-Entity MISC and discarded fields after etype are free. After rebuilding Document Entity views from written raw rows, valid sentence-bounded spans with stack-compatible same-eid nesting and writer-generated ids recover span/eid pairs (with the serialized e prefix), not original annotations or note/head text. Merely in-range spans are insufficient: crossing same-eid spans [0,2] and [1,3] read back as [0,3] and [1,2]. Existing Document Entity views remain stale after write. Head choice has multiple compatible trees.

## event_stack

Close(e) binds max index in open stack with eid=e, so nested reused ids close inner first; unknown closes fail before final-open checking.

## coordinate_transport

Span sent/start/end are zero-based inclusive; serialized relative head is one-based. No validation prevents underflow for head<start or out-of-range row indexing.

## serialization_order

Open key=(-end,eid), close key=(-start,eid); singleton closes inline. Remaining MISC sorts by lowercase field key; equal-key stable order retained.

## effect_boundary

write updates sequentially using docs.zip(per_doc), assumes valid raw indexes and does not refresh token views. strip is infallible only for structurally valid rows; neither provides rollback.

No executable percentage surrogate: categorical state/effect semantics currently lack a useful cheaper observation metric. Supported future caps are 1%, 2%, 5%, 10%; no profile is claimed here.

## tests

### Evidence

Safe synthetic command: env -u COREF_X -u COREF_MD -u PRAG_OFF cargo test -p coref tests:: -- --skip score::. Two debug and two release executions are retained externally with numeric exit status and sorted named outcomes. Selected checks:

- corefud::tests::parse_gum_values
- tests::wave27_entity_boundaries_and_head_ties
- tests::write_read_round_trip
- tests::resolver_ignores_gold_entities

See source-test-map.json and controls.json in the wave27 coref run receipts. The library declaration file has compilation evidence only. No corpus, teacher, GPU or speed measurement.

Partial scope: No arbitrary malformed public rows/spans, allocation failure or full grammar proof. Writer zip truncation and unrefreshed derived views are modeled limitations.

#### Measured receipts

Final source state: 13 named tests (12 library, 1 binary) passed twice debug and twice release, 52 total executed test instances; all four command exits0 and sorted named outcomes identical. Five added tests are four library boundary tests plus one main load test. The pre-existing score tests and groupant_transfers were filtered out.

Compiled actual-source control: tests::wave27_entity_boundaries_and_head_ties. Compile exit0, assertion-run exit101, exact bytes restored, positive rerun exit0. Retained logs: control-corefud-compile.log, control-corefud-red.log, control-corefud-restored.log.

Nine source AST projections were generated twice identically. Shared ancestor declarations are deferred to root integration; local owned leaves have no structural diagnostics. No timings or general accuracy claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: scoped view of parent state; no independent completeness claim.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.

Review witness, 2026-10-31: an external Rust binary linked the actual coref/doc public APIs against bounded four-word synthetic input. It confirmed crossing same-eid spans [0,2],[1,3] reconstruct as [0,3],[1,2], existing Document Entity caches remain stale after raw write, and retree forces output word DEPS to _ while preserving the original input. Compile and execution exited0. This single debug witness is additional to the paired suite, not a new paired/release test. Source and numeric receipts: annotation wave27 external run, coref-review-witness.rs and coref-review-witness*.exit/log.
