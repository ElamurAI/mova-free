# virtual-filesystem-state

## descriptant

### virtual-filesystem-state: wave31

Actual callers: mlab/src/interp.rs owns Box<dyn vfs::FileSystem>, mlab/src/builtins/files.rs and table.rs call it; open selects backend. Memory path->error->journal joins tested. wasi-host is sibling consumer of Vfs/Errno projections, not owned here. snapbench and test fixtures are independent observation paths, not production callers.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave31 report. Registration is not a universal proof or whole-system runtime claim.

## model

# virtual-filesystem-state

## forward

FileSystem routes tagged mem/dir/real implementations. Vfs edits resolve paths, validate permissions/quotas, COW mutate then journal. Errno projects failures to Linux/WASI. Snapshot restores tree/counters but not journal/policy. Host import/export are explicit partial-effect paths.

## invariant

Bounded Fault=None single-file quota rejection preserves dump; shared snapshots preserve prior tree bytes. Journal total counts retained+dropped before overflow. Host confinement requires stable filesystem and simple temp prefixes.

## cost

Memory resolve O(expanded bytes+sum log fanout), COW O(sum shared-directory fanouts+copied payload); snapshot O(cwd bytes), restore additionally last-reference destruction. Host I/O and sorting separate. No benchmark throughput claim.

## composition

Actual callers: mlab/src/interp.rs owns Box<dyn vfs::FileSystem>, mlab/src/builtins/files.rs and table.rs call it; open selects backend. Memory path->error->journal joins tested. wasi-host is sibling consumer of Vfs/Errno projections, not owned here. snapbench and test fixtures are independent observation paths, not production callers.

## domain

CPU synthetic finite trees, valid UTF8 paths, bounded counters/allocation, dedicated TMPDIR host fixtures. No concurrent host mutations; trusted export roots and safe temporary prefixes.

## limitations

No universal POSIX equivalence, atomic host confinement or transactional mkdir_all/import/export. RealFs operations and snapbench main unexecuted. No useful effect-preserving percentage surrogate.

## checks

- 20 exact named tests twice debug and twice release
- Seven compiled actual-source negative controls with byte restoration
- Two identical syntax projections per source

## catalog

## behavior-complement

Under Fault=None and bounded arithmetic, single-file quota failures do not install edits; virtual absolute paths cannot implicitly read host. Host race and transfer partial effects excluded from this inner source-derived claim.

## reverse-reconstruction

Restored state admits arbitrary intervening histories; journal drops and TSV sanitization erase information. Linux errno1 admits Perm and NotCapable; backend tag required to interpret metadata.

## backend-dispatch

open(mem,limits) installs Vfs, real ignores limits, dir canonicalizes root; shared trait does not imply identical journals or metadata.

## state-trace-split

Snapshot includes root,cwd,clock,next_ino,usage,tmp and excludes limits,journal,fault. Restoration equality applies to dump projection.

## failure-effects

Single-file quota preflight precedes tick; mkdir_all/import/export can preserve a successful prefix. No parent-wide rollback follows.

## caller-join

mlab file/table builtins use Interp.fs trait; sibling WASI translates errno via wasi number while host Display uses Linux number. Do not identify these numeric coordinate systems.
