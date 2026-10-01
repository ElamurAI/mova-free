# LinES — known annotation errors

**Gist.** LinES was obtained by automatic conversion from a non-UD scheme with partial review. So alongside the customs there are conversion errors: features that contradict the form itself, and inconsistent lemmas. The converter into `mova` does not fix them. They are candidates for suspicions and edits.

**Examples and numbers.**
- **Possessive *her*, *its* with personal-pronoun features**: in `nmod:poss` *her* has `Case=Acc` without `Poss=Yes` (32), *its* — `Case=Nom` without `Poss=Yes` (22). Correct is `Case=Gen|Poss=Yes`.
- **NOUN without `Number`** — 469, although the XPOS `SG-NOM`/`PL-NOM` sets the number (*garden*, *smile*, *sir*, *jews*).
- **Two norms of possessive lemmas** in one treebank: *his* → *his* 670 and *his* → *he* 136; *my* → *my* 364 and *my* → *I* 45; *our* → *we* 10; *myself* → *I* 6.
- **`Polarity=Neg` on *not*** is missing in 98 of 872.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`): 24 of 766.
- **Infinitive root without a subject and without `Mood=Imp`** (`tb.en.imp-mood`): 28 of 813. Probably imperatives, which LinES annotates as `Mood=Imp` elsewhere (130).
- **`dislocated` — 131**, 18 times more than in EWT (`old-structures.md`). Some are probably `nsubj`/`obj` under fronting, but without checking this is only a suspicion.

**In UD.** LinES is better merged only after the converter: otherwise the model will learn two lemma norms and "features only from form". The feature errors (54) do not affect parsing.

**Sources.** `data/raw/ud-docs/docs/treebanks/en_lines/index.md` («automatically converted to UD, with some manual corrections of the conversion»); Ahrenberg 2015, `W15-2103` (abstract).
