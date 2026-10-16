# PUD — known annotation errors

**Gist.** PUD syntax and UPOS were corrected manually, so there are few errors there. Lemmas and FEATS are unchecked CoreNLP output. Their errors are mostly inherited from the model (`feats-corenlp.md`), not individual faults. The converter does not fix errors.

**Examples and numbers.**
- **Annotator notes in the data** — 5 lines `# Checktree: …` and one `# notes`. Doubtful sentences that were never resolved: *Not sure about this one*, *Ellipsis*, *how to represet "to record what would be another remarkable success"*.
- **Infinitive root without a subject** (`tb.en.imp-mood`) — 4: `n01085008` *Fast forward to 2016…* — an imperative with `VerbForm=Inf` without `Mood=Imp`.
- **PROPN without Number** (`tb.en.propn-number`) — 11 of 1719; **NOUN without Number** — 5.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`) — 2 of 240.
- **Enhanced UD** was added automatically in 2.2 and not checked (README).

**In UD.** PUD is useful as an independent test of UPOS, XPOS and trees: they were corrected manually. UFeats and lemmas on it measure agreement with CoreNLP 2017, not with EWT 2.18, until there is a reverse converter (`../convert.md`).

**Sources.** README `UD_English-PUD`; Zeman et al. 2017, `K17-3001`, sec. 2.4 (PUD: Udapi conversion, manual corrections); rules in `../../seeds-overview.md`.
