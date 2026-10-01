# more/most + adjective: the adjective itself is in the positive degree

**Gist.** Degrees of comparison are formed in two ways: with the suffixes *-er/-est* (Poutsma: "terminational", the Germanic way) and with the words *more/most* (Poutsma: "periphrastic", the French way). Both together — *more happier, most unkindest* — occur only in old authors (Shakespeare: *the most boldest*) and in substandard speech.
**Conditions and exceptions.** In *more beautiful* the adjective itself stays in the positive degree, and *more* supplies the degree. Fossilized old double forms are no longer felt as such: *near*, *foremost, hindmost* (Curme). Compound adjectives: *more kind-hearted* and *kinder-hearted* (Poutsma XXX §29).
**Examples.** ✓ *happier*; ✓ *more careful*; ~ *more happier* (substandard).
**In UD.** *more/most* — ADV (RBR/RBS) with `advmod` to the ADJ; the ADJ itself is `Degree=Pos`. EWT 2.18: *more/most* + ADJ `Pos` — 175 times, + `Cmp`/`Sup` — 3 (*more happier, more drunkest, more stranger*), i.e. double comparison in the text itself.
**Sources.** Poutsma GLME vol. 3, Ch. XXX §2, §29–30 (pp. 447, 507–510 = book pp. 427, 487–490); Curme 1931 §53–54 "Older comparison, pleonasm" (pp. 499–503).

```rule
rule: en.nominal.double-comparison
what: with more/most an adjective or adverb is in the positive degree (more happier, more faster — substandard or wrong Degree)
match: a[upos=ADJ|ADV]; m[lemma=more|most, rel=advmod, head=a]
require: a[feats.Degree=Pos] or a[upos=ADV, xpos!=RBR|RBS]
severity: warn
source: Poutsma GLME III Ch. XXX §2, §30; Curme 1931 §54; P17-1074 (ADJ:FORM); W19-4406
```
