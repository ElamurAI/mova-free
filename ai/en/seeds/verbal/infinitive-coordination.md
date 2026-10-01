# Coordinated infinitives: to once

**Gist.** When two infinitives with the same role are joined by a conjunction, *to* is often put only before the first: *to read and write*, *to go to the website and click "contact us"*. The second infinitive does not become a finite form because of this: it is the same infinitive, sharing the *to*.

**Conditions and exceptions.** With three or more infinitives, *to* is mostly repeated before each; repetition stresses the separateness of the actions. After *but* and *than* the rules differ (Poutsma §§44–48).

**Examples.** *My advice is to go to the website and click "contact us".* — *She wanted to read and (to) write.*

**In UD.** The second infinitive is a `conj` of the first, has no `mark(to)` of its own, but stays `VB` with `VerbForm=Inf`.

**Sources.** Poutsma 1923, *The Infinitive…*, §§53–55 (repetition and non-repetition of to; vol. 2, pp. 67–68); Brown 1851, Rule XVII, Note V (coordinated verbs take the same form); https://universaldependencies.org/en/feat/VerbForm.html, Inf.

```rule
rule: en.verbal.conj-infinitive-shares-to
what: a VB coordinated with a to-infinitive is also an infinitive
match: h[xpos=VB]; t[form=to, rel=mark, head=h]; c[rel=conj, xpos=VB, upos=VERB, head=h]
require: c[feats.VerbForm=Inf]
severity: warn
source: Poutsma 1923 Infinitive §§53–55
```
