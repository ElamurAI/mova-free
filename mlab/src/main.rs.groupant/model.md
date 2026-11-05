# mlab-main

## descriptant

### mlab-main

CLI dispatch for code execution, filesystem flags, suite utilities, VM diagnostics and Monte Carlo gateway.

## approximant

### Forward relation

main selects command; run/-e parse FS options then open/import, instantiate interpreter, execute and finalize exports/journal. Script false yields exit1, configuration/read/finalization error exit2, successful operation exit0. Other branches run explicit suites/bench/mcgate only when selected.

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

Default backend is memory. FS export requires successful interpreter status; finalization error overrides script exit code. Journal follows export loop, hence export error can prevent journal.

#### Failure ordering

Limits parse precedes mem/import compatibility. run reads host source before run_code; test validates imports unsupported before backend validation. Trailing lone options are ignored by windows(2).

#### Cost

O(argv bytes + script/interpreter work + import/export bytes + journal bytes). VMbench performs six runs per source, mcgate defaults 20000; neither is run for this modeling epoch.

## composition

### Composition

main selects command; run/-e parse FS options then open/import, instantiate interpreter, execute and finalize exports/journal. Script false yields exit1, configuration/read/finalization error exit2, successful operation exit0. Other branches run explicit suites/bench/mcgate only when selected.

Real:virtual pair splits at first colon; repeated FS/limits/journal select last, repeated import/export accumulate. External suite and timing output are not deterministic timing guarantees.

#### filesystem-lifecycle

Build FsOpts, open backend and import memory roots, run code, export only if ok, then write journal; return status based on config/finalization versus interpreter.

Boundary: Creation/import precedes interpreter execution; journal is after export attempts and can be absent if export errors.

#### Interaction limit

Local fragment relations compose only with valid Value storage, caller error propagation and delegated providers. They do not establish whole-interpreter or host-filesystem guarantees.

## models

# Typed models — mlab-main

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

Failed script excludes configured export effects, but does not exclude journal write or earlier imports. Nonmem with imports/exports is rejected inside run_code by fs_opts, excluding make_fs/import, interpreter execution and finalization, but not run_code invocation. This is CLI policy, not arbitrary filesystem isolation proof.

## reverse_reconstruction

Exit2 admits usage, source read, options, backend initialization or finalization errors. Exit0 does not uniquely reconstruct arguments or successful code; last FS flags and first fault flag collapse many argv sequences.

## state_invariant

Default backend is memory. FS export requires successful interpreter status; finalization error overrides script exit code. Journal follows export loop, hence export error can prevent journal.

## failure_precedence

Limits parse precedes mem/import compatibility. run reads host source before run_code; test validates imports unsupported before backend validation. Trailing lone options are ignored by windows(2).

## representation_transport

Real:virtual pair splits at first colon; repeated FS/limits/journal select last, repeated import/export accumulate. External suite and timing output are not deterministic timing guarantees.

## resource_cost

O(argv bytes + script/interpreter work + import/export bytes + journal bytes). VMbench performs six runs per source, mcgate defaults 20000; neither is run for this modeling epoch.

## tests

### Evidence and limitations

Scope: Default backend is memory. FS export requires successful interpreter status; finalization error overrides script exit code. Journal follows export loop, hence export error can prevent journal.


AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.

#### Cross-review refinement — 2026-10-31

Incompatible nonmemory transfer flags are rejected inside run_code. That excludes backend/import, interpreter and finalization effects; it does not exclude the caller invoking run_code. Runtime behavior is unchanged.

### 2026-10-31 · tests

Selected named outcomes: cli_import_export_journal, cli_dir_mode_reads_and_writes_inside_root_only. Each ran twice debug and twice release with sorted identical nonempty outcomes and numeric exit 0. Source-specific compiled control exercised "cli_import_export_journal": compile exit 0, assertion-red exit 101, exact-restored rerun exit 0. External receipts: octopus-wave30-io-2026-10-31 (native-commands.json, positive-receipts.json, controls.json).
