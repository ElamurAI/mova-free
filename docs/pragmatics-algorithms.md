# Pragmatics: algorithms from the literature, for Mova (2026-10-03)

What the library holds at the level of algorithms, and how each piece fits a deterministic, explained Rust model.
Read for the students' dialogue layer (`world/src/talk.rs`) and for the pragmatics module (`prag`). Full texts were
read unless marked *(summary only)*.

## Speech acts and dialogue acts

- **ISO 24617-2, 2nd edition** (Bunt et al. 2020; function list from Bunt, Petukhova & Fang 2017, 56 functions).
  Dimensions: Task, Auto- and Allo-Feedback, Turn and Time Management, Discourse Structuring, Own and Partner
  Communication Management, Social Obligations Management (SOM), Contact Management. General-purpose functions
  (Inform → Agreement/Disagreement → Correction; Answer → Confirm/Disconfirm; Set-, Propositional-, Check-,
  Choice-Question; Offer, Promise, Accept/Decline Request/Suggest; Request, Instruct, Suggest). SOM: Init/Return
  Greeting, Self-Introduction, Apology / Accept Apology, Thanking / Accept Thanking, Init/Return Goodbye; social-talk
  plug-in: politeness question ("How are you?"), opening and closing politeness statements, farewell wish.
  Feedback on five levels (attention, perception, interpretation, evaluation, execution) × positive/negative.
  Qualifiers: conditionality ("Can you tell me…" = conditional request), certainty, sentiment. Dependence: an
  answer depends on its question; feedback depends on a segment. **Mova:** enums `Dimension`, `CommFunction` with
  the hierarchy as a parent table; several (dimension, function) pairs per segment; SOM, turn and feedback by cues.
- **Indirect speech acts** (Sarathy et al. 2020; Roque et al. 2020). Schema `(utterance, literal meaning, contexts
  c₁…cₙ, intended meanings i₁…iₙ)`; an ISA in cₓ iff literal ≠ intended. Context features: location, task,
  speaker and hearer role, co-present item, interaction history. Reasoning (Answer Set Programming): the surface form
  gives the default act (question → ask, command → request, statement → assert); "Request A" is promoted only when
  the speaker is shown to want L to do A, L is mutually known capable, willing and obligated (Brown 1980) —
  asymmetric burden of proof. Evidence rules: the current task requires A; roles leader/follower delegate; earlier
  requests answered by actions; co-presence shows capability. Human agreement on the schemas is low (44%): these
  are hard, context-driven cases. **Mova:** a rule engine with closed-world defaults; the proof trace is the
  explanation; the schemas are minimal-pair tests.

## Implicature and presupposition

- **Rational Speech Acts** (Monroe & Potts 2015 *(summary only)*; recursion from Anderson 2019/2021):
  L0(w|u) ∝ [[u]](w)·P(w); S1(u|w) ∝ exp(α(log L0(w|u) − C(u))); L1(w|u) ∝ S1(u|w)·P(w); a common ground as the
  prior, updated over turns. Learned RSA (log-linear S0 with attribute features) beats plain RSA on TUNA (accuracy
  25.3% vs 2.2%, Dice .777 vs .386). **Mova:** α a small integer and costs in base-2 units make S1 ∝ L0^α·2^(−αC)
  exact in rationals; fixed-id order and lowest-id tie-breaking; the L0/S1/L1 tables are the "why". For small closed
  sets: scalar alternatives, referring expressions.
- **Rule-based Gricean implicature** (Tatu & Moldovan 2012, LREC): logical forms + an abductive reasoner with 63
  hand-written maxim axioms; Twitter conversations P 75.8 / R 65.8 / F 70.4. Axioms that map directly to rules:
  Quantity — a weaker scalar item implicates "not the stronger one" (some < most < all, warm < hot, possible <
  certain); Manner — coordinated events are ordered (e₁ before e₂); Quality — a question implicates that the speaker
  does not know, an exclamation a strong feeling, a blatantly false statement irony; Relation — the utterance and the
  context share a predicate.
- **Adverbial presupposition triggers** (Cianflone et al. 2018): too, again, also, still, yet; the cue is a repeated
  event or entity in the preceding context ("continue" licenses "still"); a bigram logistic regression is near chance
  except for *too* and *yet*, an attention model reaches 76% on PTB. **Mova:** a trigger asserts its presupposition
  into the common ground and is flagged when no same-predicate event or coreferent entity occurs in the previous
  50 tokens; the same check licenses the trigger in generation.
- The implicature survey (Mallepally et al. 2024) lists datasets only (CIRCA, GRICE, IMPPRES, …): test sets, no
  method.

## Discourse: questions under discussion

- **QUD** (Roberts 2012): q-alt(α) — the propositions with the wh-words varied ({p, ¬p} for a yes/no question);
  partial and complete answers; a stack of accepted, unanswered questions, ordered by the subquestion constraint;
  push on accepted questions, pop on answered or unanswerable ones; a move is relevant iff it partially answers the
  top question or is a subquestion of a strategy for it; congruence — the focus alternatives equal q-alt.
- **QUDeval** (Wu et al. 2023): an edge (question, anchor S_k, answer S_a), k < a; criteria: answer compatibility,
  givenness (the question may use only concepts of S₁…S_k — a rule on lemmatised content words: new in S_a = answer
  leakage, new everywhere = hallucination; macro-F1 0.37, better than GPT zero-shot 0.33), anchor relevance.
- **Riester et al. 2021**: discourse as a tree whose inner nodes are implicit QUDs and leaves the foci of
  assertions; labels topic, contrastive topic, focus, non-at-issue. **Mova:** a QUD stack with integer ids — attach a
  sentence to the deepest congruent open question, otherwise accommodate a subquestion built only from given lemmas,
  pop the answered; the stack is the explanation.

## Social meaning

- **Politeness** (Danescu-Niculescu-Mizil et al. 2013; data CC BY-NC-SA — the strategy list as facts): 20 strategies
  detected by regular expressions over dependency parses and lexicons, each with its mean politeness score:
  gratitude +0.87, deference +0.78, indirect "by the way" +0.63, "please" inside +0.49, counterfactual modal
  ("could/would you") +0.47, greeting +0.43, apologizing +0.36, hedges +0.14, positive lexicon +0.12, first person
  start +0.12, indicative modal ("can/will you") +0.09, first person plural and first person +0.08, second person
  +0.05; negative lexicon −0.13, direct question −0.27, "please" at the start −0.30, second person start −0.30,
  factuality ("in fact") −0.38, direct start ("So/Then/And/But") −0.43. SVM with these features: Wikipedia 83.8%
  (human 86.7%), Stack Exchange 78.2%, across domains 67.5–75.4%. **Mova:** 20 boolean detectors on our trees with
  integer weights (score × 100), the fired strategies listed with their signs.
- **Sarcasm by incongruity** (Joshi 2017, abstract; features from Joshi, Sharma & Bhattacharyya 2015): explicit —
  polar words of both signs ("I love being ignored"): sentiment flips, the longest same-polarity run, counts of
  positive and negative words, overall polarity; implicit — a positive verb with a negative situation phrase
  ("getting late for work"; phrases bootstrapped after Riloff et al. 2013); pragmatic cues: emoticons, laughter,
  punctuation, capitals. Tweets F .842 (lexical) → .888 (all). **Mova:** integer counters and a learned list of
  negative situations; the matched pair is the explanation; it meets the absurdity matrix's humor trigger.

## What goes first

1. The family's dialogue layer: the rest of SOM (goodbye, politeness question, opening/closing politeness), the 20
   politeness detectors (Dev answers impolite requests too, but notices), Tatu's Quality axiom (a question means the
   asker does not know — the member answers what it knows and says what it does not).
2. `prag`: scalar implicatures (Quantity scales) and presupposition triggers with the 50-token licensing check,
   both explained; RSA for small closed sets later.
3. A QUD stack for the story reader (which question does this sentence answer).

Gaps (requested for the library): Riester, Brunetti & De Kuthy 2018 (QUD annotation guidelines), the full text of
Monroe & Potts 2015.

## Done (2026-10-03)

- `world/src/talk.rs`: the rest of SOM (goodbye, politeness question answered from the member's recent memories,
  opening and closing politeness) and the 20 politeness strategies as integer-weighted detectors; a blunt request
  is done but Dev notices it with the strategies that fired. Tests: `tests03::social_talk_and_politeness`.
- `prag/src/infer.rs`, `prag infer <model> <text>`: scalar implicatures on 17 Horn scales (blocked by a stronger item,
  negation, cancellation "if not all / in fact all / or even") and presuppositions of again, still, also, too (not
  the excess "too heavy"), stop, quit, continue, keep, know, realize, regret, forget — licensed by the same predicate
  in the previous 50 tokens or marked as new information to accommodate; every inference says why. Tests with
  negative controls. Parser slips seen on the way: "The fox ate again" — *ate* tagged NOUN; "knows that the door is
  open" — the clause as xcomp (the factive rule accepts an xcomp with its own subject or "that").
- `world/src/talk.rs` — indirect requests (Sarathy et al. 2020 after Brown 1980): "Can/Could/Would you (please)
  X?" is an ability question in form; Dev reads it as the request X only when the speaker wants it (a modal
  question), the family is capable (X is one of its commands), willing now (state-moving commands wait for the end
  of training) and obligated (it works for its clients) — and says so; otherwise the direct reading: an answer about
  ability with what it can do. Tests: `tests03::indirect_requests_need_their_preconditions`.
- `prag/src/rsa.rs`, `prag rsa scalar|reference`: Rational Speech Acts over small closed sets — L0 ∝ [[u]]·P,
  S1 ∝ L0^α·2^(−αC), L1 ∝ S1·P — in exact fractions (u128 with gcd reduction, overflow panics), given order, ties to
  the lowest index; `explain` prints the L0/S1/L1 rows as fractions and percentages. Scalar game: L1("some") =
  (0, 3/4, 1/4) over none / some-not-all / all — the implicature "not all"; a cost of 2 bits on "all" weakens it to
  3/5, and α=2 then to 5/9. Reference game of Frank & Goodman: L1("blue") = 3/5 blue square vs 2/5 blue circle,
  L1("square") = 3/5 blue square vs 2/5 green square. Negative control: utterances true of every world give L1 = the
  prior (1/6, 1/3, 1/2). Tests: `rsa::tests` (7, all pass), hand computations in the doc comments.
- `prag/src/infer.rs` `maxims`: more axioms of Tatu & Moldovan 2012 — Quality (a question → the speaker does not
  know the questioned part: what the wh-word stands for, or whether the rest of a yes/no question holds; an
  exclamation → a strong feeling) and Manner (two event verbs coordinated by "and" → e₁ before e₂; not with stative
  verbs, not with after/before/while/meanwhile or a "then" off the second verb). `prag/src/irony.rs`: sarcasm
  candidates by incongruity (Joshi, Sharma & Bhattacharyya 2015; Riloff et al. 2013) — a built-in polarity lexicon
  (~65 positive, ~70 negative lemmas; negation flips the sign) with flips, longest run and overall polarity; explicit
  incongruity (a positive word governs or precedes a negative one, no "but/although" between, +2), implicit (love,
  enjoy, adore, like, appreciate, "can't wait" with one of 45 seed negative situations in the complement, ended
  situations like "the rain stopped" excluded, +3), pragmatic cues (+1 each: emoticons, laughter, "!!", "?!", ALL
  CAPS, "yeah right", "oh great", "just what I needed"); a candidate needs an incongruity and score ≥ 2, the
  incongruent pair is the explanation; it meets the absurdity matrix's humor trigger in `world/src/domains.rs`.
  `prag infer` prints both per sentence (the splitter keeps "!!"/"?!" and a trailing emoticon with their sentence).
  Tests with negative controls (plain statements, "or", stative verbs, reordering words; "I love my mother", "The
  rain stopped", "I don't like being ignored", a mixed opinion with "but", cues alone, "like" as a preposition).
- `world/src/qud.rs`, `world qud "<text>"`: a QUD stack for stories and dialogues (Roberts 2012; Riester et al. 2021;
  QUDeval givenness). Explicit questions are pushed with their wh-type and content lemmas; an assertion attaches to
  the deepest open question with a shared lemma and an answer of the right type (animate subject/object for who,
  place oblique or adverb for where, time word or obl:tmod for when, because/advcl for why, manner adverb or by/with
  for how, the same predicate for yes/no); a complete answer pops, a partial one (another predicate, an indefinite
  filler, a hedge) does not. Otherwise an implicit QUD is accommodated from given lemmas only ("What did the fox
  do?", "What was the fox like?", "What happened next?"); a candidate that needs a lemma new in the answer ("Who is
  Reynard?") is rejected and reported as answer leakage. Pronouns resolve to the last subject entity. Tests on
  hand-built trees (no model), with a negative control.
