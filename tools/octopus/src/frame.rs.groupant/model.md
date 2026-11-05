# octopus-question-frame

## model

# Deputy question frame — source-bound model

## Descriptant

`frame.rs` reads the deputy's own role frame from the native UD tree of one sentence: the mood (imperative,
modal_request, polar, wh, declarative), the subject node and the verb node. `world::ans::parse_question` reads story
wh-questions token by token and leaves the verb slot empty for "Is Crab running normally?" or "Could you start Eel?"
(«subj crab running normally | verb -»); the deputy needs the roles, so it reads them from the parse.

## Forward

root r = the node with head 0. Wh if one of the first two words is a wh-word: verb = r, subject = nsubj(r).
ModalRequest if the first word is an AUX modal (can, could, would, will) and the second is «you»: subject = «you»,
verb = the VERB «you» hangs on, else r. Polar if the first word is any other AUX: verb = r and subject = nsubj(r) when
r is predicate-like (VERB, ADJ, -ing NOUN) with a subject; for a present-tense question whose root is a NOUN/PROPN
(the copula's subject read as root), repaired: verb = the first later acl/amod/xcomp dependent that is predicate-like
(«Is the documenter running?», «Is Eel busy?»), or the -ing root itself with its preceding compound as subject
(«Is the Nautilus running?»); otherwise verb = r and subject = nsubj(r) or the preceding compound («Is Opus busy?»).
Imperative if r is a bare VB without a subject; Declarative otherwise. `text` = `mood[ repaired] | subj «phrase» |
verb lemma`.

## Invariant

subject ≠ verb when both exist; a repaired frame names both; a past-tense polar question (first AUX tagged VBD) is
never repaired, so its reading stays as parsed and the deputy keeps refusing history questions.

## Cost

O(n) passes over at most 160 words; no model call.

## Composition

`lib.rs` records the frame in every observation (`deputy_frame`). In `resolve`, a repaired polar frame removes the
subject noun from the predicates and makes the recovered verb the checked clause head; a polar frame also marks
inversion when the auxiliary hangs on a name inside the subject. The world frame stays recorded as
`native_question_frame` for comparison.

## behavior_complement

Not read: embedded questions («whether Eel is running» stays a complement clause of the request), coordination of
several predicates, passive voice, questions without inversion («Eel is running?»), negation (refused later by the
resolver), tense beyond the VBD check, and any repair other than subject-as-root. A frame is no proof that the parse
is right; it constrains only which node may serve as the request head.

## reverse_reconstruction

Many sentences share one frame (word order, determiners, adverbs, politeness words are dropped), so from a frame
only the compatible set of sentences with that mood, subject phrase and verb lemma is recoverable, never the text.

## checks

### Checks

Unit tests on the native parses (en.bin 0.4) of real refused prompts: polar verb slot, subject-as-root repair (documenter, Eel busy, Nautilus compound), compound subject, modal request; negative controls: past-tense copula and noun-predicate polar not repaired, declarative and imperative moods. Disabling the repair turns the unit and the native deputy test red (restored).

## History

### 2026-11-04 · checks

Heading `### Checks (2026-11-04)` stays in checks as `### Checks`.

### 2026-11-05 · document

#### Epoch 2026-11-05: imperative read as a noun phrase

Observation: en.bin 2483516a (and the model before it, bff784ac) parse "Show Crab's recent reports." as one noun
phrase: «show» NOUN compound of «crab», «reports» root; the deputy found no request head and refused
(`missing_context`). Forward: `repair_imperative(w, verbs)` applies only when the sentence has no VERB, AUX or nsubj,
the first word is NOUN/NN in base form (form = lemma), its lemma is a configured request predicate, and no
det/amod/nummod/case/nmod:poss hangs on it; then word 0 becomes VERB/VB root and the old root its `obj`; all other
heads are kept. `read_repairing` reads the frame over the repaired words and marks it repaired
(`imperative repaired | subj «» | verb show`); the caller keeps the native words in the observation. A compound subject
under a polar predicate now includes nested premodifiers («is opus dolphin busy»: «opus» → «dolphin» → «busy» in
2483516a reads as subj «opus dolphin»). Invariant: the repaired words form one tree with the same forms (word 0 is the
root, so no cycle can arise). behavior_complement: no repair when any verb is present, for plural or determined first
words («The show reports.» stays a statement), for words outside the configured request predicates, or for an
imperative later in the sentence. reverse_reconstruction: the native parse is recoverable from the observation; from
the repaired words alone the native attachment of word 0 and the old root's relation are not.

### 2026-11-05 · checks

#### Checks (2026-11-05)

`imperative_read_as_a_noun_phrase_is_repaired` and `noun_phrases_and_clauses_are_not_read_as_imperatives` (native
parse of en.bin 2483516a; negative controls: a non-predicate first word, a determiner on it, a plural, a verb in the
sentence, a subject); nested compound subject. Native deputy test with MOVA_OCTOPUS_TEST_MODEL=en.bin 2483516a: "Show
Crab's (recent) reports." plan `reports` with frame `imperative repaired`; the frame test compares roles with or
without the repair marker, since whether a polar frame needs its repair depends on the model. Disabling the repair
turns one unit and two native tests red (restored). Two identical runs.
