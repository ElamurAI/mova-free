# hbo-lattice

## descriptant

### Descriptant

Measured epoch: bounded supplied fixtures passed twice in debug and release; six behavioral mutants were rejected after compilation, each restored exactly. These observations support the scoped equations without extending them to corpus accuracy, semantic understanding or persistent learning. Production code bytes are unchanged; only isolated tests appended. No benchmark or percentage surrogate was introduced.

Retained caveats: Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## approximant

### Forward relation

variants emits one base per sentence plus each admissible one-token alternative; cut callback receives contextual candidates. choose joins parsed readings to last base by sentence integer, takes strictly positive maximum gain per token. apply cuts chosen label or base and annotates first-word MISC.

Domain: k>=1 when any nonpunctuation token exists, valid token index and nonempty cut outputs, base present for every alternative. Finite scores/lambda/mu and finite operations for ordered gain meaning; word counts representable as i64 for gains. Integer comments refer to sentence/token positions, not sent_id.

## composition

### Local composition

variants emits one base per sentence plus each admissible one-token alternative; cut callback receives contextual candidates. choose joins parsed readings to last base by sentence integer, takes strictly positive maximum gain per token. apply cuts chosen label or base and annotates first-word MISC.

Nested scopes isolate the transitions described in their manifests and share the whole-source syntax projection. Other declarations and test helpers remain represented by the parent model. Cross-file joins: text units/norm feed seg Piece coordinates; conllu Token/Word carries segmenter and lattice representations; library cut_as realizes lattice callbacks; main routes supplied helpers and external workflows. These joins do not validate arbitrary external parser alignment.

## models

# Typed model catalog

## behavior_complement

With finite gains<=0 no token leaves base; equal positive gains cannot replace first winner. variants never creates a simultaneous two-token alternative; independent final choices are not guaranteed jointly parser-optimal.

## reverse_reconstruction

Choice(label,gain) admits all score/logshare/word-count tuples satisfying g=alt-base+lambda*lr+mu*delta and dominance inequalities. Empty choice conflates absent alternatives and nonpositive/NaN gains; parsed trees cannot be recovered.

## reading_cardinality

Base plus one changed token per eligible alternative gives V=1+sum max(C_i-1,0). Punctuation (including empty-word tokens by all()) has no alternatives and is cloned. k0 causes base-index panic on nonpunctuation.

## gain_ties

gain=score_alt-score_base+lambda*lr+mu*(words_alt-words_base); strict >0 and strict > incumbent retain first equal winner. Missing alternative score substitutes f64::MIN; NaN comparisons are false, not a valid probability policy.

## failure_precedence

First pass rejects any missing lattice comment and base missing gp_score; second pass indexes base and panics if absent. gains instead skips missing joins/scores. Duplicate bases last-wins.

## apply_coordinate

Choice keys index input sentence/token positions. Chosen labels are permissively parsed; apply appends '|Lattice=label:gain' to first cut word and preserves only sent_id/text comments. A choice is not validated against retained candidates.

## tests

### Bounded evidence

Primary actual-code fixture: lattice::wave32::supplied_readings_ties_missing_and_apply. Run using cargo test --offline --locked -p hbo --lib lattice::wave32::supplied_readings_ties_missing_and_apply -- --exact --test-threads=1; repeat with --release. The external wave32 Hbo receipt set contains all 14 named outcomes twice per profile, numeric exit statuses, six compiled behavioral source negatives and byte-exact restoration hashes, and two identical source syntax projections. Source-local primary tests include assertions and caught expected panics. Fragments share these source witnesses; no claim of one test per branch.

Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## invariant-and-cost

### Invariant and cost

k>=1 when any nonpunctuation token exists, valid token index and nonempty cut outputs, base present for every alternative. Finite scores/lambda/mu and finite operations for ordered gain meaning; word counts representable as i64 for gains. Integer comments refer to sentence/token positions, not sent_id.

#### Cost

S sentences,N tokens,N_np nonpunctuation tokens,W words,C_i alternatives,V=1+sum(C_i-1) emitted readings per sentence. variants pays initial contextual-candidate generation plus V*N_np cut callbacks and output/comment copying; with lib::cut_with each callback recomputes Model::candidates before cutting, so callback cost is summed over every emitted reading and token, not paid once. For R parsed readings let T_R=sum of reading token counts and B_R=total comment bytes scanned/parsed (including constant repeated passes). choose costs O(T_R+B_R+R log S+R log N), gains O(T_R+B_R+R log S); n_words scans tokens, twice per retained alternative in gains. apply pays candidate lookups plus one callback per nonpunctuation token and copied output bytes.

Resources must permit allocations; cost variables are source bytes, units, words and table entries as explicitly stated. No measured speed or universal error claim.

## History

### 2026-10-31 · descriptant

2026-10-31, prospective epoch: inspected source before edits and predicted variants emits one base per sentence plus each admissible one-token alternative; cut callback receives contextual candidates. choose joins parsed readings to last base by sentence integer, takes strictly positive maximum gain per token. apply cuts chosen label or base and annotates first-word MISC.

#### Cost clarification 2026-10-31

Independent source review refined symbolic multiplicities: lattice accounting includes token-vector/comment scans and repeated candidate-generating callbacks; raw candidate context construction includes suffix contexts rebuilt per prefix alternative. See model.md#invariant-and-cost for the separate terms. This documentation-only correction changes no Rust bytes, syntax binding or native evidence and makes no new timing claim.

### Undated · approximant

Source snapshot SHA-256: eeec78ace6a0dadfcff75205bc3718b223cf89d05fc8df94ea2884272f033fbf. This is a partial source-derived mathematical relation, not an executable surrogate. Tests exercise actual production helpers with synthetic inputs.

### Undated · document

Subject: hbo-lattice at SHA-256 eeec78ace6a0dadfcff75205bc3718b223cf89d05fc8df94ea2884272f033fbf. Domain and fixed state: k>=1 when any nonpunctuation token exists, valid token index and nonempty cut outputs, base present for every alternative. Finite scores/lambda/mu and finite operations for ordered gain meaning; word counts representable as i64 for gains. Integer comments refer to sentence/token positions, not sent_id.. Observation: return values, stated mutations and failure order of this scope. Models are partial source-derived relations; finite tests can falsify them but do not prove general completeness. Behavior complement is a scoped exclusion claim in the admitted transition universe. Reverse reconstruction retains compatible antecedents, not a unique historical cause. Specialized entries hold this domain fixed and isolate their named observation. Oracle: named actual-code fixtures in model.md#tests; unexecuted effect boundaries remain static hypotheses.
