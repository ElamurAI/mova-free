# CHILDES — what these data are and which layers exist

**Gist.** Spoken language in conversations of children with adults from CHILDES transcripts: 11 children, 48,183 sentences, 302,740 words. The treebank was merged from three previously annotated sets, each with its own guideline:
- S+24 — Adam, UD 1.0, converted to 2.0 by a script;
- LP21 — Eve;
- LP23 — the other children.

The authors harmonized them and made ≈ 8000 edits: most often UPOS versus relation label, auxiliary verbs and particles (→ `compound:prt`). README metadata:
- dependencies — manual;
- lemmas and UPOS — by software with partial manual correction;
- XPOS — by software without checking;
- FEATS — absent.

**Conditions and exceptions.**
- The split is by child. Train (34,732) and dev (3860) — Adam, Lily, Naima, Sarah, Roman, Laura, Abe. Test (9591) — Eve, Violet, Emma, Thomas: children the model has not seen.
- **11,446 sentences have `# gold_annotation = False`** — all from Adam: train 10,274, dev 1172. These are the "10k missing Adam sentences" added in 2.17. The test has only `True`. For `mova` training they are closer to silver than to gold.
- The authors added punctuation themselves: they capitalized the first letter and derived the final sign from the sentence type (`# type`). So PUNCT does not come from the transcript.
- Sentence metadata: `child_name`, `child_age`, `speaker_role`, `corpus_name`, `type` (declarative 31,996, question 14,241, imperative_emphatic 797…), `childes_toks`, `original_sent_id`.
- License CC BY-SA 4.0. Separately from the release there are ≈ 1.2M silver sentences annotated by stanza.

**Examples.** README: `sent_id = 24433`, *A world of Easter.* (Adam, mother, `type = trail off`).

**In UD.**

| layer | state |
|---|---|
| FEATS | absent: «_» in 302,504 of 302,740; the rest — only `ExtPos` (209) and `Typo` (27) |
| XPOS | PTB, automatic; punctuation `?` (8918) and `!` (734) instead of `.` (`punct.md`) |
| lemmas | customs differ by subcorpus (`pron-lemma.md`) |
| DEPS | present |
| MISC | `SpaceAfter` and `reparandum`/`parataxis` subtypes from the sources, moved into MISC |
| MWT | 12,905 |

Our annotator on the CHILDES test: UPOS 93.38, XPOS 93.48, LAS 80.01. Adding 35k CHILDES sentences to training gave EWT LAS −0.2.

**Sources.** README `UD_English-CHILDES` (in particular Changelog v2.17); `data/raw/ud-docs/docs/treebanks/en_childes/index.md`; Yang et al. 2025, `2025.udw-1.6` (papers/y/ya/yang-2025-ud-english-childes-collected-resource), sec. 3 («Harmonization»: punctuation, reparandum, UD 1 → 2); Szubert et al. 2022, `2022.scil-1.24`.
