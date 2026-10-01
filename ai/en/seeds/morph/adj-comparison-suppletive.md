# Irregular comparison: good–better–best, bad–worse–worst

**Gist.** A few of the most frequent adjectives and adverbs form their degrees from a different root: *good/well → better → best*, *bad/badly/ill → worse → worst*, *much/many → more → most*, *little → less → least*. A few more have double forms with different meanings: *far → farther/further*, *old → older/elder*, *late → later/latter, latest/last*, *near → nearer, nearest/next*.

**Conditions and exceptions.**
- *Better, best* are the degrees of both the adjective *good* and the adverb *well*; *worse, worst* — of both *bad* and *badly*. The part of speech decides the lemma: ADJ → *good, bad*; ADV → *well, badly*.
- *Elder, eldest* — only about seniority in a family (*my elder brother*), *older* — about age in general. *Latter* (the second of two) and *last* (final) have lost their link to *late* and in EWT have their own lemma and the tag JJ.
- *Further* meaning "additional" (*further details*) is an ordinary adjective JJ with lemma *further*; comparative *further* (*further away*) is RBR with lemma *far*.
- *Lesser* is a double comparative of *less*; *worser, worsest* are nonstandard.
- EWT lemma rule: *more, most, less, least* have **their own** lemmas (*more*, not *much*); *fewer* → *few*; *later, latest* → *late*; *older* → *old*.

**Examples.** *a **better** plan* (ADJ, lemma *good*); *she sings **better*** (ADV, lemma *well*); ***worse** luck* (ADJ, *bad*); ***more** people* (ADJ JJR, lemma *more*).

**In UD.** The degree is in the tag and the feature (JJR/RBR + `Degree=Cmp`, JJS/RBS + `Degree=Sup`), the lemma is the positive degree of the other root per the table above.

**Sources.** Sweet NEG I §1044–1052 (text-1, p. 359–361: *old–elder, late–latter–last, far–further, good–better–best, bad–worse–worst, little–less–least–lesser, much/many–more–most*) and §1525 (text-1, p. 469: adverbs *well, badly*); Whitney §202 (text-1, p. 107), §316 (text-1, p. 158–159); Santorini 1990, §2, p. 1, §4.1, p. 17, §4.2, pp. 25–27 (*more/less* JJR or RBR; *most/least* JJS); EWT 2.18 practice (lemmas *more, most, less, least*); Kruisinga II.3 §§1731–1733 (text-4, p. 87–88: *farther/further, nearest/next, later/latter, latest/last, elder/eldest, lesser*; forms in *-most*).

```rule
rule: en.morph.better-best-adj
what: better, best as an adjective — lemma good
match: a[upos=ADJ, form=better|best, !feats.Typo]
require: a[lemma=good]
severity: error
source: Sweet NEG I §1044–1052; UD EWT (better/JJR → good)
```

```rule
rule: en.morph.better-best-adv
what: better, best as an adverb — lemma well
match: a[upos=ADV, form=better|best, !feats.Typo]
require: a[lemma=well]
severity: error
source: Sweet NEG I §1525; UD EWT (better/RBR → well)
```

```rule
rule: en.morph.worse-worst-adj
what: worse, worst as an adjective — lemma bad
match: a[upos=ADJ, form=worse|worst, !feats.Typo]
require: a[lemma=bad]
severity: error
source: Sweet NEG I §1044–1052; UD EWT (worse/JJR → bad)
```

```rule
rule: en.morph.worse-worst-adv
what: worse, worst as an adverb — lemma badly (EWT sometimes bad)
match: a[upos=ADV, form=worse|worst, !feats.Typo]
require: a[lemma=badly|bad]
severity: warn
source: Sweet NEG I §1525; EWT 2.18 is inconsistent (worse/RBR → badly, worst/RBS → bad)
```

```rule
rule: en.morph.more-most-lemma
what: more, most, less, least in degrees — own lemma, not much/many/little
match: a[xpos=JJR|JJS|RBR|RBS, form=more|most|less|least, !feats.Typo]
require: a[lemma=more|most|less|least]
severity: error
source: EWT 2.18 practice; Santorini 1990, §4.2, pp. 25–27
```

```rule
rule: en.morph.farther-further-lemma
what: further, farther, furthest, farthest in degrees — lemma far
match: a[xpos=JJR|JJS|RBR|RBS, form=further|farther|furthest|farthest, !feats.Typo]
require: a[lemma=far]
severity: warn
source: Sweet NEG I §1044–1052; UD EWT (further/RBR → far; further/JJ "additional" — lemma further)
```
