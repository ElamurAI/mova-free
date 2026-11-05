# world-schoolbench

## model

# world-schoolbench

## forward
Options choose a validated course identifier, seed, explicit tier/round or automatic cursor+1. Selection groups by stars, ranks (SHA256(seed NUL course NUL lesson-number), original index), keeps ceil(group-size/divisor) and restores chronological indices. A plan returns before run creation or course execution. A real run writes plan, invokes run_course_lessons with the selected Lesson clones, validates and scores trace, writes report/result, then advances cursor only in automatic mode.

## invariant
Selection indices are unique and ordered; quick is contained in medium contained in full for identical lessons and seed. Scores observe recorded rows, not lesson semantics. Automatic cursor advances only after result write succeeds; previous effects are not rolled back on failure.

## cost
N lessons: hashing total identity bytes plus O(sum_s n_s log n_s) comparisons and O(N) storage; sorting selected indices O(K log K). Trace score O(trace bytes), width 15 per accepted row. Course execution and external parser/cache costs remain unknown.

## composition
Leaf projections share exact lesson identities, selected indices, text bytes and option ordering. Compose only with those shared values retained; isolated score/length observations cannot recover them. Source inspection identifies knowledge::main_args dispatch, schoolbench selected-lessons -> counts -> run_course_lessons -> trace scoring, and schooltools option_line -> canon -> tally/lint/edit. Tests execute native pure helpers and isolated snapshot/restore; no live course/parser/teacher call.

## epoch-memory
Initial coverage records tolerant parser/score semantics and per-file effect boundaries. Model review exposed the direct zero restore index panic, repaired after old-code failure. Benchmark score loop was extracted unchanged to a pure private helper to test actual classification and width/count gates. Lock constructor failure may leave a stale directory; argument failures can consume flags. These remain explicit source-inspected limitations.

## checks

Native module filters schoolbench:: and schooltools::, including eight new wave9 tests; independent finite rank/cadence/length/fraction oracles and a restore no-effect fixture. Raw evidence is external under (see History); root model.md#evidence binds outcomes. Macro bodies in syntax projection remain opaque; finite agreement does not certify unbounded input space.

## behavior-complement
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## reverse-reconstruction
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## cadence-preimage
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## selection-fiber
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## score-projection
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## cursor-boundary
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## artifact-order
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## controls-scope
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## evidence

### School coverage evidence

#### Bounded observations

- Cadence: independent twenty-entry table checks 401 rounds including zero and u64::MAX; Full fiber on 1..100 is explicitly compared to five rounds.
- Selection: 49 lengths x 3 seeds x 3 tiers = 441 native-vs-rank-count comparisons; SHA-256 primitive is shared and not independently verified. Existing tests check nesting and chronological order.
- Parser/tally: 4 star positions x 7 Unicode answer lengths = 28 generated cases, malformed/empty/comment cases, normalization idempotence. Frac cross-products: 4,560 bounded cases against small integer arithmetic.
- Trace: 4 kinds x 4 answer labels = 16 observations, each also checked with wrong expected cardinality; empty and malformed row cases. Scorer extraction preserves the prior loop.
- Edits: sequential replacement, forbidden new content, duplicate/case refusal, zero line, argument consumption on failure. Native restore fixture verifies Err and unchanged bytes for zero index, following old-code panic. Only uniquely created temporary fixture paths are written.

#### Structural checks and limits

Both final syntax projections generated twice with identical bytes. Eight groupants (two file roots plus six scoped fragments), forty typed views: eight per root and four per fragment. Every source binding is exact. `groupants` using external full-registry scaffold succeeds with no diagnostics; its deliberately unmaterialized scaffold parent is not an integrated ancestor model. `changed` registers both changed sources. Integration registry and actual ancestors remain the parent task's responsibility.

These are finite tests plus source inspection, not proofs or percentage-error profiles. Hash correctness, allocation exhaustion, filesystem crash/permission failures, external annotation/cache semantics and full school learning remain outside executed evidence. Lock directory can survive constructor pid-write failure; per-file edit/lint writes can survive later errors; scoring does not validate header/kind identities. CLI argument vector mutations on error and unbounded line-range allocation remain explicit.

## History

### 2026-10-30 · document

Source-bound epoch 2026-10-30. Every view below uses this exact source revision and finite inputs with sufficient allocation resources. Forward/absent/reverse relations observe only the stated projection; filesystem models assume fixed supplied dependencies and distinguish successful returns from partial effects. Status: partial source model, with bounded native evidence in model.md#evidence at the root sidecar. No approximation percentage, universal proof or school accuracy claim.

### 2026-10-30 · checks

Native module filters schoolbench:: and schooltools::, including eight new wave9 tests; independent finite rank/cadence/length/fraction oracles and a restore no-effect fixture. Raw evidence is external under $MOVA_DATA/runs/octopus-wave9-school-2026-10-30; root model.md#evidence binds outcomes. Macro bodies in syntax projection remain opaque; finite agreement does not certify unbounded input space.

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-schoolbench, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Universe is normal-return argument/lesson/trace/artifact transitions with fixed dependencies. Inner absence constraints exclude unsupported tier strings, zero explicit round, simultaneous round and tier, out-of-range selection indices, and successful trace scoring with wrong width/count. This is not a full complement of external course execution or failed I/O traces.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-schoolbench, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: For fixed lesson list and seed, observations of selected indices admit every tier yielding that subset; singleton strata make all tiers indistinguishable. Result totals admit many traces/questions and learning histories. Cursor alone cannot reconstruct manual runs, failures or previous artifacts.

### 2026-10-30 · cadence-preimage

Type: reverse_reconstruction. Subject: world-schoolbench, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: For round in u64, Full fiber is residues 0 mod 20; Medium is 5,10,15; Quick all other residues. CLI forbids zero although Tier::for_round(0)=Full. Bounded test enumerates 0..399 and MAX against an independent cycle table.

### 2026-10-30 · selection-fiber

Type: information_loss. Subject: world-schoolbench, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Fixed identity/stars sequences differing only in facts/questions produce identical selection indices. Counts and scores require selected content; selection does not establish equal learning trajectory. Duplicate course/number keys break rank ties by original index.

### 2026-10-30 · score-projection

Type: observation_relation. Subject: world-schoolbench, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Map each 15-cell row after first line to bucket exam iff cell 3 equals exam, otherwise test; right iff cell 11 equals right. Comments beginning # vanish. All other cells are unconstrained by this helper, so equal totals have many antecedents. Sixteen synthetic rows and count/width refusal checks falsify alternative strict-kind/case-insensitive models.

### 2026-10-30 · cursor-boundary

Type: state_transition. Subject: world-schoolbench, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Explicit round bypasses cursor reading; explicit tier still reads cursor+1 despite not advancing it. Missing cursor means zero; malformed or MAX cursor refuses before course execution. checked_add bounds arithmetic. These are source-inspected branches, not live benchmark execution.

### 2026-10-30 · artifact-order

Type: effect_boundary. Subject: world-schoolbench, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Plan mode skips schedule creation and lock but still reads cursor/course. Real mode can leave plan, trace, cache or report on later failure. Lock::take creates directory before pid write and before constructing its RAII value: pid-write failure can leave stale directory; Drop covers only constructed locks. No durability or multi-process fairness proof.

### 2026-10-30 · controls-scope

Type: constraint_relation. Subject: world-schoolbench, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Quick disables controls and before-tests, Medium enables controls only, Full enables both and parse refresh. controls_red requires exactly four #control tab lines ending tab red (three before 2026-11-04, when the cross-lesson control was added); false controls do not themselves prevent cursor advancement. Flags identify measurement scope, not a statistical error cap.

### 2026-10-30 · evidence

Heading `### School coverage evidence — 2026-10-30` stays in evidence as `### School coverage evidence`.

- schoolbench.rs SHA-256: `f969508a727a1908f74a38f41254401b68794cd472d623dc0375aa194d0d1dc3`.

- schooltools.rs SHA-256: `e1ed83bc9178ddb89ab4358347331985c13134e3a9886cb587e57012bccd1dbf`.

Eight new native tests plus eleven existing source-local regressions: 19 pass, zero ignored in each of two debug and two release runs, and restored-control check. Command: `cargo test -p world --lib [--release] -- schoolbench:: schooltools:: --test-threads=1`. Normalized test identity/result lines agree in all five logs, SHA-256 `3aa940ebe42f3bdbe08a603dde50ee3a8563eb40f0423e13ecf3a7881722717a`. Earlier broad `school` filter also passed 24 with one ignored; final claims use the focused 19. No parser, teacher, run-course or actual lesson edits were invoked.

#### Controls and provenance

Cadence production mutant changes modulus 20 to 10; `cadence_fibers` fails on mismatched enum. Character metric mutant replaces Unicode scalar count with byte length; `parser_canonical_fibers_and_tally` fails a Tally assertion. Both compile, execute the selected test, fail with status 101 and are restored byte-for-byte before the final 19-test positive check. Mutant SHA-256 records: benchmark `dd5d47c4036666ee9945dc5dd978dabcc5de0bbd4e191f55c30ae1fd62a13370`, tools `d0d7e397b2f26dfd46d222c3f4e160e0e45b90b685a9a9d8bcc9a63b07e832c7`.

Raw evidence is under `$MOVA_DATA/runs/octopus-wave9-school-2026-10-30/`.

| Artifact | SHA-256 | Result |
|---|---|---|
| final-debug1.log | ccb409629b409b2357f7e31fe59764a4b1fb847e413283cd8d331d0f4087c14e | 19 pass |
| final-debug2.log | 62e85389b2e8a5932f7de205feab69b52ee018a6565ed11c451eab2b530bb86a | 19 pass |
| final-release1.log | ccbb98332df6a0026cd2d1bb4564453593ba2041cd87f74b9a2f645f32eb6511 | 19 pass |
| final-release2.log | be2aaff7bfaf8032b8fa18fa7814554cbcb2663cf12cb77b6edcc90f9fcd2925 | 19 pass |
| final-restored.log | 02889e4028b702207bf518947c3eb1c8d160c9c7a0cb79259eea39481bfb2922 | 19 pass |
| final-mutant-bench.log | f58749d8ae4b6610af308d0b6eba6f25018f2442c5dc06d1108b589f8a4158b3 | named assertion rejected compiled mutant |
| final-mutant-tools.log | 66764920e4d59796060e3657456fcf7f6994bedc073bf55da99c7a49402f7591 | named assertion rejected compiled mutant |
| old-failure.log | 825db9e94b4bdd641efb2298a6b7435b8ca8fc5ae201872986861402f4ddbb58 | direct zero restore panicked before repair |
