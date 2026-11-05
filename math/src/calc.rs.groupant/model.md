# math-calc

## descriptant

### calc

parse normalizes symbols/aliases and builds E with precedence or < and < comparison < add < mul/mod < unary < right-associated power < postfix. Unknown f(x) is multiplication. eval returns V or String while updating Env vars/subs/steps/approx. Exact Q arithmetic switches to R for nonexact powers/transcendentals. And/or/if are lazy. Inclusive ranged forms restore bound variables on normal exit, but errors retain the current binding. Ordinary calls evaluate arguments before arity and flat_args/fact/at can evaluate again. CRT writes crt_modulus. independent prefers integer residue modulo 2^61-1 then finite f64 tolerance 1e-9 max(1,|result|).

#### Wave34 descendant epoch

Five scoped descendants deepen the existing trees; prior models are retained. Synthetic direct-helper fixtures now constrain the new relations, including state/solver and provenance branches formerly source-only. See [current evidence](model.md#wave34-evidence); finite checks do not certify English interpretation or general accuracy.

## approximant

### Forward and cost

parse normalizes symbols/aliases and builds E with precedence or < and < comparison < add < mul/mod < unary < right-associated power < postfix. Unknown f(x) is multiplication. eval returns V or String while updating Env vars/subs/steps/approx. Exact Q arithmetic switches to R for nonexact powers/transcendentals. And/or/if are lazy. Inclusive ranged forms restore bound variables on normal exit, but errors retain the current binding. Ordinary calls evaluate arguments before arity and flat_args/fact/at can evaluate again. CRT writes crt_modulus. independent prefers integer residue modulo 2^61-1 then finite f64 tolerance 1e-9 max(1,|result|).

Fixed-table lexing O(B), token/recursive parse O(B) storage. vars dedup can be O(N²). Evaluation includes reached big arithmetic/callee work, inclusive span and duplicate flat_args work; ticks are not wall-clock bounds. Sort/median O(k log k), unique O(k²). Fingerprint power O(log exponent), factorial O(residue argument) capped2e6. Resource/stack failures unbounded.

No new executable Rust surrogate: exact values, branching, domains and effects require an observation-specific metric before supported 1/2/5/10% profiles. Existing floating diagnostics are not such a profile; no speed claim.

## invariant

### Domain and invariant

Public E/V admit arbitrary f64; real() rejects nonfinite outputs but variable reads and negation do not. Counter additions/i64 MIN abs lack total overflow proof. Literal reads do not tick; deadlines sample only when steps divisible by4096. Mixed comparison tolerance need not be transitive. Empty sum/product=0/1, forall=true, exists=false, list/filter=[]; first/extrema error. Notes cap 200. Pure arithmetic and Env effects, no external I/O.

parse normalizes symbols/aliases and builds E with precedence or < and < comparison < add < mul/mod < unary < right-associated power < postfix. Unknown f(x) is multiplication. eval returns V or String while updating Env vars/subs/steps/approx. Exact Q arithmetic switches to R for nonexact powers/transcendentals. And/or/if are lazy. Inclusive ranged forms restore bound variables on normal exit, but errors retain the current binding. Ordinary calls evaluate arguments before arity and flat_args/fact/at can evaluate again. CRT writes crt_modulus. independent prefers integer residue modulo 2^61-1 then finite f64 tolerance 1e-9 max(1,|result|).

The preimage of (V,Env') includes all E and entry environments reaching that value and ordered effect trace. 1+2 and 3 share an exact result; parsing collapses aliases/whitespace. Fingerprints recover only residue classes. Ranged output and capped traces cannot recover original binding/history, and failure retains prefix effects.

Additional source limits from independent review: range converts span through z_to_u64(...).unwrap_or(0); a positive span>u64 charges0 then loops without per-iteration ticks. Fingerprint factorial reduces its operand first: original operand P61+1 yields residue argument1 and result1, whereas mathematical factorial residue is0. Restrict original factorial operand domain; residue<=2M alone is insufficient. No huge input was executed.

## composition

### Composition

parse normalizes symbols/aliases and builds E with precedence or < and < comparison < add < mul/mod < unary < right-associated power < postfix. Unknown f(x) is multiplication. eval returns V or String while updating Env vars/subs/steps/approx. Exact Q arithmetic switches to R for nonexact powers/transcendentals. And/or/if are lazy. Inclusive ranged forms restore bound variables on normal exit, but errors retain the current binding. Ordinary calls evaluate arguments before arity and flat_args/fact/at can evaluate again. CRT writes crt_modulus. independent prefers integer residue modulo 2^61-1 then finite f64 tolerance 1e-9 max(1,|result|).

calc dispatches to comb/nt. poly/linsys consume calc E and stateful Env for constants; deriv consumes E and calc::approx. fcheck consumes separate legacy expr::Expr/units, with no implicit lossless E adapter. Q/Z joins use big. Parent symbolic-computation models these edges; local checks do not certify application wiring. Nested scopes reuse parent syntax, and remaining helpers/types/tests are parent-modeled.

#### Wave34 refinement

Immediate scoped children now own deeper descendants and wave34-composition.md interaction models. These refine existing scopes without changing production bytes; costs must not double-count overlapping parent/child descriptions. See model.md#wave34-evidence for current finite observations and limits.

## models

# Typed models

## behavior_complement

For fixed admitted input/state, exclude outputs and effect traces contradicting this actual relation: parse normalizes symbols/aliases and builds E with precedence or < and < comparison < add < mul/mod < unary < right-associated power < postfix. Unknown f(x) is multiplication. eval returns V or String while updating Env vars/subs/steps/approx. Exact Q arithmetic switches to R for nonexact powers/transcendentals. And/or/if are lazy. Inclusive ranged forms restore bound variables on normal exit, but errors retain the current binding. Ordinary calls evaluate arguments before arity and flat_args/fact/at can evaluate again. CRT writes crt_modulus. independent prefers integer residue modulo 2^61-1 then finite f64 tolerance 1e-9 max(1,|result|). These exclusions are conditional on the stated domain. Unavailable checks, passing residuals and resource limits do not establish global correctness.

## reverse_reconstruction

The preimage of (V,Env') includes all E and entry environments reaching that value and ordered effect trace. 1+2 and 3 share an exact result; parsing collapses aliases/whitespace. Fingerprints recover only residue classes. Ranged output and capped traces cannot recover original binding/history, and failure retains prefix effects.

## evaluation_order

Bin is left then right; and/or/if skip unselected expressions. Calls evaluate before arity; failure is not transactional.

## binding_effect

ranged restores binding only after loop normal completion/early boolean break; ? exits preserve inserted binding, steps and prior notes.

## numeric_observation

Q comparisons exact; mixed tolerance1e-9 max(|x|,|y|,1). Exact zero times finite R stays Q(0) while Env.approx may stay true. Exact negative-modulus remainder differs from f64 rem_euclid.

## check_scope

Fingerprint collisions are multiples of2^61-1; factorial fingerprints first reduce argument. approx supports a subset and can produce nonfinite values; None means unavailable.


Additional source limits from independent review: range converts span through z_to_u64(...).unwrap_or(0); a positive span>u64 charges0 then loops without per-iteration ticks. Fingerprint factorial reduces its operand first: original operand P61+1 yields residue argument1 and result1, whereas mathematical factorial residue is0. Restrict original factorial operand domain; residue<=2M alone is insufficient. No huge input was executed.

## tests

### Actual-code evidence

Each selected named test passed twice debug/twice release, sorted outcomes identical:

- `test calc::tests::big_and_functions ... ok`
- `test calc::tests::independent_paths ... ok`
- `test calc::tests::parse_print ... ok`
- `test calc::wave28::lazy_empty_and_error_state ... ok`

Finite synthetic fixtures only; no teacher/API/corpus/GPU/heldout. No allocation/stack faults, huge sizes, deadline proof, or universal numerical bound.

Final validation: eight compiled mutants total (seven module boundaries plus linsys width guard), all compile 0 / test 101 and exact restoration / positive 0. Old-code count mismatch witness compile 0 / test 101 separately. Full-registry structural run reports only ancestor integration bindings; isolated source snapshots validate each owned sidecar. Strict Clippy: dependency invocation blocked in existing global build script; --no-deps reaches math and reports 113 existing library findings, retained externally, no clean lint claim. Added test blocks formatted independently; production layout preserved.

#### Wave34 current source

Earlier receipts above are historical. Current source-bound checks and compiled production controls are recorded in [wave34 evidence](model.md#wave34-evidence).

## wave34-evidence

### Wave34 bounded evidence

Ten exact math/lib helper tests across calc and qread pass twice debug and twice release (40 successful named invocations), with identical sorted target-qualified outcomes. This file contributes five selected tests. Actual-source tests use only bounded synthetic expressions/events/provenance; no annotation model, corpus, SVAMP, teacher, full suite or network is invoked.

Each source syntax projection was freshly materialized twice and compared byte-identically before installation; descendants share the file artifact. Partial lexical syntax is not semantic AST or proof. Event construction from language, broader rate queries, consistency/completeness of the solver, huge numeric domains, allocation failures and universal numerical accuracy remain untested. No executable percentage surrogate or timing claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read before edits, bounded actual-code tests and source controls. Linsys shape defect was witnessed on old code and repaired; other semantics retained. External receipt name octopus-wave28-mathcore-2026-10-31 includes prospective.md. Registration is partial, not verification.

### 2026-10-31 · tests

Heading `### Actual-code evidence 2026-10-31` stays in tests as `### Actual-code evidence`.

Compiled source control: control-calc.patch.txt; compiler exit0, test nonzero, exact source restoration and focused positive exit0. Full logs and numeric exits retained externally in octopus-wave28-mathcore-2026-10-31. Linsys also retains shape-old compile 0 / test 101 witness before shape repair. Seven module controls plus the additional linsys width-guard control; fragment declarations are not extra tests.

### 2026-10-31 · wave34-evidence

Heading `### Wave34 bounded evidence — 2026-10-31` stays in wave34-evidence as `### Wave34 bounded evidence`.

Source SHA-256: 81be7f264f451792e6521af7540fb14850fc88a724600813ecb4f74b0df5964e. Five new descendants refine existing scoped parents, with six typed views each (behavior complement first, reverse reconstruction second). The full original source bytes are preserved; only cfg(test) fixtures are appended. Previous evidence remains historical to its recorded source snapshot.

Three compiled production mutants reject by named runtime assertions: residue subtraction, state subtraction and partitioned provenance reuse. Each restored source hash equals its exact backup; the named positive rerun passes. No compile failure or assertion-only edit is counted as a killed behavior mutant. Detailed raw logs, commands, numeric statuses, backup identities and source-test map are retained in external receipt set octopus-wave34-math-2026-10-31.
