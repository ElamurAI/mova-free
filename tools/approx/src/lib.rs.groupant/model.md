# approximation-contract

## descriptant

### Approximation contract groupant

The shared crate gives experimental executable groupants four fail-closed
accuracy tiers. Typed obernants distinguish absent outcomes, compatible inputs,
budget monotonicity and evidence provenance.

The mathematical combiner composes caller domain/metric model and gate:
R_combined(x,c) iff some e satisfies R_caller(x,e) and R_contract(e,c).
The caller must preserve units, normalization, domain and proof assumptions.
Passing this gate does not verify that coupling. The combined behavior
complement uses the composed universe, not a union of leaf complements.
Reverse reconstruction retains every compatible metric-producing input.

## model

# Shared approximation contracts

## Forward

Inputs are one of four upper budgets b in {0.01,0.02,0.05,0.10},
and either a nonempty sequence E of error metrics or a declared analytic
bound d with a nonblank scope. Reject every nonfinite or negative metric.
Observed acceptance is max(E) <= b; analytic acceptance is d <= b.
The certificate preserves budget, maximum/bound and evidence kind.
No lower error bound is imposed. Exact outputs qualify at every tier.

For finite scalar reference r and surrogate s, relative error is
abs((s-r)/r) when r != 0. At r=0 only s=0 yields zero; otherwise return
ZeroReferenceMismatch. Nonfinite inputs and arithmetic results fail.
Arithmetic is f64; overflow fails even when the real ratio would be finite.
Budgets compare computed f64 without hidden tolerance.

## behavior-complement

Universe: tagged public constructor calls and proposed Result values, with
IEEE f64 inputs. Absent-transition relation U minus R includes successful
certificates for invalid/nonfinite metrics, empty samples, blank analytic
scopes, or error above budget; successful budgets outside the four-element
set; and certificates with a tag, sample count or maximum inconsistent with
their inputs. This is a specification complement with partial source
correspondence, not a proof of caller-supplied analytic bounds.

## reverse-reconstruction

For certificate (b,m,ObservedFinite(n)), compatible inputs are all ordered
n-tuples of finite nonnegative metrics with maximum m <= b. Ordering and
individual metrics remain ambiguous. For (b,m,AnalyticBound(scope)), constructor
inputs preserve b,m and exact scope. No proof object is recoverable because
none was supplied. Error variants retain whole failure fibers; rejection
alone cannot identify a failed metric's index.

## budget-lattice

For valid evidence, acceptance at a smaller supported b implies acceptance at
each larger b. Zero error belongs to all four. Changing budget never changes
samples or deliberately degrades accuracy. Invalid evidence fails at all tiers.

## provenance-barrier

Projecting a certificate to numeric error discards its evidence tag; the
preimage includes both finite and analytic evidence. The API never promotes
ObservedFinite to AnalyticBound. The latter is a caller-declared bound under
a named scope, not a verified proof. No sample count entails an unsampled bound.

## Invariant

Successful certificates have a supported budget and finite 0 <= error <= b.
Private fields preserve values through accessors. Samples are nonempty and
analytic scope.trim() is nonempty. Fraction configuration requires an exact
supported f64 tier.

## Cost

Budget parsing/scalar error: O(1). Observed certification: O(n) time and O(1)
scratch, with no retained samples. Analytic scope copying: O(scope bytes)
time/space. No runtime speedup is claimed by this contract crate.

## Checks and limits

The same four tests pass in release. A separately compiled mutation changes
budget rejection from error > maximum to error < maximum; the selected
negative-control test fails as required. The real source remains unchanged.

## Classification axes

MetricKind identifies scalar_relative, normalized_rank, objective_loss or
probability_relative. MethodKind identifies quantized_numeric, certified_sample,
relaxation_certificate or floating_normalization. AssuranceKind identifies
analytical, per_input_certificate or finite_observation. Each axis has a total,
injective stable snake_case rendering; reverse reconstruction of a rendered
label is unique within its named axis. Cross-axis meanings remain independent.
These labels carry no proof and introduce no evidence conversion. In particular
a certified_sample method cannot promote ObservedFinite to AnalyticBound.
Rendering costs O(1), allocates nothing and returns static text.

## Interval-certificate extension

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29: initial implementation. Prediction: linear scan work and
constant scratch memory. Four unit tests support the constructor model;
no timing or runtime gain is claimed. Mean error cannot bound worst-case
error, as retained in the negative control. Finite evidence stays finite.
Caller-declared analytic bounds need separate proof review. A source-bound
syntax projection and semantic AST remain explicit gaps.

### 2026-10-29 · Checks and limits

On 2026-10-29 four debug tests pass: tier boundaries, exact outputs, invalid
configuration/metrics, zero reference, overflow, tags and maximum preservation.
The negative control demonstrates that mean([0,0.02]) passes 1% while the
maximum gate rejects it. Tests are colocated in lib.rs under this binding.
General caller-bound proofs, semantic ASTs, interval arithmetic and exhaustive
IEEE correspondence are absent.

### 2026-10-30 · descriptant

Epoch 2026-10-30: four supported upper caps are 1%, 2%, 5%, 10%.
Metric, method and assurance are independent typed axes with stable labels;
classification does not strengthen evidence. Six isolated library tests pass
in debug and optimized builds. Exact 10% is accepted; the next f64 above it
and 11%, 15%, 25% are rejected. A temporary 25% gate mutation makes the cap
test fail, demonstrating sensitivity to the relaxed-cap regression.

Epoch 2026-10-30, second runner: interval_certificate extends the method axis to five stable labels. Existing four names and evidence provenance remain unchanged. Six library tests pass.

### 2026-10-30 · Classification axes

Epoch 2026-10-30: the accepted cap set is exactly 1%, 2%, 5%, 10%.
Six isolated library tests pass in debug and optimized builds. The new boundary
test accepts exactly 10%, rejects 11%, 15%, 25% and the adjacent f64 above 10%
for every certificate tier, and rejects adjacent-above fraction configurations
for every tier. Classification labels and finite evidence preservation are
checked. A separately compiled mutation relaxes certificate rejection to 25%;
the 10% cap test fails as expected (exit 101). Real source is unchanged by this
negative control. Earlier four-test results remain historical evidence.

### 2026-10-30 · Interval-certificate extension

Epoch 2026-10-30 adds interval_certificate as a fifth method label while preserving all four existing labels and all caps. Its interpretation is a caller-provided bounding interval, not a proof in this library. Forward rendering remains total and injective; behavior complement excludes different labels for this variant, reverse label reconstruction is unique. Assurance/evidence axes remain independent. Six library tests pass with the five-label mapping.
