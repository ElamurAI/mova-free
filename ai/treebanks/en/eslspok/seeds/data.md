# ESLSpok — what these data are and which layers exist

**Gist.** Spoken language of Japanese learners of English: a random sample of utterances from language-proficiency interviews, the NICT JLE corpus. The sample is called SL2E. The interviewers' utterances were removed. The treebank has 2320 sentences and 21,312 words. XPOS (PTB) and dependencies were annotated manually from scratch by at least two annotators, and a third resolved disagreements. Agreement before adjudication: POS — 95.1%, LAS — 86.5%. UPOS was derived from XPOS and dependencies by a model with accuracy 0.9885 and then checked manually. Dependencies follow UD 2.0; the treebank has not changed since release 2.12.

**Conditions and exceptions.**
- Split: train 1856 / dev 232 / test 232 sentences.
- Learner errors are annotated literally, by form, not by intent. Tags follow the PTB guideline and Berzak et al. 2016 (TLE): *she feel very happy* — *feel*: VBP.
- The README metadata say «Features: manual native», but the data have no FEATS (`cut -f6` — «_» in 21,312 of 21,312).

**Examples.**
- `file01145.txt_29` *And when I go to my grandmother house , she feel very happy , so she give me many money .*
- `file00207.txt_69` *he runs every morning around here* — a sentence without a capital letter and a period, as in the transcript.

**In UD.** Layers:

| layer | state |
|---|---|
| lemmas | absent: «_» in 21,312 of 21,312 |
| FEATS | absent: «_» everywhere |
| DEPS, MISC | absent: «_» everywhere, even `SpaceAfter` |
| MWT | absent: *n't*, *'s*, *'m* — separate words without a `1-2` line (`tokenization.md`) |
| XPOS | PTB, 39 tags. No EWT tags ``` `` ```, `$`, `HYPH`, `NFP`, `ADD`, `AFX`, `GW`, `-LRB-`, `-RRB-`, `LS`, `SYM`, `WP$`. There is `"` (50) instead of the pair ``` `` ```/`''` and `X` (1) |
| `# text` | tokenized: spaces before *'s*, *n't* and punctuation |

Our annotator already trains on ESLSpok train together with EWT. It skips the missing layers: `Sentence::has_lemmas` and `has_feats` (`en/src/conllu.rs`). On the ESLSpok test: UPOS 95.23, XPOS 95.45, LAS 86.23.

**Sources.** README `UD_English-ESLSpok`; `data/raw/ud-docs/docs/treebanks/en_eslspok/index.md`; Kyle et al. 2022, `2022.bea-1.7` (papers/k/ky/kyle-2022-dependency-treebank-spoken-second-language), sec. 3 (annotation, agreement, Berzak et al. 2016); digest, §1.11.
