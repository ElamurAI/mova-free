# Copula-like verbs: become, seem, remain…

**Gist.** The verbs *become, get, grow, turn, go, come, fall* (becoming: *go mad, fall ill, come true*) and *seem, appear, look, sound, feel, remain, stay, keep, prove* (being, appearance) link the subject to an adjective or noun just as *be* does: *I became very upset*, *She looks beautiful*, *It remains a mystery*. But in UD the copula is only *be*; these verbs are ordinary `VERB`s, and the predicative with them is `xcomp`, not `obj`: the noun after *become* does not undergo an action.

**Conditions and exceptions.** Many of them also have a transitive sense: *prove a theorem*, *sound the alarm*, *keep a secret*, *feel the heat*, *look at*, *get a letter*, *turn the page*, *That dress becomes you* (rare "suits"). So the rule covers only those that have almost no transitive sense.

**Examples.** *She became a doctor* — `xcomp(became, doctor)`. — *It seems a good idea* — `xcomp`. — *He proved his identity* — `obj` (a different sense).

**In UD.** *become, seem, appear, remain, stay, tend* have no `obj`.

**Sources.** https://universaldependencies.org/en/dep/cop.html (become, get, seem — raising verbs with xcomp); https://universaldependencies.org/en/specific-syntax.html, Functional control («copula-like English verbs such as become, remain»); Jespersen, MEG III, ch. XVII–XVIII «Predicatives» (predicatives of becoming; text-3, p. 369); Reed & Kellogg, Higher Lessons, Lesson 29 (*The maple leaves become red*); PropBank 3.1 `become.xml` (become.01: ARG2 — «new state», PRD), `seem.xml`.

```rule
rule: en.lexicon.copula-like-no-object
what: become/seem/appear/remain/stay/tend have no direct object (the predicative is xcomp)
match: v[lemma=become|seem|appear|remain|stay|tend]; o[rel=obj, head=v]
require: not o[rel=obj]
severity: warn
source: https://universaldependencies.org/en/dep/cop.html; Jespersen MEG III ch. XVII
```
