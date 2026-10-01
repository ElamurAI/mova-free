# Typical failures of LLM annotators for UD

**Gist.** Papers from 2024–2026 and our own measurement give a stable picture of where exactly an LLM errs when annotating UD. For each failure there is a check in this section.

**Failures and checks.**
1. **Tags are better than trees.** Mistral Large on an Alsatian dialect: POS 0.89, but UAS 0.55 and LAS 0.45. UDPipe on a related language: POS 0.82, UAS 0.76, LAS 0.60. So heads and relations need checking first.
2. **Short arcs.** The mean dependency distance for an LLM is 3.05, in the gold 3.41: the model gravitates to the nearer head. Consequences: attaching prepositional phrases and clauses to the nearest noun (see `obl-on-nominal.md`, `advcl-on-noun.md`) and an auxiliary after its head in VP ellipsis (`vp-ellipsis-aux-head.md`).
3. **Labels outside the set.** Maritaca produced 93 different labels instead of 17 UPOS. Opus gave `det:poss` in 3 of 20 sentences (see `possessives-nmod-poss.md`). Old SD or UD v1 labels (`neg`, `dobj`, `nsubjpass`, cc on the first conjunct — see `coordination-structure.md`) are already cut off by the registry gate.
4. **Unstable rare relations.** `iobj` and `csubj` are learned late and unstably in 30 of 33 languages (see `iobj-bare-nominal.md`, `clausal-subject-csubj.md`).
5. **Inconsistency** on repetitions, ellipsis and abbreviations. In BiLingua human correction raised LAS from 76 to 95. So the yardstick must be an independent gold, not corrected LLM output.
6. **Self-generated rules** are too narrow (a verb assigned to AUX) and ignore word order. UD guidelines in the prompt help only without examples.
7. **Formal failures.** Invalid CoNLL-U occurs, post-processing is needed. Cycles are rare: 3 in the whole EWT test, all from Qwen2.5.
8. **Sentence complexity.** Accuracy falls as the number of clauses grows.
9. **Silent correction of the text.** The model tags what was intended instead of what was written (see `literal-annotation-learner.md`).

**What helps.** The step sequence UPOS → head → relation gave +2.1 LAS on EWT for gpt-4o-mini. Tabular output, examples selected by similarity, and an LLM as an editor of a ready parse also help. This is exactly how our "draft" mode works: Rust annotates first, Opus on high corrects.

**Sources.** `2025.law-1.14` (Bernhard et al.: sections 4.1–4.4); `2024.propor-1.46` (Machado, Ruiz); `2025.findings-emnlp.863` (Kellert et al.); `2026.udw-1.1` (Matsuda et al.); `2025.xllm-1.17` (Ginn, Palmer); `2025.iwpt-1.2` (Matsuda et al.); `2026.lrec-1.910` (via the digest §3) (measurement 25.09.2026).
