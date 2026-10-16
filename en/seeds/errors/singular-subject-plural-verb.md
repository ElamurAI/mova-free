# Singular subject with VBP

**Gist.** A singular noun as subject requires VBZ: *My wife knows*, not *know*. VBP with a singular noun subject without a conjunct is an agreement error in the text or a wrong subject. In non-native texts the omitted third-person -s is one of the most frequent errors. Models find agreement easier when the subject is plural; most errors occur with a singular subject and a plural noun next to the verb.

**Conditions and exceptions.**
- Quantity nouns with *of* + plural agree by meaning: *a lot of people say*, *the majority want*, *half of the studies provide*, *the rest of us pay*.
- Collectives in British usage: *Argentina intend*, *the family grumble*.
- A coordinated subject is skipped by the rule if there is a `conj`.

**Examples.**
- ✗ *this dentist want to pull the tooth*
- ✗ *The coffee taste burnt.*
- ✓ *A lot of people say…*

**Check against gold.** EWT 2.18: verb — 54/22; copula or auxiliary — 129/37. Most violations are real errors in the text; the rest are quantity and collective nouns.

**Sources.** `P17-1074`, `W19-4406` (VERB:SVA); Fowler MEU 1926, NUMBER (pp. 400–402); `2023.scil-1.24` (Wilson et al.: most errors with a singular subject); `2026.conll-main.7` (Hobbs et al.: the rule "agree with the subject" vs "with the nearest noun").

```rule
rule: en.errors.singular-subject-vbp
what: singular noun subject before a VBP verb — omitted -s or wrong subject
match: v[xpos=VBP]; s[rel=nsubj|nsubj:pass, head=v, upos=NOUN|PROPN, feats.Number=Sing, before=v]
require: exists c[rel=conj, head=s]
unless: s[lemma=lot|lots|number|majority|minority|rest|remainder|half|third|quarter|percent|percentage|proportion|fraction|portion|part|share|bunch|couple|pair|variety|plenty|range|group|set|series|kind|sort|type|handful|dozen|host|mass|pile|ton|load|heap|total|wealth]; s[lemma=family|team|government|committee|army|party|board|company|council|court|crew|staff|panel|body|crowd|public|admiralty|aristocracy|multitude|assembly|congress|class|choir|gentry|navy|cavalry|infantry|police|poultry|race]
severity: warn
source: P17-1074 (VERB:SVA); Fowler MEU 1926, NUMBER (pp. 400–402); exceptions: quantity — verbal/agreement-notional.md, errors/attraction-of-phrase.md; collective — Poutsma GLME III Ch. XXVI (p. 304: board, committee, company, council, couple, court, crew, family, government, pair, party, race, staff), Curme 1931 §8 I 1 d, §55 I
```

```rule
rule: en.errors.singular-subject-vbp-auxcop
what: singular noun subject with a VBP copula or auxiliary (are, have, do)
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=NOUN|PROPN, feats.Number=Sing]; a[rel=cop|aux|aux:pass, xpos=VBP, head=h, after=s]
require: exists c[rel=conj, head=s]
unless: s[lemma=lot|lots|number|majority|minority|rest|remainder|half|third|quarter|percent|percentage|proportion|fraction|portion|part|share|bunch|couple|pair|variety|plenty|range|group|set|series|kind|sort|type|handful|dozen|host|mass|pile|ton|load|heap|total|wealth]; s[lemma=family|team|government|committee|army|party|board|company|council|court|crew|staff|panel|body|crowd|public|admiralty|aristocracy|multitude|assembly|congress|class|choir|gentry|navy|cavalry|infantry|police|poultry|race]
severity: warn
source: P17-1074 (VERB:SVA); Fowler MEU 1926, NUMBER (pp. 400–402); exceptions: quantity — verbal/agreement-notional.md, errors/attraction-of-phrase.md; collective — Poutsma GLME III Ch. XXVI (p. 304: board, committee, company, council, couple, court, crew, family, government, pair, party, race, staff), Curme 1931 §8 I 1 d, §55 I
```
