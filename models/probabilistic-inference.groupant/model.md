# probabilistic-inference

## model

# probabilistic-inference

## forward

Tagged weighted observations: alias-table sampling decisions, most-probable sequence decoding, forward/backward posterior mass, Sinkhorn matrix scaling and weighted covariance. Deterministic supplied uniforms and probability normalization are separate from external randomness. HMM w(s)=pi(s0) product_t emission_t(st) product_{t>0}transition(st-1,st); Z=sum_s w(s),V=max_s w(s),0<=V<=Z. Posterior path sums divide by Z when Z>0. Sinkhorn X=diag(u)Kdiag(v),X1=r,X^T1=c. Population covariance=sum_i w_i(x_i-mu)(x_i-mu)^T/sum_i w_i under leaf denominator rules.

## invariant

Nonnegative mass, finite values, support, zero-total handling, sequence-state indexes and normalization/status conventions remain leaf-specific. Maximum-path score is not summed evidence; posterior marginal consistency is not joint-path optimality.

## cost

Alias preparation is linear and lookup constant within leaf bounds. Viterbi/forward-backward cost sequence length times transition work; Sinkhorn cost iterations times matrix cells plus marginal checks; weighted covariance includes sample/feature accumulation. Costs of producing random draws or building upstream models are outside these kernels.

## composition

Alias may sample a normalized posterior row only after row validity and index mapping are checked; a sampled marginal sequence is not a joint HMM sample. Viterbi and forward/backward can share initial/transition/emission data but compare max versus sum semantics, with consistent log/scaled conventions. Sinkhorn outputs can represent couplings only under its support/marginal convergence conditions. Weighted covariance consumes weights, not probabilities necessarily summing to one; normalization must match its population/sample convention.

## limitations

Successful allocations and each leaf's finite domain/error precedence. These are tagged relations, not automatic production adoption. Floating residuals are not solution-error guarantees. No generic 1/2/5/10% composition-wide profile; finite evidence remains separate from analytical claims.

## epoch-memory

Wave26. Tagged library tools; no automatic training adoption, source-volume quality claim or generic numerical profile.

## checks

["cargo test -p mova-algorithms alias_method::tests","cargo test -p mova-algorithms viterbi::tests","cargo test -p mova-algorithms forward_backward::tests","cargo test -p mova-algorithms sinkhorn_scaling::tests","cargo test -p mova-algorithms weighted_covariance::tests"]

## wave26-executed-checks

# probabilistic-inference: wave26 checks

15 source-local test declarations across five modules. Every module has a compiled failing production mutation followed by exact restoration. The final whole library passed 877 checks twice debug and twice release. See [integrated evidence](../algorithm-library.groupant/model.md#wave26-evidence) for scopes and limitations. These tagged APIs are not an invented serial pipeline.

## catalog

# Typed family relations

## behavior-complement

Type: behavior_complement.

Exclude V>Z for identical nonnegative path weights beyond documented rounding tolerance; posterior normalization at Z=0 contrary to status/error rules; invalid alias variates; exhausted Sinkhorn residuals treated as exact marginals; covariance with a substituted denominator.

## reverse-reconstruction

Type: reverse_reconstruction.

Z admits all HMMs with that path-weight sum; Viterbi constrains maximizing path/score but leaves other weights ambiguous. Alias outcomes do not reconstruct random source. Sinkhorn u->a*u,v->v/a preserves X for a>0. Covariance retains moments; many weighted multisets share them.

## max_sum_separation

Type: max_sum_separation.

Viterbi optimizes a path; forward/backward sums paths. Their scores generally differ even when the selected path is valid.

## sampling_mass

Type: sampling_mass.

Alias thresholds and supplied variates preserve their discrete mass model only under specified endpoints; no random-number source quality is certified.

## support_convergence

Type: support_convergence.

Zero support can make desired marginals unattainable. Sinkhorn residual/exhaustion statuses cannot be recast as a successful exact coupling.

## weight_normalization

Type: weight_normalization.

Weighted covariance normalization and effective degrees of freedom must match the consumer; uniform rescaling and zero mass need explicit handling.

## wave26-evidence

### probabilistic-inference: wave26 checks

15 source-local test declarations across five modules. Every module has a compiled failing production mutation followed by exact restoration. The final whole library passed 877 checks twice debug and twice release. See [integrated evidence](../algorithm-library.groupant/model.md#wave26-evidence) for scopes and limitations. These tagged APIs are not an invented serial pipeline.

## History

### 2026-10-30 · epoch-memory

Wave26, 2026-10-30. Tagged library tools; no automatic training adoption, source-volume quality claim or generic numerical profile.
