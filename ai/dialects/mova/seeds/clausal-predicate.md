# Clausal predicate with a copula — `nsubj:outer` — proposal

**Proposal: accept** the analysis of the 2.10 amendment and EWT. Do not take the old analysis from `_en/specific-syntax.md:46–51`, where the copula is the head and the clause is `ccomp` (`dialects/ud-2.18/seeds/clausal-predicate.md`).

**What exactly in `mova`.** *The problem is that these sentences are difficult*: the head is *difficult*, *is* is `cop`, *problem* is `nsubj:outer`, *sentences* is `nsubj`. For a clausal subject of the outer clause — `csubj:outer`.

**Why.**
- **Matches the standard and the data:** EWT 2.18 has 257 `nsubj:outer`/`csubj:outer` and not a single instance of the old analysis. The 7 cases of *be* with `ccomp` are quotative *be like* and *there are hints … that*.
- **More consistent.** The copula stays `cop` in all sentences, with no "transitive copula" exception.
- **Lossless:** export is identity. The old analysis, if it is ever needed for UD v1 or SD conversions, is derived deterministically from `nsubj:outer` + `cop`.

**Check.** `en.verbal.outer-subject-needs-cop` (`en/seeds/verbal/outer-subject.md`), `en.verbal.cop-verb-head-outer` (`en/seeds/verbal/copula-predicate-head.md`); counters `ud218.outer-subject`, `ud218.be-ccomp-head`.

**Origin.** `dialects/ud-2.18/seeds/clausal-predicate.md`; `2.18:UD docs/changes.md#multiple-subjects`.

Mova decides.
