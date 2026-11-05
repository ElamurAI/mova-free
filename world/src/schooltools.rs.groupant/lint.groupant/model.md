# world-schooltools-lint

## model

# world-schooltools-lint

## forward
Each line first emits tab warning; recognized options then emit width, emptiness, spacing, stars, pairwise duplicates and glued-heuristic findings. lint optionally writes spacing repair before computing report, partitions case/glued as heuristic sections.

## invariant
Findings preserve line order and deterministic nested-loop order; pure lint has no filesystem effects. No assertion that a successful report means zero errors.

## cost
O(total bytes + sum per-line option-pair string comparisons + glued membership comparisons); malformed arbitrary-width lines can be quadratic.

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

## fix-effects
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## empty-malformed
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## wave34-space-normalization

Current nested child world-schooltools-lint-space-normalization: Split on literal newline including trailing empty segment. Each recognized option line rebuilds original leading indentation, trimmed key, colon-space, trimmed fields joined by space-bar-space, and original terminal CR if present. Nonoption lines are copied. Rejoin with newline.

Parent interaction: lint with fix=true writes normalized bytes before computing findings; this pure helper has no I/O. Normalization does not remove internal tabs or fix semantic/duplicate errors. Parser and rebuild must share original line indentation.

Earlier checks and epoch observations above remain historical, not new execution receipts. See file-root ../model.md#wave34-evidence for current selections.

## wave34-composition

### Wave34 composition

space-normalization borrows option_line and rebuild, preserving per-line CR/newline structure. In lint(fix=true), reading and optional write occur before lint_text; later file errors cannot undo earlier writes. Structural/heuristic findings are then separated. Pure normalization coverage is not evidence for write durability, all-file atomicity or correctness of lesson content.

The previous model.md and its composition remain historical epoch memory. Current scoped relations use the manifest source hash; partial syntax and finite helper tests do not prove semantic completeness.

## History

### 2026-10-30 · document

Source-bound epoch 2026-10-30. Every view below uses this exact source revision and finite inputs with sufficient allocation resources. Forward/absent/reverse relations observe only the stated projection; filesystem models assume fixed supplied dependencies and distinguish successful returns from partial effects. Status: partial source model, with bounded native evidence in ../model.md#evidence at the root sidecar. No approximation percentage, universal proof or school accuracy claim.

### 2026-10-30 · checks

Native module filters schoolbench:: and schooltools::, including eight new wave9 tests; independent finite rank/cadence/length/fraction oracles and a restore no-effect fixture. Raw evidence is external under $MOVA_DATA/runs/octopus-wave9-school-2026-10-30; root ../model.md#evidence binds outcomes. Macro bodies in syntax projection remain opaque; finite agreement does not certify unbounded input space.

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-schooltools-lint, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: A unique clean fixture cannot produce stars finding; multiple marked options must produce it. Heuristic glued/case findings are not automatic edit authorization or semantic contradictions.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-schooltools-lint, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Findings alone underdetermine original text, as unrelated fact lines and many unique distractors generate no findings. Even identical kind/line projections lose details and every unaffected line.

### 2026-10-30 · fix-effects

Type: effect_boundary. Subject: world-schooltools-lint, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: lint(fix=true) can persist changed files even if later file read/write fails; writes are ordinary fs::write. Source inspection supports effect ordering, no crash-safety test.

### 2026-10-30 · empty-malformed

Type: domain_boundary. Subject: world-schooltools-lint, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: option lines with zero separators still parse and yield field-count findings; pair loops handle short vectors. Tabs on non-option lines still yield findings. Existing native lint tests cover each reported kind.
