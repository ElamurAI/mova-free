# school-format

## model

# school-format

## forward

Process trimmed nonblank noncomment lines in order, split first colon, dispatch fields, normalize fact/explain text, parse labeled questions, then apply lesson admission guard.

## invariant

List order survives append; scalar metadata is last-write-wins. Citation deletion is intentionally lossy, and layered facts use exact delimiters.

## cost

Linear field scan plus string allocation; nested parenthesis scanning may revisit suffixes, so no unconditional linear whole-normalizer bound is claimed.

## checks

knowledge::tail_tests::the_starred_option_is_the_right_one and knowledge::citation_tests::citations_and_layers. See ../model.md#tests for executed scope. A proposed oracle below is not a completed check.

## epoch-memory

Inspected the existing implementation before modeling. Preserved current behavior, including lossy decoding, index-sensitive ties and conditional updates. Existing witness tests distinguish active-scope learning from a wrong global witness. Counts represent distinct observations, not independent proofs. No source changes, performance gain, full-course result or autonomous combiner is claimed. AST remains explicitly absent; ranges bind source bytes only. Parser resources, panic/allocation behavior and complete inference semantics remain gaps.

## behavior-complement

Type: behavior_complement. Domain and observation: Lesson field parsing and citation/layer normalization, as narrowed below. Operator/target: U is terminating text/file-to-Result<Lesson> transitions. R is parse. Inner-bound hypothesis of U\R: success after an unknown field or missing delimiter; success with final n=0 or empty required vectors; silently accepting two marked answers. Domain excludes allocation failure.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing label negative control rejects two stars; admission cases beyond it are prospective.

## reverse-reconstruction

Type: reverse_reconstruction. Domain and observation: Lesson field parsing and citation/layer normalization, as narrowed below. Operator/target: B(L) includes every text whose ordered field fold and normalization gives L: variable whitespace/comments, overwritten scalar prefixes and removed citation spans. File argument is retained literally. No unique source text, original citation, or overwritten number is reconstructed.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Proposed paired-source normalization oracle; citation examples establish only selected fibers.

## citation-projection

Type: information_projection. Domain and observation: Lesson field parsing and citation/layer normalization, as narrowed below. Operator/target: Delete a closed parenthesis only when its whitespace words include at least two tokens, every preceding word begins uppercase or ASCII digit, and final word contains colon using only digits/colon/hyphen/comma. Preserve other parenthesis text. Layered fact delimiters with English before idea yield exactly two trimmed strings and bypass ordinary citation stripping.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing citations_and_layers checks deletion, preservation and layered splitting.

## field-order

Type: fold_algebra. Domain and observation: Lesson field parsing and citation/layer normalization, as narrowed below. Operator/target: For fields f1..fk, L=fold(step,L0,fields), with last-write scalar assignment and ordered list append. Reordering same-key facts changes output, while swapping distinct scalar assignments commutes if both parse. First failing field terminates before final required-field guard.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Proposed metamorphic ordering/error-prefix oracle.

## composition

The nested child is an internal stage of the forward relation above. Preserve input order and all shared fields across that stage; existentially quantify compatible intermediate states when reversing. Child success alone does not prove parent admission or answer correctness. The parent also retains surrounding branches that the child range does not cover. Parent complement targets its own U\R; it is not the union of child complements. Sequential reversal retains every compatible intermediate state. Failure prefixes may terminate before later stages. No partition of all source bytes is asserted.

## wave34-composition

### Wave34 composition

The field fold invokes question decoding for exercise/test/exam fields, citation scan for explain fields and fact projection for fact fields. Fact projection chooses layered extraction before citation fallback. Helper errors terminate the fold before later fields and legacy admission. The field-fold model includes calling behavior and overlaps sibling helper scope; do not sum overlapping costs.

Reverse composition preserves compatible intermediate values and their shared ledger/course/cache constraints. Parent behavior complement is relative to its own coupled trace universe, not a union of child complements. Costs include each executed call once. Existing model.md/composition.md retain historical models and evidence.

## History

### 2026-10-29 · document

Source-bound partial models, epoch 2026-10-29. Subject: world/src/knowledge.rs at the manifest SHA-256. Observation: Lesson field parsing and citation/layer normalization. Every relation below concerns this frozen source/configuration. Structural validation does not establish semantic correctness.

### 2026-10-29 · epoch-memory

2026-10-29: inspected the existing implementation before modeling. Preserved current behavior, including lossy decoding, index-sensitive ties and conditional updates. Existing witness tests distinguish active-scope learning from a wrong global witness. Counts represent distinct observations, not independent proofs. No source changes, performance gain, full-course result or autonomous combiner is claimed. AST remains explicitly absent; ranges bind source bytes only. Parser resources, panic/allocation behavior and complete inference semantics remain gaps.
