# One subject per predicate

**Gist.** Every clause (every predicate) has at most one subject. If there seem to be two subjects, then either they are coordinated (*John and Mary* — one subject of two parts), or one of them belongs to another, subordinate clause, or it is a parenthetical, a vocative or a "dislocated" element (*My brother, he never listens*).

**Conditions and exceptions.** The only legitimate case of two subjects is a predicate clause in a copular sentence: *The problem is that this has never been tried* — the outer subject *problem* is marked `nsubj:outer`, and the inner *this* is `nsubj:pass` (see `outer-subject.md`). The "filler" pronoun *there/it* is not a subject (`expl`).

**Examples.** *Clinton defeated Dole* — one `nsubj`. — *My brother, he never listens* — *brother* is `dislocated`, not a second subject.

**In UD.** Among the children of one node, at most one of `nsubj`, `nsubj:pass`, `csubj`, `csubj:pass`.

**Sources.** UD 2.18 validator, test `too-many-subjects` (`udtools/level3.py`, check_single_subject); UD docs/changes.md` "Multiple Subjects" (v2.10); https://universaldependencies.org/en/dep/nsubj-outer.html; Brown 1851, Rule II ("subject of a finite verb… nominative") and Rule III (apposition).

```rule
rule: en.verbal.single-subject
what: a predicate has at most one inner subject
match: h[]; s[rel=nsubj|nsubj:pass|csubj|csubj:pass, head=h]
require: none t[rel=nsubj|nsubj:pass|csubj|csubj:pass, head=h, after=s]
severity: error
source: UD 2.18 validator too-many-subjects; UD docs/changes.md Multiple Subjects
```
