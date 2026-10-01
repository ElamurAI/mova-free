# Perfect: have + past participle

**Gist.** Perfect forms are built with *have* and the past participle (third form): *has gone, had seen, will have finished*. The perfect by itself is not passive: *Moths have eaten the painting* is active voice, although *eaten* has the same form as in the passive.

**Conditions and exceptions.** The archaic perfect with *be* (*He is come*, *they are gone*) survives in modern English only in *be gone, be done* — usually as an adjective after a copula. *Have* with an infinitive (*have to go*) is neither perfect nor auxiliary (see `semi-modals.md`). *Have* meaning "possess" is a lexical `VERB`. The passive perfect has both auxiliaries: *has **been** eaten* (`aux` + `aux:pass`).

**Examples.** *I have eaten the plums.* — *It has been eaten by moths* (passive: *been* is present). — *\*She has go.*

**In UD.** *have* — `aux`, `AUX`; the head is `VERB` with XPOS `VBN` (or `VBG` via *been*: *has been lying*), but not `VB`, `VBZ`, `VBP`, `VBD`. A perfect participle without `aux:pass` does not have `Voice=Pass`.

**Sources.** Jespersen, MEG IV (Time and Tense), ch. III "Auxiliaries of the Perfect and Pluperfect" (p. 65 ff.); Poutsma 1923, *Participles*, §8 (the participle is "purely verbal" in the perfect; vol. 2, p. 199); https://universaldependencies.org/en/feat/Voice.html (perfect ≠ passive); https://universaldependencies.org/en/pos/AUX_.html (have: AUX or VERB).

```rule
rule: en.verbal.perfect-have-participle
what: with perfect have the main verb is a participle, not VB/VBZ/VBP/VBD
match: h[upos=VERB]; a[lemma=have, rel=aux, head=h]
require: not h[xpos=VB|VBZ|VBP|VBD]
severity: error
source: Jespersen MEG IV ch. III; https://universaldependencies.org/en/feat/Voice.html
```

```rule
rule: en.verbal.perfect-not-passive
what: VBN with perfect have and Voice=Pass must have a passive auxiliary
match: h[xpos=VBN, feats.Voice=Pass]; a[lemma=have, rel=aux, head=h]
require: exists p[rel=aux:pass, head=h]
severity: warn
source: https://universaldependencies.org/en/feat/Voice.html ("probably an active perfect clause")
```
