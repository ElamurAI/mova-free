# ATIS — what these data are and which layers exist

**Gist.** Transcripts of spoken queries to an automatic flight-information service (ATIS, Hemphill et al. 1990): 5432 sentences, 61,879 words, all lowercase and without punctuation. The annotation is manual, but UPOS and FEATS were derived from PTB tags that were not included in the release. The workflow (2024.bucc-1.11, §3):
- PTB tags were assigned automatically in StarDust, and annotators corrected them;
- rules converted the PTB tags to UPOS and, according to the README, also to FEATS («converted from manual»);
- dependencies were annotated manually, directly in UD, by three linguists.

**Conditions and exceptions.**
- Split: train 4274 / dev 572 / test 586 sentences. The README says 4224 for train, 50 fewer than in the file.
- Annotator agreement for the English part (table 3): head and label — 82, head — 91, label — 86.
- The parallel Turkish treebank has the same `sent_id` (`# parallel_id`, since UD 2.16).

**Examples.** `0003.dev` *show me round trip flights from chicago to detroit leaving next tuesday and returning the day after*; `0002.dev` *i want a flight from memphis to seattle that arrives no later than 3 pm*.

**In UD.** Layers (all parts together, `cut -f` over the CoNLL-U columns):

| layer | state |
|---|---|
| XPOS | absent: «_» in 61,879 of 61,879 words |
| DEPS, MISC | absent: all «_», even `SpaceAfter` |
| PUNCT, SCONJ, X | 0 tokens; subordinating conjunctions have UPOS ADP (`pos-feats-from-ptb.md`) |
| lemmas | manual; unlike the forms, proper nouns are capitalized: 14,399 of 14,775 PROPN (*chicago* → *Chicago*), also *I* (2387) and day names (*Wednesday* 267) |
| FEATS | «_» in 16,276 words; the rest from the PTB tag, see `verb-feats.md` and `pos-feats-from-ptb.md` |
| MWT | 406: *i'd* 235, *what's* 88, *i'm* 46, *what're* 5, *let's* 5 |
| `# text` | lowercase, no punctuation, like a transcript |

Our annotator trained on EWT + ESLSpok gives on the ATIS test UPOS 74.50, UFeats 74.39, LAS 63.01, MLAS 22.53. This is the worst result among the treebanks. Part of the gap is genre: lowercase, no punctuation. The rest is the customs from the seeds in this folder.

**Sources.** README `UD_English-Atis` (metadata: «XPOS: not available», «Features: converted from manual»); `data/raw/ud-docs/docs/treebanks/en_atis/index.md`; Cesur et al. 2024, `2024.bucc-1.11` (papers/c/ce/cesur-2024-building-annotated-parallel-corpora-atis), §3 and table 3; digest, §1.11.
