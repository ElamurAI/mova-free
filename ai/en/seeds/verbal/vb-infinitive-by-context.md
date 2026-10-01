# Base form VB: infinitive or finite

**Gist.** The form *go, be, take* (tag `VB`) varies grammatically. If it has *to*, a modal or another auxiliary (*to go, can go, did not go*) or it serves as a complement of another verb (*let him go, made me cry*), it is an infinitive. If it forms a predicate on its own without an auxiliary, it is a finite form: imperative (*Go!*) or subjunctive (*that he go*).

**Conditions and exceptions.** The exception is imperative *do*: with it EWT gives the main verb `Imp/Fin` (see `imperative.md`). A bare infinitive in `xcomp` (*let him go*, *help me move*) is also `Inf`.

**Examples.** *I have to leave* (Inf). — *He should leave* (Inf). — *Leave now!* (Fin, Imp). — *They let him leave* (Inf, xcomp).

**In UD.** `VB` with `aux`/`aux:pass`, except imperative *do*, is `VerbForm=Inf`; `VB` as `xcomp` is always `VerbForm=Inf`.

**Sources.** https://universaldependencies.org/en/feat/VerbForm.html (Fin: VB without auxiliary; Inf: with auxiliary, modal or to); Brown 1851, Rule XVIII–XIX (infinitive with and without to); Poutsma 1923, *The Infinitive…*, §1 (the nature of the infinitive; vol. 2, p. 13).

```rule
rule: en.verbal.vb-aux-infinitive
what: VB with an auxiliary (not imperative do) is an infinitive
match: v[xpos=VB, upos=VERB]; a[rel=aux|aux:pass, head=v, feats.Mood!=Imp]
require: v[feats.VerbForm=Inf]
severity: error
source: https://universaldependencies.org/en/feat/VerbForm.html, Inf
```

```rule
rule: en.verbal.xcomp-vb-infinitive
what: VB as xcomp is an infinitive
match: x[rel=xcomp, xpos=VB]
require: x[feats.VerbForm=Inf]
severity: error
source: https://universaldependencies.org/en/feat/VerbForm.html; https://universaldependencies.org/en/dep/xcomp.html (always non-finite)
```
