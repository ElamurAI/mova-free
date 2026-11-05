# gparse-model-configuration

## descriptant

### Source-bound description

GELU chooses erf/tanh; config serialization records fields, parse starts default and processes ordered key=value lines.

## approximant

### Forward approximant

GELU chooses erf/tanh; config serialization records fields, parse starts default and processes ordered key=value lines.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Bad numerics retain current field; unknown GELU errors; duplicate valid fields last win.

#### Cost

O(config bytes), constant field state.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

Unsupported saved GELU cannot be repaired by later line because parsing aborts.

## reverse_reconstruction

Many unknown-key/invalid-number texts map to the same cfg; saved form is canonical but not source-text inverse.

## numeric_failure

parse failure retains current field, so invalid later repeated number keeps earlier successful value.

## text_projection

Unknown keys and lines without '=' ignored; whitespace not trimmed.

## activation_choice

Erf calls gelu_erf, Tanh calls gelu.

## roundtrip_domain

Finite ordinary numeric fields serialize and parse; config does not enforce heads/divisibility/dropout validity.

## tests

### Checks and limits

Selected parent checks relevant to this fragment (not exhaustive): model::tests::embeddings_get_gradients; model::fused::which_ops_backprop; activation_configuration_roundtrips; CUDA gradient/GELU/synthetic optimizer checks; CPU prediction

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.

### 2026-11-04 · descriptant

#### Epoch 2026-11-04

ngram joins the fields: written only when positive, parsed with an error on a malformed value (unlike the other numeric fields, which keep their default).
