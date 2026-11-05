# coreference-pipeline

## descriptant

### coreference-pipeline: wave27

doc supplies kids/depth/sub/quote to mention and it; mention fields feed sieve masks/head keys/candidate order; sieve returns full membership (including internal mentions) to eval; visible tests full cluster length before filtering. eval::sys_entities recomputes head via corefud::span_head, whereas out_mentions writes detector head, so no universal head equality is claimed. score is existing external sibling ownership. main::load joins strict CoNLL-U reading and optional retree, then documents. tests assertions cross actual joins and do not substitute copied algorithms.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave27 report. Registration is not a universal proof or whole-system runtime claim.

## model

# coreference-pipeline

## forward

For checked raw U, D=documents(U), M=detect(D), R=ordered_sieves(classify_it(M,D),config), E=visible_partition(R). Metric observation is score_doc(gold_of(Dgold), recomputed_span_heads(E)); serialized output is write(U,D,out_mentions(R)). doc::retree optionally replaces tree columns before D, retaining original gold Entity; gold scorer must use original gold-tree document. eval::run checks original gold self-score, then paired document count, renders repeated configurations, strips raw Entity and reconstructs documents for a leak comparison before final output.

## invariant

All transport coordinates are sentence index and zero-based inclusive word span; output relative head uses +1. On valid acyclic documents, resolver merges preserve partition and only coarsen it. Visibility removes internal mentions and singleton coordination groups before external scoring/writing. No gold Entity feeds ordinary detection/resolution. Source-specific exceptions remain: nearest bypasses reading/binding, speaker can bypass person agreement, from_words does not certify acyclicity.

## cost

Document construction O(sum n²+n log n); mention extraction conservative O(sum n³+M log M+text bytes); resolver conservative O(S M³ L+S M² n+S M² log M). Evaluation repeats these for named baseline/cumulative configurations and delegates metric assignment to existing coref-score (dense Hungarian worst O(r²c)). Writer conservative O(B+NM log M). This is symbolic source analysis, not timing or optimized-bound evidence.

## composition

doc supplies kids/depth/sub/quote to mention and it; mention fields feed sieve masks/head keys/candidate order; sieve returns full membership (including internal mentions) to eval; visible tests full cluster length before filtering. eval::sys_entities recomputes head via corefud::span_head, whereas out_mentions writes detector head, so no universal head equality is claimed. score is existing external sibling ownership. main::load joins strict CoNLL-U reading and optional retree, then documents. tests assertions cross actual joins and do not substitute copied algorithms.

## domain

Finite in-memory synthetic UTF-8 CoNLL-U with valid raw/index mappings and acyclic in-range trees, stable COREF_X/COREF_MD/PRAG_OFF snapshots. Native allocation/counter limits apply. Parser and malformed-input failure models explicitly distinguish weaker public/permissive adapters.

## limitations

No corpus/teacher/API/GPU actions. No production behavior edits; only inline tests added to existing main/tests files. Existing score.rs remains unchanged. Runtime control coverage excludes lib declarations and tests fixture source as independent production components.

## checks

- 13 named tests per execution: 12 library synthetic tests (including corefud parser) and one main load test; twice debug and twice release
- Seven temporary compiled production controls target doc quote reset, corefud rightmost close, it weather classification, mention coordination number, eval visibility, sieve principle B, main retree loading
- Exact source restoration and positive rerun after each control
- Nine source syntax projections generated twice and compared byte-for-byte

## catalog

## behavior-complement

For admitted default pipeline inputs, forbidden observations include Entity-dependent span/eid/note output, internal mentions in external entities, emitted unclosed/cross-sentence read mentions and cluster splitting. This relation is scoped: strip leak witness compares only span/eid/note; head/etype independence is not established by that check, and Nearest is outside default binding exclusions.

## reverse-reconstruction

R(output) is the set of raw documents, environment/config snapshots and merge traces whose parsing/detection/visibility/serialization equals the observation. Gold Entity variants, discarded syntax/comments, invisible mentions, OR-mask provenance and old MISC annotations can vary. After rebuilding Entity views from raw output, parsed generated output recovers span/eid pairs only for sentence-bounded, stack-compatible same-eid spans (including serialized e prefixes); valid crossing same-eid spans can reconstruct different spans. Existing Document views stay stale after raw write. Original text annotation, resolver trace and retree-discarded word DEPS are unrecoverable.

## span-head-transport

MSpan=(span, span_head(tree,span)); OutMention=(span,detected_head,visible_cluster_ordinal,note). Equal spans need not imply identical heads across gold/predicted trees. retree requires equal sentence/word forms but can change head matching.

## partition-visibility

Let C partition [0,M). Visible output filters v(m,C)=!internal(m) && !(coord(m)&&|C|=1&&!override), drops empty projected clusters, then assigns consecutive ids. A group plus one invisible member has |C|=2 and may survive; filtering and singleton detection do not commute.

## failure-effects

CLI eval mkdir precedes read. run gold parse/self-check precede length check and report; diagnostic file write precedes leak check; CoNLL-U write precedes caller report-file write. Failure may leave earlier effects; there is no cross-file transaction.

## micro-metrics

Aggregate Counts=sum over aligned documents, then compute ratios/F1. Averaging per-document F1 is a different function. Empty denominators in scorer map0, whereas CLI mention-stat raw recall/precision can print NaN; do not unify their zero semantics.
