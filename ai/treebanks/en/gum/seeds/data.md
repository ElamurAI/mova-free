# GUM — what these data are and which layers exist

**Gist.** Georgetown University Multilayer: a multilayer corpus. It is extended every year by students of the LING-4427 course; texts are taken from openly licensed sources. In UD 2.18 — 14,353 sentences and 256,739 words from 16 main genres: academic, biographies, conversations, fiction, interviews, news, textbooks, travel guides, Reddit (separately, `../../gumreddit`), wikiHow, court, essays, letters, podcasts, etc. Manual annotation:
- PTB tags and lemmas — manual;
- UPOS and FEATS — automatic from PTB and the graph;
- dependencies up to GUM v5 — Stanford Dependencies converted by DepEdit rules taking the entity and coreference layers into account; since v6 — directly UD;
- then everything was corrected manually.

**Conditions and exceptions.**
- Split: train 11,314 / dev 1575 / test 1464 sentences.
- License CC BY-NC-SA 4.0: for `mova` test and research only, not training commercial models.
- GUM is converging with EWT deliberately. From the changelog:
  - 2021: MWT, hyphen splitting, HYPH;
  - 2022: `advcl:relcl`, `nsubj:outer`, features as in EWT;
  - 2024: `:unmarked`, `ExtPos`;
  - 2025: `nmod:desc`, `dep` → `parataxis` for footnotes.

**Examples.** `GUM_bio_padalecki-2` *Jared Tristan Padalecki (born July 19, 1982) [1] is an American actor.*; `GUM_conversation_grounded-5` *You guys are always in trouble.*

**In UD.** All layers, as in EWT, and richer:

| layer | state |
|---|---|
| lemmas, XPOS | manual; XPOS without the EWT tags `ADD`, `NFP`, `AFX` (`xpos-web.md`) |
| FEATS | from PTB and the graph; «_» in 82,308 words (EWT — 79,476) |
| DEPS | enhanced UD, 154 empty nodes |
| MISC | `Entity` (coreference), `Discourse` (eRST), `MSeg` (morphemes), `Cxn`, `XML`, PDTB; «_» only in 97,819 words |
| MWT | 4455 |

Our annotator (EWT + ESLSpok train) on the GUM test: UPOS 94.26, UFeats 90.84, LAS 76.85, MLAS 60.08. UFeats is 4.8 points lower than on EWT — the customs from `you-number.md` and `adv-degree.md` are at work here. Adding GUM train to training gave EWT LAS +0.8, PUD +3.1, GUM +4.2.

**Sources.** README `UD_English-GUM` (Summary, Changelog); `data/raw/ud-docs/docs/treebanks/en_gum/index.md`; Zeldes 2017, LRE 51(3); Zeldes & Schneider 2023, `2023.udw-1.7` (papers/z/ze/zeldes-2023-are-ud-treebanks-getting-more); Peng & Zeldes 2018, `W18-4918`; digest, §1.5.
