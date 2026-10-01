# Words in -er/-est that are not degrees of comparison

**Gist.** Not every word in *-er* is a comparative, and not every word in *-est* is a superlative. The tag JJR/RBR is given only to a form with a live comparative meaning ("more X"), JJS/RBS — with a superlative one. So *other, proper, clever, bitter* are ordinary adjectives (JJ), and likewise *honest, modest, earnest* (they are never JJS).

**Conditions and exceptions.**
- The root ends in *-er*: *proper, clever, bitter, eager, tender, sober, slender, sinister, sheer, mere, severe, sincere*. They themselves can form degrees (*cleverer, bitterest*).
- Former comparatives that became ordinary adjectives: *other, former, latter, upper, inner, outer, utter* (and the derivatives *formerly, latterly*). They are not compared further and are not used with *than*; in PTB/EWT they are JJ.
- Latin comparatives *superior, inferior, senior, junior, major, minor, prior, interior, exterior*: they have comparative meaning, but no *-er*, and are used with *to*, not *than* (*superior to*). Tag — JJ (`Degree=Pos`); some are also nouns (*a senior, a minor*).
- In *-est* without superlative meaning: *honest, modest, earnest, manifest, west, interest*; they take periphrastic *more/most* (*more honest*).
- Nouns in *-er* (*teacher, computer*) — a separate agent suffix (see `wf-noun-suffixes`).
- Penn Treebank: an *-er* form without a clear "more" is JJ (*further details*), or even an adverb (*come by later/RB*).

**Examples.** *the **other** side* (JJ); *the **former** president* (JJ); ***superior** to* (JJ); *an **honest** man* (JJ); *a **clever** trick* (JJ).

**In UD.** ADJ, XPOS JJ, `Degree=Pos`.

**Sources.** Sweet NEG I §1038 (text-1, p. 357: *honest, earnest, modest* — with *more/most*), §1043 (text-1, p. 358–359: *-most* forms), §1527 (text-1, p. 470: *formerly, latterly, lastly*), §1746 (text-1, p. 525: *inferior, superior, junior, senior*); Whitney §202 (text-1, p. 107: *inner, outer, upper, utter, former*), §211 (text-1, p. 110: *other* as a comparative); Santorini 1990, §2, p. 1 (JJR without *-er* — JJ: *superior*; *-er* without comparison — JJ: *further details*), §4.1, p. 20 (*later/RB*); Kruisinga II.3 §1732 (text-4, p. 88: *other, whether, either, first* — not comparatives), §§1747, 1759–1761, 1772 (p. 95, 99–105: *inner, upper, outer, utter, latter, elder* — contrast only, without *than*); Mätzner I, p. 278–280 (text-1, p. 296–298: *inner, outer, utter, upper, nether, former*).

```rule
rule: en.morph.false-comparative-er
what: other, proper, former, upper, superior etc. — not JJR/RBR
match: a[form=other|proper|clever|bitter|eager|tender|sober|slender|sinister|sheer|mere|severe|sincere|austere|former|latter|upper|inner|outer|utter|superior|inferior|senior|junior|major|minor|prior|interior|exterior|ulterior, !feats.Typo]
require: not a[xpos=JJR|RBR]
severity: error
source: Santorini 1990, §2, p. 1; Sweet NEG I §1043, §1746; Whitney §202
```

```rule
rule: en.morph.false-superlative-est
what: honest, modest, earnest, manifest, west, interest — not JJS/RBS
match: a[form=honest|dishonest|modest|immodest|earnest|manifest|west|interest, !feats.Typo]
require: not a[xpos=JJS|RBS]
severity: error
source: Sweet NEG I §1038 (honest, earnest, modest take more/most); Santorini 1990, §2, p. 1
```
