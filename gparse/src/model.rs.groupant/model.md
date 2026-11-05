# gparse-model

## descriptant

### Source-bound description

Cfg loads defaults, valid numeric replacements and strict erf/tanh GELU; Net::new applies GP_GELU then initializes embedding/transformer/head parameters. forward_impl sums word/pre/suf/shape/position/prag embeddings; pre-LN blocks use softmax(QKᵀ/sqrt(dh)+keybias)V and residual feedforward. Arc scores dep_i W head_j+head_j u+keybias+role pull. rels gathers flat word/head states and sums bilinear and concatenated-linear logits; tags gather word states.

## approximant

### Forward approximant

Cfg loads defaults, valid numeric replacements and strict erf/tanh GELU; Net::new applies GP_GELU then initializes embedding/transformer/head parameters. forward_impl sums word/pre/suf/shape/position/prag embeddings; pre-LN blocks use softmax(QKᵀ/sqrt(dh)+keybias)V and residual feedforward. Arc scores dep_i W head_j+head_j u+keybias+role pull. rels gathers flat word/head states and sums bilinear and concatenated-linear logits; tags gather word states.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

LayerNorm uses mean, variance and epsilon 1e-5 through differentiable ops. Training fused route calls custom differentiable softmax; fused inference uses upstream no-backward operation. heads>0, d divisible heads, valid batch dimensions and embedding indices are assumed, not cfg-validated. Padding uses finite -1e9 bias, not mathematical -infinity.

#### Cost

Let B,T,d,L,H,f,a,r,N,X,U,K denote batch, padded length, encoder width, layers, heads, FF width, arc width, relation width, real rows and XPOS/UPOS/relation class counts. Forward encoder costs O(B*T*d + L*(B*T*d*d+B*T*T*d+B*T*d*f)); arc projection/scoring O(B*T*d*a+B*T*a*a+B*T*T*a); relation gather/projections/classifier O(N*d*r+N*r*r*K+N*r*K), tags O(N*d*(X+U)). Memory includes B*H*T*T attention cells, BTd encoder states and retained training graphs plus parameters/optimizer state. Embedding/final-LN BTd remains when L=0; two AFFIX*d tables remain parameter allocations. Dimension products and backend costs are not measured timing claims.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## composition

### Composition

Cfg loads defaults, valid numeric replacements and strict erf/tanh GELU; Net::new applies GP_GELU then initializes embedding/transformer/head parameters. forward_impl sums word/pre/suf/shape/position/prag embeddings; pre-LN blocks use softmax(QKᵀ/sqrt(dh)+keybias)V and residual feedforward. Arc scores dep_i W head_j+head_j u+keybias+role pull. rels gathers flat word/head states and sums bilinear and concatenated-linear logits; tags gather word states.

#### Scoped children

The byte-scoped children project distinct internal relations through the parent syntax file. The parent combines them in actual source order. Remaining declarations/tests outside child ranges belong to this parent. Their correctness does not establish external caller preconditions.

#### Interfaces and limits

LayerNorm uses mean, variance and epsilon 1e-5 through differentiable ops. Training fused route calls custom differentiable softmax; fused inference uses upstream no-backward operation. heads>0, d divisible heads, valid batch dimensions and embedding indices are assumed, not cfg-validated. Padding uses finite -1e9 bias, not mathematical -infinity.

Cfg::save identifies fields but input text is many-to-one due unknown keys, invalid numeric values and repeats. Network outputs admit parameter symmetries and many input feature collisions; invertible reconstruction is not claimed.

## models

# Typed model catalog

## behavior_complement

Training never selects the upstream no-backward softmax via public forward; train=false ignores GP_TRAIN_FUSED_SOFTMAX. Unknown GP_GELU fails before parameter allocation. Invalid numeric cfg strings preserve previous values rather than report error.

## reverse_reconstruction

Cfg::save identifies fields but input text is many-to-one due unknown keys, invalid numeric values and repeats. Network outputs admit parameter symmetries and many input feature collisions; invertible reconstruction is not claimed.

## gradient_transport

Custom layer norm and training softmax retain Q/K/embedding gradients; inference fusion intentionally excludes Q/K gradient while value route survives.

## configuration_precedence

Defaults then ordered lines then GP_GELU override; invalid saved GELU aborts immediately even if a later line is valid.

## tensor_coordinates

QKV reshape B,T,3,heads,d/heads; transpose to B,heads,T,dh. Relations require flattened heads k*T+h.

## finite_arithmetic

Variance epsilon avoids zero denominator for finite constant rows; nonfinite weights, overflow and all-masked behavior have no universal finite guarantee.

## tests

### Checks and limits

model::tests::embeddings_get_gradients; model::fused::which_ops_backprop; activation_configuration_roundtrips; CUDA gradient/GELU/synthetic optimizer checks; CPU prediction

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

See [source execution evidence](model.md#evidence) for exact names and unexecuted paths.

## evidence

### Execution evidence

Source: gparse/src/model.rs

- model::tests::embeddings_get_gradients: twice debug, twice release, all four exits 0.
- model::fused::which_ops_backprop: twice debug, twice release, all four exits 0.
- activation_configuration_roundtrips: twice debug, twice release, all four exits 0.
- cuda_values_and_derivatives: twice debug, twice release, all four exits 0.
- training_attention_keeps_gradients_and_inference_control_breaks_them: twice debug, twice release, all four exits 0.
- synthetic_cpu_prediction_preserves_slots_and_fallback: twice debug, twice release, all four exits 0.
- synthetic_single_optimizer_step: twice debug, twice release, all four exits 0.

#### Controls

1 source-local compiled assertion mutants; 3 observed control tests including dependency mutations. Mutant compile exit0, assertion exit101, byte-exact restoration and positive rerun exit0. Per-source controls are listed in external source-test-map.json, not implied for every source.

#### Limits

Finite synthetic CPU/CUDA observations; no trained-corpus quality or all-input numeric guarantee.

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

Cfg has ngram (0 by default). save appends `ngram=N` only when N>0, so a reference configuration's text is unchanged; load parses it and fails on a malformed value. Net has an optional gram embedding (ngram × d, σ 0.02) built when ngram>0; forward adds the masked mean of the batch's n-gram embeddings (row 0 masked out, divided by max(count,1)) after the position embedding, and fails when a subword model gets a batch without n-grams.

### 2026-11-04 · behavior_complement

Since 2026-11-04: a model with ngram=0 has no gram table and ignores any n-grams in the batch; a subword model never runs silently without them (error); padding bucket 0 receives no gradient (checked).

### 2026-11-04 · tests

Since 2026-11-04: model::subword::subword_input_learns_and_is_required (save/load round trip, unchanged reference text, malformed value error, gradient on the n-gram table but not on row 0, missing n-grams rejected).
