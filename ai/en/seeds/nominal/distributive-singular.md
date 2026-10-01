# every, each, either, neither, another — with the singular

**Gist.** *Every, each, either, neither* and *another* treat things one at a time, so the noun after them is singular: *every day, each student, either side, another chance*. The verb with such a subject is also singular (Curme).
**Conditions and exceptions.** With a numeral or *few* the group becomes the unit: *every two weeks, every few days, another five years* — then the noun is plural. *Each* after a pronoun is a different construction (*they each got one*).
**Examples.** ✓ *every day*; ✓ *each student*; ✓ *every three hours*; ✗ *every days*; ✗ *each students*.
**In UD.** DET with `det`; the head is `Number=Sing`. EWT 2.18: *every* + singular 104, + plural 4 (all with a numeral or *few*); *another* + plural 9 (*another two weeks*).
**Sources.** Poutsma GLME vol. 4, Ch. XL §35–36 (*each*), §51 (*every*), §109–111 (*neither*) (pp. 386, 398, 446 = book pp. 1066, 1078, 1126); Curme 1931 §8 1 e (pp. 50–51: singular after *each, everybody, either, neither*).

```rule
rule: en.nominal.distributive-sing
what: every/each/either/neither/another + plural — only with a numeral or few (every two weeks)
match: n[upos=NOUN, feats.Number=Plur]; d[lemma=every|each|either|neither|another, rel=det, head=n]
require: exists c[head=n, rel=nummod|amod]
severity: warn
source: Poutsma GLME IV Ch. XL §35–36, §51, §109–111; Curme 1931 §8 1 e
```
