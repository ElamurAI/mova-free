# gparse-library-loading

## descriptant

### Source-bound description

load_vars reads safetensors then sets present variables; load builds config/vocabs/device/net first.

## approximant

### Forward approximant

load_vars reads safetensors then sets present variables; load builds config/vocabs/device/net first.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Missing named tensor preserves random init, shape errors can occur after other sets.

#### Cost

O(file bytes+parameter elements), device memory proportional to model.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

Missing tensor name cannot force zero initialization; it preserves allocated value.

## reverse_reconstruction

Final variables are compatible with files omitting initialized names or including equal values; missing/explicit equality is indistinguishable.

## read_order

cfg precedes words/xpos/upos/rels, then device/Net then safetensors.

## device_choice

Presence of GP_CPU forces CPU, irrespective of its text value.

## partial_effect

var.set may fail after previous variables were changed; no rollback.

## missing_parameter

Unknown extra tensors ignored, absent expected tensors keep initialization.

## tests

### Checks and limits

This exact fragment remains source-derived and unexecuted in this epoch. Parent/sibling selected evidence, not runtime coverage of this fragment: synthetic_cpu_prediction_preserves_slots_and_fallback (actual predict and predict_scored); load/load_vars remain source-modeled, no model-file runtime claim

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.
