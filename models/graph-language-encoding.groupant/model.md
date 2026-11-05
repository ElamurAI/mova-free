# graph-language-encoding

## descriptant

### graph-language-encoding: wave29

Actual module declarations in kg/src/main.rs connect cols/gram/id/morph/uk/textgen; Morph and uk share Gram; textgen consumes uk::Fact and Morph synthesis; main writes Registry IDs and Edges using relation.local and full vertex Id. Registry keys are append IDs, not lexical concept indexes. global/Cargo.toml has no runtime dependencies and kg/Cargo.toml has no global dependency. Compile seed path is a separate tagged branch, not a claimed graph pipeline. Tests exercise actual Morph→parse→TSV and Fact→generate joins on invented inputs.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave29 report. Registration is not a universal proof or whole-system runtime claim.

## model

# graph-language-encoding

## forward

Tagged disjoint relation G = KG ⊎ GlobalBuild. KG: dictionary/vocabulary --Morph::load(Gram projection)--> analysis/synth maps; text+names --uk::parse--> Fact or Reject; Fact+Morph --render_check--> multiset-completeness result; Fact+Morph+Freq+OrderModel --generate--> optional sentence. In main ingestion each accepted Fact encodes one provenance and one edge/member through Registry IDs and Edges, sorted by(src,rel,dst). GlobalBuild separately maps fenced seed/optional TSV inputs to lexical concept-indexed Rust tables and expressions output. No adapter from GlobalBuild indices to kg Registry IDs, or persisted Edges back to Fact, is implemented in these sources.

## invariant

For valid aligned columns n<2^32 and single-writer valid registry state, per-member lemma/relation/grammar/flags survive encoding and codec roundtrip, with stable prior IDs after save/reopen. Word coordinates and member order are not persisted. Registry checked exhaustion returns error before mutation. render_check success constrains normalized word multiset only; not semantic roles or correct inference. All claims scoped to admitted representations and finite synthetic witnesses.

## cost

KG costs compose two dictionary decompressions plus per-text parsing with n<=8 but unbounded analyses, registry scans/hash lookups, E log E edge sorting and compression; optional generation O(h^2*2^h),1<=h<=12. Ingestion retains corpus/Facts/edges. GlobalBuild cost O(B+C log C+LC+R log R) plus duplicate scans and generated text; effect/resource costs are separate, not benchmarked.

## composition

Actual module declarations in kg/src/main.rs connect cols/gram/id/morph/uk/textgen; Morph and uk share Gram; textgen consumes uk::Fact and Morph synthesis; main writes Registry IDs and Edges using relation.local and full vertex Id. Registry keys are append IDs, not lexical concept indexes. global/Cargo.toml has no runtime dependencies and kg/Cargo.toml has no global dependency. Compile seed path is a separate tagged branch, not a claimed graph pipeline. Tests exercise actual Morph→parse→TSV and Fact→generate joins on invented inputs.

## domain

Finite UTF-8 synthetic text, unchanged tiny bz2 dictionary between passes, valid parser-derived Gram,1..8 word parse gate,1..12 explicit generation heads, aligned edge vectors below2^32, sorted src for out, valid unique registry files under single writer. Build fixtures use isolated temporary seed/out directories with absent optional closed tables. Resource exhaustion outside tested domain.

## limitations

No live datasets/DB/teacher/GPU/held-out execution. Tatoeba ingestion/show/stats and nonempty optional build tables remain unexecuted. Hash-map frequency tie order in stats is not deterministic; float/u32 frequency overflow and nontransactional persistence have no new guarantee. Exact categorical semantics have no useful executable percentage approximant established.

## checks

- 15 kg named outcomes twice debug/twice release, normalized outcomes equal; 3 standalone global build outcomes twice debug/twice optimized (2 substantive plus child driver).
- Eight compiled mutations, compile exit0/test exit101; byte-exact restoration and complete positive rerun.
- Old-code max-global-heap synthetic witness panicked overflow; corrected test passes debug/release and checks no state mutation.
- Eight parent source syntax projections generated twice byte-identically;12 scoped child ranges hash-bound to parent source.
- Exact native command specs and per-source mappings retained as native-commands.json/source-test-map.json.

## catalog

## behavior-complement

KG rejected parser gates emit no Fact; successful render excludes mismatched normalized multiset only. Mask filtering excludes any missing-bit row; codecs exclude value loss on encoder-domain streams. Checked registry exhaustion excludes ID wrap/heap insertion. GlobalBuild success excludes duplicate canonical concept/link codes but does not exclude ignored blocks or partial write effects on failure. These exclusions are tag-specific and cannot transfer to absent KG↔GlobalBuild adapter.

## reverse-reconstruction

KG output edges+Registry admit antecedent Facts with same member lemma/relation/gram but different lost word coordinates/order; filters admit arbitrary rejected rows; sorted graphs admit input permutations. Realization sentence admits multiple Fact/dictionary/count antecedents. GlobalBuild table preimage admits whitespace/comments/ignored or excluded input and absent/unreadable optional files. No unique source-text or registry reconstruction from lexical global index is implied.

## grammar-transport

Morph Gram::from_vesum keys meet uk analysis grammar and textgen synth grammar after NEG/IMPLICIT masking. Implicit subject copies predicate person/number/gender; main stores fact gram in every gs and member gram in gd. This preserves packed bits, not original VESUM tags, discarded xp tags in Gram, or semantic correctness.

## identity-boundary

Registry::Id=(heap<<32)|local is split exactly; relation edges use local only and require separate rel heap context. Global concept IDs are lexical vector positions with no heap/local encoding. Equating these numbers would fail even for a one-concept compiler and first global heap2^27.

## completeness-vs-realization

render_check sorts normalized words; generate chooses conditional/global-frequency variants and DP order. Therefore render_check(text,Fact)=Ok does not imply generate(Fact)=text, nor does generation establish role correctness. Synthetic tests separately witness each actual function.

## effect-frontiers

sub bad mask precedes reads; parse_lines loads input/names/Morph before output creation; ingestion creates names before Registry open and writes columns before registry save. Build expressions writes before global.rs. Success/failure does not compose into a transaction across these files.

## wave38

# Registry composition models

These source-derived relations refine the registry branch of this parent. Other grammar, edge and GlobalBuild branches keep their existing domains. The manifest binds the actual child revisions; no composed semantic Rust proof or transaction guarantee is inferred.

## admitted_identity_composition

Type: persistence_composition. Observe the exact heap label n, key k and full identifier I across heap(kind,n) -> id(h,k) -> save -> open -> lookup. Domain: UTF-8 labels H={n: no LF and no terminal CR}, keys K={k: no TAB, LF or CR}; existing heap names/numbers and key rows are unique, well-formed and consistent, local indices fit u32, one writer and successful actual writes/flushes. Imported data outside these premises is not certified.

For an admitted new or existing pair, I=(h<<32)|l, with heap(I)=h and local(I)=l. Heap encoding keeps the entire third TSV field, while key encoding keeps the remainder after the first TAB. Both codecs preserve tabs in that final field; K is deliberately stricter than codec invertibility. Their admitted encodings retain n and k after reopen, so the name/number and key/local lookups return the same I. A second save with no new rows appends nothing. This composition retains byte distinctions between canonically equivalent Unicode strings.

Failure is not a multi-file rollback: name/key refusal precedes their own insertion, but an earlier successful heap creation can remain in memory if a later key call fails. save writes metadata and key stages separately. No crash, flush-failure or concurrency guarantee follows. Actual-source witnesses: id::wave38::unusual_exact_heap_names_reopen_with_stable_numbers and id::wave37::admitted_unicode_keys_reopen_with_exact_ids. Their separate success supports the scoped composition; it does not prove every graph ingestion adapter.

## identifier_history_preimages

Type: identity_preimage_product. Observe persisted metadata, a full I and its decoded label/key. Under the unique valid registry domain above, require a registered h and an allocated local index 1<=l<names.len(). Splitting I fixes h and l and the metadata fixes their decoded strings. Without that premise, heap_name uses the fallback "?" and key uses an empty-string fallback, which collide with admitted literal names/keys. It leaves all call histories that allocated the same ordered distinct names and keys, interleaved repeated lookups, saved-counter/cache states and unrelated heaps compatible. Neither the original text, grammar roles nor member order is reconstructed by an identifier alone.

For legacy row bytes ending in CRLF, the old LF writer with an extra terminal CR and a CRLF-formatted importer can yield the same decoded label/key. The preimage must retain both writer histories unless independent provenance narrows it. Newly narrowed admission prevents those new names, while open/find still decode old bytes as before. Heap and key policies have different admitted sets, so an inverse law for one cannot be transferred to the other without checking its premises. Native legacy-heap and legacy-key fixtures witness this ambiguity; arbitrary imported duplicate names/numbers and multiwriter races remain outside this model.

## History

### 2026-10-31 · descriptant

2026-10-31 wave38: [scoped child-composition relations](model.md#wave38) distinguish admission, persistence, strict-consumer framing and history preimages. Integrated validation and its limits are recorded in the wave38 receipt; these additions do not certify every parent branch.

### 2026-11-01 · descriptant

#### Paired composition memory — 2026-11-01

The persistence vidtvorant and decoder-fiber/admission obernants jointly explain the key and heap repairs: exact admitted identity depends on each codec's own domain, actual save/reopen and state-preserving rejection. Child key policy cannot be copied to heap labels without losing valid TAB/interior-CR names. Parent reconstruction also cannot recover overwritten byte histories or lookup history from stored IDs alone. See the [source-grounded cases](../../docs/mathematical-modeling.md); retain the wave38 premises and historical receipts rather than promoting them to a whole-graph proof.

### 2026-11-01 · document

#### Imported duplicate boundary — 2026-11-01

The unique-row premise above is necessary. In the actual registry, duplicate heap numbers can make two labels point to one number while reverse lookup picks the first label; duplicate decoded keys at consecutive indices produce distinct full IDs with the same key while find picks the last index. Thus even if an edge preserves a full vertex ID, decoding its label is not an inverse of every imported name-to-ID association. `kg/src/main.rs` stores relation IDs using `local()`: `(h1,l)` and `(h2,l)` project to the same relation number, so the relation heap context is also necessary to reconstruct a full ID. The synthetic actual-API witness checks these collisions and reopen with successful I/O; it does not execute graph ingestion or establish graph-wide semantics. Nontransactional saves and arbitrary malformed imports remain outside this observation.
