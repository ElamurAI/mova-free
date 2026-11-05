# gparse-library

## descriptant

### Source-bound description

load reads cfg/vocabs, chooses CPU if GP_CPU exists otherwise cuda_if_available, creates Net then applies available safetensors; missing tensors keep initialization. predict_impl selects length<max_len; long sentences preserve original tuples with score zero. predict_fitting keeps empty slots, validates GP_FUSED_SOFTMAX, plans only nonempty, forwards network and transposes [k,d,h] log scores to Eisner sc[h][d], flattens decoded heads, obtains tags/relations, rewrites root labels and scatters original sentence order.

## approximant

### Forward approximant

load reads cfg/vocabs, chooses CPU if GP_CPU exists otherwise cuda_if_available, creates Net then applies available safetensors; missing tensors keep initialization. predict_impl selects length<max_len; long sentences preserve original tuples with score zero. predict_fitting keeps empty slots, validates GP_FUSED_SOFTMAX, plans only nonempty, forwards network and transposes [k,d,h] log scores to Eisner sc[h][d], flattens decoded heads, obtains tags/relations, rewrites root labels and scatters original sentence order.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Output outer/inner lengths equal input on successful valid models; empty and overlength scores zero. Scored/unscored share decoding. Score sums arc+raw relation log probability+0.5*(xpos+upos), before root/dep label rewriting. Device/backend failures return String except panics from poisoned/invalid index/thread conditions. Net cfg.max_len>0 is required for overlength logging subtraction; invalid zero configurations are outside the admitted domain.

#### Cost

Model loading O(bytes+parameters). Encoder O(B*T*d+L*(B*T*d²+B*T²*d+B*T*d*f)), arc O(B*T*d*a+B*T*a²+B*T²*a), relation O(N*d*r+N*r²*K+N*r*K), tags O(N*d*(X+U)), plus sum n³ decoding. Attention storage B*H*T²; arc host transfer BT²; threads capped min(available,16). Scored adds reductions/copies; parameter/device allocation and no timing claim.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## composition

### Composition

load reads cfg/vocabs, chooses CPU if GP_CPU exists otherwise cuda_if_available, creates Net then applies available safetensors; missing tensors keep initialization. predict_impl selects length<max_len; long sentences preserve original tuples with score zero. predict_fitting keeps empty slots, validates GP_FUSED_SOFTMAX, plans only nonempty, forwards network and transposes [k,d,h] log scores to Eisner sc[h][d], flattens decoded heads, obtains tags/relations, rewrites root labels and scatters original sentence order.

#### Scoped children

The byte-scoped children project distinct internal relations through the parent syntax file. The parent combines them in actual source order. Remaining declarations/tests outside child ranges belong to this parent. Their correctness does not establish external caller preconditions.

#### Interfaces and limits

Output outer/inner lengths equal input on successful valid models; empty and overlength scores zero. Scored/unscored share decoding. Score sums arc+raw relation log probability+0.5*(xpos+upos), before root/dep label rewriting. Device/backend failures return String except panics from poisoned/invalid index/thread conditions. Net cfg.max_len>0 is required for overlength logging subtraction; invalid zero configurations are outside the admitted domain.

Predicted categorical tuples admit all weights/logits with same argmax/Eisner outcome and input features in the same vocabulary/hash classes. A fallback tuple is compatible with any sentence length>=max_len carrying that tuple; scores zero alone do not identify empty versus fallback.

## models

# Typed model catalog

## behavior_complement

Overlength words cannot acquire new tuples through predict; empty sentences never enter tag reduction. Invalid fused-softmax string returns error even when no nonempty batches. Missing tensor is not an error and load_vars does not promise transaction rollback.

## reverse_reconstruction

Predicted categorical tuples admit all weights/logits with same argmax/Eisner outcome and input features in the same vocabulary/hash classes. A fallback tuple is compatible with any sentence length>=max_len carrying that tuple; scores zero alone do not identify empty versus fallback.

## coordinate_transport

Arc tensor dependent-major transposes to sc[h][d]; flattened head index is k*T+h. Swapping these changes dependency direction.

## effect_boundary

Config/vocab I/O precedes device/network creation then weights; load_vars updates each named variable encountered, preserving missing ones.

## failure_precedence

Length filtering precedes fitting; GP_FUSED_SOFTMAX parsing occurs before loop. Out-of-range vocab IDs or tensors are not repaired.

## score_observation

The label displayed as root/dep may differ from raw relation argmax contributing score; score is not normalized tree posterior.

## tests

### Checks and limits

synthetic_cpu_prediction_preserves_slots_and_fallback (actual predict and predict_scored); load/load_vars remain source-modeled, no model-file runtime claim

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

See [source execution evidence](model.md#evidence) for exact names and unexecuted paths.

## evidence

### Execution evidence

Source: gparse/src/lib.rs

- synthetic_cpu_prediction_preserves_slots_and_fallback: twice debug, twice release, all four exits 0.

#### Controls

1 source-local compiled assertion mutants; 1 observed control tests including dependency mutations. Mutant compile exit0, assertion exit101, byte-exact restoration and positive rerun exit0. Per-source controls are listed in external source-test-map.json, not implied for every source.

#### Limits

CPU predict/predict_scored only; load/load_vars I/O/partial-update paths unexecuted.

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

Prediction builds its batches with data::batch_for(…, net.cfg.ngram, …), so a subword model gets its n-grams; for ngram=0 the batch equals the former data::batch. parse_forms fills Word.misc with `_`.
