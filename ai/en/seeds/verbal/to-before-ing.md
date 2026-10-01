# To before -ing — a preposition, not a particle

**Gist.** After some words *to* is a preposition, and then it is followed not by an infinitive but by an *-ing* form (gerund): *I look forward **to seeing** you*, *She objected to being called*, *used to working*, *with a view to reducing*. The infinitive particle *to* before *-ing* is possible only in the progressive infinitive with *be*: *to be going*.

**Conditions and exceptions.** Test: can a noun replace the *-ing*? *look forward to the trip* — yes, so *to* is a preposition. *I want to go* — *\*I want to the trip* — no, it is the particle.

**Examples.** *I look forward to seeing you* (to — ADP). — *She seems to be going* (to — PART on *going* via *be*).

**In UD.** *to* as `PART` with a `VBG` head must have an `aux` between them; otherwise *to* is `ADP` (with `mark`, since it introduces a clause).

**Sources.** https://universaldependencies.org/en/pos/PART.html (PART to — only the infinitive marker); https://universaldependencies.org/en/dep/mark.html (a preposition before a clause is mark); https://universaldependencies.org/en/feat/VerbForm.html (Ger: *I look forward to seeing you*, comment); Poutsma 1923, *The Gerund*, §§43–45 (gerund vs infinitive; vol. 2, pp. 152–153).

```rule
rule: en.verbal.to-part-before-ing
what: the particle to with VBG is possible only with an auxiliary (to be going); otherwise to is the preposition ADP
match: v[xpos=VBG]; t[form=to, upos=PART, head=v]
require: exists a[rel=aux|aux:pass, head=v, after=t]
severity: error
source: https://universaldependencies.org/en/pos/PART.html; https://universaldependencies.org/en/dep/mark.html
```
