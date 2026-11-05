# computing-execution

## descriptant

### computing-execution: wave30

Interp::run calls vm::run_program only after registration; VM Fallback invokes Interp::exec and DynEval invokes eval_multi through sync_out/sync_in. lib.rs exposes item identities consumed by tests. suite::run_fs consumes parser scenarios and feeds captured output to check; vm_gate compares engines independently of expected answers, so shared broken builtins can pass equivalence. layers/lib.rs is a separate tagged routing relation to math/global and Domain callbacks, with no mlab caller/callee edge; grouping does not imply runtime transport.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave30 report. Registration is not a universal proof or whole-system runtime claim.

## model

# computing-execution

## forward

For mlab, parse(text)=Err implies no statement/function-registration effects; otherwise Interp registers functions and dispatches tree or VM. VM compile produces prototypes, imports workspace, executes native instructions with tree fallback/dynamic synchronization, then exports workspace even on returned runtime error. suite.parse produces Scenario(code,expect); run_fs executes each in a new interpreter/FS then check(expect,captured_output). vm_gate compares fresh tree/VM outputs, stops first differing scenario. Independent tagged relation layers.route(q,C): normalize explanation phrases for exact global math; otherwise stable finite-score descending domain attempts at score>=threshold under a finite threshold; otherwise Escalate value.

## invariant

The admitted synthetic programs preserve tree/VM expected output, prefix state on runtime error, no prefix state on parse error, matrix alias independence and short-circuit effects. Fresh suite scenarios do not share default workspace/FS. Gate line offsets are1-based and totals count visited prefix. Finite equal routing scores preserve registration order, global math precedes scoring. These are source-derived and finitely checked relations, not universal proofs.

## cost

mlab costs parse+compile+executed instructions plus delegated matrix/builtin operations, register/frame synchronization and COW copies; loops lack fuel limits, function depth is256. Suite parse/check linear in text/expectation/output; gate twice execution per visited scenario. layers adds global calc under five-second Limits, all D coverage callbacks, O(D log D) stable sorting, attempted solvers and logs. Public callbacks, allocation and wide integer casts preclude a general resource guarantee.

## composition

Interp::run calls vm::run_program only after registration; VM Fallback invokes Interp::exec and DynEval invokes eval_multi through sync_out/sync_in. lib.rs exposes item identities consumed by tests. suite::run_fs consumes parser scenarios and feeds captured output to check; vm_gate compares engines independently of expected answers, so shared broken builtins can pass equivalence. layers/lib.rs is a separate tagged routing relation to math/global and Domain callbacks, with no mlab caller/callee edge; grouping does not imply runtime transport.

## domain

CPU-only bounded synthetic programs, in-memory Vfs, valid parser-generated AST/VM prototypes and well-formed Value matrices within integer/memory limits; finite coverage scores and finite threshold for stable threshold claims. Empty programs/scenarios/output, zero-length loops and missing-variable errors explicitly represented.

## limitations

No frozen VMM/Octave/Boost/NIST or mlab suite evaluation, corpus, Monte Carlo defaults, teacher, GPU, real model or host filesystem execution. Nonfinite coverage ordering, arbitrary malformed public bytecode, overflow/allocation exhaustion and unrestricted Domain callbacks remain outside checked claims. f64 arithmetic admits NaN/Inf; no blanket finite-output guarantee.

## checks

- native-commands.json: 12 exact named tests twice debug and twice release; sorted outcomes equal
- negative-controls.json: five compiled assertion mutants, exit101, exact restoration and positive rerun
- Seven lexical source projections generated twice identically; source/fragment hashes bound in manifests
- source-test-map.json separates lib.rs compile/type evidence from actual interpreter, VM, suite, SHA and routing execution

## wave31-virtual-filesystem-state

Actual callers: mlab/src/interp.rs owns Box<dyn vfs::FileSystem>, mlab/src/builtins/files.rs and table.rs call it; open selects backend. Memory path->error->journal joins tested. wasi-host is sibling consumer of Vfs/Errno projections, not owned here. snapbench and test fixtures are independent observation paths, not production callers.

## wave31-wasi-guest-runtime

Real join: Ctx::path_open resolves through Vfs, allocates descriptor; fd_write/read use canonical stored path and Vfs::write_at/read_at, with Vfs errno and journal propagation. Tests observe holes/append/read-only/limits/escape effects. main constructs Ctx from imported Vfs, Host::run returns same context for gated exports. Tagged independent paths: example mlab_gate and tests/mlab compare external mlab::Interp to Host, neither is called by main. Memory-unit tests bypass add_wasi ABI wrappers; tiny guests have no imports, so complete linker wiring is not claimed.

## catalog

## behavior-complement

Parse failure excludes prefix effects; short-circuit false&&/true|| excludes RHS calls; invalid fast indices exclude direct writes; mismatch excludes remaining gate scenarios. Global math success excludes domain callbacks; finite below-threshold score excludes solve. Equivalent engine output does not exclude shared bugs.

## reverse-reconstruction

Preimage of (captured bytes,final workspace) contains all source/initial-state/fault triples yielding those observations; suppressed writes and overwritten locals make it many-to-one. Scenario parsing drops ignored text and trailing blank code, so inverse is set-valued. A route value admits many queries and declining-domain histories; Domain may supply arbitrary Answer.by.

## frame-register-transport

sync_out(regs,names) moves defined values into frame and removes U names; sync_in removes frame values back into Reg. Fallback wraps actual tree execution with this transport, including returned errors. Reg scalar packing is bit-sensitive for logical0/1 and preserves f64 scalar observations.

## parse-runtime-boundary

Entire parse precedes registration/execution. Runtime failure follows already executed assignments and precedes remaining statements. Ordinary indexed '=' assignment orders RHS type, target type, then indices; compound indexed assignment first reads the indexed old value and applies its binary operation, then invokes the assignment helper, evaluating indices again; user calls order excess inputs, excess outputs, depth.

## scenario-gate-join

run_fs maps each Scenario to fresh engine output and compares expected exact/prefix lines; vm_gate compares tree bytes to VM bytes without expectations. check strips exactly one terminal LF; first difference is1-based. Gate total increments before first mismatch break.

## separate-routing-selection

For finite scores and finite threshold, stable descending sort retains equal-score registration order; a0.9 solver returning None permits0.5 first solver at threshold0.5, and raising threshold0.5001 yields escalation. This relation is tagged separately and has no mlab transport edge.

## wave31-virtual-filesystem-state-backend-dispatch

open(mem,limits) installs Vfs, real ignores limits, dir canonicalizes root; shared trait does not imply identical journals or metadata.

## wave31-virtual-filesystem-state-state-trace-split

Snapshot includes root,cwd,clock,next_ino,usage,tmp and excludes limits,journal,fault. Restoration equality applies to dump projection.

## wave31-virtual-filesystem-state-failure-effects

Single-file quota preflight precedes tick; mkdir_all/import/export can preserve a successful prefix. No parent-wide rollback follows.

## wave31-virtual-filesystem-state-caller-join

mlab file/table builtins use Interp.fs trait; sibling WASI translates errno via wasi number while host Display uses Linux number. Do not identify these numeric coordinate systems.

## wave31-wasi-guest-runtime-vfs-coordinate-transport

Relative UTF8 path + directory fd -> canonical Vfs path -> descriptor carrying path/offset/flags -> iovec bytes -> Vfs read_at/write_at -> copied output/count. Explicit positioned writes override append and preserve fd position; implicit append uses file size. Finite direct-helper fixtures exercise this actual caller join.

## wave31-wasi-guest-runtime-failure-order

Validation precedes CLI imports; import failure precedes guest; export gate follows run; first export failure precedes journal. Within helpers clock/seek/read/write/open effects can precede output FAULT, so success-only adapters cannot infer atomicity.

## wave31-wasi-guest-runtime-resource-product

Fuel, memory ceiling, descriptor count and Vfs byte/file limits form separate axes. One axis cannot stand in for others: fuel counts engine work while host IO/copies and imported tree size require independent accounting.

## wave31-wasi-guest-runtime-test-scope

Nine helper/tiny-engine and two integration tests produce11 named observations per profile. Example compile and freshness metadata tests do not imply native/guest frozen-suite equality; mlab remains a tagged external reference.
