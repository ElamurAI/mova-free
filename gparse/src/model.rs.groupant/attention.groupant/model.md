# gparse-model-attention

## descriptant

### Source-bound description

Embedding sum and pre-LN transformer dispatch attention softmax by (fused,train), then biaffine masked arcs.

## approximant

### Forward approximant

Embedding sum and pre-LN transformer dispatch attention softmax by (fused,train), then biaffine masked arcs.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Fused training uses p*(g-dot) backward; inference fused path has no Q/K backward. Valid B,T,heads dimensions required.

#### Cost

O(B*T*d+L*(B*T*d²+B*T²*d+B*T*d*f)+B*T*d*a+B*T*a²+B*T²*a); attention probabilities occupy B*H*T², with retained graphs in training. BTd embedding/final-LN work remains at L=0.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

Public train fused cannot invoke upstream inference-only softmax.

## reverse_reconstruction

Categorical outputs cannot recover encoder or logits; encoder itself admits embedding-sum parameter symmetries.

## gradient_dispatch

(true,true)=custom differentiable;(true,false)=upstream fused;(false,_)=reference.

## mask_bias

keys-1 multiplied1e9 on attention/head columns; finite negative bias not hard exclusion for arbitrary huge logits.

## residual_state

Pre-LN attention and FF outputs add to h; final LN precedes head projections.

## dimension_domain

heads nonzero,d divisible heads,position/feature indices valid; no centralized config validation.

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

The input sum gains the subword mean (Net::subword) for models with ngram>0; the attention path itself is unchanged.
