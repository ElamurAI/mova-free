# Periphrastic comparison: more/most + adjective

**Gist.** Long adjectives and almost all adverbs in *-ly* are compared not with an ending but with the words *more* and *most* (downward — *less, least*): *more difficult, most carefully, less expensive*. The adjective itself does not change: it stays in the positive degree, and the degree is carried by *more/most*.

**Conditions and exceptions.**
- *More/most* before an adjective or adverb is an adverb (RBR/RBS, relation `advmod`). Before a noun (*more money, most people*) it is a quantifying adjective (JJR/JJS, relation `amod`). Without a noun, as an object (*eat more*) Penn Treebank assigns JJR, as an adverbial (*relax more*) — RBR.
- Double comparison (*more braver, most unkindest*) was normal in the 16th century (Shakespeare), and is now nonstandard. If it occurs, the annotation follows the form: *braver* stays JJR with `Degree=Cmp`.
- *Most* meaning "very" (*a most interesting book*) is also RBS by tag.
- In the combination *more than* (*more than ten*) EWT has *more* as ADJ with `ExtPos=ADV`.

**Examples.** *a **more** difficult task* (*more* RBR, *difficult* JJ `Degree=Pos`); *the **most** carefully planned trip*; ***less** expensive*.

**In UD.** *More/most/less/least* as `advmod` of an adjective or adverb — UPOS ADV, XPOS RBR/RBS, `Degree=Cmp/Sup`. The adjective after them — JJ, `Degree=Pos` (except in double comparison).

**Sources.** Sweet NEG I §1038–1039 (text-1, p. 356–357: *more/most* for long and suffixed words), §1041 (text-1, p. 357: *more braver, most unkindest* — now nonstandard), §1524 (text-1, p. 469: *-ly* adverbs periphrastically); Whitney §200 (text-1, p. 106); Santorini 1990, §4.2, pp. 25–27 (*more/less* JJR vs RBR); https://universaldependencies.org/en/feat/Degree.html (*more quietly, most seriously*); Kruisinga II.3 §§1727–1729, 1777–1778 (text-4, p. 85–86, 108–109: *more/most* with *exact, eager, proper* and with *-ed* adjectives; *a most memorable event*); Mätzner I, p. 280–282 (text-1, p. 298–300: *most strange*; *more better, most unkindest*).

```rule
rule: en.morph.more-most-advmod-adv
what: more, most, less, least as advmod — an adverb (except combinations with ExtPos, like more than)
match: m[form=more|most|less|least, rel=advmod, !feats.ExtPos, !feats.Typo]
require: m[upos=ADV, xpos=RBR|RBS]
severity: warn
source: Santorini 1990, §4.2, pp. 25–27; UD en feat/Degree; EWT 2.18 practice (more than — ExtPos=ADV)
```
