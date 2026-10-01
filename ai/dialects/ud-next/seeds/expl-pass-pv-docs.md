# `expl:pass` and `expl:pv`: pages changed from Czech to universal (documentation)

**Gist.** The universal pages `expl:pass` (reflexive passive) and `expl:pv` (reflexive verbs) used to describe Czech: they had links to `cs-feat/Reflex`, the PDT label `AuxR` and history from UD 1.2 and 2.0. Now they describe Slavic, Romance and Germanic languages, link to the universal `Reflex` and have Czech examples with an English translation and a parallel analysis. The annotation rule did not change.

**Level.** Universal, documentation.

**Before (2.18).** `2.18:https://universaldependencies.org/u/dep/expl-pass.html:8–12`: «Reflexive pronouns (see the feature [cs-feat/Reflex]()) are used in various constructions in Czech … In PDT, their relation to the verb is labeled `AuxR`». `2.18:https://universaldependencies.org/u/dep/expl-pv.html:8–16` — likewise, with the history of the labels `auxpass:reflex` and `compound:reflex`.

**Now (snapshot 24.09.2026).** https://universaldependencies.org/u/dep/expl-pass.html:8–11`: «used in various constructions in Slavic, Romance and Germanic languages, including so-called _reflexive passive,_ which in UD uses the relation subtype `expl:pass`». https://universaldependencies.org/u/dep/expl-pv.html:8–15`: «inherently reflexive (also called pronominal verbs in some grammatical traditions)».

**Evidence.** `diff` of both files.

**What it means for English annotation.** Nothing: the en registry has no `expl:pass` and `expl:pv`, English has only `expl`. For `mova` as a language-independent layer one thing is useful: these subtypes are described as common to three language groups, not as a Czech custom.

**Registry.** Not applicable.

**Sources.** https://universaldependencies.org/u/dep/expl-pass.html, https://universaldependencies.org/u/dep/expl-pv.html.
