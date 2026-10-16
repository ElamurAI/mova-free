# Plural subject with VBZ

**Gist.** A present-tense verb agrees with its subject in number: *The results reduce*, not *reduces*. If the annotation has a plural subject and the verb (copula or auxiliary) is VBZ, there are two possible causes:
- the text has an agreement error — ERRANT VERB:SVA, about 2 % of edits in learner corpora;
- the annotator picked the wrong subject. Often this is an "attractor" — the nearest noun instead of the real head of the subject phrase (*The key to the cabinets*).

Both humans and models err more often when the attractor stands right before the verb, but models much more so: in the hardest conditions almost at chance. The T5 transformer is affected more by the linear proximity of the distractor than by structural proximity.

**Conditions and exceptions.**
- A plural name with singular meaning — *The United States goes*, book titles. Proper names (PROPN) are not taken by the rule.
- Measures: *5 kg per gun means*.
- *as follows* with `nsubj:outer`.
- Subject after the verb (*there's lots of…*) — a separate rule in `existential-there.md`.

**Examples.**
- ✗ *The results of the test reduces the gas.*
- ✗ *My parents even plays with them.*
- ✗ *what people knows about Philippines*

**In UD.** nsubj with Number=Plur with a VBZ verb, VBZ copula or VBZ auxiliary.

**Check against gold.** EWT 2.18: verb — 1166/7; copula or auxiliary — 2528/8. All violations are text errors, measures and names.

**Sources.** Fowler MEU 1926, NUMBER §4 "Red herrings" (p. 401); `2023.emnlp-main.998` (Zacharopoulos et al.: attractor next to the verb); `2023.scil-1.24` (Wilson et al.: linear proximity stronger than structural); `2020.tacl-1.25` (BLiMP: SVA with attractors in a relative clause and a prepositional phrase); `P17-1074`, `W19-4406` (VERB:SVA: 2.2 % of W&I edits, 1.5 % FCE, 3.5 % NUCLE).

```rule
rule: en.errors.plural-subject-vbz
what: plural subject before a VBZ verb — agreement error or wrong subject (attractor)
match: v[xpos=VBZ]; s[rel=nsubj|nsubj:pass, head=v, before=v, upos=NOUN|PRON]
require: not s[feats.Number=Plur]
unless: exists q[rel=nummod, head=s]
severity: warn
source: Fowler MEU 1926, NUMBER §4 (p. 401); 2023.emnlp-main.998; P17-1074 (VERB:SVA); exception: measures and sums with a numeral are singular (Three hours isn't far; 5 kg per gun means): verbal/agreement-third-singular.md, verbal/agreement-notional.md
```

```rule
rule: en.errors.plural-subject-vbz-auxcop
what: plural subject with a VBZ copula or auxiliary (is, has, does)
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=NOUN|PRON]; a[rel=cop|aux|aux:pass, xpos=VBZ, head=h, after=s]
require: not s[feats.Number=Plur]
unless: exists q[rel=nummod, head=s]
severity: warn
source: Fowler MEU 1926, NUMBER §4 (p. 401); 2020.tacl-1.25 (BLiMP SVA); P17-1074 (VERB:SVA); exception: measures and sums with a numeral are singular (Three hours isn't far; 5 kg per gun means): verbal/agreement-third-singular.md, verbal/agreement-notional.md
```
