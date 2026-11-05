# hbo-library

## descriptant

### Descriptant

Measured epoch: bounded supplied fixtures passed twice in debug and release; six behavioral mutants were rejected after compilation, each restored exactly. These observations support the scoped equations without extending them to corpus accuracy, semantic understanding or persistent learning. Production code bytes are unchanged; only isolated tests appended. No benchmark or percentage surrogate was introduced.

Retained caveats: Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## approximant

### Forward relation

Raw lines tokenize -> normalize keys -> contextual split -> cut_as or PUNCT; parser_view copies and normalizes word forms/clears UPOS; merge copies columns2..7 by flat word order after per-sentence counts. load_seg optionally reads sibling unpointed fallback. annotate writes parser input, spawns, reads, merges, removes temporary paths only on success.

Domain: cut_as requires nonempty cut result; sentence ID addition representable. merge equal sentence counts checked globally, word counts incrementally; earlier sentences can mutate before late error. Parser alignment is positional only; equal count does not establish same form or sentence identity.

## composition

### Local composition

Raw lines tokenize -> normalize keys -> contextual split -> cut_as or PUNCT; parser_view copies and normalizes word forms/clears UPOS; merge copies columns2..7 by flat word order after per-sentence counts. load_seg optionally reads sibling unpointed fallback. annotate writes parser input, spawns, reads, merges, removes temporary paths only on success.

Nested scopes isolate the transitions described in their manifests and share the whole-source syntax projection. Other declarations and test helpers remain represented by the parent model. Cross-file joins: text units/norm feed seg Piece coordinates; conllu Token/Word carries segmenter and lattice representations; library cut_as realizes lattice callbacks; main routes supplied helpers and external workflows. These joins do not validate arbitrary external parser alignment.

## models

# Typed model catalog

## behavior_complement

merge never copies ID, FORM, DEPS or MISC; a sentence-count error cannot mutate base, but a later word-count error can leave earlier annotations. parser_view cannot preserve UPOS. No parser execution follows from supplied helper tests.

## reverse_reconstruction

Merged result fixes preserved base columns and parsed2..7 but admits arbitrary old annotations, parsed IDs/forms/MISC and equal-count wrong alignments. Seg metadata is explanatory output, not enough to reconstruct original whitespace or segmenter state.

## spacing_transport

SpaceAfter=No goes on token wrapper for multiword tokens and word MISC for single words. H anywhere adds Swallowed; why spaces become commas. Empty pieces panic at first-word access.

## parser_projection

ASCII : . ? ! -> ׃; other noncanonical PUNCT -> ׀; else norm at parser level. Every UPOS resets '_', while token written form and comments remain base values.

## merge_partial_effect

After global S equality, each successful sentence mutates columns2..7 before next size check. Transactional rollback is absent; filesystem wrapper avoids writing output on merge Err but in-memory base may already change.

## annotate_effect_order

write input precedes subprocess spawn/status, output read and merge. Failed spawn/status/read/merge leaves input and possibly output; cleanup is best-effort only after success. Real process/model paths remain unexecuted.

## tests

### Bounded evidence

Primary actual-code fixture: wave32::cuts_views_and_partial_merge_effects. Run using cargo test --offline --locked -p hbo --lib wave32::cuts_views_and_partial_merge_effects -- --exact --test-threads=1; repeat with --release. The external wave32 Hbo receipt set contains all 14 named outcomes twice per profile, numeric exit statuses, six compiled behavioral source negatives and byte-exact restoration hashes, and two identical source syntax projections. Source-local primary tests include assertions and caught expected panics. Fragments share these source witnesses; no claim of one test per branch.

Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## invariant-and-cost

### Invariant and cost

cut_as requires nonempty cut result; sentence ID addition representable. merge equal sentence counts checked globally, word counts incrementally; earlier sentences can mutate before late error. Parser alignment is positional only; equal count does not establish same form or sentence identity.

#### Cost

B bytes,W words,S sentences: segment includes model candidate costs plus O(B+W) text assembly; parser_view/merge O(B+W) copying, merge per-sentence word pointers O(Wmax). annotate costs serialization, filesystem IO and external subprocess unknown time/resource use.

Resources must permit allocations; cost variables are source bytes, units, words and table entries as explicitly stated. No measured speed or universal error claim.

## History

### 2026-10-31 · descriptant

2026-10-31, prospective epoch: inspected source before edits and predicted Raw lines tokenize -> normalize keys -> contextual split -> cut_as or PUNCT; parser_view copies and normalizes word forms/clears UPOS; merge copies columns2..7 by flat word order after per-sentence counts. load_seg optionally reads sibling unpointed fallback. annotate writes parser input, spawns, reads, merges, removes temporary paths only on success.

### Undated · approximant

Source snapshot SHA-256: 660faca62417f9a1eb9807a014127f1b4b868caf5e3f488a8cd6a1e716572143. This is a partial source-derived mathematical relation, not an executable surrogate. Tests exercise actual production helpers with synthetic inputs.

### Undated · document

Subject: hbo-library at SHA-256 660faca62417f9a1eb9807a014127f1b4b868caf5e3f488a8cd6a1e716572143. Domain and fixed state: cut_as requires nonempty cut result; sentence ID addition representable. merge equal sentence counts checked globally, word counts incrementally; earlier sentences can mutate before late error. Parser alignment is positional only; equal count does not establish same form or sentence identity.. Observation: return values, stated mutations and failure order of this scope. Models are partial source-derived relations; finite tests can falsify them but do not prove general completeness. Behavior complement is a scoped exclusion claim in the admitted transition universe. Reverse reconstruction retains compatible antecedents, not a unique historical cause. Specialized entries hold this domain fixed and isolate their named observation. Oracle: named actual-code fixtures in model.md#tests; unexecuted effect boundaries remain static hypotheses.
