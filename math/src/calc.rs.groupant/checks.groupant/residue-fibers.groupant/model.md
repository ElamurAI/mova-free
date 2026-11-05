# math-calc-checks-residue-fibers

## Scope

Source-derived bounded relation for the manifest byte interval. All relations observe one invocation and its stated state effects. Valid caller structures and finite values where specified are premises. No universal proof, percentage-error surrogate or production replacement is claimed.

## Forward

Integer literals/variables are reduced modulo P61. Negation and add/sub/mul combine residues. Power evaluates exponent exactly in a fresh Env cloned from vars, rejects negative/noninteger exponents, then square-and-multiplies over exponent bits. Factorial uses the operand residue as its argument and rejects residue>2M.

## Invariant

Every Some residue lies in [0,P61). Correct factorial interpretation requires original operand a natural number below P61, with supported residue cap. None means unavailable, not a disproved result.

## Cost

Tree traversal plus big-integer reduction cost. Power adds exponent evaluation and O(bit_length(exponent)) modular multiplications; factorial performs residue-argument multiplications up to2M. Fresh Env clones variable values.

## behavior_complement

Supported subtraction 7-9 excludes residue2; unsupported rational literal or Div excludes Some. Equal residues do not exclude integers differing by P61.

## reverse_reconstruction

Observed r represents claimed integer class {r+k*P61:k integer}; many syntax trees and environments inhabit each class. An observed None admits unsupported syntax, exponent failure, negative exponent or factorial cap.

## collision_class

independent(Num7,result7+P61) passes its modular check. This concrete compatible error shows that a passing certificate is a residue observation, not exact answer equality.

## exponent_environment

Exponent eval uses a fresh default Limits and vars.clone; effects do not propagate to the caller's vars. Availability depends on that separate evaluation and exact integer conversion.

## factorial_argument_loss

The operand is reduced before factorial. Source-derived example original P61+1 maps to1! though true factorial is0 mod P61. No huge factorial is executed; bound the admitted operand domain.

## signed_residue

Negative literals and subtraction normalize through nonnegative residue arithmetic. For -2+5 output3; for7-9 outputP61-2. Oracle compares literal residues with those exact constants.

## Tests

Actual helper fixture: `calc::wave34::fingerprint_congruence_and_unavailable_domains`. See file-root ../../model.md#wave34-evidence for named profile/control receipts. Complement relations above are exclusions, not merely test lists; inverse relations retain compatible antecedents. Unexecuted paths remain source-derived.

## Epoch

Wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.

## History

### 2026-10-31 · Epoch

2026-10-31 wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.
