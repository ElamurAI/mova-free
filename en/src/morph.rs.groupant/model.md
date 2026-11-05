# en-morph

## descriptant

### en-morph
Dictionary/rule morphology, case escape codec and tree agreement

## approximant

### Forward relation
Apostrophe normalize, archaic, UD max count later tie, base, REV slot/first, rule reproduction/known priorities. Inflect UD/base/first AGID/suffix. Encode AsIs,Title,Upper(len>1),Raw. Tree one-based heads, first subject.

No executable Rust surrogate attached: no demonstrated metric for preserving these categorical, stateful or effect observations. Supported future caps are 1/2/5/10%, not a claim of current error.

## invariant

### Domain, invariant and cost
Apostrophe normalize, archaic, UD max count later tie, base, REV slot/first, rule reproduction/known priorities. Inflect UD/base/first AGID/suffix. Encode AsIs,Title,Upper(len>1),Raw. Tree one-based heads, first subject.
No general lemma/inflect inverse. Tree arrays aligned and conjunction ancestry acyclic required; non-self cycles may loop. Rules not linguistic proof.
Record scans/string candidates; tree adjacency O(n), conjunction walks up to quadratic or cyclic nontermination; u32 sums unchecked.
Inputs must satisfy the indexing/generated-data assumptions stated by the typed catalog; allocation failure and overflow beyond bounded fixtures remain untested.

## composition

### Mathematical combiner
Source bytes and twice-generated syntax constrain this model. Forward relation: Apostrophe normalize, archaic, UD max count later tie, base, REV slot/first, rule reproduction/known priorities. Inflect UD/base/first AGID/suffix. Encode AsIs,Title,Upper(len>1),Raw. Tree one-based heads, first subject.
Reverse relation: decode(encode(f,t),t)=f for terminating calls; decode preimages include Raw and inflection/case equivalents; lemma alone many-to-one.
The ordered typed family records independent exclusions, compatible antecedents and source-specific boundaries. Fragment ranges compose through actual control flow; they do not cover all helpers.
Epoch memory: empty learned support, lossy projections and non-atomic effects cannot inherit stronger downstream guarantees.

## models

## behavior_complement
No general lemma/inflect inverse. Tree arrays aligned and conjunction ancestry acyclic required; non-self cycles may loop. Rules not linguistic proof.

## reverse_reconstruction
decode(encode(f,t),t)=f for terminating calls; decode preimages include Raw and inflection/case equivalents; lemma alone many-to-one.

## escape_roundtrip
Encode tests exact inflection then title then uppercase; Raw stores original bytes and bypasses morphology on decode.

## dictionary_ties
UD max_by_key chooses last equal count; reverse AGID chooses first compatible slot else first record, so different tie policies coexist.

## archaic_priority
ONLY_VERBS overrides even noun tag; remaining archaic irregular forms require verb/MD, known forms block ending rules.

## agreement_coordinates
Tree heads are 1-based with0 root, child vector keeps token order; first nsubj wins; pronoun conjunction chain has no general visited set.


## tests

### Actual source checks
Named check: morph::wave27_tests::lossless_escape_and_features. Command: cargo test -p en --lib wave27_tests -- --test-threads=1 (also --release). Four runs: each 10 passed, 0 failed, numeric exit0; sorted named outcomes equal. This file contributes one named check per run.
Control present for this source: false. Hash zeroed byte contribution, text changed empty chrF to1, lexgen removed three-cell PastPart copy: each compiled and failed one assertion (exit101), then exact source restoration and ten-test positive rerun (exit0). Other sources have no compiled mutation claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: read complete source before prospective model and bounded tests. Production code retained byte-for-byte; only inline synthetic checks added. Coverage is partial, not a proof.

### 2026-10-31 · tests

See evidence.json and external octopus-wave27-lexical-2026-10-31 receipts. Static built dictionary read is part of the linked library; no corpus or external services used. Unknown support, malformed model/blob, resource failures, cycles and exhaustive Unicode behavior are unexecuted gaps.

### 2026-11-04 · descriptant

2026-11-04 epoch (en-morph): known_form is the dictionary test (now derive::in_dictionary) or a derivation by derive::analyses; lemmatize_any_case tries, after a known candidate that reinflects, a candidate derived by derive::derived_as (N for plural, V or A otherwise) that reinflects. Source be759bd5cb5e; earlier receipts and evidence refer to the previous source hash and are kept as history.

2026-11-04 epoch (lexicon growth): known_form also accepts a form of the lexicon extension; lemmatize_any_case takes the extension's lemma for a form the dictionary lacks (vertices NNS → vertex) and accepts extension lemmas among the suffix-rule candidates (re-emerged → re-emerge); inflect falls back to the extension's paradigm for a lemma the dictionary does not inflect. EN_LEXEXT=0 restores the previous outputs (measured byte-identical on EWT and GUM dev).

### 2026-11-04 · approximant

2026-11-04: known_form(w) = in_dictionary(w) ∨ analyses(w) ≠ ∅. Lemma priority adds a derived candidate after the known one: unsupervised VBN → unsupervise, tokenizers NNS → tokenizer.
