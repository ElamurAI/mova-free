# hbo-seg

## descriptant

### Descriptant

Measured epoch: bounded supplied fixtures passed twice in debug and release; six behavioral mutants were rejected after compilation, each restored exactly. These observations support the scoped equations without extending them to corpus accuracy, semantic understanding or persistent learning. Production code bytes are unchanged; only isolated tests appended. No benchmark or percentage surrogate was introduced.

Retained caveats: Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## approximant

### Forward relation

Candidate pipeline: fitting commonest lex label; partial-point lex merge; eligible pointed thin fallback; longest accepted prefix context and rest/suffix Cartesian alternatives; host/chain filters; greedy-first or stable share ordering; consonant validation and unsplit fallback; sequential neighbour smoothing. save/load project tables/options and rebuild letters index.

Domain: Nonempty well-formed gold words; unit sums/usize and u32 counts representable. Valid cuts sum to units and exclude zero-length Cut. H consumes0; leading cut moves previous vowels. Probability domain positive table totals, finite host factor>=0, finite K>=0 with total+K>0. Public malformed states can violate premises.

## composition

### Local composition

Candidate pipeline: fitting commonest lex label; partial-point lex merge; eligible pointed thin fallback; longest accepted prefix context and rest/suffix Cartesian alternatives; host/chain filters; greedy-first or stable share ordering; consonant validation and unsplit fallback; sequential neighbour smoothing. save/load project tables/options and rebuild letters index.

Nested scopes isolate the transitions described in their manifests and share the whole-source syntax projection. Other declarations and test helpers remain represented by the parent model. Cross-file joins: text units/norm feed seg Piece coordinates; conllu Token/Word carries segmenter and lattice representations; library cut_as realizes lattice callbacks; main routes supplied helpers and external workflows. These joins do not validate arbitrary external parser alignment.

## models

# Typed model catalog

## behavior_complement

Consonant-checked nonempty admissible outputs cannot lose/invent unpointed letters after removing H. This does not hold as a blanket claim for empty form: fallback Cut(0) is returned even though spells rejects it. Context does not rerank lexp/rules.

## reverse_reconstruction

Pieces plus written form determine cut strings under valid coordinates; pieces alone admit all forms with matching unit length. Candidate winners admit many count tables and context histories. save cannot recover fallback, lex/rest booleans, absent level versus default Pointed, or original TSV order.

## piece_transport

Cut(n,lead) advances n units; H emits ה without advancement. lead transfers only vowel subset from previous non-H output and keeps dagesh. Direct cut can truncate oversized first piece then panic on later slices; spells checks length before calling cut.

## fallback_precedence

Exact accepted lex precedes partial lex, which precedes fallback. Fallback requires partial, Pointed, thinly and attached model; it calls raw_candidates and prefixes why, leaving outer consonant check active. Mater-only אב is not thin; bare בב is.

## context_smoothing

For each neighbour in previous then next order q=(count(label)+K*p)/(total+K); next smoothing uses updated p, so order matters. Stable descending sort preserves earlier order on equal/NaN comparison. Original lex tie uses smaller label.

## codec_counts

load duplicate TSV key/label rows add u32 counts, unknown rows skip, malformed counts become0. save emits sorted BTree entries, optional switches and no fallback. Index rebuilt from lex groups by unpointed form.

## tests

### Bounded evidence

Primary actual-code fixture: seg::wave32::supplied_states_context_fallback_and_codec. Run using cargo test --offline --locked -p hbo --lib seg::wave32::supplied_states_context_fallback_and_codec -- --exact --test-threads=1; repeat with --release. The external wave32 Hbo receipt set contains all 14 named outcomes twice per profile, numeric exit statuses, six compiled behavioral source negatives and byte-exact restoration hashes, and two identical source syntax projections. Source-local primary tests include assertions and caught expected panics. Fragments share these source witnesses; no claim of one test per branch.

Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## invariant-and-cost

### Invariant and cost

Nonempty well-formed gold words; unit sums/usize and u32 counts representable. Valid cuts sum to units and exclude zero-length Cut. H consumes0; leading cut moves previous vowels. Probability domain positive table totals, finite host factor>=0, finite K>=0 with total+K>0. Public malformed states can violate premises.

#### Cost

U units,B bytes,E table entries,L alternatives,P prefix alternatives,C generated prefix×suffix candidates,K=min(max_k,U). Prefix context construction copies O(K²) units. For each prefix p whose rest lookup does not supply suffixes, suffix contexts copy O(K_p²) units, K_p=min(max_k,remaining units). Total context copying is O(K²+sum_p K_p²), conservatively O((P+1)K²), with actual UTF-8 unit byte lengths multiplying copy work; per-prefix rest concatenation additionally totals O(PB). BTree lookup O(log E) comparisons, candidate sorting O(C log C); cuts/normalization add O(C(B+U)), partial lex scans matching forms. Evaluation span membership O(Pgold*Ppred). Memory O(E text+C(U+B)+sentence keys).

Resources must permit allocations; cost variables are source bytes, units, words and table entries as explicitly stated. No measured speed or universal error claim.

## History

### 2026-10-31 · descriptant

2026-10-31, prospective epoch: inspected source before edits and predicted Candidate pipeline: fitting commonest lex label; partial-point lex merge; eligible pointed thin fallback; longest accepted prefix context and rest/suffix Cartesian alternatives; host/chain filters; greedy-first or stable share ordering; consonant validation and unsplit fallback; sequential neighbour smoothing. save/load project tables/options and rebuild letters index.

#### Cost clarification 2026-10-31

Independent source review refined symbolic multiplicities: lattice accounting includes token-vector/comment scans and repeated candidate-generating callbacks; raw candidate context construction includes suffix contexts rebuilt per prefix alternative. See model.md#invariant-and-cost for the separate terms. This documentation-only correction changes no Rust bytes, syntax binding or native evidence and makes no new timing claim.

### Undated · approximant

Source snapshot SHA-256: 1efafc6b8585f52c95ae888311bced3f5a9519555edaf78ff0e536450221ce5d. This is a partial source-derived mathematical relation, not an executable surrogate. Tests exercise actual production helpers with synthetic inputs.

### Undated · document

Subject: hbo-seg at SHA-256 1efafc6b8585f52c95ae888311bced3f5a9519555edaf78ff0e536450221ce5d. Domain and fixed state: Nonempty well-formed gold words; unit sums/usize and u32 counts representable. Valid cuts sum to units and exclude zero-length Cut. H consumes0; leading cut moves previous vowels. Probability domain positive table totals, finite host factor>=0, finite K>=0 with total+K>0. Public malformed states can violate premises.. Observation: return values, stated mutations and failure order of this scope. Models are partial source-derived relations; finite tests can falsify them but do not prove general completeness. Behavior complement is a scoped exclusion claim in the admitted transition universe. Reverse reconstruction retains compatible antecedents, not a unique historical cause. Specialized entries hold this domain fixed and isolate their named observation. Oracle: named actual-code fixtures in model.md#tests; unexecuted effect boundaries remain static hypotheses.
