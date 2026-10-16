# Attraction through an of-phrase: "A pattern of arrests indicate"

**Gist.** A classic agreement error. The subject is a singular noun with a plural *of*-phrase dependent, and the verb is made to agree with the nearer plural noun. Fowler calls these "red herrings": the writer starts with a singular, but a plural turns up before the verb and he "loses the scent" (*The foundation of politics are…*, *An immense amount of confusion & indifference prevail*). The exception is quantity nouns, where the number is determined by the of-phrase: *a lot of, a number of, the majority of, the rest of, half of, a couple of*.

**Conditions and exceptions.**
- The list of quantity nouns is open: lot, number, majority, rest, half, percent, bunch, couple, variety, plenty, range, group, total, part, series, kind, sort, type… The rule skips these lemmas.
- Coordination in the subject (*Information on archive sites, and indices … are posted*) gives a legitimate plural. The rule does not distinguish it.

**Examples.**
- ✗ *A pattern of arrests and seizures indicate…*
- ✗ *The return to flight activities are funded.*
- ✓ *A number of people believe…*

**In UD.** nsubj(v, pattern), Number=Sing; nmod(pattern, arrests), Number=Plur; v is VBP (or a VBP copula or auxiliary).

**Check against gold.** EWT 2.18: verb — 7/1 (exactly *pattern … indicate*); copula or auxiliary — 13/5. GUM — 16/3 and 27/8.

**Sources.** Fowler MEU 1926, NUMBER §4 «Red herrings» (p. 401); `2023.emnlp-main.998` (Zacharopoulos et al.); `2020.tacl-1.25` (BLiMP: attractor with a «relational noun» and in a relative clause); `2025.depling-1.4` (Garcia et al.: constructions with attractors are the hardest for LLMs).

```rule
rule: en.errors.attraction-of
what: a singular subject with a plural of-phrase, and the verb is VBP — attraction (except quantity nouns lot/number/majority…)
match: v[xpos=VBP]; s[rel=nsubj|nsubj:pass, head=v, upos=NOUN, feats.Number=Sing, before=v]; q[rel=nmod, head=s, feats.Number=Plur]
require: s[lemma=lot|number|majority|rest|half|percent|bunch|couple|variety|plenty|range|group|total|third|quarter|portion|part|series|kind|sort|type|pair|set|handful|dozen|host|minority|proportion|fraction|share|percentage|mass|pile|ton|load|heap|lots|most|body|crowd|team|family|staff|panel|board|committee|none|each|any|all|remainder]
severity: warn
source: Fowler MEU 1926, NUMBER §4 (p. 401); 2023.emnlp-main.998; 2025.depling-1.4
```

```rule
rule: en.errors.attraction-of-auxcop
what: attraction through an of-phrase with a VBP copula or auxiliary (The return to flight activities are funded)
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=NOUN, feats.Number=Sing]; q[rel=nmod, head=s, feats.Number=Plur]; a[rel=cop|aux|aux:pass, xpos=VBP, head=h, after=s]
require: s[lemma=lot|number|majority|rest|half|percent|bunch|couple|variety|plenty|range|group|total|third|quarter|portion|part|series|kind|sort|type|pair|set|handful|dozen|host|minority|proportion|fraction|share|percentage|mass|pile|ton|load|heap|lots|most|body|crowd|team|family|staff|panel|board|committee|none|each|any|all|remainder]
severity: warn
source: Fowler MEU 1926, NUMBER §4 (p. 401); 2020.tacl-1.25 (BLiMP)
```
