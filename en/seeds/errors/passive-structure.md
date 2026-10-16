# Passive: nsubj:pass and aux:pass — only with a VBN participle

**Gist.** The English passive is *be* or *get* + past participle (VBN). Hence three consequences:
- `aux:pass` and `nsubj:pass` are possible only with a head with XPOS VBN;
- if there is `aux:pass`, the subject must be `nsubj:pass`, not `nsubj`.

Typical mistakes:
- the subject of an intransitive change-of-state verb (*The door opened*, *The ice melted*) is marked `nsubj:pass`, although there is no passive;
- *be* + adjective in -ed (*is interested*) is confused with the passive;
- the perfect (*has gone*, in German and Alsatian — *ist gekommen*) is confused with a copula. This is exactly how all three pre-annotation systems erred in the Alsatian project.

**Conditions and exceptions.** Reduced passives without an auxiliary are legitimate: headlines (*Passport needed*, *Key suspect arrested*), absolute constructions (*many staffed by former officers*). There `nsubj:pass` is with VBN, but without `aux:pass` (EWT: 40).

**Examples.**
- *The cat was chased by the dog* → nsubj:pass(chased, cat), aux:pass(chased, was), obl:agent(chased, dog).
- *The door opened* → nsubj(opened, door).
- *He got fired* → aux:pass(fired, got).

**Check against gold.** EWT 2.18:
- `nsubj:pass` with a non-VBN head — 0 of 1445;
- `aux:pass` with a non-VBN head — 0 of 1643;
- `aux:pass` together with plain `nsubj` — 0.

GUM — 2, 0 and 3.

**Sources.** UD `_en/dep/nsubj-pass.md`, `_en/dep/aux-pass.md`, `_en/dep/cop.md` (I was given a horse); `2025.law-1.14` (Bernhard et al.: all three tools confused the perfect with a copula); `P17-1074` (ERRANT VERB:FORM).

```rule
rule: en.errors.nsubjpass-vbn
what: nsubj:pass with a non-VBN head — a passive subject occurs only with a participle (The door opened → nsubj)
match: v[]; s[rel=nsubj:pass, head=v]
require: v[xpos=VBN]
severity: error
source: UD _en/dep/nsubj-pass.md; 2025.law-1.14
```

```rule
rule: en.errors.auxpass-vbn
what: aux:pass with a non-VBN head — the passive auxiliary stands only before a participle
match: v[]; a[rel=aux:pass, head=v]
require: v[xpos=VBN]
severity: error
source: UD _en/dep/aux-pass.md; P17-1074 (VERB:FORM)
```

```rule
rule: en.errors.auxpass-subject
what: with aux:pass the subject is marked nsubj — should be nsubj:pass
match: v[]; a[rel=aux:pass, head=v]; s[rel=nsubj, head=v]
require: not s[rel=nsubj]
severity: error
source: UD _en/dep/nsubj-pass.md, _en/dep/aux-pass.md
```
