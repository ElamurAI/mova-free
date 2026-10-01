# Reflexive and intensive pronouns in -self

**Gist.** Pronouns in *-self/-selves* have two roles. Reflexive: an object coreferent with the subject (*He hurt himself*). Intensive (emphatic): "oneself, in person" — on a noun or subject (*The king himself came*, *I did it myself*).
**Conditions and exceptions.** In writing the roles cannot always be told apart (Poutsma XXXIV §29); the criterion is function. The reflexive takes the place of an object or of a word after a preposition (`obj`, `iobj`, `obl`); the intensive attaches to a noun or verb without a preposition. Colloquial *myself* instead of *I/me* (*John and myself went*) is non-standard.
**Examples.** *She blamed herself.* (reflexive); *Einstein himself was there.* (intensive); *Do it yourself.* (intensive).
**In UD.** PRON PRP, `Case=Acc|Reflex=Yes` + Person/Number/Gender. Reflexive — `PronType=Prs`; intensive — `PronType=Emp`, relation `nmod:unmarked` (on a noun) or `obl:unmarked` (on a verb). EWT 2.18: `Emp` only in these relations (46 of 47, plus 1 `appos`); `Prs` never `:unmarked`. *Yourself* is `Number=Sing`, *yourselves* `Plur`.
**Sources.** Poutsma GLME vol. 4, Ch. XXXIV §1–4, §29–31 (pp. 150–158, 190–192 = book pp. 830–838, 870–872); Curme 1931 §56 D "Intensifying myself, himself" (pp. 514–517); https://universaldependencies.org/en/dep/nmod-unmarked.html (iv), `feat/Reflex.md`, `feat/PronType.md`.

```rule
rule: en.nominal.self-reflex
what: pronouns in -self — Reflex=Yes, Case=Acc, singular
match: r[upos=PRON, suffix=self]
require: r[feats.Reflex=Yes, feats.Case=Acc, feats.Number=Sing]
severity: error
source: Poutsma GLME IV Ch. XXXIV §1; UD en Reflex
```

```rule
rule: en.nominal.selves-reflex
what: pronouns in -selves — Reflex=Yes, Case=Acc, plural
match: r[upos=PRON, suffix=selves]
require: r[feats.Reflex=Yes, feats.Case=Acc, feats.Number=Plur]
severity: error
source: Poutsma GLME IV Ch. XXXIV §1; UD en Reflex
```

```rule
rule: en.nominal.emphatic-self-rel
what: intensive -self (PronType=Emp) is nmod:unmarked, obl:unmarked or appos
match: r[feats.Reflex=Yes, feats.PronType=Emp]
require: r[rel=nmod:unmarked|obl:unmarked|appos]
severity: warn
source: Poutsma GLME IV Ch. XXXIV §29; UD en nmod:unmarked (iv)
```

```rule
rule: en.nominal.unmarked-self-emp
what: -self as nmod:unmarked or obl:unmarked is intensive (PronType=Emp)
match: r[feats.Reflex=Yes, rel=nmod:unmarked|obl:unmarked]
require: r[feats.PronType=Emp]
severity: error
source: Poutsma GLME IV Ch. XXXIV §3 b, §29; UD en nmod:unmarked (iv)
```
