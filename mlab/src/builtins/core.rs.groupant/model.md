# mlab-builtins-core

## descriptant

### Core creation, shape and interpreter controls

register binds matrix construction, shape/type/string, RNG, callbacks and control functions. reshape gathers dimensions and computes the unchecked known-dimension product before hole counting; after bounded nonoverflowing arithmetic it allows one empty inferred dimension, checks known-product divisibility then 2D/trailing singleton dimensions then numel equality, and changes only rows/cols. linspace floors count, returns 1x0 for !(n>=1), hi for n=1 and forces final hi otherwise. magic selects odd/doubly/singly even constructions with n=2 exceptional matrix. warning on/off changes interpreter flag before formatting paths.

## approximant

### Core creation, shape and interpreter controls forward relation

register binds matrix construction, shape/type/string, RNG, callbacks and control functions. reshape gathers dimensions and computes the unchecked known-dimension product before hole counting; after bounded nonoverflowing arithmetic it allows one empty inferred dimension, checks known-product divisibility then 2D/trailing singleton dimensions then numel equality, and changes only rows/cols. linspace floors count, returns 1x0 for !(n>=1), hi for n=1 and forces final hi otherwise. magic selects odd/doubly/singly even constructions with n=2 exceptional matrix. warning on/off changes interpreter flag before formatting paths.

Domain: Safe checks use tiny finite dimensions, capture interpreter and deterministic helpers. Unchecked usize products, casts, empty randi limit indexing and huge allocation remain gaps. No external file, RNG shuffle, wall-clock, arbitrary callback or source-evaluation paths executed.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

Creation and transforms use column-major Mat. reshape preserves re/im/class; magic n=2 is intentionally not claimed to have all magic sums. size adds trailing singleton dimensions; arrayfun demands equal shapes and scalar outputs, with final output class taken from final callback result. Randomness/state/time are distinct effects.

Cost: Creators O(output area), reshape O(input area clone+dimension count), magic O(n²), repmat O(repeated area), strfind O(text*pattern); arrayfun O(cells*callback cost) with retained outputs. No bounded whole-file callback/runtime guarantee.

Failure ordering: reshape first computes the unchecked known-dimension product; after successful bounded arithmetic, holes>1 precedes divisibility, then dimensionality, then product mismatch. Product overflow can precede these Result errors. arrayfun rejects UniformOutput=false before shape iteration; callback errors may occur after earlier effects. warning toggles on/off before format_message; format_message selects optional identifier before sprintf.

## composition

### Parent composition

core uses ops concatenation/range/truthy, value representation, parser+Interp for str2num/str2func, files::exist_path, rng and io::sprintf_impl. Therefore pure constructor checks do not verify full registry or effects. Tests explicitly cover reshape/linspace/magic/warning/parse_double only.

Scoped fragments are projections over exact byte intervals in this source; they share one parent syntax projection. Uncovered declarations and embedded checks remain parent modeled. reshape first computes the unchecked known-dimension product; after successful bounded arithmetic, holes>1 precedes divisibility, then dimensionality, then product mismatch. Product overflow can precede these Result errors. arrayfun rejects UniformOutput=false before shape iteration; callback errors may occur after earlier effects. warning toggles on/off before format_message; format_message selects optional identifier before sprintf.

The relation is source-derived and finite checks cannot establish complete correctness or a composition-wide numerical error bound.

## models

## behavior_complement

Bounded reshape excludes buffer reordering or size change on success and rejects two unknown dimensions. linspace n=1 excludes lo when lo!=hi. Core does not exclude side effects: RNG mutates, exist can inspect files, callbacks execute user functions and tic/toc use wall time. assert checks real parts only; imaginary equality is not guaranteed.

## reverse_reconstruction

reshape result and original shape reconstruct original matrix buffers/class/im if dimensions multiply to same n; dimensions arguments are nonunique due trailing ones and inferred hole. linspace output has multiple count spellings after floor, empty fibers include NaN/nonpositive. warning final flag does not reveal prior flag or all prior commands.

## representation_transport

Creation and transforms use column-major Mat. reshape preserves re/im/class; magic n=2 is intentionally not claimed to have all magic sums. size adds trailing singleton dimensions; arrayfun demands equal shapes and scalar outputs, with final output class taken from final callback result. Randomness/state/time are distinct effects.

## failure_precedence

reshape first computes the unchecked known-dimension product; after successful bounded arithmetic, holes>1 precedes divisibility, then dimensionality, then product mismatch. Product overflow can precede these Result errors. arrayfun rejects UniformOutput=false before shape iteration; callback errors may occur after earlier effects. warning toggles on/off before format_message; format_message selects optional identifier before sprintf.

## resource_domain

Safe checks use tiny finite dimensions, capture interpreter and deterministic helpers. Unchecked usize products, casts, empty randi limit indexing and huge allocation remain gaps. No external file, RNG shuffle, wall-clock, arbitrary callback or source-evaluation paths executed.

## interaction_relation

core uses ops concatenation/range/truthy, value representation, parser+Interp for str2num/str2func, files::exist_path, rng and io::sprintf_impl. Therefore pure constructor checks do not verify full registry or effects. Tests explicitly cover reshape/linspace/magic/warning/parse_double only.


## tests

### Source-specific bounded evidence

`builtins::core::wave30::constructors`: reshape inference/two-hole rejection, linspace singleton/zero, magic sums 3/4/6, warning flags, D-exponent parse.

Unexecuted here: No RNG, filesystem exist, timers, callbacks, registry dispatch, str2num, assertion numeric/message paths. No corpus, GPU, live filesystem, timer or callback run. No broad Clippy claim; dependency/style checks were not repeated. Only appended test blocks were rustfmt-formatted; original production bytes are unchanged.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: core uses ops concatenation/range/truthy, value representation, parser+Interp for str2num/str2func, files::exist_path, rng and io::sprintf_impl. Therefore pure constructor checks do not verify full registry or effects. Tests explicitly cover reshape/linspace/magic/warning/parse_double only.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.

### 2026-10-31 · tests

Seven named tests each passed twice debug and twice release on 2026-10-31, with actual named log lines sorted and identical across all four runs (28 selected successful executions). Each source has a compiled changed-assertion negative control; six runtime sources additionally have compiled production-body mutants that fail those unchanged tests. AST has representation evidence only. Every mutant is byte-restored and its positive test rerun. See `evidence.json` for this source's exact hash, mutation and exit statuses. External receipts retain full logs, command metadata, control records and prospective notes.
