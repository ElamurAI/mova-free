# english-lexical

## descriptant

### english-lexical: wave27

Tag and ptag call dict/hash/gram; ptag uses TnT prior only unknown words. Morph's UD/AGID/rule priority interacts with lexgen src flags and record order: changing a/r/u alters preferred analyses. tok spacing bits are not exact whitespace; consumers cannot infer original byte offsets from detokenize. text order is 1-based while array positions and dictionary indexes are 0-based. udreg validates selected enum features, so upstream Feats::parse unknown-dropping cannot become a complete annotation gate. Existing conllu callers supply Sentence/Token to learning/generation.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave27 report. Registration is not a universal proof or whole-system runtime claim.

## model

# english-lexical

## forward

Input characters → tok FSA/splits/merge → forms; tag/PTagger map forms to PTB classes. Morph maps form/tag to lemma and inflects with optional tree_feats agreement; encode retains Raw when inflection/case cannot reproduce form. gram codecs/feature bits connect parsed annotations to udreg predicates. hash numeric/string keys feed lexical/tagger tables; dict probes resolve generated blob entries. lexgen converts synthetic UD+AGID to append-position lemma heap and source-flagged forms, which build.rs compiles into dict (build stage not exercised by runtime temp-file test). text detok consumes one-based order and spacing; chrF/LCS observe lossy output summaries.

## invariant

On valid generated dictionary layout and supported trained candidate sets, categorical outputs follow explicit source ordering. Morph decode(encode(form,tag),tag)=form for terminating calls. tok preserves form characters except NUL and normalizes whitespace; text detok preserves given ordered form strings with spacing policy. Dictionary IDs stable only across append-preserving generation; fingerprint binds loaded persisted model to blob. No blanket determinism claim for external environment-sensitive training.

## cost

Token FSA linear in characters; postprocessing can be quadratic. Dictionary probing needs valid empty slots; fingerprint linear once. TnT training NT^3 plus counts, Viterbi n*K^3 time and n*K^2 storage. Perceptron beam roughly n*b*(features+NT log NT)+n*b^2. Lexgen retained rows and sorted lemma sets U log U; two sequential file writes. chrF six ngram orders expected linear hashing; LCS A*B time/B live space. No benchmark or speed claim.

## composition

Tag and ptag call dict/hash/gram; ptag uses TnT prior only unknown words. Morph's UD/AGID/rule priority interacts with lexgen src flags and record order: changing a/r/u alters preferred analyses. tok spacing bits are not exact whitespace; consumers cannot infer original byte offsets from detokenize. text order is 1-based while array positions and dictionary indexes are 0-based. udreg validates selected enum features, so upstream Feats::parse unknown-dropping cannot become a complete annotation gate. Existing conllu callers supply Sentence/Token to learning/generation.

## domain

Bounded UTF-8 synthetic fixtures, frozen generated dictionary from repository build inputs; valid index domains and aligned arrays. Tagging nonempty sequences requires supported candidate paths; tree conjunction ancestry must terminate. Tiny in-memory training only; lexgen tempfile effects.

## limitations

No corpus, network, teacher, GPU, held-out evaluation or model mutation. No assurance for malformed serialized models/blobs, huge counters, allocation/I/O fault injection, cyclic tree paths. Build-time dictionary generation not semantically revalidated by temp output fixture.

## checks

- cargo test -p en --lib wave27_tests -- --test-threads=1: 10 named tests, twice debug and twice release, each exit0
- Three compiled controls: hash zero byte contribution, text empty chrF=1, lexgen omitted PastPart; each 0 passed/1 failed, exit101; restored ten-test run exit0
- Paired mova-modelcheck syntax projections for ten sources

## catalog

## behavior-complement

Composition excludes exact raw-whitespace recovery, unknown-feature validation after lossy parse, and transactional two-file generation. Hash equality cannot establish lexical identity; dict must compare bytes. Beam output cannot be claimed a global Viterbi optimum.

## reverse-reconstruction

Compatible antecedents for output tags include all lexical/model states attaining chosen score/order; they are many-valued. Morph codec restores exact original form via case/Raw witness, but lemmas alone do not invert forms. Tokenized rendering has arbitrary whitespace antecedents. TSV artifacts reconstruct IDs only with old-heap/order provenance.

## dictionary-flags

lexgen UD rows precede AGID; a/ar/r/b flags become dict UD/AGID/REV/BASE bits via build.rs. Morph UD max-count uses later tie while AGID uses first main match; preserving rows and flags is required.

## coordinate-transport

gram Sym high bit distinguishes graph-local heap from frozen global dictionary. text detok and Morph tree heads use1-based token coordinates; dict record ids and arrays use0-based coordinates; blind interchange invalid.

## candidate-support

TnT emits only seen positive candidate classes, PTagger masks unseen unless forced or known shortcut. Empty training supports empty tagging but nonempty unconstrained inference is not total; parents cannot assume length-preserving success on unsupported models.

## annotation-projection

Feats parses known pairs to bits and counts unknowns separately; udreg sees only bits. Reconstructing original annotation or validating dropped pairs requires caller to preserve unknown count/raw source.

## History

### 2026-11-04 · descriptant

2026-11-04 epoch (en-morph): two children added. en-derive reads a form the dictionary lacks as a known open-class word with affixes (seed data, spelling by join); morph::known_form and the lemmatizer call it, and ptag/tag take its tags as the unknown-word dictionary hint (EN_DERIVE_TAGS=0 is the reference). en-tagrule forces IN on «so» of purpose; annotate::tags (english-structure) merges it after emode::force. Interaction assumptions: derive reads only the compiled dictionary (no model state), so a model file's dictionary fingerprint is unchanged; tag hints change only words absent from the dictionary; forced tags reach the parser as ordinary tags. Measured on 2026-11-04: Owls' papers unknown 13.59% → 12.19%; EWT dev LAS with en.bin 80.30 → 80.30.

2026-11-04 epoch (lexicon growth): two children added. en-lexgrow generates the lexicon extension from OEWN 2025, SPECIALIST 2026, Census surnames and the Names Corpus; build.rs compiles it into a blob of its own; en-lexext reads it for morph (known_form, lemmas, inflection), derive (bases and classes) and the taggers' unknown-word hint (case-gated). Interaction assumptions: the dictionary blob and fingerprint are unchanged, so models load; the extension answers only for forms the dictionary lacks (plus classes a dictionary word lacks, for derive bases only); EN_LEXEXT=0 is byte-identical to before. Measured 2026-11-04: Owls' papers unknown 9.24% → 5.54% (55 texts); EWT dev LAS 80.30 → 80.28, GUM dev 79.07 → 79.07.
