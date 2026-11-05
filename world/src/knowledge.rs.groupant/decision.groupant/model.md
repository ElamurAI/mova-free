# school-decision

## forward

Read question/tail; derive masked frames and anchors; expand typed targets; score option comparisons; choose global or current-lesson witness scope; break permitted ties; apply unique quantity override; learn only supported exercise mistakes.

## invariant

Answer observation is a projection of intermediate parses and witnesses. Current-support choice/explanation/training share witnesses; counting bypasses the role-weight correction.

## cost

Question/option reads plus candidate alignments and target comparisons, then O(active features) correction. Cache hits alter parse cost; no fixed asymptotic in input text alone.

## checks

knowledge::school_math_tests plus tail_tests; full answer requires external parser resources. See ../model.md#tests for executed scope. A proposed oracle below is not a completed check.

## epoch-memory

Three changes in `answer`, all measured. (1) The question's things include its entities' noun compounds the lessons name, possessors and prepositional-modifier heads (on; PRAG_OFF=student-qents-mods): only the own-route aboutness filter widens. (2) A «when» question (the reader asks Role(Attr)) fills that role from a bare option with its «when:» attributes alone, else «when:<head>»; a time attribute meets only a time attribute. (3) A «which/what N» question in a core role: against a target whose entity in that role is N (same or a kind) and names the option (`Ent::names`), the role's conflict becomes a match (on; PRAG_OFF=wh-which); other options keep the conflict. Weight table unchanged (17 × 10 × 7). Ten courses 3 204 → 3 234 → 3 248 (odd 1 596 → 1 617, even 1 608 → 1 631); Wikipedia when 259 → 881 and which 65 → 200 of 1 106 and 214, articles-shuffled 263 and 64; subjects −12 net there (side effect of the time role).

`gap_diag` / `gap_diag_option` sit inside this range after `answer`: a read-only diagnostic of an Answer that was not understood (categories in the parent note), recomputing the question's things as `answer` does without compound modifiers. It never changes weights, frames or the choice; it runs only under MOVA_GAPDIAG=1 from the course loop. Checked by gap_tests (not-about with its distance, verb-unknown, and the reached-gated control).

`kinds` may add the loaded dictionary's genus of a question entity (`dict_kinds`: the student's own
reading of the first meaning, head noun only, then up to three WordNet kinds, kept when remembered) while the question's
own frames and things are coded (`q_side`), behind the opt-in `PRAG_VN=student-dict`. Off, the decision is
byte-identical to the previous revision (ten and twelve primary courses, three Wikipedia courses). On, it gained on
the ten courses with flat controls but raised the cross-lesson control on twelve others as much as the student
(docs/training.md section 32).

`kinds` may also add a question entity's dictionary nyms (`dict_nyms`: the synonyms and form-of lemmas
of its noun entries in a layer with extra columns, the Wiktionary module), kept when remembered, behind the opt-in
`PRAG_VN=student-wikt` (`student-wikt-lack`: only nouns WordNet lacks; `student-wikt-mutual`: only nyms whose own
entries name the entity back). Off, or without such a layer, the decision is byte-identical to the base build. On,
the keys-removed and cross-lesson controls rose as much as the student (docs/training.md, «Building the dictionary»).

## behavior-complement

Type: behavior_complement. Domain and observation: Question interpretation, anchor/target scoring, option selection and correction, as narrowed below. Operator/target: U is finite well-indexed answer/learn traces with fixed parser and flags; R is the declared pipeline. Inner-bound U\R includes retaining >10 anchors, learning from count results, and keeping an old-only best witness after current_support activates. These exclusions are conditional on normal completion.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing fixture checks last two; anchor bound is inspected.

## reverse-reconstruction

Type: reverse_reconstruction. Domain and observation: Question interpretation, anchor/target scoring, option selection and correction, as narrowed below. Operator/target: Given choice/understood/count/witnesses, retain all parses, candidates, rejected anchors, route histories and weight vectors satisfying the ordered filters and comparisons. Equal scores and hidden earlier frames leave many antecedents. Observing index0 with understood=false does not identify why evidence was absent.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Proposed finite rank/selection solver; no executable global inverse.

## choice-ties

Type: ordered_selection. Domain and observation: Question interpretation, anchor/target scoring, option selection and correction, as narrowed below. Operator/target: Global selection maximizes (score,recent,tie_sum,-index). Current-lesson selection maximizes (recent_score,tie_sum,-index) and disables extra-content alternative selection. Without lesson-first, an equal-score strict word-set subset with lower extra may replace the winner, minimizing (extra,index). Count override occurs last.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Source-derived ordering; full tie branch oracle remains proposed.

## count-learning-coupling

Type: interaction_relation. Domain and observation: Question interpretation, anchor/target scoring, option selection and correction, as narrowed below. Operator/target: A unique numeric option sets count Some and understood true even without frame support; learn_answer then rejects that update. Thus successful understanding is necessary but insufficient for learning. Parent composition must pass count and active witnesses, not only chosen index.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing scope_fallback_and_non_weight_decisions_do_not_learn_wrong_features tests the exclusion.

## composition

The nested child is an internal stage of the forward relation above. Preserve input order and all shared fields across that stage; existentially quantify compatible intermediate states when reversing. Child success alone does not prove parent admission or answer correctness. The parent also retains surrounding branches that the child range does not cover. Parent complement targets its own U\R; it is not the union of child complements. Sequential reversal retains every compatible intermediate state. Failure prefixes may terminate before later stages. No partition of all source bytes is asserted.

## History

### 2026-10-29 · document

Source-bound partial models, epoch 2026-10-29. Subject: world/src/knowledge.rs at the manifest SHA-256. Observation: Question interpretation, anchor/target scoring, option selection and correction. Every relation below concerns this frozen source/configuration. Structural validation does not establish semantic correctness.

### 2026-10-29 · epoch-memory

2026-10-29: inspected the existing implementation before modeling. Preserved current behavior, including lossy decoding, index-sensitive ties and conditional updates. Existing witness tests distinguish active-scope learning from a wrong global witness. Counts represent distinct observations, not independent proofs. No source changes, performance gain, full-course result or autonomous combiner is claimed. AST remains explicitly absent; ranges bind source bytes only. Parser resources, panic/allocation behavior and complete inference semantics remain gaps.

### 2026-11-04 · epoch-memory

2026-11-04 (student-control): three changes in `answer`, all measured. (1) The question's things include its entities' noun compounds the lessons name, possessors and prepositional-modifier heads (on; PRAG_OFF=student-qents-mods): only the own-route aboutness filter widens. (2) A «when» question (the reader asks Role(Attr)) fills that role from a bare option with its «when:» attributes alone, else «when:<head>»; a time attribute meets only a time attribute. (3) A «which/what N» question in a core role: against a target whose entity in that role is N (same or a kind) and names the option (`Ent::names`), the role's conflict becomes a match (on; PRAG_OFF=wh-which); other options keep the conflict. Weight table unchanged (17 × 10 × 7). Ten courses 3 204 → 3 234 → 3 248 (odd 1 596 → 1 617, even 1 608 → 1 631); Wikipedia when 259 → 881 and which 65 → 200 of 1 106 and 214, articles-shuffled 263 and 64; subjects −12 net there (side effect of the time role).

2026-11-04: `gap_diag` / `gap_diag_option` sit inside this range after `answer`: a read-only diagnostic of an Answer that was not understood (categories in the parent note), recomputing the question's things as `answer` does without compound modifiers. It never changes weights, frames or the choice; it runs only under MOVA_GAPDIAG=1 from the course loop. Checked by gap_tests (not-about with its distance, verb-unknown, and the reached-gated control).

2026-11-04: `kinds` may add the loaded dictionary's genus of a question entity (`dict_kinds`: the student's own
reading of the first meaning, head noun only, then up to three WordNet kinds, kept when remembered) while the question's
own frames and things are coded (`q_side`), behind the opt-in `PRAG_VN=student-dict`. Off, the decision is
byte-identical to the previous revision (ten and twelve primary courses, three Wikipedia courses). On, it gained on
the ten courses with flat controls but raised the cross-lesson control on twelve others as much as the student
(docs/training.md section 32).

2026-11-04: `kinds` may also add a question entity's dictionary nyms (`dict_nyms`: the synonyms and form-of lemmas
of its noun entries in a layer with extra columns, the Wiktionary module), kept when remembered, behind the opt-in
`PRAG_VN=student-wikt` (`student-wikt-lack`: only nouns WordNet lacks; `student-wikt-mutual`: only nyms whose own
entries name the entity back). Off, or without such a layer, the decision is byte-identical to the base build. On,
the keys-removed and cross-lesson controls rose as much as the student (docs/training.md, «Building the dictionary»).

### 2026-11-04 · composition

2026-11-04 (student-rules): `answer` = `answer_with(.., rules)`. After the weighted choice, the rules compiled from remembered lines (`rule_base`, incremental over the frames, no persisted state) are applied: a rule fires when a condition frame matches a question frame other than the asked one (`match_pattern`: quoted patterns only against quoted frames, a «can» pattern only against an asked «can», verbs equal or synonyms both ways, polarity equal, every named entity of the pattern present, placeholders bound from the question only; content required: a content verb or named entity of an utterance, a content verb and a named entity of a situation). Its conclusion is instantiated (`conclusion`: «do it» = the matched action, carriers want/need/mean/ask/tell/expect/like/wish skipped before a complement) and an option fits when one of its frames has the conclusion's content verb (required), the same polarity and no conflicting core entity (`says_conclusion`). Exactly one fitting option is chosen, understood; the count still overrides. The learning path calls `answer_by_weights` (rules off), so the weights learn exactly as before. Invariants: bindings from the question only; a binding alone never decides (verb required); weight table NF 17, NA 10 unchanged. Complement: no decision with 0 or ≥ 2 fitting options; no rule from an unread link. Reverse: a rule decision names its rule, binding and conclusion in the trace, not the alternatives that did not fire. Measured on the ten courses: 3 248 → 3 260, paired +14/−2 (z 3.0), all within the 46 rule-decided rows; cross-lesson control 2 174 → 2 176. Test rule_tests (red controls: stated «can», another question, a binding-only option; the mutant without the verb check fails).

Verb gate, 2026-11-04 (student-verb). Model: for a role question (not the verb) whose gap frame has a content verb qv (not light, copular, an event predicate «happen / occur / take place», a carrier, a verb of saying or a VerbNet clause-taking verb by its preferred class: `verb_light_q`), a remembered frame m supports a frame that fills the gap (a bare option) only when `verb_agrees(qv, m.verb)`: the same verb, a WordNet synonym of the first two senses either way (`verb_same`), the same verb without its particle, or a hypernym within two levels either way; a light or verbless m counts (PRAG_VN=student-verb-strict: it does not); the quote route is exempt. A frame left out is never support: its score is kept as `OptionRead::assoc` and only orders options tied on support. The learning path (`answer_by_weights`) runs without the gate, so the weights learn as before. Invariants: the gate only removes supports (G1); it depends on the question's verb and the remembered frame's verb, not on the option text (G2); PRAG_OFF=student-verb gives byte-identical traces (G3); the weight table NF 17, NA 10 is unchanged. Complement: clause options (PRAG_VN=student-verb-all: ten courses −223), «what did X do» (verb asked), why / what next and frame questions are not gated. Reverse: a guess after the gate says only that no supporting frame stated the verb, not which association it dropped. Measured: Wikipedia verb control 4 066 → 2 017 of 5 117 kept the original answer; originals 5 154 → 5 252; ten courses 3 260 → 3 259 (+10/−11). Test verb_tests (gate off: the control is answered, understood; the mutant without `verb_agrees` fails «no remembered frame states sell»).

### 2026-11-05 · composition

Counted-against witnesses, 2026-11-05 (student-neg-counter). Model stated before code: a remembered frame m that
denies what an option frame o asserts — the same content verb (Match(Verb)), a shared core role (Match of agent,
patient, recipient or place), m negated, o not — is evidence against o, so its score is s' = −s when the weighted sum
s > 0, else s (`counter`, `contradicts`). Prediction: the 2026-11-05 L302 #9 failure («blinded by the sun» 7 against
«daylight NOT blind owl») flips to the right option; other decisions change only where such a witness was an option's
best. Invariants: a witness is never support for what it denies; a sum already ≤ 0 is unchanged; strength gates and
the weight table (NF 17, NA 10) are unchanged; PRAG_OFF=student-neg-counter restores the previous scores. Complement:
no counting against for a light verb, an irrealis-only difference, no shared core role, or the option's own denial of a
lesson's assertion (PRAG_VN=student-neg-both; reader misplaces options' «not», e.g. «It does not help; keep her
inside» → NOT keep). Reverse: a negative score names its witness; a positive one does not reveal that no denial
existed. Measured (CPU, en.bin bff784ac, bench seed 42, 26 primary courses, with hyphen-adj and conj-pp): full tier
tests 19 813 → 19 817 (odd −6, even +10), exam 5 793 → 5 811 (odd +26, even −8), controls red; L302 #9 right. Medium:
6 811 → 6 799, exam 1 932 → 1 944; directional scorer alone −18 (its own direct flips 9 lost / 8 gained, the rest the
learning path); both directions alone 42 / 29 direct (−4 total), with the reader fixes 6 785. Tests: a_contradicting_*
and learning_follows_the_sign_* (red with PRAG_OFF=student-neg-counter).
