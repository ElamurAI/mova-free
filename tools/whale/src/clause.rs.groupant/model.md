# mova-whale-clause

## Descriptant

Whole clauses from Mova's UD tree of a paper sentence, restated in the describer's perspective: clause splitting (conj, parataxis, subject relative clauses), shared subjects and auxiliaries, we/our -> the authors/their, bracketed spans left out, past passives with a by-agent turned active, and a verbatim cap enforced by moving adverbial clauses and obliques (fronted or put last) before leaving out optional modifiers. Source: `tools/whale/src/clause.rs`.

## Forward approximant

clauses(tree, toks, cap): heads = root + conj (of a clause head, fixed point) + parataxis with subject + acl:relcl with a relative-pronoun subject and a nominal antecedent; per head: subtree minus other heads, brackets removed; conj without subject takes the governor's subject (and its auxiliaries for VBN/VBG); relcl replaces the pronoun with the antecedent phrase; main clause with its own non-'to' marker refused; activize (was/were + VBN == past form + by-agent); loop: build (perspective, own cc removed, edge trimming, doubled punctuation, lowered opener) -> verbatim run in alphanumeric source tokens; if > cap choose the step with key (reaches cap?, move before drop, removed size, rank advmod<advcl<obl<appos<acl<amod<nmod, index), at most 8 steps; accept 4..60 words, finite verb (or activized), head kept, subject kept, no wh/cc/own-marker opener, letters >= 70%, balanced quotes.

## Invariant

Every word is a source token of the sentence except the perspective words and inserted commas; negation is never left out; the longest stretch shared word for word with the source sentence is at most cap alphanumeric tokens; deterministic (index order, fixed tie keys).

## Cost

Per clause head: at most 8 steps, each trying every optional node (O(n) trials of an O(n) build and an O(n) run count), so O(8 n^2) per head; sentences <= 80 words; about 0.2 s of a 1 s paper read (release).

## Composition

Parent: whale-workers; called by reader.rs for every admitted sentence, for table/figure captions restated as «Table N shows …», and for comma/semicolon segments when the whole sentence yields nothing; output consumed by compose.rs.

## behavior_complement

Not done: paraphrase beyond these operations, present or modal passives, object relative clauses, verb agreement repair, pronoun resolution beyond the authors' we, meaning checks of a dropped modifier (a dropped obligatory oblique can leave 'relies solely'), repairing Mova's wrong roots (long sentences parsed with a noun root yield nothing and become teaching material). Counterexample: «For instance, the participant role doesn't make a difference would be labeled as ARG0 …» after a misparse. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Each clause names its kind, its verbatim run and how many modifiers were dropped; with the source sentence and its tree one recovers which tokens were kept, moved or dropped (the set difference of source and clause tokens, plus order). Dropped content is not recoverable from the clause alone; no unique inverse is claimed.

## History

### 2026-11-04 · Cost

Per clause head: at most 8 steps, each trying every optional node (O(n) trials of an O(n) build and an O(n) run count), so O(8 n^2) per head; sentences <= 80 words; about 0.2 s of a 1 s paper read (2026-11-04, release).

### 2026-11-04 · behavior_complement

Not done: paraphrase beyond these operations, present or modal passives, object relative clauses, verb agreement repair, pronoun resolution beyond the authors' we, meaning checks of a dropped modifier (a dropped obligatory oblique can leave 'relies solely'), repairing Mova's wrong roots (long sentences parsed with a noun root yield nothing and become teaching material). Counterexample 2026-11-04: «For instance, the participant role doesn't make a difference would be labeled as ARG0 …» after a misparse. Evidence status: source-derived model with finite checks, not a proof.

### 2026-11-04 · document

#### Epoch 2026-11-04 (more of each sentence restated)

Prospective model: most words lost inside restated sentences come from clauses the parse attached wrongly, not from the cap; restating cut-off parts with their own anchors adds words without new vocabulary. Added: «Note that X» -> X; a coordinated base form under a present plural governor counts as finite (the shared subject); a subordinate root with its main clause coordinated under it is attached after that clause (`fragment`); an uncovered oblique, adverbial clause or adverb phrase of a realized head is restated with the head's core (`split`, refused under a clausal complement); an uncovered apposition or past-participle modifier becomes «N is/are M» (`predication`); `question` restates a question as «The paper asks: …?». A block moved to the end now goes before the closing punctuation (it used to follow the period). Observed: `split` and `predication` fire rarely (a handful per paper); most loss stays in obliques and nominal modifiers of embedded verbs and in rejected roots. Invariants kept: every word a source token or perspective/frame word, the cap, negation never dropped. Remaining counterexamples: fragments like «This means, for example.» from segment parses, agreement after a shared subject («This also standardise …»).

#### Epoch 2026-11-04 (correlatives, grammar gate)

A `correlate` (an advcl with its own «but»/«and» and no subordinator or subject) is the second half of a coordination; «not only» of a head is left out together, never «only» alone. `grammatical` re-parses a written sentence and refuses fragment marks (bare subordinated -ing at the end, a participle after a determiner read as the verb, a plural noun read as the verb, no finite verb by parse, tag or dictionary, a closing function word). Mova's parse of written scientific sentences is unreliable, so the gate looks for marks, not a perfect parse; on the ten papers it refuses 1-6 clauses per paper, most of them real fragments, some sound sentences the parser misreads.

### 2026-11-05 · document

#### Epoch 2026-11-05 (Note that, label-independent)

The 0.5 model (2483516a) labels the clause of "Note that this is a representative example …" `obl` instead of `ccomp`, so the reader dropped it. The noted clause is now recognised by its shape: a dependent of the imperative «note» with the marker «that» that is `ccomp` or a predicate with its own subject. Red control: the ccomp-only rule fails the reader test.
