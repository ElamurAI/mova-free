# enrichment-cli

## descriptant

### CLI groupant

The [forward model](model.md#approximant) gives all 16 command/arity routes, limit
parsing, argument forwarding, invariants and a structural cost model.
The typed family contains [behavior complement](model.md#nonbehavior),
[reverse reconstruction](model.md#reconstruction) and
[run failure-prefix reconstruction](model.md#failure-prefix).

Scope is the dispatcher. Route selection does not establish callee success,
teacher passivity or transactional behavior. The source hash in manifest.json
binds this description to the reviewed file. A partial source-bound AST projection is stored in syntax.json.

#### Checked observations

The [specialized catalog](model.md#catalog) expands this family to 20 models. Each
record identifies its observation, relation, assumptions and check. Proposed
checks remain distinguishable from evidence already obtained.

## approximant

### CLI dispatch approximant

Subject: main.rs, source identity in manifest.json. Scope: UTF-8 argv after
program name, dispatch selection, limit parsing, and command sequencing.
OS argument decoding failure and callee internals are outside this projection.

Let c be the first argument and n the argument count including c.
The route table is:

| command | n | delegate |
|---|---|---|
| study | 3,4 | study_with_base |
| finish-improve | 2 | finish_improve |
| study-exam | 4 | exam_process |
| prepare-next | 4 | prepare_next |
| improve | 3,4 | improve_with_base |
| prepare | 6 | prepare |
| check-source | 5 | check_source |
| run | 6 | prepare, then review, then stage |
| refine | 3 | refine |
| require-staged | 2 | require_staged |
| restage | 3 | restage |
| profile | 2 | profile, then JSON print |
| review | 2 | review |
| stage | 2 | stage |
| status | 2 | status, then JSON print |
| simulate | 2 | simulate, serialize, write_new |

D(argv) is usage_error if the name/count pair is absent. For prepare/run with
valid arity, parsing the final argument must succeed before delegation.
Otherwise D is the selected route with original path arguments, and an optional
base argument only for study/improve. A route selection does not imply callee
success. run stops after the first failing phase.

Invariants: no vector indexing before the matching arity guard, argument
positions preserved, a successful prefix only in run, one wall-time line after
run returns normally (Ok or Err). Panics are outside that last guarantee.

Cost model: argv collection O(total argument bytes), at most 16 command
comparisons, and numeric parse O(limit string length). Child work dominates
normal training/exam use; T_run=T_prepare+I_prepare_ok*T_review+
I_prepare_and_review_ok*T_stage. This is a structural cost model, not a timing fit.

## catalog

# Specialized obernant catalog

Subject/source hash: manifest.json. Each model is a partial source-derived
relation on its stated observation. Normal execution and pinned dependencies
are shared assumptions. Checks are proposed oracles unless existing evidence
is explicitly cited. Related specializations need not be independent.

## arity-fiber

Type: arity_preimage. Status: partial_source_derived_model.

Domain and relation: For each command c, B(route_c) restricts argument count n to the forward table; B(usage) contains excluded pairs. Payload strings stay unconstrained.

Assumptions and check: Domain: UTF-8 argv; normal dispatch only. Check: existing invalid-count probe; valid branches require delegate fixtures.

## unknown-name

Type: unknown_command_complement. Status: partial_source_derived_model.

Domain and relation: For c outside the recognized-name set, every delegate result is in the absent-behavior set. Case variants are separate names.

Assumptions and check: Domain: command strings and delegate labels. Check: unknown, case-changed and empty commands must return usage.

## limit-fiber

Type: numeric_parse_preimage. Status: partial_source_derived_model.

Domain and relation: B(k)={s | parse::<usize>(s)=Ok(k)} for correctly shaped prepare/run. The error fiber is the failing parse domain.

Assumptions and check: usize width matters; parsing is Rust's grammar. Check malformed, negative, overflow and whitespace strings before any input/output paths are touched.

## optional-base

Type: optional_argument_preimage. Status: partial_source_derived_model.

Domain and relation: For study/improve, B(None) has n=3; B(Some(p)) has n=4 and argv[3]=p. Empty text is Some(empty), not absence.

Assumptions and check: Domain: optional base observation. Check forwarding with a delegate fixture; this does not assert callee acceptance.

## path-loss

Type: path_observation_loss. Status: partial_source_derived_model.

Domain and relation: An observed Path constrains its argument slot but cannot reconstruct file bytes, symlink targets, permissions or later filesystem state.

Assumptions and check: No canonicalization in main.rs. Check same argv with distinct filesystem states; only callee tests can establish outcome changes.

## parse-effects

Type: parse_failure_effect_complement. Status: partial_source_derived_model.

Domain and relation: For prepare/run, n=6 and invalid limit, tuples that invoke prepare/review/stage from the dispatcher are excluded: parse occurs first.

Assumptions and check: Normal execution excludes panics. Existing invalid-limit probe checks no output; a logging delegate would test all call effects.

## stage-preimage

Type: stage_reachability_precondition. Status: partial_source_derived_model.

Domain and relation: Observed stage invocation from run requires valid shape, parsed limit, prepare success and review success.

Assumptions and check: Callee-internal stage calls are outside the domain. Check: inject failure at each predecessor using a delegate fixture; source-derived, not yet executed.

## error-fiber

Type: error_observation_preimage. Status: partial_source_derived_model.

Domain and relation: A nonzero normal exit can arise from usage, numeric parsing or a propagated child error; the exit alone has many antecedents.

Assumptions and check: Signals/panics excluded. Check two safe commands with different error causes but identical nonzero outcomes.

## stdout-fiber

Type: stdout_projection. Status: partial_source_derived_model.

Domain and relation: Wrapper-owned JSON printing occurs for profile/status on success; identical returned Values can leave the originating route ambiguous.

Assumptions and check: Exclude child-owned stdout. Check complete safe fixtures and parse captured JSON; current evidence is source inspection.

## time-fiber

Type: timing_observation_preimage. Status: partial_source_derived_model.

Domain and relation: B(t) contains executions whose rounded elapsed duration prints t; timing does not identify inputs or a unique stage decomposition.

Assumptions and check: Six-decimal Instant elapsed field, normal return. Check several safe invocations, without treating equal durations as equivalent work.

## path-permutation

Type: argument_permutation_counterfactual. Status: partial_source_derived_model.

Domain and relation: Permuting path slots while preserving command/count leaves branch selection equal but permutes the ordered callee arguments.

Assumptions and check: Branch equality is weaker than effect equivalence. Check argument positions with a delegate fixture rather than real teacher operations.

## budget-fiber

Type: resource_budget_preimage. Status: partial_source_derived_model.

Domain and relation: A successful serial run fits budget B only if p+r+s<=B; a prepare-failing prefix reveals no completed suffix costs.

Assumptions and check: Nonnegative additive stage-cost hypothesis; include overhead before calibration. Check symbolic boundary cases, then actual stage measurements.

## write-prestate

Type: write_once_precondition. Status: partial_source_derived_model.

Domain and relation: A successful simulate write requires destination absence at create_new time plus permitted writes; an unchanged existing file excludes success.

Assumptions and check: Filesystem races/partial failures need other models. Existing probe succeeds once, rejects the second call and checks byte preservation.

## json-fiber

Type: serialization_observation. Status: partial_source_derived_model.

Domain and relation: Parsing wrapper JSON constrains the returned Value but not the source artifacts; different whitespace can encode the same Value.

Assumptions and check: Complete attributed stdout assumed. Check JSON parsing on safe fixtures; syntactic validity does not establish artifact integrity.

## diagnostic-trace

Type: normal_return_trace. Status: partial_source_derived_model.

Domain and relation: After run returns, main attempts the elapsed diagnostic before returning its Result. Missing observed text can reflect termination or stderr delivery.

Assumptions and check: Attempted call order differs from delivered IO. Existing success/error probes observe timing; panic/IO-failure cases remain untested.

## program-name

Type: argv0_quotient. Status: partial_source_derived_model.

Domain and relation: With the same executable and remaining arguments, changing argv[0] leaves D unchanged because skip(1) removes it.

Assumptions and check: Hold cwd/environment fixed. Check CommandExt::arg0 variants and compare output after removing elapsed timing.

## environment-fiber

Type: dispatch_environment_projection. Status: partial_source_derived_model.

Domain and relation: Fixed argv selects the same route across environment/filesystem states; full child results can still differ.

Assumptions and check: No non-argv environment lookup in dispatch; process/allocation failures excluded. Check invalid command under two benign environment values.

## tests

### CLI checks

Use the built mova-enrich binary in a fresh temporary output directory.

- Unknown command, empty argv and wrong argument counts must return usage before
  delegation. The bounded probe covers counts 1..8 for each of the 16 command
  names, excluding each route's declared valid counts.
- Correctly shaped prepare/run with a noninteger limit must fail numeric parsing
  before touching the placeholder input/output paths.
- simulate NEW_JSON succeeds once, produces parseable JSON, and refuses an
  existing output. The second attempt must preserve the original bytes.
- The usage output must mention study-exam.
- Normal returning error/success paths print the wall_seconds diagnostic.

These probes cover dispatch and a safe local writing path. They do not execute
teacher modes, learning, GPU work or valid study-exam inference. The retained
study tests and historical real CLI probe cover selected delegated paths.

## failure-prefix

### Failure-prefix obernant: combined run

Type: failure_prefix. Subject: run with valid arguments and parsed limit.
Observed phases are P=prepare, R=review, S=stage. The compatible normal
execution traces are [P_fail], [P_ok,R_fail], [P_ok,R_ok,S_fail] and
[P_ok,R_ok,S_ok]. The prefix constraint follows from the three fallible calls.

An error attributed only to run leaves three possible failed phases; a recorded
completed prefix narrows that set. It does not show which internal side effects
a failing callee already performed, and it does not imply transactional rollback.
Use this model to design resume diagnostics without rerunning a completed
teacher phase blindly. This is a source-derived partial trace model.

## nonbehavior

### Behavior-complement obernant: dispatch

Type: behavior_complement. Scope: name/count/limit-parse projection described
in model.md#approximant. Let inputs be finite UTF-8 argv vectors. Let output labels
be usage_error, limit_parse_error and the route labels from the table.
U is the Cartesian product of those inputs and labels.

R={(a,D(a))}. The complement C={(a,o) in U | o != D(a)} describes the dispatch
results the model does not produce. For an unknown command or wrong arity,
every delegate label belongs to C. For a correctly shaped prepare/run with
invalid limit text, delegation belongs to C because numeric parsing precedes it.
For a successful route selection, usage_error belongs to C.

This is exact for the source-level dispatch projection under normal control
flow. It does not complement all callee effects, errors or process crashes.
The invalid-arity probe supplies runtime samples; unsampled callee behavior is
not classified as impossible.

## reconstruction

### Reverse-reconstruction obernant: dispatch

Type: reverse_reconstruction. B(o)={argv | D(argv)=o}, with D and domain defined
in model.md#approximant. For a route label, B constrains command and allowed arity;
path strings remain unconstrained at this layer. For usage_error, B includes
empty argv, unknown names and wrong counts for known commands. For
limit_parse_error, B requires prepare/run, six arguments and invalid limit text.

A generic nonzero process exit additionally includes callee errors and cannot
identify a unique command or cause. A printed wall-time value does not restore
arguments. Command/argument traces would narrow the fiber; inferred missing
path contents require separate evidence.

This is a relational description, not an implemented reverse CLI.

## History

### 2026-10-29 · descriptant

On 2026-10-29 source/model review found study-exam missing from the usage text
although its four-argument dispatch existed. The usage text now lists it.
The invalid-arity/limit and write-once simulation probes are described in
[tests](model.md#tests). No training-throughput improvement is inferred from this fix.

### 2026-10-29 · tests

Checked 2026-10-29: 112 invalid-dispatch cases, two numeric-parse cases and
two simulate write-once cases passed against the rebuilt Rust executable.

Additional bounded dispatch observations, 2026-10-29: 24 subprocess cases passed.
Twelve combine three rejected command strings (unknown, case-changed and empty)
with two argv[0] values and two benign environment values; exit, stdout and
stderr agree after excluding elapsed time. Twelve combine prepare/run with
empty, negative, leading/trailing-space, nonnumeric and overflowing usize text;
each fails before creating the supplied output. These cover selected observations
for unknown-name, program-name, environment-fiber, limit-fiber and parse-effects.
They do not prove arbitrary environment independence or every callee side effect.
