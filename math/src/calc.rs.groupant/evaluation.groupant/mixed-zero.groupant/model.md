# math-calc-evaluation-mixed-zero

## Scope

Source-derived bounded relation for the manifest byte interval. All relations observe one invocation and its stated state effects. Valid caller structures and finite values where specified are premises. No universal proof, percentage-error surrogate or production replacement is claimed.

## Forward

Before scalar fallback and tick, finite R times exact Q(0), in either operand order, returns Q(0). Q(0)/R(x) returns Q(0) only for finite nonzero x. All other pairs continue through arith; this fragment is a conditional short-circuit with identity Env effects.

## Invariant

The shortcut never modifies Env.steps or approx. A previously set approx flag is not cleared. Q0/R0 falls through and eventually rejects nonfinite division result.

## Cost

Constant tag/finite/zero checks; cloning or evaluation of operands occurs outside this scope. Successful shortcut avoids tick and mixed arithmetic.

## behavior_complement

An admitted finite mixed multiplication with exact zero excludes R(0) output and a newly incremented step count. Zero divided by approximate zero is excluded from this shortcut's successful transition.

## reverse_reconstruction

Observing Q0 with unchanged Env admits either operand order for multiplication and any finite nonzero divisor for division. Observing only Q0 cannot distinguish these cases or the incoming approx flag.

## signed_zero_divisor

Both +0.0 and -0.0 compare equal to0 and block the division shortcut; neither sign grants an exact quotient.

## nonfinite_operand_exclusion

NaN and infinities fail is_finite and therefore bypass the exact-zero shortcut. If the later tick and conversions succeed, exact zero multiplied by NaN/infinity and exact zero divided by NaN produce nonfinite results rejected by real(). Exact zero divided by positive or negative infinity instead produces finite signed floating zero, returned as R with approx=true after a tick. Shortcut exclusion is not universal rejection of nonfinite operands.

## sticky_approximation

Env.approx may already be true because eval computed an R operand. Returning exact Q0 preserves that historical flag, so value exactness and trace approximation are distinct observations.

## tick_avoidance

Direct arith(Q0,R2.5,Mul) succeeds at unchanged steps. Non-shortcut Q0/R0 ticks before real() fails, so failure effects differ from early exact return.

## Tests

Actual helper fixture: `calc::wave34::syntax_zero_and_binding_boundaries`. See file-root ../../model.md#wave34-evidence for named profile/control receipts. Complement relations above are exclusions, not merely test lists; inverse relations retain compatible antecedents. Unexecuted paths remain source-derived.

## Epoch

Wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.

## History

### 2026-10-31 · Epoch

2026-10-31 wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.
