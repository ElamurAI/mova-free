# gparse-library-inference

## descriptant

### Source-bound description

Nonempty plans forward, transpose arcs, decode Eisner, gather heads/tags/relations and scatter into original order.

## approximant

### Forward approximant

Nonempty plans forward, transpose arcs, decode Eisner, gather heads/tags/relations and scatter into original order.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Empty slots remain empty/zero; root labels rewrite after scores are obtained.

#### Cost

Encoder O(B*T*d+L*(B*T*d²+B*T²*d+B*T*d*f)) plus arc projections/scoring, relation/tag projections and sum n³ decoder; attention B*H*T², transfer BT². Parent cost states each independent projection term.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

Empty slots cannot receive a word prediction; root labels override raw relation argmax.

## reverse_reconstruction

Output trees admit score cones and many vocabulary/hash antecedents.

## layout_transport

lp(k,d,h) copied to sc[h][d]; decoded h flattened at k*T+h.

## score_branch

scored=false avoids tag/relation log reductions but shares argmax/decoder.

## label_rewrite

h=0 forces root; h!=0 with predicted root becomes dep.

## environment_gate

GP_FUSED_SOFTMAX only0/1; parse before plan loop even for empty list.

## tests

### Checks and limits

Selected parent checks relevant to this fragment (not exhaustive): synthetic_cpu_prediction_preserves_slots_and_fallback (actual predict and predict_scored); load/load_vars remain source-modeled, no model-file runtime claim

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.

### 2026-11-04 · descriptant

#### Epoch 2026-11-04

Batches come from batch_for with the model's ngram (identical to batch for ngram=0).
