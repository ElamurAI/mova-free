# How to find annotation errors automatically

**Gist.** Errors in a treebank (and in an annotator's output) leave statistical traces. The literature offers several independent methods. Each gives its own type of rules for the engine.

**Methods.**
1. **Variation nuclei.** The same word pair in the same context has different relations in different sentences — so one of them is an error. On English UD v2 the lemma-based variant found 266 suspicious pairs with 62 % precision (French — 65 %, Finnish — 19 %, because there case changes the role). An automatically parsed parsebank raises recall moderately: precision 41 %. Fixing such places with a classifier: 76.7 % vs 70.1 % without changes.
2. **Arc direction.** For each relation, the share of right-side dependents is counted in every treebank of a language. A large gap (MBD) reveals a divergence of conventions. Among the 20 most inconsistent "language–relation" pairs in UD 2.5 two are English: `compound` and `csubj`. Hence the direction rules in this section: `flat` and `conj` go to the right, `compound` and `nummod` to the left, `cc` stands before its conjunct.
3. **Distribution of UPOS trigrams.** A symmetric KL measure θpos between two treebanks: at θpos ≤ 0.5 the annotation is consistent, at θpos ≥ 4 it is not. The method does not depend on parser quality.
4. **Cross-parsing.** A model trained on EWT is tested on GUM and vice versa. Growth of LAS across versions shows alignment: GUM → EWT from 81.78 to 84.27 (v2.6 → v2.11). The confusion matrix shows exactly what diverges.
5. **Robustness to small changes.** Replace the year in a sentence with another; the Stanza parser changes its parse in 44 % of variant batches on average. Unstable places are error candidates.

**For the engine.** A rule that fires often on the gold is either wrong or catches errors in the gold (the principle of `ai/en/src/expert.rs`). Among the violations of this section's rules on EWT 2.18 there are errors in the gold itself: Case=Nom on *it* under a preposition, det on *soldiers* in *an old soldiers' home*, Number=Plur on *species*.

**Sources.** `W17-6514` (de Marneffe et al. 2017); `dickinson-2009-correcting-dependency-annotation-errors`; `2020.udw-1.8` (Dönicke et al.: MBD); `2020.tlt-1.9` (Aggarwal, Zeman: θpos); `2023.udw-1.7` (Zeldes, Schneider: tables 1–2); `2021.udw-1.8` (Kalpakchi, Boye).
