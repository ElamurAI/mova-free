# Adjective used as a noun: the rich vs the natives

**Gist.** The modern English adjective does not inflect for number, case or gender: *a young man, young men*. When an adjective itself becomes the head of a noun phrase, there are two cases. **Partial** conversion: *the rich, the poor, the English, the unknown* — plural or abstract meaning, but no *-s* and no article *a*; it is still an adjective. **Full** conversion: *a native → natives, a criminal, the ancients, his betters, goods* — the word takes *-s* and the article *a*, and it is now a noun.

**Conditions and exceptions.**
- Part-of-speech test: can the word take plural *-s* and *a/an*? If yes — noun (NOUN). If not, but it can be intensified by an adverb (*the very rich*) — adjective (ADJ).
- Nationalities ending in a sibilant (*the English, the French, the Dutch, the Chinese*) — partial conversion (*an Englishman*, not *an English*); ending in other sounds — full (*an American, Americans; a Greek, Greeks*).
- Languages: *He speaks Chinese* — the language name as a noun (in EWT — PROPN or NOUN depending on context).
- The prop-word *one* takes the noun endings: *a good one, the red ones*.
- A noun before a noun (*gold watch, stone wall, silk thread*) stays a noun: you cannot add *very* to it (*very silk* is impossible).

**Examples.** *The **rich** get richer* (ADJ); *the **natives** of the island* (NOUN, NNS); *the **unknown*** (ADJ); *two **Americans*** (PROPN/NOUN with `Number=Plur`).

**In UD.** Partial: ADJ, JJ, `Degree=Pos`, in the role of `nsubj`, `obj` etc., with `det` *the*; has no `Number` feature. Full: NOUN, NN/NNS with `Number`. Noun modifier: NOUN with the `compound` relation.

**Sources.** Sweet NEG I §107, §179–180 (text-1, p. 69, 96–97: full and partial conversion, prop-word *one*), §106 (text-1, p. 69: *silk thread, gold watch* — nouns), §1035 (text-1, p. 355: the adjective is uninflected); Whitney §144 (text-1, p. 78–79: *the poor, the dead* without *-s*; *Americans, nobles, the ancients* with *-s*; *the English* vs *Americans*), §76, §196 (text-1, p. 51, 104); Santorini 1990, §4.1, p. 13 (*The (very) rich/JJ*; *Little good/NN*); Jespersen MEG II 11.3₁–11.5₈ (text-2, p. 305–313: *the poor, the rich* without *-s*; *the English* vs *Germans*); Kruisinga II.3 §§1789–1803, 1850 (text-4, p. 118–127, 153: *the rich* — partial conversion without plural; *drunks, illustrateds* — full); Mätzner I, p. 270–272 (text-1, p. 288–290: *Italians, natives, betters* with *-s*; *the English, the poor* without).
