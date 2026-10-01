# Do-support: do + infinitive

**Gist.** In negation, questions, emphasis and ellipsis, when there is no other auxiliary, English adds *do*: *I do not love*, *Did you see?*, *I **do** believe it*. After *do* the main verb is an infinitive without *to*. *Do* never combines with modals or with perfect *have*: *\*does can*, *\*did have eaten*.

**Conditions and exceptions.** In the imperative *do* combines with *be* and the passive: *Don't be silly*, *Do be informed*, *did not get assigned* — then the head can be an adjective or a `VBN` participle (with `aux:pass`). *Do* as a lexical verb (*do the work*) is `VERB` (see `aux-no-object.md`). Old "redundant" *do* (*all the beasts … do creep forth*) is also do-support.

**Examples.** *He does not know.* — *Don't be evil.* — *\*He did not went.* — *\*She doesn't can swim.*

**In UD.** *do* — `aux`, `AUX`; a verb head has XPOS `VB` (or `VBN` with `aux:pass`), but not `VBZ`, `VBP`, `VBD`, `VBG`; among the other `aux` of the same head there are no modals and no *have*.

**Sources.** Brown 1851, Part II, Ch. VI, "IV. Form of Negation", "V. Form of Question" (); Brown 1851, Obs. 11 to the definition of auxiliary (do as expletive); Poutsma 1923, *The Infinitive…*, §4 c (without to after do; vol. 2, p. 20); Jespersen, MEG V, ch. XXIII "Negation" (text-1, p. 438 ff.); UD registry: `aux do int,neg`.

```rule
rule: en.verbal.do-support-base
what: after do-support the main verb is neither finite nor -ing
match: h[upos=VERB]; d[lemma=do, rel=aux, head=h]
require: not h[xpos=VBZ|VBP|VBD|VBG|MD]
severity: error
source: Brown 1851 Part II Ch. VI (Forms of Negation, Question); Poutsma 1923 Infinitive §4
```

```rule
rule: en.verbal.do-support-vbn-pass
what: VBN with do is possible only with a passive auxiliary (did not get assigned)
match: h[upos=VERB, xpos=VBN]; d[lemma=do, rel=aux, head=h]
require: exists p[rel=aux:pass, head=h]
severity: error
source: Brown 1851 Part II Ch. VI (Form of Negation); https://universaldependencies.org/en/dep/aux-pass.html
```

```rule
rule: en.verbal.do-support-no-modal
what: do does not combine with modals and perfect have
match: h[]; d[lemma=do, rel=aux, head=h]
require: none a[rel=aux|aux:pass, head=h, lemma=can|could|may|might|must|shall|should|will|would|have|ought]
severity: error
source: Brown 1851 Part II Ch. VI (conjugation: do only in simple forms)
```
