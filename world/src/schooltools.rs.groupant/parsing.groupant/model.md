# world-schooltools-parsing

## model

# world-schooltools-parsing

## forward
Split recognized keys at first colon and values at every bar, trimming fields. canon moves exactly one marked option to the front after question, dropping all initial stars and then initial whitespace; otherwise copies fields.

## invariant
No unique star implies unchanged field vector. Unique-star canonical result preserves field count and wrong-option relative order.

## cost
O(bytes + field count), borrowing slices with new vectors; Unicode trim uses standard Rust semantics.

## composition
Leaf projections share exact lesson identities, selected indices, text bytes and option ordering. Compose only with those shared values retained; isolated score/length observations cannot recover them. Source inspection identifies knowledge::main_args dispatch, schoolbench selected-lessons -> counts -> run_course_lessons -> trace scoring, and schooltools option_line -> canon -> tally/lint/edit. Tests execute native pure helpers and isolated snapshot/restore; no live course/parser/teacher call.

## epoch-memory
Initial coverage records tolerant parser/score semantics and per-file effect boundaries. Model review exposed the direct zero restore index panic, repaired after old-code failure. Benchmark score loop was extracted unchanged to a pure private helper to test actual classification and width/count gates. Lock constructor failure may leave a stale directory; argument failures can consume flags. These remain explicit source-inspected limitations.

## checks

Native module filters schoolbench:: and schooltools::, including eight new wave9 tests; independent finite rank/cadence/length/fraction oracles and a restore no-effect fixture. Raw evidence is external under (see History); root ../model.md#evidence binds outcomes. Macro bodies in syntax projection remain opaque; finite agreement does not certify unbounded input space.

## behavior-complement
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## reverse-reconstruction
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## unicode-length
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## normalization-idempotence
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## wave34-key-fields

Current nested child world-schooltools-parsing-key-fields: Trim line, reject leading #, split at first colon, trim key, accept exactly exercise/test/exam. Split value at every | and trim each field, retaining empty fields and arbitrary width.

Parent interaction: Parsing precedes canon in tally/lint/edit. Lint separately emits tabs on all lines before recognizing option lines; parse refusal does not suppress that nonoption diagnostic.

Earlier checks and epoch observations above remain historical, not new execution receipts. See file-root ../model.md#wave34-evidence for current selections.

## wave34-marked-answer

Current nested child world-schooltools-parsing-marked-answer: Collect indices >=1 with starts_with('*'). If exactly one r, output question then field[r] stripped of all initial stars and following leading whitespace, then all other options in their original order. Zero or multiple marked options return a copy unchanged.

Parent interaction: Parent parser supplies trimmed slices; canon itself accepts arbitrary slices and does not trim unmarked options. tally compares canonical right against wrongs; lint counts ambiguous marker options before canon; apply_edits canonicalizes both before/after for guards.

Earlier checks and epoch observations above remain historical, not new execution receipts. See file-root ../model.md#wave34-evidence for current selections.

## wave34-composition

### Wave34 composition

key-fields first strips allowed outer whitespace and recognizes three literal kinds; marked-answer then acts on the exact field order. No recognition result establishes five-field width or a unique star. Tally and lint share canonicalization but have different acceptance rules. Distinct original right positions collapse into one canonical ordering while wrong order remains stable.

The previous model.md and its composition remain historical epoch memory. Current scoped relations use the manifest source hash; partial syntax and finite helper tests do not prove semantic completeness.

## History

### 2026-10-30 · document

Source-bound epoch 2026-10-30. Every view below uses this exact source revision and finite inputs with sufficient allocation resources. Forward/absent/reverse relations observe only the stated projection; filesystem models assume fixed supplied dependencies and distinguish successful returns from partial effects. Status: partial source model, with bounded native evidence in ../model.md#evidence at the root sidecar. No approximation percentage, universal proof or school accuracy claim.

### 2026-10-30 · checks

Native module filters schoolbench:: and schooltools::, including eight new wave9 tests; independent finite rank/cadence/length/fraction oracles and a restore no-effect fixture. Raw evidence is external under $MOVA_DATA/runs/octopus-wave9-school-2026-10-30; root ../model.md#evidence binds outcomes. Macro bodies in syntax projection remain opaque; finite agreement does not certify unbounded input space.

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-schooltools-parsing, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Inner complement excludes recognizing comments, uppercase Test or fact as option kind, and forbids deleting wrong options in unique-star canonicalization. Empty vectors remain empty.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-schooltools-parsing, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Canonical outputs cannot reconstruct whitespace, leading star multiplicity or original right-option position. Complete reconstruction requires retaining these lost annotations; finite position oracle illustrates ambiguity.

### 2026-10-30 · unicode-length

Type: observation_relation. Subject: world-schooltools-parsing, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: chars counts Unicode scalar values, not UTF-8 bytes or grapheme clusters; accented scalar repetitions are used in seven length comparisons per star position.

### 2026-10-30 · normalization-idempotence

Type: identity_relation. Subject: world-schooltools-parsing, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: fix_spaces applied twice equals once on synthetic option lines; non-option text is copied. Tabs inside fields and semantic wording remain unchanged; this normalization is not full lint repair.
