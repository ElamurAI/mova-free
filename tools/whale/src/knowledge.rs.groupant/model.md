# mova-whale-knowledge

## Descriptant

A Whale's retained knowledge: typed facts with provenance (stands_for, is_a, means, does, reads_as) that Mova reads out of its terminal's answers or out of the paper, kept in the Whale's private state across rounds and papers; the set of questions ever asked; answer texts kept privately only for the anti-padding gate. Source: `tools/whale/src/knowledge.rs`.

## Forward approximant

Store::open(dir): items.jsonl, asked.jsonl, answers/. retain appends an Item (fsync); mark_asked appends {key, state, ...}; keep_answer writes answers/SHA.txt once. question_key: term:T (once per Whale), sentence/section:paper:sha. term_facts(answer): acronym expansion by initials, then per answer sentence (<= 4) a root clause whose subject mentions the term: copula -> is_a NP, mean/refer/denote/stand -> means NP, verb + obj -> does (lemma, NP); NP = det/amod/compound/nummod/poss + head (+ one nmod) <= 5 words; a means repeating the expansion is dropped. reading_facts(source, answer): subject/verb/object of answer clauses all of whose words occur in the source. explain(): «T» is short for X / I understand «T» as X / I take «T» to mean X / As I understand it, «T» does-3sg X.

## Invariant

Answer text is never returned for writing, only facts with object phrases <= 5 words (below the 6-gram overlap gate); appends are fsynced and never rewritten; deterministic.

## Cost

One parse per answer sentence (<= 4) per answer; store load linear in its lines.

## Composition

Parent: whale-workers; written by learn.rs (answers, imports), read by compose.rs (Known::of) and batch.rs (candidates).

## behavior_complement

Not done: resolving which sense of a term a paper uses, merging or retracting facts, sharing knowledge between Whales, trusting an answer (facts are evidence with provenance, not truth), keeping facts from an answer whose subject does not name the term. Vague objects occur («I understand «EWT» as a collection.» import of live round 1). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Each item names its question, the terminal question id, the answer's SHA-256 and the round; with answers/SHA.txt the full answer is recovered for review. The facts do not determine the answer text (many answers give the same facts).

## History

### 2026-11-04 · behavior_complement

Not done: resolving which sense of a term a paper uses, merging or retracting facts, sharing knowledge between Whales, trusting an answer (facts are evidence with provenance, not truth), keeping facts from an answer whose subject does not name the term. Vague objects occur («I understand «EWT» as a collection.», 2026-11-04 import of live round 1). Evidence status: source-derived model with finite checks, not a proof.

### 2026-11-04 · document

#### Epoch 2026-11-04 (answer reader 2)

`reading_facts` reads every clause of every answer sentence (root, conj, ccomp, advcl, parataxis), shares a governor's subject with a coordinated verb, keeps auxiliaries and negation in the verb group (the earlier reader dropped them: «A system can identify errors» became «A system identify errors»), takes an «on/for … NP» complement when there is no object, and refuses pronoun and meta subjects. `READER` names the version; learners re-read kept answers once per version. Measured 2026-11-04 on 61-70 kept sentence answers per Whale: no new facts, because the terminal paraphrases with words outside the source sentence, which the all-words-in-the-source rule rightly refuses; readings stay 0-10 words per paper.

#### Epoch 2026-11-04 (normalized terms, bare facts, batched answers)

`norm_term` (case-fold, plural acronyms and nouns singular) keys questions and known terms; legacy asked keys are aliased under their normal form at load. `bare` marks category-only facts for review. `split_answer` splits a numbered batched answer; `paper_words` strips the «In the paper's words:» frame. `Store::review` records flagged facts without a pod.
