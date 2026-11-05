# roundtrip-hebrew-row-codec

## descriptant

### Description and epoch memory

predict chooses given/exact/bare/compose/segment/unknown analyses, emits ranges and numbered words, derives Ref/MId, uses inner head or next-UPOS then generic offset. diff stores fields or canonical keyed patches; apply sorts keys. Learn tallies analyses/offsets before chunk-cut validation, chooses BTree lexical/offset tie winner, derives affix summaries. write/read serializes core lexicon and recomputes affixes.

## approximant

### Forward relation

predict chooses given/exact/bare/compose/segment/unknown analyses, emits ranges and numbered words, derives Ref/MId, uses inner head or next-UPOS then generic offset. diff stores fields or canonical keyed patches; apply sorts keys. Learn tallies analyses/offsets before chunk-cut validation, chooses BTree lexical/offset tie winner, derives affix summaries. write/read serializes core lexicon and recomputes affixes.

Domain: Ten fields per row; unique canonical FEATS/MISC keys for pair identity, '_' empty convention, delimiter-free nonempty analyses for serialization. Finite counts/products/indices, signed offsets avoiding overflow; supplied lexicon only, no global load.

No executable percentage surrogate is supplied: these categorical codec/effect observations have no justified lossy metric. Future caps are 1/2/5/10% only after defining metric, zero behavior and differential reference.

## composition

### Local composition

predict chooses given/exact/bare/compose/segment/unknown analyses, emits ranges and numbered words, derives Ref/MId, uses inner head or next-UPOS then generic offset. diff stores fields or canonical keyed patches; apply sorts keys. Learn tallies analyses/offsets before chunk-cut validation, chooses BTree lexical/offset tie winner, derives affix summaries. write/read serializes core lexicon and recomputes affixes.

Children scope selected internal mechanisms by byte ranges; remaining branches are covered only by the parent partial model. Their costs are included in parent costs, not added again. Shared values and caller order are those named above; sibling fragments are tagged independent where no direct call is named.

## catalog

# Source-bound typed models

Each section is a partial source-derived relation on the bound snapshot and declared domain; finite checks are falsification evidence, not a universal proof. Observation horizon is one call unless history is explicitly named.

## behavior_complement

Given analysis outranks lexicon; unknown token does not acquire a known lemma. External heads clamp but inner heads need not. Noncanonical MISC uses whole field. Lexicon text codec does not preserve arbitrary derived caches or counts.

Domain and held state: Ten fields per row; unique canonical FEATS/MISC keys for pair identity, '_' empty convention, delimiter-free nonempty analyses for serialization. Finite counts/products/indices, signed offsets avoiding overflow; supplied lexicon only, no global load.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## reverse_reconstruction

For fixed pred and admitted target fields, apply(diff(pred,lit)) recovers lit; target alone permits many predictions and patches. bare removes accents so fiber contains accent variants. read(write(lex)) reconstructs core records only under delimiter/nonempty restrictions; derived fields recomputed, training counts lost.

Domain and held state: Ten fields per row; unique canonical FEATS/MISC keys for pair identity, '_' empty convention, delimiter-free nonempty analyses for serialization. Finite counts/products/indices, signed offsets avoiding overflow; supplied lexicon only, no global load.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## analysis_precedence

given token index wins even if empty; exact form then accentless form, two-level prefix composition then frequency affixes.

Domain and held state: Ten fields per row; unique canonical FEATS/MISC keys for pair identity, '_' empty convention, delimiter-free nonempty analyses for serialization. Finite counts/products/indices, signed offsets avoiding overflow; supplied lexicon only, no global load.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## head_transport

inner k translates first-word-offset+k; external id+offset clamped 0..nw; offset0 means root.

Domain and held state: Ten fields per row; unique canonical FEATS/MISC keys for pair identity, '_' empty convention, delimiter-free nonempty analyses for serialization. Finite counts/products/indices, signed offsets avoiding overflow; supplied lexicon only, no global load.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## pair_canonicality

MISC must follow unique MISC_ORDER, FEATS lowercase order; duplicate FEATS and empty-string pairs are explicit identity gaps.

Domain and held state: Ten fields per row; unique canonical FEATS/MISC keys for pair identity, '_' empty convention, delimiter-free nonempty analyses for serialization. Finite counts/products/indices, signed offsets avoiding overflow; supplied lexicon only, no global load.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## partial_learning

Learn.add updates token/offset counts before mismatched chunk returns; this partial progress is not rollback.

Domain and held state: Ten fields per row; unique canonical FEATS/MISC keys for pair identity, '_' empty convention, delimiter-free nonempty analyses for serialization. Finite counts/products/indices, signed offsets avoiding overflow; supplied lexicon only, no global load.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.


## tests

### Actual checks

- `blocks::site_heb::wave32::patch_fields_and_canonical_pair_fibers`
- `blocks::site_heb::tests::affixes_cut_an_unknown_token`
- `blocks::site_heb::tests::a_learned_tree_comes_back_with_its_patches`

The listed parent/source tests exercise only their named mechanisms; other model statements are source inspection. Sixteen selected actual roundtrip library tests passed twice debug and twice release (64 exact-name executions), with identical sorted test outcomes. Eight compiled production-behavior mutants, one per source, failed assertions; exact originals were restored and the selected check passed. No assertion-only mutants or surviving compiled mutants. An initial mutation selector was ambiguous and stopped before writing that source; corrected selector retained separately.

## invariant-and-cost

### Invariants and cost

Given analysis outranks lexicon; unknown token does not acquire a known lemma. External heads clamp but inner heads need not. Noncanonical MISC uses whole field. Lexicon text codec does not preserve arbitrary derived caches or counts.

For fixed pred and admitted target fields, apply(diff(pred,lit)) recovers lit; target alone permits many predictions and patches. bare removes accents so fiber contains accent variants. read(write(lex)) reconstructs core records only under delimiter/nonempty restrictions; derived fields recomputed, training counts lost.

#### Cost

Prediction scans words per token and position per word: O(W²+T*W) plus string/map work; compose bounded depth2. diff/apply O(K log K) keyed pairs; Learn next-UPOS linear searches can be quadratic. Lexicon decoding affix suffix totals O(U*S).

Integer additions/products and allocation sizes must fit native types. Floating costs require finite values; no throughput or all-input guarantee follows from synthetic tests.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained externally before edits. Existing production bytes preserved; only bounded inline tests appended to six owned files. Models distinguish byte reconstruction with residual from semantic understanding and persistent learning. Finite tests and compiled mutants support selected boundaries only. Source inspection identified noninjective projections and explicit admissibility restrictions; no bug repair or performance claim.

Combiner input is this source snapshot, syntax projection, models, costs and retained receipts. Proposed next epoch: target the stated unexecuted branches with bounded witnesses before expanding any claim. AST remains partial, macros opaque; no name resolution or proof.

### 2026-10-31 · tests

Raw logs, numeric statuses, original/restored hashes, prospective.md, native-commands.json and syntax pairs: `$MOVA_DATA/runs/octopus-wave32-rt_blocks-2026-10-31`. No live model/corpus/site/learning workflow or full pipeline was executed.
