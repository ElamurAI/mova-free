# vfs-host

## descriptant

### vfs-host

Confined DirFs and explicit unrestricted RealFs adapters.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

Confined DirFs and explicit unrestricted RealFs adapters.

DirFs.new canonicalizes existing directory. real_path normalizes virtual components, joins root, checks canonical deepest existing ancestor; final symlinks require canonical target inside root. with runs OS operation after check then logs result; RealFs.with directly applies OS operation.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Confined DirFs and explicit unrestricted RealFs adapters. Shared state and order: DirFs.new canonicalizes existing directory. real_path normalizes virtual components, joins root, checks canonical deepest existing ancestor; final symlinks require canonical target inside root. with runs OS operation after check then logs result; RealFs.with directly applies OS operation.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Under stable filesystem and safe temp prefix, DirFs regular access remains rooted; TOCTOU invalidates concurrent adversarial confinement. Metadata reports ctime=mtime,nlink1; host logs bytes0. remove_dir_all rejects exact root.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-host. Domain: Confined DirFs and explicit unrestricted RealFs adapters. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Stable-root confined read/write cannot follow an outside final symlink or dangling outside target. RealFs has no corresponding confinement complement.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-host. Domain: Confined DirFs and explicit unrestricted RealFs adapters. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Canonical target admits symlink/path aliases and unknown prior host contents. Error/zero-byte journal does not recover actual payload. Metadata projection loses native ctime,nlink and directory-entry inode.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## canonical_preflight

Subject: vfs-host. Domain: Confined DirFs and explicit unrestricted RealFs adapters. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Final dangling symlink returns NotCapable; absent ordinary child probes existing ancestors. Missing interior errors can be deferred to OS.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## race_boundary

Subject: vfs-host. Domain: Confined DirFs and explicit unrestricted RealFs adapters. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Check and access are separate syscalls; no openat capability proof. NoConfine mutation bypasses the preflight.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## metadata_projection

Subject: vfs-host. Domain: Confined DirFs and explicit unrestricted RealFs adapters. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: mtime nanos cast to u64, ctime copied from mtime, nlink1; read_dir skips nonUTF8 names, sorts byte names, ino0.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## temp_effects

Subject: vfs-host. Domain: Confined DirFs and explicit unrestricted RealFs adapters. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: DirFs temp_name creates root/tmp ignoring mkdir error and loops counters until !exists; prefix assumed simple basename, no reservation or sanitation.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: dirfs_confines_to_root, gates_green_and_negative_controls_red.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Under stable filesystem and safe temp prefix, DirFs regular access remains rooted; TOCTOU invalidates concurrent adversarial confinement. Metadata reports ctime=mtime,nlink1; host logs bytes0. remove_dir_all rejects exact root.

## Cost

O(path bytes+ancestor canonicalization syscalls) resolution; OS byte I/O costs; read_dir sort O(k log k), memory O(names+data). No measured speed claim.

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: O(path bytes+ancestor canonicalization syscalls) resolution; OS byte I/O costs; read_dir sort O(k log k), memory O(names+data). No measured speed claim.

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.
