# no is a determiner, none is a pronoun

**Gist.** *No* stands only before a noun (*no money, no friends*) — it is a negative determiner and cannot be used on its own. Its independent counterpart is *none* (*None of them came*), for persons *nobody/no one*. Poutsma: *none* is "the absolute form of *no*", as *mine* is of *my*.
**Conditions and exceptions.** *No* can also be an adverb before a comparative (*no better, no longer*) and an interjection (*No, thanks*). After *none* the verb can be either singular or plural.
**Examples.** ✓ *no idea*; ✓ *none of us*; ✗ *no of us*; ✓ *no longer* (adverb).
**In UD.** *no* before a noun is DET DT `PronType=Neg`, relation `det` (EWT 2.18: 323 of 323); the adverb is ADV RB `advmod`; the interjection is INTJ UH `Polarity=Neg`. *None* is PRON NN `PronType=Neg`.
**Sources.** Poutsma GLME vol. 4, Ch. XL §114–115 (*no*), §135–137 (*none*) (pp. 447, 472–473 = book pp. 1127, 1152–1153); https://universaldependencies.org/en/feat/Polarity.html, `feat/PronType.md`.

```rule
rule: en.nominal.no-det
what: no as DET — a negative determiner on a noun (det)
match: d[upos=DET, lemma=no]
require: d[rel=det, feats.PronType=Neg]
severity: error
source: Poutsma GLME IV Ch. XL §114–115; UD en PronType
```

```rule
rule: en.nominal.none-pron
what: none is a negative pronoun, not a determiner
match: n[form=none, !feats.Typo]
require: n[upos=PRON, feats.PronType=Neg]
severity: warn
source: Poutsma GLME IV Ch. XL §135–137
```
