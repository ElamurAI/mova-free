# The relative pronoun is inside the relative clause

**Gist.** Since UD 2.11, EWT analyzes relative clauses as follows. The pronoun (*who, which, that*) is a member of the relative clause (`nsubj`, `obj`, `obl`…) and hangs on its predicate, and the clause itself is `acl:relcl` of the antecedent. *that* in a relative clause is PRON (XPOS WDT) with a role, not `mark`: `mark` *that* occurs only in a complement clause (*the fact that he left*). A pronoun attached directly to the antecedent means an inverted parse.

**Conditions and exceptions.**
- Free relatives: in *whatever deal you want* whatever is `det` of *deal*.
- Nested relatives, where the pronoun is the subject of the outer relative clause.
- The rule takes only a pronoun between the antecedent and the relative predicate.

**Examples.**
- *the man who left* → nsubj(left, who), acl:relcl(man, left).
- *the book that I read* → obj(read, that), that: PRON.
- *the fact that he left* → mark(left, that), acl(fact, left).

**Check against gold.** EWT 2.18:
- pronoun between antecedent and predicate, attached to the antecedent — 1;
- *that* in `acl:relcl` not PRON — 1 of 619.

GUM — 0 and 6.

**Sources.** `2023.udw-1.7` (v2.11: changes in the annotation of relative constructions and clefts); UD `_en/dep/acl-relcl.md`, `_en/dep/mark.md`; `2025.law-1.16` (ICLE-RC: markers that, wh- and zero; 42 non-restrictive relatives with *that* in learner texts); `2025.tlt-1.12` (Czerniak et al.: antecedent functions).

```rule
rule: en.errors.relpron-inside-clause
what: the relative pronoun hangs on the antecedent — it should be a member of the relative clause (nsubj, obj, obl)
match: n[]; r[rel=acl:relcl, head=n, after=n]; w[upos=PRON, feats.PronType=Rel, head=n, after=n, before=r]
require: not w[upos=PRON]
severity: warn
source: 2023.udw-1.7 (v2.11 relatives); UD _en/dep/acl-relcl.md
```

The rule is `en.nominal.relative-that-not-mark` in `nominal/relative-that.md`.
