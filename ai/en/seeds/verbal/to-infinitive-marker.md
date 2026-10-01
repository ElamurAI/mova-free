# To with the infinitive — a function particle (mark)

**Gist.** *To* before an infinitive (*to go*) is neither a preposition nor an auxiliary but the infinitive marker particle. It attaches to the verb it introduces, and that verb is non-finite: *to go* (VB), *to be defanged* (VBN with *be*), *to have gone* (VBN with *have*), *to be going* (VBG with *be*).

**Conditions and exceptions.** An adverb (*to boldly go*) or negation (*to not attempt*) can stand between *to* and the verb. Under ellipsis *to* becomes the head (*update whatever you need to*). Old grammars (Brown, Reed & Kellogg) called *to* a preposition that "governs" the infinitive; UD does not. Early SD and Pullum considered *to* an auxiliary — UD does not accept this either.

**Examples.** *I tried to finish it.* — *It should continue to be defanged.* — *Change anything you need to.* (to — head).

**In UD.** *to*: UPOS `PART`, XPOS `TO`, relation `mark` (never `aux`, `case`). The head is `VB` with `VerbForm=Inf`; if the head is `VBN`, there is an `aux`/`aux:pass` between *to* and it; the head is never finite.

**Sources.** https://universaldependencies.org/en/dep/mark.html ("The infinitive marker to is also analyzed as a mark"); https://universaldependencies.org/en/dep/aux_.html (to is not aux); https://universaldependencies.org/en/pos/PART.html; Brown 1851, Rule XVIII ("The Infinitive Mood is governed … by the preposition TO"); Poutsma 1923, *The Infinitive…*, §2 (to "now mostly" before the infinitive; vol. 2, p. 14), §58 (to be given, to have been given; p. 71).

```rule
rule: en.verbal.to-not-aux
what: infinitival to (PART) is never aux, case, cop
match: t[form=to, upos=PART]
require: not t[rel=aux|aux:pass|case|cop|compound:prt]
severity: error
source: https://universaldependencies.org/en/dep/aux_.html; https://universaldependencies.org/en/dep/mark.html
```

```rule
rule: en.verbal.to-head-nonfinite
what: the head of infinitival to is not a finite form
match: v[upos=VERB|AUX]; t[form=to, upos=PART, rel=mark, head=v]
require: not v[xpos=VBZ|VBP|VBD|MD]
severity: error
source: https://universaldependencies.org/en/feat/VerbForm.html, Inf; Brown 1851 Rule XVIII
```

```rule
rule: en.verbal.to-vb-infinitive
what: VB with infinitival to — VerbForm=Inf
match: v[upos=VERB|AUX, xpos=VB]; t[form=to, upos=PART, rel=mark, head=v]
require: v[feats.VerbForm=Inf]
severity: error
source: https://universaldependencies.org/en/feat/VerbForm.html, Inf
```

```rule
rule: en.verbal.to-vbn-needs-aux
what: to with VBN is possible only via be/have (to be done, to have done)
match: v[upos=VERB, xpos=VBN]; t[form=to, upos=PART, rel=mark, head=v]
require: exists a[rel=aux|aux:pass, head=v, after=t]
severity: error
source: Poutsma 1923 Infinitive §58
```
