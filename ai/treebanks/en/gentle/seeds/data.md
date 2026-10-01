# GENTLE — what these data are and which layers exist

**Gist.** Genre Tests for Linguistic Evaluation — an out-of-domain test annotated with the same multilayer pipeline as GUM. 26 documents of eight unusual genres, ≈ 2100–2400 words each:
- dictionary entries;
- live esports commentary;
- legal documents;
- medical notes;
- poetry;
- mathematical proofs;
- course syllabi;
- threat letters.

In total 1334 sentences and 17,799 words. Test only. License CC BY-NC-SA 4.0.

**Conditions and exceptions.**
- The annotation is as in GUM: XPOS and lemmas manual, UPOS and FEATS from PTB and the graph, dependencies manual in UD. The GUM customs are inherited completely (`gum-family.md`).
- The genre is visible in `# newdoc id` (*GENTLE_poetry_…*). The genres differ a lot in annotation:
  - syllabi: `dep` 31, X 149;
  - dictionary: `dep` 23, X 64;
  - poetry: `dep` 0, X 1.
- X — mostly item numbers *1.*, *2.* with XPOS `LS` and relation `discourse` (230), as in EWT since 2.14.
- `Style=Arch` (29, poetry) and `Abbr=Yes` (90) — more often than elsewhere.

**Examples.** Poetry: `Style=Arch` on *thy* (7), *quoth* (5), *thee* (4), *hath* (3), *o'er* (3). Syllabus: *1.* — X/LS, `discourse`.

**In UD.** Layers — as in GUM: lemmas, XPOS (without `$`, `ADD`, `AFX`, `NFP`), FEATS, DEPS, MISC with Entity/Discourse. MWT — 180.

Our annotator on GENTLE: UPOS 88.47, XPOS 86.76, UFeats 87.30, LAS 67.94, MLAS 48.18. By LAS this is the third result from the bottom, after CTeTex (61.49) and ATIS (63.01). The main cause is the genres, not the customs: the GUM customs affect hundreds of tokens, not thousands (`gum-family.md`).

**Sources.** README `UD_English-GENTLE`; `data/raw/ud-docs/docs/treebanks/en_gentle/index.md`; Aoyama et al. 2023, `2023.law-1.17` (papers/a/ao/aoyama-2023-gentle-genre-diverse-multilayer-challenge); digest, §1.11.
