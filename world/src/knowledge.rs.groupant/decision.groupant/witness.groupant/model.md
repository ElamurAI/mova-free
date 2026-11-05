# school-witness

## forward

On understood non-count wrong decisions with supported right and chosen options, increment right feature coordinates and decrement chosen coordinates (both reversed for a counted-against witness); otherwise keep weights.

## invariant

Only addressed ask/route/feature coordinates change; shared coordinates cancel according to multiplicity. Caller must supply valid option indexes and a valid weight table.

## cost

O(number of feature occurrences in the two support vectors); no frame search inside correction.

## checks

knowledge::school_math_tests::selected_witness_updates_the_decision_gap_and_explanation and scope_fallback_and_non_weight_decisions_do_not_learn_wrong_features. See ../../model.md#tests for executed scope. A proposed oracle below is not a completed check.

## epoch-memory

Inspected the existing implementation before modeling. Preserved current behavior, including lossy decoding, index-sensitive ties and conditional updates. Existing witness tests distinguish active-scope learning from a wrong global witness. Counts represent distinct observations, not independent proofs. No source changes, performance gain, full-course result or autonomous combiner is claimed. AST remains explicitly absent; ranges bind source bytes only. Parser resources, panic/allocation behavior and complete inference semantics remain gaps.

## behavior-complement

Type: behavior_complement. Domain and observation: Supported structured perceptron correction with active witness identity, as narrowed below. Operator/target: U is well-indexed (w,Answer,right,w') transitions without arithmetic overflow. R is learn_answer. U\R contains any w'!=w when !understood, count is Some, choice==right, or either required support is absent. In the active case it contains w'!=w+sigma_right*phi_right-sigma_choice*phi_choice (sigma=-1 for a counted-against witness, else 1). This is an exact algebraic target with source-derived correspondence only.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing focused fixtures exercise guard branches and active +2; no universal arithmetic proof.

## reverse-reconstruction

Type: reverse_reconstruction. Domain and observation: Supported structured perceptron correction with active witness identity, as narrowed below. Operator/target: For fixed Answer/right, w=w'-d in the active branch; otherwise w=w'. If Answer/right is hidden, B(w') is the union over all compatible d and inactive histories. Thus fixed metadata yields unique subtraction, but weights alone never identify the exercise.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Proposed enumeration of bounded integer weights and support multiplicities.

## coordinate-cancellation

Type: update_sparsity. Domain and observation: Supported structured perceptron correction with active witness identity, as narrowed below. Operator/target: For index j, delta_j=count(right features mapped to j)-count(chosen features mapped to j). feat_idx merges Conflict slots other than Verb/Agent/Patient into coordinate12; the model uses coordinate multisets, not distinct semantic feature names.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing fixture cancels shared verb in the old-witness control; noncanonical conflict collisions remain proposed.

## local-gap

Type: energy_identity. Domain and observation: Supported structured perceptron correction with active witness identity, as narrowed below. Operator/target: With fixed d and g(w)=w dot d, g(w+d)-g(w)=sum_j d_j^2. This says nothing about next-call witness selection, global held-out accuracy, or overflow. A wrong witness can improve its own gap while worsening the actually selected scope.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Executed current-scope +2 versus old-scope -2 fixture; formal/School.lean is an external specification, not newly executed here.

## History

### 2026-10-29 · document

Source-bound partial models, epoch 2026-10-29. Subject: world/src/knowledge.rs at the manifest SHA-256. Observation: Supported structured perceptron correction with active witness identity. Every relation below concerns this frozen source/configuration. Structural validation does not establish semantic correctness.

### 2026-10-29 · epoch-memory

2026-10-29: inspected the existing implementation before modeling. Preserved current behavior, including lossy decoding, index-sensitive ties and conditional updates. Existing witness tests distinguish active-scope learning from a wrong global witness. Counts represent distinct observations, not independent proofs. No source changes, performance gain, full-course result or autonomous combiner is claimed. AST remains explicitly absent; ranges bind source bytes only. Parser resources, panic/allocation behavior and complete inference semantics remain gaps.

### 2026-11-05 · local-gap

2026-11-05: a counted-against witness (its stored score ≠ the weighted sum of its features under the weights of the
decision; `counter`) steps along the negation of its features, so the local gap identity holds for the signed score
s' = −w·φ: the step is d = σ_right φ_right − σ_chosen φ_chosen with σ = −1 for a counted witness. Signs are taken
before either update. learning_follows_the_sign_of_a_counted_against_witness: the gap rises; the unsigned step lowers it.
