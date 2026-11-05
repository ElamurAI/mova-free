# mlab-builtins-files-status

## descriptant

### mlab-builtins-files-status

status(ok,msg,n) always constructs one boolean and appends msg iff n>1; mkdir/rmdir callers choose empty result, status or MError by n and trait result.

## approximant

### Forward relation

status(ok,msg,n) always constructs one boolean and appends msg iff n>1; mkdir/rmdir callers choose empty result, status or MError by n and trait result.

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

Successful existing mkdir preserves the directory and signals its existence; recursive rmdir is a branch on exact string s.

#### Failure ordering

Path conversion fails before trait operation; mkdir may join parent/name before stat. Trait failures branch on nargout.

#### Cost

O(path/message bytes + delegated stat/create/remove costs).

## models

# Typed models — mlab-builtins-files-status

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

For n=1 status excludes a message result; existing mkdir at n=0 excludes returned status and emits a warning.

## reverse_reconstruction

status(true,msg,2) admits existing mkdir when msg is Directory already exists., or fresh successful operations when msg is empty; n cannot be recovered beyond the >1 partition.

## state_invariant

Successful existing mkdir preserves the directory and signals its existence; recursive rmdir is a branch on exact string s.

## failure_precedence

Path conversion fails before trait operation; mkdir may join parent/name before stat. Trait failures branch on nargout.

## representation_transport

Boolean status is Value::boolean; requested outputs greater than two do not create extra values.

## resource_cost

O(path/message bytes + delegated stat/create/remove costs).

## tests

### Evidence and limitations

Parent source checks: builtins::files::wave30_tests::status_and_partial_delete. The fragment's actual statements lie in the recorded byte range; parent checks need not exercise every branch in this fragment. See ../model.md#tests for source-level control and limitations.

AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.
