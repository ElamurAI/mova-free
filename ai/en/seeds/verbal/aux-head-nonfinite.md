# The verb with an auxiliary is always non-finite

**Gist.** The finite form (tense, person, number) in the predicate is carried by the first auxiliary. The main verb after an auxiliary is non-finite: an infinitive (*can go*), an *-ing* participle (*is going*) or an *-ed/-en* participle (*has gone, was taken*). An *-s* form or past tense after an auxiliary is impossible (*\*does goes, \*will went*).

**Conditions and exceptions.** If the error is in the text itself (*does has*), the annotation must reflect the form, and the rule will show the text error — that is useful too. When the auxiliary goes with a nominal predicate (*is a doctor*), it is `cop`, and the rule does not apply.

**Examples.** *She **will go**.* — *They **have eaten**.* — *He **was killed**.* — *\*He does goes* (text error).

**In UD.** The head of `aux`/`aux:pass`, if it is a verb, has XPOS `VB`, `VBG` or `VBN`, but never `VBZ`, `VBP`, `VBD`, `MD`.

**Sources.** Brown 1851, Part II, Ch. VI ("prefixed to one of the principal parts"; ); https://universaldependencies.org/en/feat/VerbForm.html (Inf/Part with an auxiliary); Reed & Kellogg, Higher Lessons, Lesson 131–132 (verb forms; ).

```rule
rule: en.verbal.aux-head-nonfinite
what: a verb with an auxiliary cannot be a finite form
match: h[upos=VERB|AUX]; a[rel=aux|aux:pass, head=h]
require: not h[xpos=VBZ|VBP|VBD|MD]
severity: error
source: Brown 1851 Part II Ch. VI (auxiliary); https://universaldependencies.org/en/feat/VerbForm.html
```
