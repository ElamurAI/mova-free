# Bare infinitive after make, let, see, hear…

**Gist.** After verbs of causing and permitting *make, let, bid, have* (causative) and of perception *see, hear, feel, watch, notice, observe*, an infinitive with an object stands without *to*: *You make me blush*, *Let us go*, *I heard him say so*, *I felt something sting me*. *Help* allows both: *help me (to) move*. In the passive *to* comes back: *He was made **to** wait*, *He was seen **to** go*.

**Conditions and exceptions.** *See* in the sense "acknowledge, understand" takes *to be*: *I saw it to be so*. *Feel* without *to* is only for bodily sensation; for opinion it takes *to*: *I feel it to be my duty*. *Know* in older English also goes without *to* (*I have known him do it*). *Have* is excluded from the rule, because *have to* is a different construction.

**Examples.** *They let him leave.* — *\*Nobody saw him to leave* (in the active, without to). — *He was heard to say so* (passive, with to).

**In UD.** The infinitive is `xcomp` (`VB`, `VerbForm=Inf`) of the verb; in the active without `mark(to)`, in the passive (`Voice=Pass`) with `mark(to)`.

**Sources.** Brown 1851, Rule XIX (bid, dare, feel, hear, let, make, need, see — «without the preposition TO») and Obs. 5 (in the passive — to), Obs. 8 (feel), Obs. 10 (let), Obs. 11 (make); Poutsma 1923, *The Infinitive…*, §35 (perceiving: without to, except to be; vol. 2, p. 49), §43 (after a passive — to, also after let; p. 57); Jespersen, MEG V, ch. XVIII «Subject + Infinitive as Object» (text-1, p. 304).

```rule
rule: en.lexicon.bare-infinitive-active
what: after active make/let/see/hear/watch/feel/bid/notice/observe the infinitive xcomp has no to
match: h[lemma=make|let|see|hear|watch|feel|bid|notice|observe, !feats.Voice]; v[xpos=VB, rel=xcomp, head=h]
require: none t[form=to, rel=mark, head=v]
severity: warn
source: Brown 1851 Rule XIX; Poutsma 1923 Infinitive §35
```

```rule
rule: en.lexicon.to-infinitive-after-passive
what: after passive make/see/hear… the infinitive takes to
match: h[lemma=make|see|hear|watch|feel|bid|notice|observe, feats.Voice=Pass]; v[xpos=VB, rel=xcomp, head=h]
require: exists t[form=to, rel=mark, head=v]
severity: warn
source: Brown 1851 Rule XIX, Obs. 5; Poutsma 1923 Infinitive §43
```
