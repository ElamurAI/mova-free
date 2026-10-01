# Coordinated subjects: and — plural, or/nor — by the nearest

**Gist.** Two or more subjects joined by *and* take a plural verb: *John and Mary **are** here*. Joined by *or*/*nor* — they agree with the nearest: *Neither you nor I **am** concerned*, *Either the boys or the teacher **is** coming*.

**Conditions and exceptions.** Singular with *and* when two names denote one whole (*Bread and butter **is** my breakfast*) or when preceded by *each, every, no* (*Every phrase and every figure **tends**…*). *As well as, with, together with, not* do not form a coordinated subject: *Veracity, as well as justice, **is** our rule*. In *there is a force and beauty* the text often keeps the singular. So the rule only warns.

**Examples.** *He and I are friends.* — *Their sense of humour and calmness **amazes** me* (singular despite and — a feature of the text). — *The ship, with all her furniture, was destroyed.*

**In UD.** A coordinated subject is the first conjunct as `nsubj`, the rest are `conj` to it with `cc`. A `VBZ` verb with an `nsubj` that has a `conj` via *and* is suspicious (except clauses with `expl`).

**Sources.** Brown 1851, Rule XVI ("agree with them jointly in the plural") and Notes II–IV; Rule XVII and Note I ("agree with the nearest"; ); Reed & Kellogg, Higher Lessons, Lesson 20 (Compound subject), Lesson 142; https://universaldependencies.org/en/specific-syntax.html, Coordination.

```rule
rule: en.verbal.vbz-and-subject
what: VBZ with a subject coordinated by and — suspected non-agreement or false coordination
match: v[xpos=VBZ]; s[rel=nsubj|nsubj:pass, head=v]; c[rel=conj, head=s]; k[rel=cc, form=and, head=c]
require: exists e[rel=expl, head=v]
severity: warn
source: Brown 1851 Rule XVI
```
