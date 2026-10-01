# Clausal predicate with a copula: ccomp in the text, nsubj:outer in the data

**Gist.** In *The problem is that these sentences are difficult* the predicate is a whole clause. `specific-syntax.md` describes the old analysis: the head is *is*, the clause is `ccomp`. The 2.10 amendment and `_en/dep/cop.md` make the predicate of the inner clause the head, and mark the subject of the outer clause `nsubj:outer`. EWT follows the amendment.

**Guideline.**
- `2.18:https://universaldependencies.org/en/specific-syntax.html:46–51`: «exceptionally, the copular verb is treated as the predicate, and the clause is given attached to it, with the `ccomp` label», example `ccomp(is, difficult)`;
- `2.18:https://universaldependencies.org/en/dep/cop.html:62–79`: `nsubj:outer(tried, problem)`;
- `2.18:UD docs/changes.md` No. 5 (05.2022, 2.10, AMENDMENT, VALIDATOR) «Multiple Subjects»: the old exception is cancelled.

**EWT 2.18 data** (engine):
- `nsubj:outer` and `csubj:outer` — 257. Only 3 of them are on a head without `cop`: *with conditions as they are*, the ellipsis *probably to get*, *the first thing I do is change*;
- *be* as a head with `ccomp` and a subject — 7. None is the old analysis: these are quotative *be like* (*I was like …*, 5) and *there are hints … that …* (2).

**Where the discrepancy comes from.** The "Copulas" section of `specific-syntax.md` was not updated after 2.10.

**Sources.** `2.18:https://universaldependencies.org/en/specific-syntax.html, `2.18:https://universaldependencies.org/en/dep/cop.html, `2.18:https://universaldependencies.org/en/dep/nsubj-outer.html, `2.18:UD docs/changes.md#multiple-subjects`; seeds `en/seeds/verbal/outer-subject.md`, `en/seeds/verbal/copula-predicate-head.md`; report `verbal.md`, discrepancy 5.

```rule
rule: ud218.outer-subject
what: nsubj:outer / csubj:outer — the clausal-predicate analysis of the 2.10 amendment
match: p[]; s[rel=nsubj:outer|csubj:outer, head=p]
require: exists c[rel=cop, head=p]
severity: warn
source: UD 2.18 UD docs/changes.md#multiple-subjects; https://universaldependencies.org/en/dep/cop.html:62
```

```rule
rule: ud218.be-ccomp-head
what: be — head with ccomp and a subject (the old analysis of specific-syntax.md:46)
match: b[lemma=be]; x[rel=ccomp, head=b]; s[rel~nsubj, head=b]
require: not b[lemma=be]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/specific-syntax.html:46–51
```
