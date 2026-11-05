# coref-lib

## descriptant

### coref-lib

Public Rust module surface joining documents, mention detection, resolution, CorefUD I/O and scoring.

State: Rust module declarations and cfg(test) inclusion; no independent runtime state.

Compilation/representation evidence only, not a separate semantic test or compiled runtime mutant.

## approximant

### Forward model

Exports corefud,doc,eval,it,mention,score,sieve; includes tests only under cfg(test). Successful coref build resolves these declarations to actual sibling implementations.

Domain: acyclic, in-range document views from checked parsing unless explicitly discussing permissive adapters; stable environment during each call; finite addressable allocations. Public vectors can be malformed and panic.

No runtime substitute or numerical error budget is established.

## invariant

### Invariant and cost

Public module names provide paths used by binary and external callers. No validation/effect/ordering is performed by declarations themselves.

No runtime work from declarations; compilation/transitive codegen costs depend on reachable modules and build profile.

Integer counters/lengths use Rust native arithmetic with no separate overflow certificate. Allocation exhaustion, panics and I/O failures are not experimentally characterized.

## composition

### Source-fragment composition

Fragments project exact byte ranges of this source; shared types/imports outside ranges remain prerequisites. Models compose through actual caller/callee state, not an error-budget sum.


Parent relation: Exports corefud,doc,eval,it,mention,score,sieve; includes tests only under cfg(test). Successful coref build resolves these declarations to actual sibling implementations.

Integration: checked document tree -> mention spans/features -> ordered resolver partition -> eval visibility -> CorefUD rows or existing score module. Source-specific exceptions and projection losses remain in model.md#models.

## models

# Typed models

## behavior_complement

The module declarations alone cannot read a corpus, resolve a mention or validate a tree. They do not establish callers' valid-domain assumptions.

## reverse_reconstruction

Given this public module surface, compatible implementations include many sibling bodies with matching export names; compilation does not reconstruct semantic implementations. Cfg(test) inclusion only constrains availability in test builds.

## visibility_surface

Seven pub mod declarations expose source modules; tests module is private and conditional.

## compile_join

main imports coref:: modules; library tests access crate:: siblings. Compiler establishes name/type joins, not semantic agreement.

## cfg_boundary

Production library omits tests declaration; test profile includes hand-authored fixtures.

## semantic_gap

No independent runtime or negative semantic control exists for a reexport/declaration file; evidence is successful compilation plus downstream source-specific tests.

No executable percentage surrogate: categorical state/effect semantics currently lack a useful cheaper observation metric. Supported future caps are 1%, 2%, 5%, 10%; no profile is claimed here.

## tests

### Evidence

Safe synthetic command: env -u COREF_X -u COREF_MD -u PRAG_OFF cargo test -p coref tests:: -- --skip score::. Two debug and two release executions are retained externally with numeric exit status and sorted named outcomes. Selected checks:

- Compilation in repeated cargo test -p coref tests:: -- --skip score::

See source-test-map.json and controls.json in the wave27 coref run receipts. The library declaration file has compilation evidence only. No corpus, teacher, GPU or speed measurement.

Partial scope: Compilation/representation evidence only, not a separate semantic test or compiled runtime mutant.

#### Measured receipts

Final source state: 13 named tests (12 library, 1 binary) passed twice debug and twice release, 52 total executed test instances; all four command exits0 and sorted named outcomes identical. Five added tests are four library boundary tests plus one main load test. The pre-existing score tests and groupant_transfers were filtered out.

Compilation only; no independent semantic control.

Nine source AST projections were generated twice identically. Shared ancestor declarations are deferred to root integration; local owned leaves have no structural diagnostics. No timings or general accuracy claim.

Strict scoped Clippy: exit101 on four pre-existing production lints (corefud unnecessary_sort_by, doc if_same_then_else and clone_on_copy, mention collapsible_if). Those source files are byte-identical to the base. No unrelated lint refactor was made.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: prospective analysis preceded test additions. Production behavior is unchanged. A registry entry records partial source-bound modeling, not verification.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
