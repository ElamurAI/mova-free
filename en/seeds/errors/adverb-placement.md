# An adverb between the verb and the object: "I like very much football"

**Gist.** In English an adverb usually does not stand between the verb and the direct object: ✓ *I like football very much*, ✗ *I like very much football*. In languages with freer word order (Ukrainian, French) this is possible, so learners transfer the order. In ERRANT this is WO (word order) — 1.6 % of edits in W&I.

**Conditions and exceptions.**
- A very long object may move to the end: *I understand better the risks that we face*. This is legitimate shifting of a "heavy" phrase.
- Frequency adverbs (*always, often, never*) stand before the verb, so after it they are already an order error.

**Examples.**
- ✗ *I like very much football.*
- ✓ *I understand better the risks we face.* (heavy object)

**In UD.** advmod(v, m) after v and before obj(v, o).

**Check against gold.** EWT 2.18: 21/3, all three are heavy objects.

**Sources.** `P17-1074` (WO); `W19-4406` (table 4: WO 1.64 % W&I train, 1.82 % FCE).

```rule
rule: en.errors.adverb-before-object
what: an adverb (much, well, always, often…) between the verb and the direct object — English order is V + O + Adv
match: v[upos=VERB]; m[lemma=much|well|often|always|never|usually|also|very, rel=advmod, head=v, after=v]; o[rel=obj, head=v, upos=NOUN|PROPN]
require: o[before=m]
severity: warn
source: P17-1074 (WO); W19-4406
```
