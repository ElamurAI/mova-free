# A missing article with a countable noun

**Gist.** A singular countable noun is not used "bare": it needs an article, a possessive, a numeral or another determiner (*I have a car*, not *I have car*). The superlative also requires *the*: ✗ *give best answer*. Omitting the article is one of the most frequent errors of speakers whose language has no articles (Slavic, Chinese, Japanese). The DET class is 11 % of all edits in W&I and 16 % in NUCLE.

**Conditions and exceptions.**
- Telegraphic style of reviews and headlines (*Room was amazing*).
- Set phrases: *go to school*, *by car*, *in bed*.
- Mass sense: *give him room to progress*.
- The rule takes only a few dozen unquestionably countable words as subject or object.

**Examples.**
- ✗ *Now I have wife and son.*
- ✗ *I have friend who drive from…*
- ✗ *and did great job*

**Check against gold.** EWT 2.18: 953/42 — mostly real article omissions and telegraphic style. GUM — 512/15.

**Sources.** `W19-4406` (table 4: DET 11.25 % W&I train, 15.98 % NUCLE, 10.86 % FCE); `P17-1074` (M:DET); `P16-1070` (Berzak et al.: TLE — 10 native languages, 2.67 errors per sentence on average).

```rule
rule: en.errors.bare-count-noun
what: a singular countable noun as subject or object without an article, possessive or numeral
match: n[xpos=NN, rel=obj|nsubj|nsubj:pass, lemma=car|book|dog|cat|house|friend|problem|question|job|computer|phone|idea|person|city|country|student|teacher|child|week|bed|table|chair|apartment|bag|pen|letter|message|ticket|boyfriend|girlfriend|husband|wife|brother|sister|laptop|camera|restaurant|hotel|doctor|dentist|lawyer|vet|website|mistake|decision|chance|reason|answer|result|report|meeting]
require: exists d[rel=det|nmod:poss|nummod|det:predet, head=n]
unless: none c[before=n, upos!=PUNCT, head!=n]
severity: warn
source: W19-4406 (table 4: DET); P17-1074 (M:DET); exceptions from the seed: an article dropped at the start of a sentence (Room was amazing — prosiopesis, Jespersen 1924 PoG p. 310); room and company removed from the list — they have a mass sense (give him room to progress)
```
