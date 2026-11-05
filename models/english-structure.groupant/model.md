# english-structure

## descriptant

### english-structure: wave27

Heads use root0 and1-based word coordinates; Graph truncates heads to u16, while parser/UD use usize. Ud children returns0-based child indices. GNode lemma references global dictionary or local graph heap; Store fingerprint binds global IDs but not arbitrary target serde schema. annotate repairs tree whereas kbest/complete/explain/Model.encode do not; treating them as identical is invalid. Expert consumes conllu Sentence, not Graph directly; callers must preserve tags/features/relations when adapting. Construct callbacks must return enough dependent entries and valid renderable heads.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave27 report. Registration is not a universal proof or whole-system runtime claim.

## model

# english-structure

## forward

Model encodes tok->PTagger->Parser->Morph into Graph; Linearizer generates an order for graph decode. Annotator composes tag->parse->EModE repair->Ud child flags/UPOS/two-pass FEATS->morph/EModE word completion. Store envelopes serialized Annotator/Model with dictionary-bound header checks. Explained separately parses and encodes, then exposes trace/graph/order/decode. Expert checks sentence facts against injective rule bindings; Construct aggregates exact target arc matches and compares rounded recall against floors.

## invariant

For aligned arrays with valid indices and a nonempty-trained tagger, token forms/count survive annotation completion; valid parser transitions preserve root0 and produce one-root projective completed trees. Serialized values recover under the same schema/dictionary subject to serde domain. These do not imply accuracy, valid arbitrary public Graphs, or byte-lossless original whitespace.

## cost

Let n tokens, b beam, F features, R labels, k side degree, A target arcs, m expert variables, B wire bytes. Parser O(n*F*(3+R)*b+b*n^2+n*b log b); UD O(n^2); linearizer sum O(2^k*k^2) for k<=10 else O(k^2), plus construction scans. Expert worst n!/(n-m)! predicate work. Construct recall O(total token count + A + map/key work + callbacks); misses additionally formats each missed sentence, proportional to emitted bytes. Store O(B + typed element/visitor work) plus typed allocations; zero-byte element sequences can require work beyond wire length. Explain repeats parse/encode and invokes ordering for presentation/decode; no measured speed claim.

## composition

Heads use root0 and1-based word coordinates; Graph truncates heads to u16, while parser/UD use usize. Ud children returns0-based child indices. GNode lemma references global dictionary or local graph heap; Store fingerprint binds global IDs but not arbitrary target serde schema. annotate repairs tree whereas kbest/complete/explain/Model.encode do not; treating them as identical is invalid. Expert consumes conllu Sentence, not Graph directly; callers must preserve tags/features/relations when adapting. Construct callbacks must return enough dependent entries and valid renderable heads.

## domain

Bounded synthetic sentences and same-build serializable types; parallel arrays aligned, graph symbols valid, valid tree indices, finite table scores and bounded counters. Tests exercise2 synthetic one-token training sentences only for model/explain, never corpus/held-out/API/GPU.

## limitations

No allocator exhaustion, integer-overflow proof, fsync or concurrent save guarantee. Environment EN_AGR/EN_PX/EN_BEAM can change semantics; tests use explicit beam1 where available. Preflight discovered empty-trained PTagger panics on nonempty tagging; model/explain use nonempty synthetic training and preserve that gap.

## checks

- Nine actual-source wave27::bounded_contract tests, twice debug/twice release, normalized names identical
- Compiled construct floor, parser root-legality and store checksum mutations rejected with exit101; restored9-test runs exit0
- Each source partial syntax generated twice identically; source/fragment byte hashes retained

## catalog

## behavior-complement

Compatible composition cannot equate completion/kbest with repair-aware annotate, cannot accept changed envelope checksum, cannot bind one sentence word to two expert variables, cannot label equality with a rounded construction floor red. Invalid public coordinates and empty-trained taggers are outside admitted runtime contract.

## reverse-reconstruction

For observed graph/annotation/report y, preimage is all text/model/table/rule states satisfying the forward pipeline and same y; morphology/backoff, aggregated scores and parse selection lose information. Store typed roundtrip recovers model values but not unique map byte order or training antecedents. Explained.text alone exactly preserves its input string.

## head-coordinate-transport

Parser tree index i is token i+1; annotation heads stay usize; GNode head=(parser head as u16); Ud children maps head h to slot h-1. Validity requires heads<=n and graph n<=65535 for lossless coordinate transport.

## annotation-branch-join

repair is between parse and child features only in annotate. complete and kbest recompute child-derived UPOS/FEATS without that transformation; equal raw parser candidates need not imply equal final annotations.

## model-wire-boundary

Annotator save packs before directory/write/rename. Unpack gate order magic/size->version->enum sizes->dictionary->length->checksum->typed body. Accepted envelope does not certify caller-selected target type.

## observability-cost

Explain requests parser trace then independently Model.encode, so semantic presentation includes repeated inference and trace stack copies. Parent runtime is sum of real calls, not max child cost or assumed shared cache.
