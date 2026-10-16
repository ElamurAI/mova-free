# Prepositions with verbs: depend on, discuss without about, arrive at

**Gist.** Prepositions are one of the most frequent categories of learner errors: PREP is about 10 % of edits. ERRANT distinguishes three kinds:
- wrong (R:PREP): *depend of → depend on*, *arrive to → arrive at/in*;
- unnecessary (U:PREP): *discuss about → discuss*;
- missing (M:PREP): *listen music → listen to music*.

The preposition with a verb is lexical knowledge; it has to be remembered for each verb. In UD the error shows up as `obl` with a "foreign" `case` or as `obj` where `obl` is needed.

**Conditions and exceptions.**
- *depending on* is a fixed expression.
- *arrived to find…* is an infinitive of purpose: there *to* is `mark`, not `case`, so the rule does not take it.

**Examples.**
- ✗ *Depends of what you want.*
- ✗ *We discussed about the plan.*
- ✗ *I listen music.*

**Check against gold.** EWT 2.18:
- depend + preposition — 8/1 (*Depends of what*);
- discuss + about — 0;
- arrive + to — 27/0;
- listen + obj — 0.

**Sources.** `P17-1074` (table 2: PREP, *(look) in → (look) at*); `W19-4406` (table 4: PREP 9.79 % W&I train, 11.21 % FCE); `2025.acl-long.1026` (CTSEG: minimal pairs by CEFR level).

```rule
rule: en.errors.depend-on
what: depend with a preposition other than on/upon (depend of)
match: v[lemma=depend]; p[rel=obl, head=v]; c[rel=case, head=p]
require: c[lemma=on|upon]
severity: warn
source: P17-1074 (R:PREP); W19-4406
```

```rule
rule: en.errors.discuss-no-about
what: discuss about — discuss takes a direct object without a preposition
match: v[lemma=discuss]; p[rel=obl, head=v]; c[rel=case, head=p, lemma=about]
require: not c[lemma=about]
severity: warn
source: P17-1074 (U:PREP); W19-4406
```

```rule
rule: en.errors.arrive-not-to
what: arrive to + place — should be arrive at/in
match: v[lemma=arrive]; p[rel=obl, head=v]; c[rel=case, head=p]
require: not c[lemma=to]
severity: warn
source: P17-1074 (R:PREP); W19-4406
```

```rule
rule: en.errors.listen-to
what: listen with a direct object — the preposition to is needed (listen to music)
match: v[lemma=listen]; o[rel=obj, head=v]
require: not o[rel=obj]
severity: warn
source: P17-1074 (M:PREP); W19-4406
```
