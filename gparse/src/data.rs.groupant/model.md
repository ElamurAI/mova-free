# gparse-data

## descriptant

### Source-bound description

read scans UTF-8 lines, ignores short (<8 columns), range and decimal-ID rows, retains first 8 fields, parses invalid head as zero and emits only nonempty sentences. write renumbers words and discards original DEPS/MISC/roles. Vocs lowercases word counts with lexical ordering; tags preserve first-seen order. batch_prag uses T=1+max(length), row=k*T+i, key root/real=1, mask real=1; gold heads clamp to sentence length. plan/plan_of sort (length,index), flush only a nonempty batch exceeding tokens. Lex::fill precedence is EModE table, form/tag frequency maximum with lexicographically smaller tie, then tag-frequency features and morphology. inference_cost sums BT, BT² and (n+1)³ for 0<n<max_len.

## approximant

### Forward approximant

read scans UTF-8 lines, ignores short (<8 columns), range and decimal-ID rows, retains first 8 fields, parses invalid head as zero and emits only nonempty sentences. write renumbers words and discards original DEPS/MISC/roles. Vocs lowercases word counts with lexical ordering; tags preserve first-seen order. batch_prag uses T=1+max(length), row=k*T+i, key root/real=1, mask real=1; gold heads clamp to sentence length. plan/plan_of sort (length,index), flush only a nonempty batch exceeding tokens. Lex::fill precedence is EModE table, form/tag frequency maximum with lexicographically smaller tie, then tag-frequency features and morphology. inference_cost sums BT, BT² and (n+1)³ for 0<n<max_len.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Root features [2,1,2,8]; role_arc[(k*T+d)*T+p]=r, first nonzero filler role retained and predicate flag adds 8 at most once for valid 1..=n roles. Empty shape is punctuation 6, empty Voc::load has one empty name; word hashing uses wrapping FNV and buckets 3..AFFIX. Valid roles are required for manually constructed Sent. No structural UD-tree validation.

#### Cost

O(input bytes + vocabulary log V + sort S log S + BT² + word characters) time, O(BT²+retained words/vocab) storage; lexical learning retains distinct form/tag/feature keys. Count/usize products and u32 narrowing are unchecked.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## composition

### Composition

read scans UTF-8 lines, ignores short (<8 columns), range and decimal-ID rows, retains first 8 fields, parses invalid head as zero and emits only nonempty sentences. write renumbers words and discards original DEPS/MISC/roles. Vocs lowercases word counts with lexical ordering; tags preserve first-seen order. batch_prag uses T=1+max(length), row=k*T+i, key root/real=1, mask real=1; gold heads clamp to sentence length. plan/plan_of sort (length,index), flush only a nonempty batch exceeding tokens. Lex::fill precedence is EModE table, form/tag frequency maximum with lexicographically smaller tie, then tag-frequency features and morphology. inference_cost sums BT, BT² and (n+1)³ for 0<n<max_len.

#### Scoped children

The byte-scoped children project distinct internal relations through the parent syntax file. The parent combines them in actual source order. Remaining declarations/tests outside child ranges belong to this parent. Their correctness does not establish external caller preconditions.

#### Interfaces and limits

Root features [2,1,2,8]; role_arc[(k*T+d)*T+p]=r, first nonzero filler role retained and predicate flag adds 8 at most once for valid 1..=n roles. Empty shape is punctuation 6, empty Voc::load has one empty name; word hashing uses wrapping FNV and buckets 3..AFFIX. Valid roles are required for manually constructed Sent. No structural UD-tree validation.

A parsed Sent has all UTF-8 antecedents differing in ignored rows, extra columns and invalid heads mapping to zero; write is a canonical projection, not original-text recovery. A batch has compatible original forms colliding in lowercase vocabulary/affix hashes and compatible dropped word IDs; fixed Vocs and no dropout do not make it injective.

## models

# Typed model catalog

## behavior_complement

No range/decimal row can become a word through read. Invalid numeric head cannot survive as an error: it becomes 0. Budget zero does not exclude nonempty singleton plans; absent sentence IDs do not clear old roles, while matching IDs replace them. Unknown lexical relation becomes its known base or dep unless relation set is empty.

## reverse_reconstruction

A parsed Sent has all UTF-8 antecedents differing in ignored rows, extra columns and invalid heads mapping to zero; write is a canonical projection, not original-text recovery. A batch has compatible original forms colliding in lowercase vocabulary/affix hashes and compatible dropped word IDs; fixed Vocs and no dropout do not make it injective.

## coordinate_transport

Local head h becomes flat k*T+h only at model loss/relation boundary; rows exclude root/pad but keys include root.

## failure_precedence

read checks row width and ID markers before indexing; attach_roles parses row count/role/numbers before bounds filtering; invalid manual roles can panic during batching.

## tie_policy

Word vocab counts iterate BTreeMap sorted lowercase forms; Lex::top chooses smallest equal-count key; role filler preserves first accepted role.

## stochastic_state

Rng zero is absorbing; unit uses top 24 bits divided by 2^24. Drop branches draw only when drop>0, role vectors nonempty, and prag_drop>0; no guarantee for nonfinite probabilities.

## tests

### Checks and limits

data::tests::features_line_up_with_words; data::cost_tests::cost_counts_real_batch_shapes_and_filters_skipped_sentences; data::wave30::projection_roles_and_zero_budget

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

See [source execution evidence](model.md#evidence) for exact names and unexecuted paths.

## evidence

### Execution evidence

Source: gparse/src/data.rs

- data::tests::features_line_up_with_words: twice debug, twice release, all four exits 0.
- data::cost_tests::cost_counts_real_batch_shapes_and_filters_skipped_sentences: twice debug, twice release, all four exits 0.
- data::wave30::projection_roles_and_zero_budget: twice debug, twice release, all four exits 0.
- training_attention_keeps_gradients_and_inference_control_breaks_them: twice debug, twice release, all four exits 0.
- synthetic_cpu_prediction_preserves_slots_and_fallback: twice debug, twice release, all four exits 0.
- synthetic_single_optimizer_step: twice debug, twice release, all four exits 0.

#### Controls

1 source-local compiled assertion mutants; 3 observed control tests including dependency mutations. Mutant compile exit0, assertion exit101, byte-exact restoration and positive rerun exit0. Per-source controls are listed in external source-test-map.json, not implied for every source.

#### Limits

Lex helpers unexecuted in selected checks; invalid manual role indices, large integer products and RNG distributions not exhaustively tested.

CUDA softmax comparisons use absolute tolerance2e-6. GELU grid maxima observed: values0.0004734993, derivatives0.00086838007, finite difference0.0000005534244635896357; respective strict caps5e-4/1e-3/1e-5. These are finite observations, not percentage profiles or throughput claims.

Receipts: external wave30 gparse run native-commands.json, results.json, controls.json and source-test-map.json. Sorted exact test names/outcomes match per profile; numeric diagnostics and timings need not be byte-identical. Strict scoped library Clippy reports ten pre-existing production lints; appended test blocks were formatted independently without production reformatting.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.

### 2026-10-31 · evidence

Heading `### Execution evidence — 2026-10-31` stays in evidence as `### Execution evidence`.

### 2026-11-04 · descriptant

#### Epoch 2026-11-04

Word now keeps MISC (column 10, `_` when absent); write still emits `_` for DEPS and MISC, so annotate output is unchanged. vote parses `Vote`, `VoteHead` (defaults to Vote) and `VoteAlt=h:s,…` from MISC, clamps finite shares to [0,1] and returns None without a finite Vote. ngrams lowercases the word, adds boundary marks and hashes its 3- to 6-grams (shorter n first, at most NGRAM_CAP=32) by salted FNV-1a into 1..buckets. batch_full is the general batch: batch_prag and batch are its ngram=0 wrappers, batch_for its inference form; it adds ngrams (B·T·g, g = max per-word count, at least 1 when on), per-row arc_conf (VoteHead or 1), rel_conf (Vote or 1) and alts (other heads inside the sentence, not the gold head). Prediction of the earlier inputs: no change to inp, rows, gold or the RNG sequence for any ngram value or MISC content (checked by ngrams_and_votes, which compares inp with and without n-grams).

### 2026-11-04 · behavior_complement

Since 2026-11-04: a word without a finite Vote is never marked uncertain (arc_conf = rel_conf = 1); an alternative head outside 0..n or equal to the gold head never reaches alts; ngrams never yields bucket 0 for a real n-gram, and root/padding rows never carry n-grams.

### 2026-11-04 · reverse_reconstruction

Since 2026-11-04: n-gram buckets are a lossy many-to-one image of the lowercased spelling (hash collisions, the 32-gram cap, case folding); vote shares carry two decimals, so the (weight, version) histogram behind them is not recoverable.

### 2026-11-04 · tests

Since 2026-11-04: data::tests::ngrams_and_votes (n-gram count, case folding, shared n-gram bucket, cap, vote parsing, batch confidence rows, n-gram placement, unchanged inp).
