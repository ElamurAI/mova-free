# someone, anything, nobody, everything: compound indefinite pronouns

**Gist.** Pronouns of *some-, any-, no-, every-* + *-one, -body, -thing* are noun-like singular words. An adjective modifier with them stands **after**: *something new, anyone else, nothing special, everything English*.
**Conditions and exceptions.** The type is set by the first part: *some-/any-* — indefinite, *every-* — total, *no-* — negative. *No one* is written separately (in UD: *no* — DET, *one* — PRON). The verb is singular (*Everyone is here*), while the referring pronoun is often *they* (Curme §8 1 e). Poutsma explains *-body, -thing* as "prop words" with indefinite pronouns.
**Examples.** *something new*; *anybody else*; *Nobody knows.*; *everything you need*.
**In UD.** PRON, XPOS NN, `Number=Sing` + `PronType=Ind` (*some-, any-*), `Tot` (*every-*), `Neg` (*no-*). The modifier is `amod` to the right (EWT 2.18: 71 to the right, 1 to the left).
**Sources.** Curme 1931 §10 I 1 (p. 64: a single adjective after an indefinite pronoun), §8 1 e (pp. 50–51); Poutsma GLME vol. 4, Ch. XLIII §27–28, §35–36 (*body, thing* as prop words; pp. 621–627 = book pp. 1301–1307); https://universaldependencies.org/en/dep/amod.html (*Anything else*).

```rule
rule: en.nominal.every-compound-tot
what: everyone/everybody/everything — PRON NN, singular, PronType=Tot
match: p[upos=PRON, !feats.Typo, form=everyone|everybody|everything]
require: p[xpos=NN, feats.Number=Sing, feats.PronType=Tot]
severity: error
source: Curme 1931 §8 1 e; UD en PronType
```

```rule
rule: en.nominal.some-any-compound-ind
what: someone/somebody/something/anyone/anybody/anything — PRON NN, singular, PronType=Ind
match: p[upos=PRON, !feats.Typo, form=someone|somebody|something|anyone|anybody|anything]
require: p[xpos=NN, feats.Number=Sing, feats.PronType=Ind]
severity: error
source: Curme 1931 §8 1 e; UD en PronType
```

```rule
rule: en.nominal.no-compound-neg
what: nobody/nothing — PRON NN, singular, PronType=Neg
match: p[upos=PRON, !feats.Typo, form=nobody|nothing]
require: p[xpos=NN, feats.Number=Sing, feats.PronType=Neg]
severity: error
source: Poutsma GLME IV Ch. XL §114; UD en PronType
```

```rule
rule: en.nominal.adj-after-indef-pronoun
what: an adjective with something/anyone/nobody… stands after the pronoun
match: p[upos=PRON, xpos=NN]; a[rel=amod, head=p]
require: a[after=p]
severity: warn
source: Curme 1931 §10 I 1; UD en amod (Anything else)
```
