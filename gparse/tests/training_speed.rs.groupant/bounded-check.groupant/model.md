# gparse-test-training-speed-bounded-check

## descriptant

### Source-bound description

step forwards train=true, gathers real arc rows and local-to-flat gold heads, constructs arc/relation/tag losses, differentiates joint loss, optionally validates four gradient norms, updates AdamW, returns pre-update scalar loss.

## approximant

### Forward approximant

step forwards train=true, gathers real arc rows and local-to-flat gold heads, constructs arc/relation/tag losses, differentiates joint loss, optionally validates four gradient norms, updates AdamW, returns pre-update scalar loss.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Flattened gold head is r-r%T+h; loss=La+Lr+0.5*(Lx+Lu). Inspect assertions precede opt.step.

#### Cost

One forward/backward plus O(parameters) optimizer update; storage includes gradients/moments and batch tensors.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

Missing or nonpositive/nonfinite inspected gradient prevents optimizer update; loss return is not post-step reevaluation.

## reverse_reconstruction

Loss admits many logits with equal cross entropy; post-step parameter values do not reconstruct absent AdamW moments.

## head_transport

Subtract row remainder to get batch offset before adding gold head.

## loss_weights

Both tag losses get weight0.5; arc and relation weight1.

## mutation_order

gradients=total.backward then optional inspect then opt.step; norm failure has no update effect.

## observation_scope

Return total.to_scalar after update reads existing pre-update loss tensor; it does not run network again.

## tests

### Checks and limits

Selected parent checks relevant to this fragment (not exhaustive): synthetic_single_optimizer_step

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.
