# A clausal subject requires the singular

**Gist.** When the subject is a whole clause, an infinitive or a gerund phrase, the verb is third person singular: *To lie **is** base*, *That she lied **was** suspected*, *Taking a nap **relaxes** you*.

**Conditions and exceptions.** Several clausal subjects joined by *and* take the plural: *To be wise in our own eyes, to be wise in the opinion of the world … **are** three things*. Joined by *or* — the singular.

**Examples.** *Whether he lied **is** beside the point.* — *What he says **is** true.* — *\*To err are human.*

**In UD.** A head that has `csubj`/`csubj:pass` (without conjuncts) is not `VBP` and has no `VBP` among its `cop`/`aux`.

**Sources.** Brown 1851, Rule XIV, Note III ("requires a verb in the third person singular"); Rule XVI, Note VII; Rule XVII, Note IV; https://universaldependencies.org/en/dep/csubj.html.

```rule
rule: en.verbal.csubj-not-vbp
what: a VBP predicate with a clausal subject — suspicious
match: v[xpos=VBP]; c[rel=csubj|csubj:pass, head=v]
require: exists k[rel=conj, head=c]
severity: warn
source: Brown 1851 Rule XIV, Note III
```

```rule
rule: en.verbal.csubj-not-vbp-aux
what: a VBP copula or auxiliary with a clausal subject — suspicious
match: h[]; c[rel=csubj|csubj:pass, head=h]; a[xpos=VBP, rel=cop|aux|aux:pass, head=h]
require: exists k[rel=conj, head=c]
severity: warn
source: Brown 1851 Rule XIV, Note III
```
