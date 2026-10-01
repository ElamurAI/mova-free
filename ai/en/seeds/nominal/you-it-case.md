# you and it: case is visible only from the role in the sentence

**Gist.** *You* and *it* have one form for nominative and objective, so case is determined by function: subject of a finite verb — nominative; object and word after a preposition — objective.
**Conditions and exceptions.** The subject of an infinitive after *for* (*for you to decide*) and the subject of a gerund are objective. Formal *it* (`expl`) as subject is nominative, and as object (*I find it hard to…*) objective.
**Examples.** *You* (Nom) *saw it* (Acc). *It* (Nom) *hit you* (Acc). *It is time for you* (Acc) *to go.*
**In UD.** EWT sets `Case` for *you/it* by function: subject of a finite verb — `Case=Nom`; `obj`, `iobj`, `obl`, `nmod` — `Case=Acc` (EWT 2.18: no exceptions, apart from 3 infinitive subjects with *for*). Here a mismatch is an annotation error, not a text error.
**Sources.** Poutsma GLME vol. 4, Ch. XXXII §4, §8, §10 (pp. 29, 35, 37 = book pp. 709, 715, 717); Curme 1931 §3 (p. 3); https://universaldependencies.org/en/feat/Case.html.

```rule
rule: en.nominal.you-it-nom
what: you/it as subject of a finite verb has Case=Nom
match: v[feats.VerbForm=Fin]; p[upos=PRON, !feats.Typo, form=you|it, rel~nsubj, head=v]
require: p[feats.Case=Nom]
severity: error
source: Poutsma GLME IV Ch. XXXII §8; UD en Case (EWT)
```

```rule
rule: en.nominal.you-it-acc
what: you/it as object, indirect object, adverbial, nmod has Case=Acc
match: p[upos=PRON, !feats.Typo, form=you|it, rel=obj|iobj|obl|obl:unmarked|obl:agent|nmod|nmod:unmarked]
require: p[feats.Case=Acc]
severity: error
source: Poutsma GLME IV Ch. XXXII §4; UD en Case (EWT)
```
