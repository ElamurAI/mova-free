# computing-io

## descriptant

### computing-io: wave30

builtins/mod registers files/io; io calls display::format_disp and exp_str; fileio uses table inference/date/name adapters and column-major Mat; main creates Interp with selected fs and selected tests/fs CLI helper observes exit/stdout/stderr/journal/files. sprintf column-major traversal and display row layout are different adapters; JSON orientation/nonfinite loss prevents identity roundtrip. Octave calls lexical filter is not parser validation, and VMM digest helper evidence is not scenario correctness.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave30 report. Registration is not a universal proof or whole-system runtime claim.

## model

# computing-io

## forward

Registry::with_defaults wires files and io builtin pointers into Interp. Files maps validated char paths to Interp.fs effects/status; io maps Value storage and fmt to String then Interp.out. fileio transports CSV/JSON to typed columns and matrices used by table builtins. CLI make_fs imports before Interp.run; finish_fs exports only on ok, then journal. Selected fs tests invoke this CLI on temporary synthetic data; Octave and VMM harnesses contribute only bounded selector/hash-helper evidence.

## invariant

Memory is default CLI/interpreter backend; file builtins do not bypass it. Formatting and serialization themselves perform no filesystem writes. Failed interpreter status suppresses CLI export; successful finalization is required for exit0. Test helpers use bounded synthetic data and frozen baselines stay unchanged.

## cost

Parsing has a linear input baseline; formatting adds repeated segment/flag scans up to O(format length * expanded atoms) and output allocation. JSON serialization copies each subtree at every ancestor, bounded by O(depth * output bytes); a singleton-array chain can require quadratic copying. Table key discovery/lookup can cost O(rows*keys^2). VFS/import/export and interpreter costs are delegated; external subprocess and frozen-suite costs remain unmeasured and unexecuted.

## composition

builtins/mod registers files/io; io calls display::format_disp and exp_str; fileio uses table inference/date/name adapters and column-major Mat; main creates Interp with selected fs and selected tests/fs CLI helper observes exit/stdout/stderr/journal/files. sprintf column-major traversal and display row layout are different adapters; JSON orientation/nonfinite loss prevents identity roundtrip. Octave calls lexical filter is not parser validation, and VMM digest helper evidence is not scenario correctness.

## domain

Valid Value/Mat/Table storage, UTF-8 input, synthetic in-memory or owned temporary filesystem fixtures, bounded output widths and available resources. CPU only.

## limitations

Delegated VFS and interpreter contracts are required; allocation and host fault injection absent. No external Octave corpus, VMM400 evaluation, benchmark or Monte Carlo run.

## checks

- 13 exact named tests twice debug/twice release; identical sorted outcomes
- 9 compiled assertion mutants across 9 sources; byte-exact restore and positive rerun
- 18 fresh syntax projections paired equal; source-scoped nested byte/hash bindings

## catalog

## behavior-complement

Interp failure excludes CLI export_dir but permits journal and previous imports; VFS error with requested status excludes throw for mkdir/rmdir/move/copy. J::Num/numeric-class nonfinite conversion excludes the original numeric literal; logical NaN instead converts to true under the nonzero predicate. No runtime scenario guarantee follows from helper checks.

## reverse-reconstruction

Printed output maps to sets of Value/format antecedents, serialization admits missing/null and shape ambiguities, exit codes admit multiple failure stages. Joint journal and filesystem observations narrow compatible histories but do not reconstruct all interpreter steps.

## value-transport

sprintf flattens m.re column-major, display accesses j*rows+i into row strings, JSON vectors collapse orientation. Matching scalar numbers is insufficient to prove shape-preserving adapters.

## effect-order

parse FS flags -> open/import -> run -> conditional export -> journal. Export failure can suppress journal; failed script skips each export but still attempts journal. Delete loops preserve earlier removals on later error.

## failure-precedence

CLI limit parsing precedes mem/import compatibility; atom validation precedes output; CSV syntax precedes table-width validation. These branches must be preserved across caller wiring.

## resource-composition

Captured strings plus serialized buffers plus VFS memory add rather than cancel; repeated format/flag scans, requested precision/width, JSON subtree copying and key lookup amplify input cost. No measured latency or global cap is supplied.
