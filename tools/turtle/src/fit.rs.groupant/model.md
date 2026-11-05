# mova-turtle-fit

## Descriptant

The fit check of gap questions' wrong options. A candidate is refused when it is a near-synonym of the answer (a shared WordNet synset, one a hypernym of the other at any depth, or one's meaning naming the other in the layer or the pupils' dictionary) or when every real form of it could stand in the gap's slot; otherwise it is shown in a form that breaks the slot. The slot is read by Mova's annotator: the answer's PTB tag, whether it governs an object, the determiner before it. Source: `tools/turtle/src/fit.rs`.

## Forward approximant

judge(answer, cand, slot, sentence words) = Err if cand = answer, cand in sentence, or kin(answer, cand); Ok(cand) if a noun slot's determiner refuses it (a/an spelling, singular/plural); Ok(cand) if a base verb slot has an object and WordNet gives cand no transitive frame; Ok(f) for the first f in forms_of(cand) (AGID ∩ UD forms, VBZ, VBD, VBN first) not in the sentence whose lexicon tags are nonempty and disjoint from group(slot.tag); else Err("every form … can stand"). group: VB/VBP → {VB,VBP,VBD,VBG,MD}; VBZ → {VBZ,VBD,MD}; VBD → {VBD,VBP,VBZ,MD}; VBN → {VBN,VBD,JJ}; VBG → {VBG,NN,NNS,JJ}; any other tag → every tag but VBD, VBZ, MD.

## Invariant

An accepted form never carries a tag of the slot's group in Mova's lexicon (or breaks the determiner or the verb frame); a refused near-synonym is refused in every form; the result depends only on the lexicons, the slot and the words.

## Cost

Lex::load: one parse of the OEWN entry files plus a hypernym walk per layer word (seconds, once per making). judge: O(forms × tags) lexicon lookups plus set intersections of the words' synsets and hypernyms.

## Composition

Parent: `turtle`. Used by make::wrong_options; slot_of uses en::annotate::Annotator; Lex::load uses world::wordnet and world::dictstore.

## behavior_complement

Does not judge meaning: no meaning-frame rule (a noun of another kind is not a sound break; tried and dropped when «do you know the teacher?» passed for «… the time?»). Does not know idioms, ellipsis or a form the lexicon lacks; an unknown form is never accepted. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From an accepted form and the slot, the rule that accepted it is recovered by rerunning judge; the refusal reasons are journaled by kind in made.jsonl. The candidate lemma behind a form is one of its lexicon lemmas; no unique inverse is claimed.

## tests

### Checks

Red controls kept in the tests: only/just and mum/parent refused as near-synonyms, now/out/just and go/say as words that fit the gap.
These are finite fixtures; no teacher was called; not a proof.

## History

### 2026-11-04 · behavior_complement

Does not judge meaning: no meaning-frame rule (a noun of another kind is not a sound break; tried and dropped on 2026-11-04 when «do you know the teacher?» passed for «… the time?»). Does not know idioms, ellipsis or a form the lexicon lacks; an unknown form is never accepted. Evidence status: source-derived model with finite checks, not a proof.

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source. Measured on the commands dictionary layer of the repository (main of 2026-11-04 evening): 171 questions in 8 lessons, all passing turtle check, twice identically; 5 gaps dropped; refusals: fits the gap 6765, in the sentence 435, WordNet near-synonyms 855, meanings 1204 (candidates are judged until twelve are accepted, so one candidate is counted once per gap). Non-verb slots are broken by few forms of that layer (went, happens, gave, comes, asks, does about 48 times each out of 513 options): sound but easy; a richer layer gives more variety.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/turtle/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin and Open English WordNet under $MOVA_DATA/raw/en-oewn/json (14 unit + 2 English + 2 gap tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): check without the school's rule; held-out course names read; the deputy vocabulary without «command»; the fit check switched off (4 unit and both real-data gap tests red: «just» for «only», «dad» for «mum»); the sweep reading held-out courses; a teacher step after a sweep that closed lessons.
