# algorithm-smawk-tests

## descriptant

### smawk tests.rs

Original std-only implementation and independent fixtures. Aggarwal, Klawe, Moran, Shor and Wilber (1987), Geometric applications of a matrix-searching algorithm.

## approximant

### Forward model

Execute exhaustive finite oracle comparisons and focused boundary assertions for smawk. A mismatch panics; successful observations cover only enumerated fixtures.

### Cost

Exhaustive fixture growth is exponential in fixed small dimensions; independent full scans intentionally cost more than optimized source. Test allocation can panic; test harness is not a fallible production API.

### Limits

No timing guarantee, allocator injection, or general proof. No useful cheaper executable surrogate for assertions; omitted explicitly.

## models

## behavior_complement

A successful harness run excludes disagreement on the enumerated fixtures, not on untested inputs; an intentionally changed production recurrence must yield an assertion failure, not merely compilation rejection.

## reverse_reconstruction

The same all-pass observation admits distinct implementations agreeing on finite fixtures and differing elsewhere; complete source behavior is not recoverable from test output.

## oracle_separation

Small exhaustive expected values use subset enumeration, full row scans, full predecessor scans or cubic BST recurrence, independently of optimized candidate pruning.

## boundary_partition

Budget-zero fixtures and empty-domain fixtures distinguish successful mathematical emptiness from rejected resource exhaustion; the two statuses are not interchangeable.

## compiled_fault

The production control changes a recurrence comparison, initial reachability or objective result while retaining compilation; the named focused test must reject that semantic change.

## tests

### Checks

cargo test -p mova-algorithms smawk::tests

## combiner

### Composition

Raw tests.rs syntax and mathematical recurrence are separate evidence. The partial syntax projection binds the exact source bytes but does not certify the recurrence. Oracle tests compare observable outputs; typed relations constrain lawful transformations and compatible preimages. For smawk, resource caps and checked failures remain separate from mathematical objective equivalence. Prospective expectations remain unchanged; finite validation cannot turn monotonicity assumptions into runtime validation.

## validation

### Validation

Compiled production controls: zero initial subset bit, forced zero MITM objective, right-tie SMAWK reduction, forced zero layered-DP objective and forced zero BST objective all fail named equality assertions. Byte restoration followed by full positive library rerun passes. A separate SMAWK interpolation tie mutation fails lawful_matrices after adding half-integer squared-distance centers; the restored focused test passes. Receipts: controls.log, mutant-*.log, restored.log, smawk-restored.log in the same external run directory. The original constant-matrix fixture alone did not distinguish interpolation ties; the stronger fixture closes that observed gap.

Every source/test file has two independently generated identical syntax projections and current source SHA. An isolated ten-leaf modelcheck groupants run reports valid=true, diagnostics=[], ten sidecars, zero missing source sidecars; the temporary collection parent has no sidecar and makes no composition claim. Root integration owns global registry and ancestor binding checks.

No timing speedup, machine-independent resource guarantee, allocation-failure injection, monotonicity validation, or end-to-end Mova improvement is inferred. Successful finite tests certify the listed observations only. No teacher, model, corpus or live database operations were used.

## History

### 2026-10-30 · descriptant

Prospective model saved before changes in $MOVA_DATA/runs/octopus-wave20-dp-2026-10-30/prospective.md. Current epoch 2026-10-30: exact optimization with explicit bounded-resource outcomes. Finite oracle agreement does not establish the caller's monotonicity premises. No timing or whole-Mova speed claim.

### 2026-10-30 · tests

Independent small oracle and resource boundary fixtures live in ../tests.rs. Repeated debug/release and compiled mutation receipts: $MOVA_DATA/runs/octopus-wave20-dp-2026-10-30.

### 2026-10-30 · validation

Heading `### Validation — 2026-10-30` stays in validation as `### Validation`.

Ten new native tests cover five exact kernels. Full mova-algorithms library: 557 tests pass in two debug and two release executions; sorted native test outcomes agree byte-for-byte across all four. Strict all-target Clippy and focused rustfmt pass. External receipts: $MOVA_DATA/runs/octopus-wave20-dp-2026-10-30/{debug1,debug2,release1,release2,clippy}.log.
