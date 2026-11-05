# gparse-model-heads

## descriptant

### Source-bound description

rels gathers flattened rows/heads and evaluates bilinear+linear logits; tags projects gathered words.

## approximant

### Forward approximant

rels gathers flattened rows/heads and evaluates bilinear+linear logits; tags projects gathered words.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

rows and heads refer to same flattened B*T coordinate frame; classes index vocab ordering.

#### Cost

O(N*d*r+N*r²*K+N*r*K+N*d*(X+U)) time for relation projections/bilinear/linear and tag projections, plus O(N*d) gathers; K/X/U class counts independent.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

A local head index without k*T offset does not represent later sentence root.

## reverse_reconstruction

Argmax tags/relations admit cones of logits, not unique hidden state.

## gather_coordinates

flat out.h shape B*T,d; rows and heads index first axis.

## relation_equation

dd*rel_w reshape N,L,r then elementwise hh and sum plus concat-linear.

## tag_projection

XPOS/UPOS linear classifiers share gathered encoder z.

## dropout_effect

Relation projections use train dropout; tags function itself has no train/dropout parameter.

## tests

### Checks and limits

Selected parent checks relevant to this fragment (not exhaustive): model::tests::embeddings_get_gradients; model::fused::which_ops_backprop; activation_configuration_roundtrips; CUDA gradient/GELU/synthetic optimizer checks; CPU prediction

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.
