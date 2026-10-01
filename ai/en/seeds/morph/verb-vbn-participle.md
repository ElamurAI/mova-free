# Past participle: VBN

**Gist.** The past participle (*worked, taken, gone*) is a non-finite form: it has no person, number or mood. It works in the perfect (*have taken*), in the passive (*was taken*) and as a modifier (*a broken cup*, if it is still a verb). The PTB tag is VBN.

**Conditions and exceptions.**
- For regular verbs VBN coincides with VBD (*worked*); only syntax distinguishes them: after *have* and in the passive — VBN, as a predicate without an auxiliary — VBD.
- Many strong verbs have a separate form in *-en/-n* (*taken, written, known*) or with another vowel (*sung, begun*); such forms are only VBN (see `verb-strong-ablaut`).
- A participle that has become an adjective (*a very interested reader, broken glass* in the sense of a state) is annotated as ADJ/JJ; the boundary is drawn by adjective features: *very*, degrees, *un-* (*unbroken*).

**Examples.** *It has **been** done. The cup was **broken** by the cat. **Written** in 1914, the book…*

**In UD.** UPOS VERB or AUX (*been*); XPOS VBN; FEATS `Tense=Past|VerbForm=Part`, in the passive also `Voice=Pass`. VBN has no `Mood`, `Person`, `Number`. The lemma is the infinitive (*taken → take*).

**Sources.** https://universaldependencies.org/en/feat/Tense.html (Past: VBD and VBN), `feat/VerbForm.md` (Part), `feat/Voice.md`; Santorini 1990, §2, p. 5 (VBN), §4.1, pp. 15–17 (JJ/VBN tests: degree, *un-*, *by*-phrase, *become/seem*); Sweet NEG I §335 (text-1, pp. 147–148: participles as adjectives; *much pleased*); Whitney §302–303, §309 (text-1, pp. 147, 154: *he is fatigued* — state, *fatigued by* — passive; *very* only with an adjective); Kruisinga II.1 §§64–69 (text-2, pp. 85–89: participles that drifted away from the verb).

```rule
rule: en.morph.vbn-feats
what: VBN — past participle without person, number or mood
match: v[xpos=VBN, upos=VERB|AUX]
require: v[feats.Tense=Past, feats.VerbForm=Part, !feats.Mood, !feats.Person, !feats.Number]
severity: error
source: UD en feat/Tense (Past: VBN), feat/VerbForm (Part); Santorini 1990, VBN
```
