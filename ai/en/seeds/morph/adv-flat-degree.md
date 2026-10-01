# Adverbs without -ly and their degrees

**Gist.** Some adverbs have the same form as the adjective, without *-ly*: *work hard, run fast, come late, stay long, fly high*. It is these (plus a few more: *soon, often, well, badly, far, little*) that have their own degrees of comparison in *-er/-est* or irregular ones: *harder, fastest, sooner, better, worse, further, less*. Adverbs in *-ly* have no degrees — only periphrastic *more/most*.

**Conditions and exceptions.**
- Pairs with different meanings: *hard* "with difficulty, strongly" ≠ *hardly* "barely"; *late* "not on time" ≠ *lately* "recently"; *most* ≠ *mostly*; *near* ≠ *nearly*. The lemma of each is its own form.
- Colloquial speech also compares *-ly* adverbs with plain adjective forms: *easier said than done, it's done cheapest*.
- *Seldom* has the rare *seldomer*; *rather* is a frozen comparative without a positive.
- In UD, the feature `Degree=Pos` in the positive degree is carried exactly by such adverbs (those with comparison forms); other ADV have no `Degree`.

**Examples.** *She works **hard*** (RB, `Degree=Pos`) — ***harder*** (RBR) — ***hardest*** (RBS); *come **soon*** — ***sooner***; *sing **well*** — ***better*** — ***best***.

**In UD.** ADV; RB with `Degree=Pos` only for adverbs from the guideline's list (*hard, fast, late, long, high, easy, early, far, soon, low, close, well, badly, little*; the rule also allows *near, deep, quick, slow, loud, often, seldom* — they also have comparison forms); RBR — `Degree=Cmp`; RBS — `Degree=Sup`.

**Sources.** https://universaldependencies.org/en/feat/Degree.html (list of RB with `Degree=Pos`; "most adverbs … don't have a Degree feature"); Sweet NEG I §1498–1499 (text-1, p. 459–460: *pull hard, speak loud*), §1524–1525 (text-1, p. 469: comparison of adverbs), §342 (text-1, p. 149); Whitney §99, §313d (text-1, p. 60, 157: *hard/hardly, late/lately, most/mostly*), §316 (text-1, p. 158–159: *better, worse, faster, sooner, oftener*); Jespersen MEG VI 22.9₁ (text-5, p. 428: *hard/hardly, near/nearly, short/shortly*); Kruisinga II.2 §§943–945 (text-3, p. 133–135: *work hard, speak plain*; colloquial *quicker, easier*), II.3 §1714 (text-4, p. 77); Mätzner I, p. 396–398 (text-1, p. 414–416: *better, worse, farther/further, sooner, louder*).

```rule
rule: en.morph.rb-degree-flat-only
what: RB with a Degree feature — only an adverb that has its own degrees
match: r[xpos=RB, feats.Degree]
require: r[lemma=hard|fast|late|long|high|easy|early|far|soon|low|close|well|badly|little|near|deep|quick|slow|loud|often|seldom]
severity: warn
source: UD en feat/Degree (Pos: adverbs hard, fast, late, long, high, easy, early, far, soon, low, close, well, badly, little)
```

```rule
rule: en.morph.rb-degree-pos
what: RB, if it has Degree, — only the positive degree
match: r[xpos=RB, feats.Degree]
require: r[feats.Degree=Pos]
severity: error
source: UD en feat/Degree (Cmp: RBR, Sup: RBS)
```
