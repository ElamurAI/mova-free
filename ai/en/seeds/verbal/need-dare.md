# Need and dare: modal and lexical

**Gist.** *Need* and *dare* have two behaviours. As modals they have no *-s*, take no *do* and go with a bare infinitive, mostly in questions and negations: *Need I say more?*, *He need not go*, *How dare you?*. As lexical verbs they inflect, take *do* and *to*: *He needs to go*, *You don't need to shout*.

**Conditions and exceptions.** *Dare* even after *do* is often without *to*: *don't dare go* — but this is already lexical *dare* with a bare `xcomp`. *I dare say* is frozen. Brown considered *need* with a bare infinitive an auxiliary ("an auxiliary of the potential mood") — UD does the same.

**Examples.** *You needn't shout* (AUX). — *You need to shout* (VERB + xcomp). — *He dare not come* (AUX).

**In UD.** Modal: `AUX`, `aux`, head without `mark(to)`. Lexical: `VERB`, the infinitive is `xcomp` with `mark(to)`.

**Sources.** https://universaldependencies.org/en/pos/AUX_.html (Need I say more?; You needn't shout); Brown 1851, Rule XIX, Obs. 7 (dare), Obs. 12–13 (need as auxiliary); Poutsma 1923, *The Infinitive…*, §§6–15 (need; vol. 2, pp. 21–30), §§16–31 (dare; pp. 31–42).

```rule
rule: en.verbal.need-aux-bare
what: modal need/dare (AUX) — no to with the infinitive
match: h[]; n[lemma=need|dare, upos=AUX, head=h]
require: none t[form=to, rel=mark, head=h]
severity: error
source: Brown 1851 Rule XIX Obs. 12; https://universaldependencies.org/en/pos/AUX_.html
```

```rule
rule: en.verbal.need-verb-to
what: lexical need with an infinitive — with to
match: n[lemma=need, upos=VERB]; x[rel=xcomp, xpos=VB, head=n]
require: exists t[form=to, rel=mark, head=x]
severity: warn
source: Poutsma 1923 Infinitive §§6–14; https://universaldependencies.org/en/pos/AUX_.html
```
