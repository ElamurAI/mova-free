# algorithm-weighted-interval-scheduling-tests

## descriptant

### weighted_interval_scheduling tests.rs

Independent exponential oracle plus metamorphic and boundary checks. Source is authoritative; sidecar models and syntax are partial. Models compose recurrence, bounds and public result; tests observe this composition rather than establish a universal proof. No approximate execution profile is supplied because this API promises exact results; finite evidence is not an analytical error guarantee.

## model

# weighted_interval_scheduling tests.rs model

## Forward

Execute deterministic finite fixtures with independent exponential oracle and production-linked relational assertions. Passing supports only the observed domain.

## Invariant

Oracle enumerations are finite and include empty choices. Assertions inspect actual production returns. No random generator, network or imported implementation.

## Cost

Bounded exponential test oracle; suitable only for small fixtures. Production cost is modeled separately.

## nonbehavior

Type: behavior_complement. A passing outcome excludes observed disagreements on enumerated fixtures and oracle comparisons; untested behaviors remain admitted. This relation describes exclusions within the finite observation universe.

## preimages

Type: reverse_reconstruction. A passing report admits all implementations agreeing on these fixtures, including incorrect ones elsewhere. A failed assertion constrains a mismatch at its fixture, not unique source reconstruction.

## oracle

Type: observational_equivalence. Enumerate all subsets, reject pairwise overlaps and maximize explicit sums; compare all ordered arrays through length six from four choices. Oracle does not share sort, predecessor search or recurrence.

## relations

Type: metamorphic_relation. Tests observe exact endpoint translation, positive weight scaling, permutation score invariance, original-index reconstruction, first invalid index, extremes, adjacency and zero-weight ties.

## sensitivity

Type: counterfactual_model. Changing compatible end<=start to end<start is a compiled production fault that adjacency/oracle checks must reject. Other faults remain unproven.

## evidence

### Evidence

LCS independent oracle enumerates subsequences of each first argument and scans the second, checking all 63 squared binary-word pairs through length five (3,969 pairs). Scheduling independently enumerates subsets and pairwise compatibility for all 5,461 arrays through length six over four item choices. Metamorphic tests include explicit input-fiber ambiguity, boundary and invalid-input cases. Allocator exhaustion and unreachable allocated-slice dimension/weight overflow cases were not executed.

Strict Clippy all-targets with -D warnings passed. rustfmt --check passed for all four owned Rust files. Installed mova-modelcheck syntax ran twice per source and produced byte-identical projections. Syntax is partial: macros and semantic typing are not expanded.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective cost and invariants retained in implementation prospective.md. The reverse models retain ambiguous input fibers and separate behavior exclusions from reconstruction. No timing optimization claimed.

### 2026-10-30 · evidence

Heading `### Evidence epoch 2026-10-30` stays in evidence as `### Evidence`.

Final source validation: two debug and two release executions of the complete mova-algorithms package passed 176 unit tests and one doctest each. Nine new tests belong to these two algorithms (four LCS, five scheduling). Sorted named test outcomes were identical across all four runs; elapsed time and build chatter were excluded. Normalized outcome SHA-256: c38c9be833de92508d6cd6304a57c36c196c4c4ef57561fa9c7f361dd7816255.

Compiled production negative control: replace end <= start by end < start; real tests fail. Mutant log SHA-256 08941153398c7a62ca10a1734b7fe22517d3bcb3f0b900dc6482e8e19d33b966. Original source restored before final repeated runs.

Artifacts live under $MOVA_DATA/runs/octopus-wave4-dynamic-2026-10-30. A temporary local lib declaration was used for package integration tests and restored. Registry-entry copies are supplied for main-instance integration; shared registry hierarchy checks remain the integrator's responsibility. These observations establish bounded correctness evidence and sensitivity to the named fault, not a proof or benchmark gain.
