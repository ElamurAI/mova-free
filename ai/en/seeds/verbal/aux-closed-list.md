# Auxiliary verbs — a closed list

**Gist.** An auxiliary verb carries no meaning of its own but adds tense, aspect, voice or modality to the main verb. English has few such words, and the list is closed: *be, have, do*, passive *get* and the modals *can, could, may, might, shall, should, will, would, must, ought, need, dare*. Any other word in the auxiliary slot is an annotation error.

**Conditions and exceptions.** The particle *to* before an infinitive is not an auxiliary (in UD it is `mark`). *Let* (Let us go) is a lexical verb with an object, not an "auxiliary of the imperative mood" as old grammars (Brown) wrote. *Become, seem, keep* are not auxiliaries either: they take `xcomp`. *Have to, be going to, used to* are separate verbs with an infinitive (see `semi-modals.md`).

**Examples.** *Reagan has died.* — *Reagan might have been lying.* — *The book got stolen.* — but *I tried **to** finish it* (to — mark).

**In UD.** Relations `aux` and `aux:pass`: UPOS `AUX`, lemma from the UD 2.18 registry list (`en/data/ud-registry-en.tsv`, `aux` rows). `cop` is also always `AUX` (only *be*, see `copula-be-only.md`).

**Sources.** https://universaldependencies.org/en/dep/aux_.html, https://universaldependencies.org/en/pos/AUX_.html; UD auxiliary registry (`ud-registry-en.tsv`); UD 2.18 validator, test `rel-upos-aux` (`udtools/level3.py`); Brown 1851, Part II, Ch. VI "The Conjugation of Verbs", definition of auxiliary and Obs. 4 (); Brown 1851, Rule XIX, Obs. 10 (let is not an auxiliary).

```rule
rule: en.verbal.aux-lemma
what: an auxiliary (aux, aux:pass) only from a closed list of lemmas
match: a[rel=aux|aux:pass]
require: a[lemma=be|have|do|get|will|would|shall|should|can|could|may|might|must|ought|need|dare]
severity: error
source: https://universaldependencies.org/en/pos/AUX_.html; UD 2.18 aux registry
```

```rule
rule: en.verbal.aux-upos
what: aux, aux:pass and cop have UPOS AUX
match: a[rel=aux|aux:pass|cop]
require: a[upos=AUX]
severity: error
source: UD 2.18 validator rel-upos-aux, rel-upos-cop; https://universaldependencies.org/en/pos/AUX_.html
```
