# Verbs that take an infinitive, not a gerund

**Gist.** Verbs of wanting, intending, deciding, attempting and being able take a *to*-infinitive as complement: *I want to go*, *We decided to stay*, *She managed to escape*, *He pretended not to see*. A gerund after them is impossible: *\*I want going*. The infinitive here "looks into the future" relative to the main action, unlike the gerund.

**Conditions and exceptions.** Colloquial *want* + object + *-ing* (*You don't want it getting too warm*) is a different construction (object with a participle). *Seem/tend to be going* is a progressive infinitive, where the `VBG` has `to be`. Verbs with both constructions (*begin, start, continue, like, love, hate, prefer*; with a change of meaning — *remember, forget, stop, try, regret*) are not listed here.

**Examples.** *We hope to see you.* — *He refused to answer.* — *\*She decided going home.*

**In UD.** An `xcomp` with `VBG` under these verbs has its own `aux`/`mark` (to be going); without them it is suspect.

**Sources.** Poutsma 1923, *The Gerund*, §44 (the infinitive keeps the time distinction, «except so far as futurity»; vol. 2, p. 152); Poutsma, *GLME*, Part II, ch. XIX, §§19–20 (the infinitive is the norm after verbs with a personal object; verbs with both constructions; vol. 2, p. 326); Jespersen, MEG V, ch. XII «Infinitive as Object» (text-1, p. 210).

```rule
rule: en.lexicon.infinitive-verb-no-gerund
what: want/hope/decide… take no gerund xcomp (except to be + -ing)
match: v[lemma=want|hope|decide|agree|refuse|promise|plan|expect|manage|fail|afford|offer|pretend|seem|tend|wish|choose|deserve|threaten|learn]; x[rel=xcomp, xpos=VBG, head=v]
require: exists a[rel=aux|mark, head=x]
severity: warn
source: Poutsma 1923 Gerund §44; Poutsma GLME II ch. XIX §§19–20; Jespersen MEG V ch. XII
```
