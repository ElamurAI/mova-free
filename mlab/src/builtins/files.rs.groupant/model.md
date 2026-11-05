# mlab-builtins-files

## descriptant

### mlab-builtins-files

Filesystem builtins dispatch through Interp.fs: file/dir predicates, directory creation/removal, sequential deletion, move/copy and directory listing.

## approximant

### Forward relation

F(op,args,n,fs) validates path strings then performs the selected trait operation. mkdir/rmdir/move/copy return no result on successful n=0, true status for n>0; trait errors throw for n=0 and become false/message for n>0. delete processes paths left-to-right; readdir returns dot entries plus provider order.

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

All operational mutations route through it.fs. Path-string validation precedes the corresponding trait effect; earlier delete effects survive later errors.

#### Failure ordering

mkdir validates parent then optional name; move/copy validates both paths before destination stat. readdir failure throws for n<2, otherwise yields empty string column and -1. isfile/isfolder suppress stat failures to false.

#### Cost

O(argument bytes + VFS operation work); delete adds O(number of paths) sequential stat/remove calls; listing retains O(entries + name bytes). No transaction, allocation or traversal cap is added here.

## composition

### Composition

F(op,args,n,fs) validates path strings then performs the selected trait operation. mkdir/rmdir/move/copy return no result on successful n=0, true status for n>0; trait errors throw for n=0 and become false/message for n>0. delete processes paths left-to-right; readdir returns dot entries plus provider order.

exist_path uses 7 for directories, 2 for any other stat success, 0 for error. readdir prepends . and .. and delegates remaining ordering to provider. Recursive rmdir is enabled only by second string equal to s.

#### status

status(ok,msg,n) always constructs one boolean and appends msg iff n>1; mkdir/rmdir callers choose empty result, status or MError by n and trait result.

Boundary: Successful existing mkdir preserves the directory and signals its existence; recursive rmdir is a branch on exact string s.

#### transfer

Destination directory redirects to child(dst,base_name(src)); fname movefile selects rename, other branch rejects source directory then copies bytes.

Boundary: Both source and destination strings are extracted before destination stat; failure message carries operation and normalized destination.

#### Interaction limit

Local fragment relations compose only with valid Value storage, caller error propagation and delegated providers. They do not establish whole-interpreter or host-filesystem guarantees.

## models

# Typed models — mlab-builtins-files

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

For valid path strings, an FsError from rmdir with n>0 excludes a thrown MError; the same FsError with n=0 excludes false-return success. delete(directory) excludes remove_file(directory), but does not exclude earlier successful deletions.

## reverse_reconstruction

Successful status has all compatible VFS antecedents permitting the requested operation, including already-existing mkdir directories. exist_path=0 includes every stat error, not just absence; status/messages cannot reconstruct original filesystem contents.

## state_invariant

All operational mutations route through it.fs. Path-string validation precedes the corresponding trait effect; earlier delete effects survive later errors.

## failure_precedence

mkdir validates parent then optional name; move/copy validates both paths before destination stat. readdir failure throws for n<2, otherwise yields empty string column and -1. isfile/isfolder suppress stat failures to false.

## representation_transport

exist_path uses 7 for directories, 2 for any other stat success, 0 for error. readdir prepends . and .. and delegates remaining ordering to provider. Recursive rmdir is enabled only by second string equal to s.

## resource_cost

O(argument bytes + VFS operation work); delete adds O(number of paths) sequential stat/remove calls; listing retains O(entries + name bytes). No transaction, allocation or traversal cap is added here.

## tests

### Evidence and limitations

Scope: All operational mutations route through it.fs. Path-string validation precedes the corresponding trait effect; earlier delete effects survive later errors.


AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.

### 2026-10-31 · tests

Selected named outcomes: builtins::files::wave30_tests::status_and_partial_delete. Each ran twice debug and twice release with sorted identical nonempty outcomes and numeric exit 0. Source-specific compiled control exercised "builtins::files::wave30_tests::status_and_partial_delete": compile exit 0, assertion-red exit 101, exact-restored rerun exit 0. External receipts: octopus-wave30-io-2026-10-31 (native-commands.json, positive-receipts.json, controls.json).
