# algorithm-horner

## descriptant

### horner mod.rs

Ascending power coefficients define p=sum c_k x^k and derivative sum k*c_k*x^(k-1). Reverse traversal updates derivative=d*x+old_value and value=value*x+c. Machine results are rounded and both must remain finite.

Original std-only implementation. Literature: see module-level Clenshaw (1955), Horner (1819), Neumaier (1974), or Lyness (1969) citation as applicable; citations are algorithm references, not copied code.

Wave39 reviews integer digit/Horner transport and its own typed model sections. Finite checks, dependent formula derivations and algebraic claims retain separate scopes; no production speed or universal source proof is inferred.

## model

## behavior_complement

Let D be validated inputs and F the source f64 transition relation including tagged errors/status. Behavior complement is all candidate (input, output) pairs not in F: in particular nonfinite successful values, wrong recurrence results and falsely universal error guarantees. Invalid-input observations are errors, not approximate numeric answers. Ascending power coefficients define p=sum c_k x^k and derivative sum k*c_k*x^(k-1). Reverse traversal updates derivative=d*x+old_value and value=value*x+c. Machine results are rounded and both must remain finite.

## reverse_reconstruction

At x=0 value=c0 and derivative=c1; all higher coefficients remain free. [2,3,7] and [2,3,99] have identical outputs. With fixed degree and x, real fiber is intersection of two affine constraints; dependent constraints and f64 rounding require separate treatment.

## dual_recurrence

Derivative uses the previous accumulated value. Substituting the newly updated value breaks derivative semantics. Power oracle checks derivative separately.

## zero_argument_fiber

At zero two leading coefficients suffice; empty polynomial and all-zero coefficients share (0,0). This is a concrete noninjective observation.

## degree_cost

n reverse updates each perform two products and additions, after n finite checks. Derivative computation shares value state; no power table.

## finite_pair_gate

Reject either nonfinite output or intermediate; a finite mathematical result does not guarantee admission after overflowing intermediates. All finite x are allowed.


## Wave19 measured cost calibration

Two bounded release executions calibrate this cost model; see model.md#benchmark-wave19. Integer certificates and floating checks retain their original scope. A smaller asymptotic recurrence does not guarantee faster execution on a small checked input.

## nonfinite_absorption

With finite validated inputs under ordinary IEEE-754, no traps or fast-math, nonfinite value cannot recover in v*x+c. A nonfinite derivative with finite value also cannot recover; if value later becomes nonfinite the final value gate rejects. Final finite pair therefore implies every previous pair finite. This is a scoped argument, not a Lean proof.

## failure_work_relation

One final gate preserves Result variants and successful arithmetic order, but overflow may traverse remaining coefficients. Removing O(n) pair gates does not guarantee speed; input validation remains O(n).

## Wave20 measured cost calibration

The final-pair gate preserves the recurrence and error contract under the nonfinite-absorption premise. Input validation and arithmetic remain linear; failure-path work can increase. See model.md#benchmark-wave20 for actual CPU outcomes; wave19 measurements remain historical.

## approximant

### Forward model and invariant

Ascending coefficients; preserve finite input validation and recurrence order, then reject a nonfinite final value/derivative pair. Empty polynomial returns (0,0). Absorption preserves Result observations while changing rejection work.

Invariant: Each suffix update uses previous value for the derivative. Success returns a finite pair. Invalid input precedes arithmetic; nonfinite intermediates cannot recover.

Cost: O(n) validation, two products and additions per coefficient, one final pair gate; O(1) scratch. Overflow rejection may do more work than the early gate.

IEEE-754 assumptions; no universal real accuracy guarantee. Wave20 is exact in its Result observation, not an approximate surrogate.

## tests

### Evidence

power_oracle independently computes powers and differentiated powers, degrees 0..14 over 17 dyadic points; invalid/overflow cases.

## benchmark-wave19

### Wave19 cost calibration

## benchmark-wave20

The optimization retains O(n) finite input validation and O(n) dual Horner recurrence, reducing pair-output checks to one final gate; overflow paths may perform more work. [Actual CPU report](../../../../models/algorithm-library.groupant/model.md#wave20-benchmark) records measured outcomes without extrapolation. Source SHA-256 (see History); benchmark-summary SHA-256 (see History).

### Wave20 cost calibration

## prospective-wave20

### Horner finite-gate optimization — prospective

Baseline: wave19 validates all input coefficients and x, then checks both accumulators on every recurrence step. Propose preserving input validation and arithmetic order but testing the final pair once. This removes O(n) repeated gate branches on admitted inputs; coefficient validation remains O(n), recurrence O(n), storage O(1). Rejected arithmetic inputs may run more iterations than the early-return baseline.

Model: under ordinary IEEE-754 operations with no traps or unsafe fast-math, nonfinite value cannot become finite through multiply by a finite x and add a finite coefficient: infinity persists or becomes NaN at zero; NaN propagates. If derivative becomes nonfinite while value remains finite, its recurrence also remains nonfinite. If value later becomes nonfinite, the final value gate already rejects. Therefore a final finite pair implies every preceding pair was finite. Both variants return identical tagged Result observations; inputs are validated before arithmetic in both. This does not preserve timing or floating exception timing.

Preserve invalid-input precedence, empty-polynomial behavior, signed-zero bits and exact successful recurrence order. Differential test against the original per-iteration-gate implementation on a bounded full-width alphabet, plus existing independent power oracle and overflowing/invalid cases. Compile an intentional missing-derivative-gate mutation and demand a named failure. Record paired release timing against the old implementation and direct monomials; keep all samples, not only gains. No approximation/error budget is introduced, and no Lean proof is claimed.

## wave20-evidence

### Horner optimization evidence

[Native receipts](../../../../models/algorithm-library.groupant/model.md#wave20-evidence) and [actual CPU report](../../../../models/algorithm-library.groupant/model.md#wave20-benchmark) distinguish the scoped absorption argument, finite observations and measured work. Historical wave19 measurements remain unchanged.

## wave39-digits

# Coefficient transport to integer digits — wave39

## coefficient_order_transport

The algorithm accepts ascending coefficients c and returns Result<(value,derivative)>. Integer `math::nt::from_digits` accepts MSB-first coefficients d. The adapter is c=reverse(d), x=b, and observation projects only the returned value. On the finite catalogue's twelve integer inputs this value equals H_b(d). Independent positional-power sums check 5,467 enumerated small cases (length0..4, coefficients -2..2, base -3..3), including derivative observations. Small integer inputs bound all intermediate products/sums within exact f64 integer range; the catalogue's length<=3, |coefficient|<=3, |base|<=10 inputs likewise satisfy this premise. The general transfer condition requires exact representability of the converted base/coefficients AND each integer value multiplication/addition, together with algorithm success. A sufficient conservative condition is absolute magnitude <=2^53 for all of those integer quantities. This is a source-derived sufficient condition, not a machine-proved theorem over arbitrary floating-point execution.

Counterexample: d=[1,2], b=10 gives H=12, while the unreversed algorithm input returns21. Palindromes or b=1 do not reliably falsify the wrong-order adapter. No full API equivalence follows from value agreement: H has no derivative, no f64 rounding and no Result rejection gate.

## exact_intermediate_exclusion

Observation: projected integer value versus rounded f64 value, conditional on correct coefficient order. Individually exact inputs and exact final mathematical output are insufficient. With b=2^53 and d=[1,1,-2^106], exact H=2^53 while the algorithm returns0: the prefix b+1 rounds before cancellation. With integer b=2^53+1 and d=[1,-2^53], H=1 but conversion aliases the base and the algorithm returns0. These are finite actual-source counterexamples outside the conservative intermediate-bound domain. They are not a relative-error certificate or a source bug within a declared exact domain.

## paired_result_boundary

Even exact bounded VALUE intermediates do not ensure success. At b=2, d=[1,-2,0 repeated1024], every value accumulator is exact and the final value is0; the derivative grows until the algorithm returns NonFiniteArithmetic. Thus the transfer on a broad exact-value domain is conditional on Ok, and derivative failure remains a distinct observable error, not a value mismatch. The finite small table has both finite exact accumulators and admits Ok. A separate [10^308,0,0] at b=1 also reproduces derivative overflow after converting its leading coefficient to finite f64. Failure partitions and value fibers must not be conflated.

## finite_catalogue

Forward records compare only twelve named exact-value observations with `from_digits`; unlisted inputs are unsupported. Reverse reconstruction and behavior-complement records are declared converse and full finite-rectangle complement of the forward atom. Their equality is dependent evidence. Composing value then converse relates different coefficient/base inputs with the same value and is not identity or functional; composing converse then value is identity only on the listed output image. Allocation, input-conversion, control-flow, derivative and error equivalence are not claimed. O(k) paired arithmetic does not imply a faster replacement for the integer fold.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: independent polynomial/basis/integer fixtures and explicit finite-error partitions. Simpson hidden bump demonstrates limits of sampled estimates. An initial correction-denominator mutant survived tight-tolerance tests; a loose-tolerance quartic assertion was added to isolate the correction, and that mutant was then rejected. No speed claim; no percentages inferred from finite observations. Rust syntax projection is partial, without macro expansion, type resolution or semantic proof.

### 2026-10-30 · tests

Compiled production negative control: Replace derivative +value with -value; power_oracle fails. Original bytes restored and named positive test rerun. Full algorithms library passes twice debug and twice release with identical sorted test-name/results; receipts bind logs under $MOVA_DATA/runs/octopus-wave19-numeric-2026-10-30. Source projection generated twice and compared before installation. Strict crate all-target Clippy attempted; outcome in receipt.

### 2026-10-30 · prospective-wave20

Heading `### Horner finite-gate optimization — prospective, 2026-10-30` stays in prospective-wave20 as `### Horner finite-gate optimization — prospective`.

### 2026-10-30 · wave20-evidence

Heading `### Horner optimization evidence — 2026-10-30` stays in wave20-evidence as `### Horner optimization evidence`.

One final finite-pair gate replaces per-coefficient output gates after finite input validation. Under ordinary IEEE-754 arithmetic, nonfinite recurrence states cannot recover to an admitted finite final pair. Successful arithmetic order and Result variants are retained; failure can traverse more coefficients. Root's compiled production mutant rejects; restored code passes four integrated algorithm profiles. Source SHA-256 591283fb09317b37ebe6aadf1d0e6704737909a9aa91fda410abaa827cfd4de3.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

### Undated · benchmark-wave19

This source was measured in two release executions, with three warmups and seven alternating samples. See [complete results and prospective model](../../../../models/algorithm-library.groupant/model.md#wave19-benchmark). Horner is slower than the unchecked direct-monomial baseline; the FPTAS fixed instance is faster and returns the same objective. These facts apply only to their named fixtures. Source SHA-256 0b468f42194662921f4716501003c2d0c41a9395878ed06573f5a54289e0ba63. Evidence receipt SHA-256 1e26c2f14bcfe85fff4a56094ff8b61f13c4b12142e036a7644396a9ed805715.

### Undated · benchmark-wave20

The optimization retains O(n) finite input validation and O(n) dual Horner recurrence, reducing pair-output checks to one final gate; overflow paths may perform more work. [Actual CPU report](../../../../models/algorithm-library.groupant/model.md#wave20-benchmark) records measured outcomes without extrapolation. Source SHA-256 591283fb09317b37ebe6aadf1d0e6704737909a9aa91fda410abaa827cfd4de3; benchmark-summary SHA-256 bc25c8e4c21a6950df57217dbe56c8e2f20e8771f0606574a831c239010ed59b.
