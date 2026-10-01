# Subject "X and Y" — the verb is plural

**Gist.** A coordinated subject with the conjunction *and* is plural: *Mother and children were killed*. Fowler cites the error *Their lives, their liberties, & their religion is in danger*. In his view it comes from the false idea that the verb agrees with the nearest conjunct. For *or/nor* the rule is different: the verb agrees with the nearest alternative (*Mother or children are to die*).

**Conditions and exceptions.**
- One entity in two words is singular: *bread and butter is*, firm names (*Cullen and Dykman is*), titles of works (*Monotheism and Holy War does…*).
- Several descriptions of one person: *Interim leader and front-runner Mahmoud Abbas talks*.
- A verb before the subject allows the singular (*There is a table and some chairs*, Fowler §7). So the rule takes only a subject before the verb.
- *each/every X and Y* is singular.

**Examples.**
- ✗ *green curry and red curry is awesome* → ✓ *are*.
- ✗ *Mercury and Venus is conjunct*.
- ✓ *Mother or children are to die*.

**In UD.** nsubj(v, X), conj(X, Y), cc(Y, and). The number is visible from the XPOS of the verb, the copula or the first auxiliary: VBZ with such a subject is a VERB:SVA suspect.

**Check against gold.** EWT 2.18: verb — fired 394, violations 5; copula or auxiliary — 267/11. The violations are real agreement errors in the text, firm names, and descriptions of one person.

**Sources.** Fowler MEU 1926, NUMBER §2–3, 7 (pp. 400–402); Fowler, IS §4 «Is after compound subjects» (ibid., p. 310); `P17-1074` (ERRANT, VERB:SVA); `2020.tacl-1.25` (BLiMP, subject-verb agreement).

```rule
rule: en.errors.coord-subject-verb
what: a coordinated subject with and before the verb, and the verb is VBZ — plural agreement broken
match: v[]; s[rel=nsubj|nsubj:pass, head=v, before=v]; c[rel=conj, head=s]; k[rel=cc, lemma=and, head=c]
require: not v[xpos=VBZ]
unless: c[after=v]; exists d[rel=det, lemma=each|every, head=s]
severity: warn
source: Fowler MEU 1926, NUMBER §2 (pp. 400–402), IS §4 (p. 310); P17-1074 (VERB:SVA); exceptions from the seed: a conjunct after the verb (agreement with what precedes it, Fowler §7); each/every X and Y — singular
```

```rule
rule: en.errors.coord-subject-auxcop
what: a coordinated subject with and, and the copula or auxiliary after it is VBZ (is, has, does)
match: h[]; s[rel=nsubj|nsubj:pass, head=h]; c[rel=conj, head=s]; k[rel=cc, lemma=and, head=c]; a[rel=cop|aux|aux:pass, head=h, after=s]
require: not a[xpos=VBZ]
unless: c[after=a]; exists d[rel=det, lemma=each|every, head=s]
severity: warn
source: Fowler MEU 1926, NUMBER §2 (pp. 400–402), IS §4 (p. 310); P17-1074 (VERB:SVA); exceptions from the seed: a conjunct after the verb (agreement with what precedes it, Fowler §7); each/every X and Y — singular
```
