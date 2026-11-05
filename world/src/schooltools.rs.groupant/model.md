# world-schooltools

## model

# world-schooltools

## forward
option_line trims key/fields for three case-sensitive kinds. canon moves a uniquely starred answer to position one and strips leading stars; zero/multiple marked options remain unchanged. Tally compares Unicode scalar counts. Lint reports structural and heuristic findings; spacing fixer normalizes option lines. Edits fold in table order on a working text and return either complete edited text or accumulated refusals. File edit command processes files in BTreeMap order with per-file writes. Restore validates line count and indices before copying requested saved lines.

## invariant
Pure apply_edits exposes no partially changed string on Err; command-level edit is not all-files atomic. Original option positions and star of replaced answer are preserved for supported edits. Restore rejects zero and out-of-range indices before writing.

## cost
Text parsing linear bytes; lint pair comparisons quadratic in options and string lengths plus glued scans. apply_edits clones/splits text and scans fields per edit, with final join linear bytes. Filesystem cost proportional to bytes transferred; no atomic/durable write guarantee.

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

## parser-fibers
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## lint-versus-parse
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## edit-order
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## restore-domain
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## argument-residual
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## numeric-resource
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## Epoch notes

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

## wave34-composition

### Wave34 composition

key-fields precedes marked-answer before tally and edit guards. lint separately inspects spacing and can call space-normalization before diagnostics. edit-sequence produces either one whole accepted buffer or refusals; command writes each accepted file independently. restore-selection validates saved/current cardinality and all requested indices before copying. File identities and complete strings must be retained across these stages; no multi-file transaction or crash guarantee.

The previous model.md and its composition remain historical epoch memory. Current scoped relations use the manifest source hash; partial syntax and finite helper tests do not prove semantic completeness.

## wave34-evidence

### Wave34 current evidence

#### Current selections

- schooltools::wave9_models::parser_canonical_fibers_and_tally
- schooltools::wave9_models::edit_transaction_and_argument_effects
- schooltools::tests::fix_spaces_changes_spaces_only
- schooltools::wave34_models::restore_preflight_and_duplicate_selection

#### Controls

Two compiled production mutants in the worker slice are rejected by named assertions (exit 101), with exact original backup restoration and green selected reruns (exit 0): inclusive span changed to exclusive; restore drops saved CR, causing duplicate restoration count to become two. The latter uses a custom assertion message, not an incidental runtime panic. No compile failures or survivors are counted as kills.

#### Provenance and limits

## History

### 2026-10-30 · document

Source-bound epoch 2026-10-30. Every view below uses this exact source revision and finite inputs with sufficient allocation resources. Forward/absent/reverse relations observe only the stated projection; filesystem models assume fixed supplied dependencies and distinguish successful returns from partial effects. Status: partial source model, with bounded native evidence in model.md#evidence at the root sidecar. No approximation percentage, universal proof or school accuracy claim.

### 2026-10-30 · checks

Native module filters schoolbench:: and schooltools::, including eight new wave9 tests; independent finite rank/cadence/length/fraction oracles and a restore no-effect fixture. Raw evidence is external under $MOVA_DATA/runs/octopus-wave9-school-2026-10-30; root model.md#evidence binds outcomes. Macro bodies in syntax projection remain opaque; finite agreement does not certify unbounded input space.

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-schooltools, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Universe is finite UTF-8 text, edit lists and supplied fixture filesystem transitions under successful allocations. Inner absence excludes successful replacement with forbidden separators, ambiguous old text, duplicate case-folded options or invalid line indices. Lint clean does not prove correctness of teaching content; errors in later files do not undo earlier writes.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-schooltools, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Canonicalization loses whitespace, original marked position and repeated leading stars. Tally loses every question and option string beyond length order. Successful edit result with unknown edits has many earlier texts; snapshot restore reconstructs selected saved lines only, not external file history.

### 2026-10-30 · parser-fibers

Type: reverse_reconstruction. Subject: world-schooltools, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: All leading/trailing whitespace variants of recognized option lines share trimmed fields; three accepted kind strings remain distinct. For one starred answer, placing it in any of four option positions yields same canonical right answer while wrong order remains. Tested using all four positions and seven Unicode answer lengths.

### 2026-10-30 · lint-versus-parse

Type: constraint_relation. Subject: world-schooltools, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: option_line accepts arbitrary field count; tally counts any line with at least three canonical fields, whereas lint expects five. Multiple-star lines are reported but canonicalization intentionally leaves them as-is; this is tolerant parsing, not a validation gate.

### 2026-10-30 · edit-order

Type: state_transition. Subject: world-schooltools, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Edits see earlier successful tentative edits: b->z then z->y succeeds, reverse order fails. An invalid later edit makes pure result Err even when earlier edits changed the private buffer. CLI per-file success can persist before a different file refuses.

### 2026-10-30 · restore-domain

Type: effect_boundary. Subject: world-schooltools, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: restore_lines accepts arbitrary usize slices from direct callers, beyond CLI line_numbers. Zero must be refused before x-1 indexing. Regression reproduced old panic, then verified Err and exact current bytes in a uniquely owned fixture. Duplicate valid requests write once effectively but requested count still includes duplicates.

### 2026-10-30 · argument-residual

Type: information_loss. Subject: world-schooltools, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: take_value removes first occurrence and next token; chunking consumes flag/prefix before numeric validation or pairing error. An Err does not restore argument vector. take_switch removes all exact matches; knowledge_args indexes all[1] assuming dispatcher supplied at least two arguments.

### 2026-10-30 · numeric-resource

Type: resource_boundary. Subject: world-schooltools, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Rational Frac uses u64 numerator/denominator and u128 cross-products with usize counts; on supported <=64-bit usize the products fit u128. Rational values above one and reversed Band bounds are accepted, unlike out-of-range decimal values. line_numbers materializes ranges, so MAX-sized requests may exhaust resources; no allocation cap is promised.

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

### 2026-10-31 · wave34-evidence

Heading `### Wave34 current evidence — 2026-10-31` stays in wave34-evidence as `### Wave34 current evidence`.

Source SHA-256: a892a1ce9f8c6855c3d28711fea2bbf66933e62827377e972d4606aee43ee519. Five genuinely new nested groupants, six typed views each; earlier models and IDs preserved. Current native evidence is 4 exact tests, each passing twice debug and twice release. Two appended tests across the school slice cover priority/None shadowing and restore preflight/duplicate selection. Production bytes are unchanged; the original source is an exact prefix and the appended ranges are cfg(test) only.

Raw source-bound receipts, normalized target-qualified outcomes, syntax pairs, backup hashes and prospective model are in $MOVA_DATA/runs/octopus-wave34-school-2026-10-31. Syntax generated twice is byte-identical, partial and macro-opaque. Historical model.md#evidence and previous model checks remain historical. Supplied helpers and temporary files only: no live lessons, model loads, corpus, teacher, frozen suites, network or publication. World arithmetic retains representability/nonzero-divisor premises. No performance improvement or approximate-error cap is claimed.

### 2026-11-04 · Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.

### 2026-11-05 · school data outside the repository

school_root() is knowledge::courses_root() (the school data outside the repository); messages name the school courses instead of data/school.
