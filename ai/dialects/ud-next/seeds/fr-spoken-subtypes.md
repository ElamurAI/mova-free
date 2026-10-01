# French subtypes for spoken language (another language, a borrowing candidate)

**Gist.** After 2.18, French documents three subtypes for spoken treebanks:
- `discourse:filler` — a filled pause (*euh* ≈ *um*);
- `discourse:tag` — a tag question (*…, hein ?*, *…, non ?*, *tu t'en souviens ?*);
- `conj:reform` — a reformulation of an already completed utterance: *une maison en banlieue, la banlieue parisienne*. An incomplete utterance is, as before, `reparandum`.

**Level.** Language-specific (fr). For English this is a candidate for borrowing into `mova`. A related open universal discussion is #1290 (the UniDive spoken-language working group: parataxis, tag questions, backchannels).

**Now (snapshot 24.09.2026).** New files `docs/_fr/dep/discourse-filler.md`, `docs/_fr/dep/discourse-tag.md`, `docs/_fr/dep/conj-reform.md` (not in 2.18). Changed `docs/_fr/dep/dislocated-mod.md` (only a typo in an example).

**Evidence.** `diff -rq`, «Only in docs/_fr/dep». In `tools/data/deprels.json` the permitted relations for fr changed (see `validator-data.md`).

**What it means for English annotation.**
- EWT has no such distinction. Fillers (*um*, *uh*) are INTJ with `discourse`, together with *oh*, *yes*, *wow*: INTJ with `discourse` — 775. A tag question is rare in EWT: grep found one, *…, aren't they?* (train), and it is `parataxis`;
- for spoken English treebanks (ESLSpok, the spoken part of GUM, CHILDES) the subtypes would make it possible to distinguish a filler from an interjection and a tag from a parenthetical clause;
- the en registry has no such subtypes, so they must be dropped on export to UD. A subtype can be dropped without loss: `discourse:filler` → `discourse`.

**Registry.** Not allowed for en; allowed for fr in the snapshot.

**Sources.** `docs/_fr/dep/*.md`; #1290 (`grew.fr/spoken-language-guidelines/workgroups/spoken-data/specific_syntax.html`), #1280 (co-constructions by several speakers), #1273 (segmentation of spoken language).
