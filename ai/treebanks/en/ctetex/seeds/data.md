# CTeTex — what these data are and which layers exist

**Gist.** Technical text: 196 software requirements, mostly from the PURE corpus. That is 276 sentences and 9273 words. The annotation is manual, directly in UD; the main annotator spent ≈ 180 hours. The sentences are long: 33.6 tokens on average versus 17.7 in eight other English UD 2.8 treebanks. Typical phenomena:
- abbreviations and acronyms — 579;
- specialized vocabulary — 503;
- lists — 41, 2736 tokens in total.

The treebank is test-only: fewer than 20k tokens, so it is not split.

**Conditions and exceptions.**
- Lemmas were added in 2.16. There is no XPOS.
- `# sent_id` — a number, `# newdoc id` — the PURE file name and section, 196 documents.
- MISC has its own attribute `LineAfter=Yes` (175): in the source text there was a line break after the word. It is needed to reconstruct vertical lists.
- License CC BY-SA 4.0.

**Examples.** `199` *Lag Frames – The BE shall receive LTA or Speed Dump Lag Frames from the Correlator.*; `270` *User's will be able to pick up a tile from the wall if it is their turn.*

**In UD.**

| layer | state |
|---|---|
| XPOS | absent: «_» in 9273 of 9273 |
| FEATS | only `Number` (NOUN, AUX, some VERB) and `Tense` (VERB, AUX); «_» in 6070 words (`feats-minimal.md`) |
| lemmas | manual since 2.16; *an* → *an* (41) |
| DEPS | absent |
| MWT | absent: *User's* — two words without a `1-2` line |
| MISC | `SpaceAfter` 1560, `LineAfter` 175, `Foreign` 21 (in MISC, not in FEATS, on formula symbols), `CorrectForm` 1 |

Our annotator on CTeTex: UPOS 87.03, UFeats 58.07, LAS 61.49, MLAS 13.26. UFeats and MLAS are low primarily because the gold data lack features that we set: only part of them agrees with the reduced FEATS. Stanza 1.2.3 on CTeTex, according to the authors: UPOS 0.870, LAS 0.688.

**Sources.** README `UD_English-CTeTex` (Specificities: `LineAfter`); `data/raw/ud-docs/docs/treebanks/en_ctetex/index.md`; Hassert et al. 2021, `2021.udw-1.5` (papers/h/ha/hassert-2021-ud-software-requirements-application-challenges), sec. 3–4 and table 1.
