# math-deriv

## descriptant

### deriv

simplify recursively folds Q constants/identities including0*x and0/x, with constant power |k|<=64. derive returns0 first if target absent; otherwise applies arithmetic and supported unary chain rules, appends steps and simplifies. Supported calls sin/cos/tan/exp/ln/log/sqrt/asin/acos/atan. check_derivative samples eight fixed points, central difference h=1e-5 max(|p|,1), skips unsupported/nonfinite samples and accepts >=3 usable points with each normalized discrepancy<=1e-5 against max(|df|,1).

## approximant

### Forward and cost

simplify recursively folds Q constants/identities including0*x and0/x, with constant power |k|<=64. derive returns0 first if target absent; otherwise applies arithmetic and supported unary chain rules, appends steps and simplifies. Supported calls sin/cos/tan/exp/ln/log/sqrt/asin/acos/atan. check_derivative samples eight fixed points, central difference h=1e-5 max(|p|,1), skips unsupported/nonfinite samples and accepts >=3 usable points with each normalized discrepancy<=1e-5 against max(|df|,1).

Repeated has/vars scans and dedup, subtree cloning and simplify can be quadratic or output-size dominated, without AST budget. Numerical check uses up to24 evaluations plus cloned envs: O(8*(cost(f)+cost(df)+env size)), with delegated arithmetic and recursion.

No new executable Rust surrogate: exact values, branching, domains and effects require an observation-specific metric before supported 1/2/5/10% profiles. Existing floating diagnostics are not such a profile; no speed claim.

## invariant

### Domain and invariant

Rules need valid mathematical domains; simplify retains none and0/0 becomes0. i64 MIN abs unchecked at power guard. Steps can append before child failure; errors are not atomic. Expression copying may grow or exhaust stack/resources. Checker clones env and overwrites target; calc::approx subset can leave fewer than3 usable points, which fails. No proof from finite samples.

simplify recursively folds Q constants/identities including0*x and0/x, with constant power |k|<=64. derive returns0 first if target absent; otherwise applies arithmetic and supported unary chain rules, appends steps and simplifies. Supported calls sin/cos/tan/exp/ln/log/sqrt/asin/acos/atan. check_derivative samples eight fixed points, central difference h=1e-5 max(|p|,1), skips unsupported/nonfinite samples and accepts >=3 usable points with each normalized discrepancy<=1e-5 against max(|df|,1).

Derivative g admits all supported source trees whose rule expansion simplifies to g. On connected valid domains antiderivatives already differ by additive constants; syntax simplification expands fibers. Unknown target-free calls also map0. Numeric Check constrains only sampled neighborhoods, not a unique antecedent or global identity; steps are partial rule traces, not reversible serialization.

## composition

### Composition

simplify recursively folds Q constants/identities including0*x and0/x, with constant power |k|<=64. derive returns0 first if target absent; otherwise applies arithmetic and supported unary chain rules, appends steps and simplifies. Supported calls sin/cos/tan/exp/ln/log/sqrt/asin/acos/atan. check_derivative samples eight fixed points, central difference h=1e-5 max(|p|,1), skips unsupported/nonfinite samples and accepts >=3 usable points with each normalized discrepancy<=1e-5 against max(|df|,1).

calc dispatches to comb/nt. poly/linsys consume calc E and stateful Env for constants; deriv consumes E and calc::approx. fcheck consumes separate legacy expr::Expr/units, with no implicit lossless E adapter. Q/Z joins use big. Parent symbolic-computation models these edges; local checks do not certify application wiring. Nested scopes reuse parent syntax, and remaining helpers/types/tests are parent-modeled.

## models

# Typed models

## behavior_complement

For fixed admitted input/state, exclude outputs and effect traces contradicting this actual relation: simplify recursively folds Q constants/identities including0*x and0/x, with constant power |k|<=64. derive returns0 first if target absent; otherwise applies arithmetic and supported unary chain rules, appends steps and simplifies. Supported calls sin/cos/tan/exp/ln/log/sqrt/asin/acos/atan. check_derivative samples eight fixed points, central difference h=1e-5 max(|p|,1), skips unsupported/nonfinite samples and accepts >=3 usable points with each normalized discrepancy<=1e-5 against max(|df|,1). These exclusions are conditional on the stated domain. Unavailable checks, passing residuals and resource limits do not establish global correctness.

## reverse_reconstruction

Derivative g admits all supported source trees whose rule expansion simplifies to g. On connected valid domains antiderivatives already differ by additive constants; syntax simplification expands fibers. Unknown target-free calls also map0. Numeric Check constrains only sampled neighborhoods, not a unique antecedent or global identity; steps are partial rule traces, not reversible serialization.

## domain_erasure

0/x and0*undefined rewrite to0, losing singularity/error effects. Syntax equality does not prove preserved domain.

## rule_order

Target absence precedes unsupported-call rejection; chain derives child first, and rule steps can survive failure.

## sample_metric

Eight fixed points, >=3 usable and normalized error<=1e-5 are finite diagnostics, not supported percent surrogate profiles/global bounds.

## constant_fiber

Every target-free tree maps0, including unsupported calls; reverse recovery needs domain and constant ambiguity.

## tests

### Actual-code evidence

Each selected named test passed twice debug/twice release, sorted outcomes identical:

- `test deriv::tests::derivatives_with_checks ... ok`
- `test deriv::wave28::unsupported_constants_and_domain_erasure ... ok`

Finite synthetic fixtures only; no teacher/API/corpus/GPU/heldout. No allocation/stack faults, huge sizes, deadline proof, or universal numerical bound.

Final validation: eight compiled mutants total (seven module boundaries plus linsys width guard), all compile 0 / test 101 and exact restoration / positive 0. Old-code count mismatch witness compile 0 / test 101 separately. Full-registry structural run reports only ancestor integration bindings; isolated source snapshots validate each owned sidecar. Strict Clippy: dependency invocation blocked in existing global build script; --no-deps reaches math and reports 113 existing library findings, retained externally, no clean lint claim. Added test blocks formatted independently; production layout preserved.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read before edits, bounded actual-code tests and source controls. Linsys shape defect was witnessed on old code and repaired; other semantics retained. External receipt name octopus-wave28-mathcore-2026-10-31 includes prospective.md. Registration is partial, not verification.

### 2026-10-31 · tests

Heading `### Actual-code evidence 2026-10-31` stays in tests as `### Actual-code evidence`.

Compiled source control: control-deriv.patch.txt; compiler exit0, test nonzero, exact source restoration and focused positive exit0. Full logs and numeric exits retained externally in octopus-wave28-mathcore-2026-10-31. Linsys also retains shape-old compile 0 / test 101 witness before shape repair. Seven module controls plus the additional linsys width-guard control; fragment declarations are not extra tests.
