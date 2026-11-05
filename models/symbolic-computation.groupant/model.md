# symbolic-computation

## descriptant

### symbolic-computation: wave28

calc->comb/nt coerces Q to Z/u64 and appends certificates; poly/linsys->calc may mutate Env for constants; deriv->calc::approx inherits unsupported/numerical domains. comb->nt::sieve supports Legendre, poly->nt::factor/divisors supports candidates/surds. fcheck->expr/units is legacy SI path, not an adapter for dimensionless calc E. math/src/plan.rs is an existing downstream caller; ownership remains with this source list.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave28 report. Registration is not a universal proof or whole-system runtime claim.

Wave39 reviews integer digit/Horner transport and its own typed model sections. Finite checks, dependent formula derivations and algebraic claims retain separate scopes; no production speed or universal source proof is inferred.

## model

# symbolic-computation

## forward

calc parses E and threads Env through exact/approximate evaluation, dispatching comb/nt with labeled Check diagnostics. poly/linsys consume E and evaluate selected-variable-free constants, producing Q polynomials/linear coefficients. deriv transforms E and checks via calc::approx. fcheck projects separate legacy Expr into SI f64; these languages/observations are not interchangeable.

## invariant

Supported exact Q/Z operations retain exact representation; rational holes and gauss dimensions remain explicit obligations. check_solution now validates shape before dot products. Numeric roots/R remain approximate. Check.ok may express exact, finite, probable or unavailable evidence and requires its label.

## cost

Add visited expression work, operand-width arithmetic, recurrence tables, factor search, degree growth and elimination. Env ticks/factor budgets do not uniformly bound all called work. Checker shape scan O(m) time/O(1) storage before O(mn) rational substitution; no timing/speed claim.

## composition

calc->comb/nt coerces Q to Z/u64 and appends certificates; poly/linsys->calc may mutate Env for constants; deriv->calc::approx inherits unsupported/numerical domains. comb->nt::sieve supports Legendre, poly->nt::factor/divisors supports candidates/surds. fcheck->expr/units is legacy SI path, not an adapter for dimensionless calc E. math/src/plan.rs is an existing downstream caller; ownership remains with this source list.

## domain

Finite synthetic fixtures, normalized Poly, matched rectangular gauss inputs, supported expression and canonical digit domains. Resource/overflow assumptions are source-specific.

## limitations

No teacher/API/corpus/GPU/heldout/expensive benchmark execution. Tests inspected first. One root-authorized checker shape repair; otherwise production semantics retained. No new executable percent surrogate.

## checks

- 18 selected actual-code tests twice debug/twice release with identical sorted outcomes
- Seven compiled source mutants rejected, exact restoration and focused positives
- Old shape-check code compiled0 and failed101 before repair
- Seven syntax projections each generated twice identically; scoped fragments reuse parent AST
- Additional linsys width-guard compiled mutant compile0/test101; restored positive0. Eight compiled controls total.
- Seven isolated source snapshot groupant validations exit0; full ancestor integration left to root.

## catalog

## behavior-complement

For admitted fixed inputs/state exclude outputs/effect prefixes inconsistent with exact dispatch relations. Some(f64) does not exclude NaN, probable does not exclude lack of proof, unavailable Check.ok does not exclude arbitrary claimed values, and numerator roots do not exclude canceled holes. Shape-invalid substitution results now cannot be accepted.

## reverse-reconstruction

Join fibers: quotient/remainder with divisor reconstruct dividend; factors/lead reconstruct polynomial after product check; x,A constrain b=Ax. E syntax, Env history, denominator provenance, signed factor/digit inputs and SI representation are many-to-one. Root-only/Check-only results cannot uniquely reconstruct antecedents.

## exact-transport

calc Q feeds exact big Q/Z; poly/linsys reject R constants. fcheck uses legacy Rat/SI scales; f64 magnitude is not a lossless dimensional/calculation adapter.

## certificate-join

Exact substitution/product, modular collision-prone fingerprint, probable labels, finite floating samples and unavailable routes have distinct assurance. Boolean conjunction alone cannot prove the composition.

## state-prefix

poly/linsys constant eval can retain calc effects on failure; ranged errors retain binding. Capped traces omit work; local resource gates do not bound all callees.

## domain-provenance

Retain ratfunc dens across cancellation/solving. Derivative simplify forgets singularities and SI projection forgets units representation. Arithmetic checks must be joined with original domain obligations.

## wave39-digits

# Digit-codec composition boundary — wave39

## magnitude_retraction_composition

Selected child: math-nt, arithmetic fragment. Scope: fixed u32 b>=2, arbitrary finite Z coefficient lists, normal return/no allocation failure. H is exact integer MSB evaluation, D emits canonical magnitude digits. The interaction N=D after H is a retraction onto canonical digit words and N²=N; H after D is abs, not identity on Z. Observation retains canonical digits and magnitude while discarding sign, word length, leading zeros and carry history. Reverse reconstruction admits every coefficient list with |H(d)|=H(c) for canonical c, not just c itself. For k>=1, preimages of +/-H(c) use last=target-b*H(prefix); duplicates collapse at target0. The composed behavior complement excludes noncanonical N outputs and signed-identity claims for H<0. [1,-20] in base10 maps to [1,0]; [] maps to [0]. Source-derived algebra and finite native witnesses support this selected interaction, not the whole symbolic subsystem or a Lean theorem. Child byte/source hashes and parent transitive source bindings remain separate from this semantic scope.

## transport_domain_pullback

The separate algorithm Horner relation lives in the correspondence graph, not as duplicate source ownership here. An integer-domain codec statement may cross to f64 only after coefficient reversal and intersection with base/coefficient/value-intermediate exactness and paired-Result success. Input/final representability alone does not supply these premises. At b=2^53, coefficients [1,1,-2^106] return exact integer2^53 but rounded value0. At b=2, [1,-2,0 repeated1024] has exact value0 and derivative failure. Thus the parent cannot infer a successful floating implementation, a percentage bound or a production optimization from the integer normalization law. All effects outside these pure APIs remain unreviewed by this model.

## History

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
