# Degrees of comparison in tags and features

**Gist.** The English adjective has three degrees: positive (*young*), comparative (*younger*) and superlative (*youngest*). PTB tags encode the degree right in the name: JJ — positive, JJR — comparative, JJS — superlative; likewise for adverbs RB, RBR, RBS. The UD feature `Degree` duplicates what the tag already says, so tag and feature must agree.

**Conditions and exceptions.**
- `Degree=Pos` is carried by all adjectives tagged JJ. Among adverbs — only those with their own comparison forms (*hard, fast, late, soon, well*…; see `adv-flat-degree`); ordinary *-ly* adverbs have no `Degree` feature.
- In EWT an adjective inside a proper name keeps the tag NNP but gets UPOS ADJ and its degree: *Greater* in *Greater London* — ADJ, NNP, `Degree=Cmp`.
- Periphrastic comparison (*more beautiful*) does not change the adjective's degree: *beautiful* stays JJ, `Degree=Pos`, and the degree is carried by *more* (RBR); see `adj-comparison-periphrastic`.

**Examples.** *young* JJ `Degree=Pos` — *younger* JJR `Degree=Cmp` — *youngest* JJS `Degree=Sup`; *sooner* RBR `Degree=Cmp`; *best* (adverb) RBS `Degree=Sup`.

**In UD.** ADJ ↔ JJ, JJR, JJS (except adjectives in proper names tagged NNP); ADV ↔ RB, RBR, RBS. `Degree=Cmp` ↔ JJR/RBR, `Degree=Sup` ↔ JJS/RBS.

**Sources.** https://universaldependencies.org/en/feat/Degree.html (Pos: all JJ and a list of RB; Cmp: JJR, RBR; Sup: JJS, RBS), https://universaldependencies.org/en/pos/ADJ.html (ADJ = JJ ∪ JJR ∪ JJS); Santorini 1990, §2, pp. 1–2 (JJ, JJR, JJS, RBR, RBS).

```rule
rule: en.morph.jj-degree-pos
what: an adjective tagged JJ — positive degree
match: a[xpos=JJ, upos=ADJ]
require: a[feats.Degree=Pos]
severity: error
source: UD en feat/Degree (Pos: all JJ)
```

```rule
rule: en.morph.jjr-rbr-degree-cmp
what: JJR and RBR — comparative degree
match: a[xpos=JJR|RBR]
require: a[feats.Degree=Cmp]
severity: error
source: UD en feat/Degree (Cmp: JJR, RBR); Santorini 1990, JJR, RBR
```

```rule
rule: en.morph.jjs-rbs-degree-sup
what: JJS and RBS — superlative degree
match: a[xpos=JJS|RBS]
require: a[feats.Degree=Sup]
severity: error
source: UD en feat/Degree (Sup: JJS, RBS); Santorini 1990, JJS, RBS
```

```rule
rule: en.morph.degree-cmp-tag
what: comparative degree on an adjective — tag JJR (in a proper name NNP), on an adverb — RBR
match: a[feats.Degree=Cmp, upos=ADJ|ADV]
require: a[xpos=JJR|RBR|NNP]
severity: error
source: UD en feat/Degree; EWT 2.18 practice (NNP in proper names)
```

```rule
rule: en.morph.degree-sup-tag
what: superlative degree on an adjective — tag JJS (in a proper name NNP), on an adverb — RBS
match: a[feats.Degree=Sup, upos=ADJ|ADV]
require: a[xpos=JJS|RBS|NNP]
severity: error
source: UD en feat/Degree; EWT 2.18 practice (NNP in proper names)
```

```rule
rule: en.morph.jj-is-adj
what: tags JJ, JJR, JJS — always an adjective (or X for foreign words)
match: a[xpos=JJ|JJR|JJS]
require: a[upos=ADJ|X]
severity: error
source: UD en pos/ADJ (ADJ = JJ ∪ JJR ∪ JJS)
```
