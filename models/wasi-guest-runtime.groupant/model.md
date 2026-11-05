# wasi-guest-runtime

## descriptant

### wasi-guest-runtime: wave31

Real join: Ctx::path_open resolves through Vfs, allocates descriptor; fd_write/read use canonical stored path and Vfs::write_at/read_at, with Vfs errno and journal propagation. Tests observe holes/append/read-only/limits/escape effects. main constructs Ctx from imported Vfs, Host::run returns same context for gated exports. Tagged independent paths: example mlab_gate and tests/mlab compare external mlab::Interp to Host, neither is called by main. Memory-unit tests bypass add_wasi ABI wrappers; tiny guests have no imports, so complete linker wiring is not claimed.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave31 report. Registration is not a universal proof or whole-system runtime claim.

## model

# wasi-guest-runtime

## forward

CLI parse/read/validate/import → Ctx(Vfs,args,seed,limits) → Host fresh Store/instantiate/_start → Outcome → success-gated export then optional journal/stats. WASI memory/iovec/path helpers transform guest coordinates into actual Vfs calls; errno transports Vfs errors. Independent example and legacy mlab test paths compare external native/guest output but are not invoked by CLI. Bounded tests directly exercise private helpers and tiny engine/CLI paths.

## invariant

Memory accesses require checked ranges, but output FAULT can follow irreversible state transitions within the guest context. Vfs confinement and limits are delegated through actual path/read/write calls. Successful CLI exports require exit0 and no trap. Fresh Store does not imply fresh Vfs unless caller supplies it.

## cost

Module validation and interpreter steps plus host O(iov+bytes), O(descriptor slots), O(directory entries+names), O(subscription count); optional engine fuel is not a bound on all host work. CLI imports/exports/journal charge real IO and bytes. Native mlab and full guest costs unmeasured and excluded.

## composition

Real join: Ctx::path_open resolves through Vfs, allocates descriptor; fd_write/read use canonical stored path and Vfs::write_at/read_at, with Vfs errno and journal propagation. Tests observe holes/append/read-only/limits/escape effects. main constructs Ctx from imported Vfs, Host::run returns same context for gated exports. Tagged independent paths: example mlab_gate and tests/mlab compare external mlab::Interp to Host, neither is called by main. Memory-unit tests bypass add_wasi ABI wrappers; tiny guests have no imports, so complete linker wiring is not claimed.

## domain

CPU; nine memory/engine tests, two synthetic integration tests; memory<=8192 bytes and minimal empty/unreachable wasm; filesystem effects restricted to unique own temporary roots. No full mlab.wasm, frozen suites, large guest build, live data or network.

## limitations

Source-derived branch/ABI relations require valid host resources and finite guest input. is_wasi_name only checks namespace/name, not type. Metadata rights differ from cached file access flags. Example compiles but is not executed; mlab source is external reference only. No useful percentage surrogate established.

## checks

- tests::args_and_prestat
- tests::clock_is_virtual_and_random_is_seeded
- tests::escape_and_absolute_paths_are_not_capable
- tests::limits_reach_the_guest_as_errno
- tests::open_read_seek_write
- tests::readdir_with_small_buffer_and_cookies
- tests::wave31_late_faults_keep_prior_effects
- tests::wave31_memory_poll_and_descriptor_boundaries
- tests::wave31_tiny_engine_outcomes
- wave31_newest_mtime_isolated_tree
- wave31_cli_tiny_guest_isolated_effects
- mlab_gate compile only
- five compiled source mutants with exact byte restoration

## catalog

## behavior-complement

For successful CLI parsing and completed Host run, trap/nonzero outcome excludes export execution through main's gate; memory-gated socket helpers exclude network effects. Late FAULT does not exclude earlier Vfs changes. These are scoped source-derived exclusion relations, not whole-sandbox proofs.

## reverse-reconstruction

Outcome stdout/code/final Vfs constrain compatible guest initial contexts and modules. Capture, truncation, overwritten guest memory and Vfs mutations lose histories; equal output is not unique reconstruction and exit0 merges normal return with proc_exit(0). Journal, argv and original state can narrow but not generally invert the relation.

## vfs-coordinate-transport

Relative UTF8 path + directory fd -> canonical Vfs path -> descriptor carrying path/offset/flags -> iovec bytes -> Vfs read_at/write_at -> copied output/count. Explicit positioned writes override append and preserve fd position; implicit append uses file size. Finite direct-helper fixtures exercise this actual caller join.

## failure-order

Validation precedes CLI imports; import failure precedes guest; export gate follows run; first export failure precedes journal. Within helpers clock/seek/read/write/open effects can precede output FAULT, so success-only adapters cannot infer atomicity.

## resource-product

Fuel, memory ceiling, descriptor count and Vfs byte/file limits form separate axes. One axis cannot stand in for others: fuel counts engine work while host IO/copies and imported tree size require independent accounting.

## test-scope

Nine helper/tiny-engine and two integration tests produce11 named observations per profile. Example compile and freshness metadata tests do not imply native/guest frozen-suite equality; mlab remains a tagged external reference.
