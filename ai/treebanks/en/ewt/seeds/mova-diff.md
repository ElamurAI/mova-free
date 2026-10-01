# EWT and `mova`: identical today, except for the MISC trace

**Gist.** `mova` = UD 2.18 with EWT conventions plus three normalizations (`train/dialects-and-converters.md`). On the EWT 2.18 data each of them is empty:
1. `det:poss` → `nmod:poss`: EWT has no `det:poss` (0; `nmod:poss` — 4466).
2. `:tmod`/`:npmod` → `:unmarked`: there are no old subtypes (0). `nmod:unmarked` — 1360, `obl:unmarked` — 1244.
3. Number/Person of verbs from the subject: EWT already writes it this way. All 19,126 finite verbs with Tense have Number and Person (`tb.en.fin-agree` 0 / 19,126). MD has only `VerbForm=Fin` (`tb.en.modal-fin` 0 / 4048).

These normalizations are needed not for EWT but for Opus annotation and `annot` silver data, where `det:poss` and `:tmod` occur, and for the other treebanks.

**Conditions and exceptions.**
- **MISC is part of the dialect.** `TemporalNPAdjunct=Yes` (1077) is the only thing distinguishing a former `:tmod` from a former `:npmod`. The `mova` → treebank converter with old subtypes (ESLSpok, CHILDES, LittlePrince) has to guess by lemma without it (`../../eslspok/convert.md`). So `mova` must keep the EWT MISC, and the `en` annotator should eventually predict `TemporalNPAdjunct`.
- Likewise `Superlocation=Yes` (170) distinguishes *Dallas, **Texas*** from other `nmod:unmarked`, and `FlatType` (425) distinguishes *Chapter 1*, phone numbers, file names from other `flat`.
- **`ud-next`.** Changes after 2.18 that will go into 2.19 are not yet visible in the EWT data. The guideline snapshot has clarification No. 18 (UD docs/changes.md`): a feature underspecified by the form is taken from context. It legitimizes exactly the EWT custom (Number/Person on VBP/VBD from the subject), so `mova` already has it too.

**Examples.**
- *Dallas, Texas* — *Texas*: `nmod:unmarked` + `Superlocation=Yes`.
- *every day* as an adverbial — `obl:unmarked` + `TemporalNPAdjunct=Yes`.

**In UD.** The counters of all general rules in `../../seeds-overview.md` on EWT are zeros or small residues (`errors.md`). This is also a check of the rules themselves: a counter rule that gives many "violations" on EWT is wrong for `mova`.

**Sources.** `train/dialects-and-converters.md` (the `mova` dialect, own normalizations); EWT README v2.15 (`TemporalNPAdjunct`), v2.17 (`Superlocation`, `FlatType`), section «Idiosyncratic MISC Attributes»; UD docs/changes.md` No. 18 (2.19).
