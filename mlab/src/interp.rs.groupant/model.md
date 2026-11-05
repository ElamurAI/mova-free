# mlab-interp

## descriptant

### mlab-interp

Stateful tree evaluator with workspace frames, captured or console streams, function registry, RNG, in-memory default filesystem and optional register VM dispatch. Parse failure precedes function registration and statements; returned runtime errors retain earlier effects.

#### Limits

Partial lexical syntax, no macro expansion or semantic resolution. Finite synthetic CPU checks are not a proof. Frozen suites, corpus, teacher, GPU, allocation exhaustion and host IO failure injection were not run. No useful executable percent surrogate for categorical control/effects; unsupported gap, no 1/2/5/10% claim.

## approximant

### Forward relation

run(S,text): parse the entire text; on failure append parser diagnostic and return false. Otherwise register functions, then execute sequentially (or delegate VM). Flow Err updates last_error and prints; top-level Return/Break/Continue are successful termination. Expression evaluation is left to right; variables shadow function names; user functions precede builtins.

#### Domain

Valid source-generated AST/values within memory and integer/index limits; deterministic bounded synthetic observations. Arbitrary public malformed representations are outside safety claims.

#### Executable approximant gap

Exact categorical state/dispatch behavior has no supported percentage-error surrogate here.

## invariant

### Invariant

Returned function calls restore frame count, call depth and saved end context; scalar short circuit evaluates RHS only when needed. For ordinary indexed '=' assignment, RHS type precedes target type and index evaluation in assign_index. Compound indexed assignment evaluates the RHS expression, reads the old indexed value (including index evaluation), applies the binary operation, then calls assign_index and evaluates indices again. Parse failure leaves workspace/function registrations unchanged, but emits a diagnostic. Gate warnings bypass ordinary warning suppression.

#### Cost

Parse O(source plus parser work); runtime O(executed AST visits plus delegated arithmetic/index/builtin costs and value clones). For loops snapshot the iterable; matrix column copies total its elements. Calls use frames up to depth 256; while/do-until have no fuel cap. Dimension products/allocation and host IO failures are not generally certified.

## composition

### Parent composition

run-state parses before execution; index-assignment is reached after RHS evaluation; calls receives evaluated arguments and restores local frames. VM dispatch happens after function registration. Fallback can return into these same tree operations; no transactional rollback is added.

#### Limits

Partial lexical syntax, no macro expansion or semantic resolution. Finite synthetic CPU checks are not a proof. Frozen suites, corpus, teacher, GPU, allocation exhaustion and host IO failure injection were not run. No useful executable percent surrogate for categorical control/effects; unsupported gap, no 1/2/5/10% claim.

## models

# Typed models

## behavior_complement

On parse failure, exclude execution of any prefix assignment or function registration. On false && or true || exclude RHS invocation. Ordinary warning suppression excludes ordinary warning bytes but does not exclude gate warnings. Runtime failure does not imply transaction rollback.

## reverse_reconstruction

Preimage of a captured stream is a set of source, workspace, sink and fault states. Empty output includes empty text, suppressed assignments and disabled warnings; no unique program recovery. A final variable alone omits earlier overwrites, local frames and evaluated operands.

## failure_precedence

Ordinary indexed '=' assignment first evaluates RHS, then assign_index rejects nonmatrix RHS, rejects nonmatrix existing target, then evaluates indices. Compound indexed assignment instead reads old indexed value after RHS evaluation, evaluates indices for that read, applies the binary operation, and calls assign_index with a second index evaluation. User calls reject excess inputs before excess outputs before recursion depth; missing inputs are allowed until referenced.

## frame_restoration

Anonymous calls capture sorted free names excluding parameters at creation, enter depth, push frame and clear end context; returned success/error restores saved end context and pops frame. x=2; f=@(a) a+x; x=9 retains captured x=2.

## coordinate_transport

end is rows*cols for one index, rows for first of multiple, cols for second, 1 for later dimensions. Numeric/mask interpretation delegates index.rs; column-major storage is not a zero-based language index.

## effect_boundary

Default Vfs is in-memory; with_fs can install real effects. Capture joins diagnostic/output streams, Console routes separately and ignores write errors. take_output drains only Capture. Loops are not sandbox resource limits.


## tests

### Executed checks

Selected exact names: wave30_parse_runtime_and_output_state, wave30_bounded_state_and_fallback, parse_error_runs_nothing. All selected native commands ran twice debug and twice release with matching sorted named outcomes (12 distinct tests, 48 positive executions across subsystem). Five compiled assertion mutants fail with exit101; restored byte-exactly and rerun green. Per-source associations appear in external source-test-map.json.

Controls change expected parse state, VM persisted value, suite factory count, SHA known digest and routing tie winner. These are compiled assertion controls exercising real code, not claims of implementation-mutant completeness. lib.rs is compile/type identity evidence only.

No frozen suites, Monte Carlo or recursion-heavy EDGE runs. No speed claim. Scoped Clippy limits retained externally; lexical syntax generated twice identically.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective source model preceded test additions. No production semantics changed. Seven legacy files are modeled, with source-scoped fragments for complex files. Retain parse-before-effects versus runtime partial effects, output equality versus correctness, and independent layers routing as distinct observations.

### 2026-10-31 · tests

Heading `### Executed checks — 2026-10-31` stays in tests as `### Executed checks`.
