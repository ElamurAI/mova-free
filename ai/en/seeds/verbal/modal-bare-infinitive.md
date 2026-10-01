# After a modal — a bare infinitive

**Gist.** A modal verb (*can, may, must, will, shall* etc.) requires an infinitive without *to* after it: *I can swim*, *you must go*. If the predicate has more auxiliaries, the next one after the modal is also an infinitive: *might **have** been*, *could **be** done*. A finite form after a modal is impossible, and two modals in a row are not used in the standard language.

**Conditions and exceptions.** *Ought* takes *to* (*ought to go*), but *ought* itself stays a modal `AUX`. Double modals (*might could*) occur in Southern US dialects — so the second rule only warns; more often "two modals" in one predicate mean that one of them is wrongly attached to someone else's verb (different subordinate clauses).

**Examples.** *He should leave.* — *Reagan might have been lying.* — *\*He can goes.* — *\*The hope may … can be offered* (the second modal is from another clause).

**In UD.** The modal is `aux` with XPOS `MD`; other `aux`/`aux:pass`/`cop` of the same head to its right do not have XPOS `VBZ`, `VBP`, `VBD`.

**Sources.** Poutsma 1923, *The Infinitive…*, §4 (without to "after any auxiliary of mood or tense", after can, may, must, shall, will, after do; vol. 2 (text-2), p. 20); Poutsma 1923, §§60–62 (modals + perfect infinitive, p. 72); Brown 1851, Part II, Ch. VI (potential mood: may, can, must + verb); https://universaldependencies.org/en/dep/aux_.html.

```rule
rule: en.verbal.modal-next-nonfinite
what: other auxiliaries or the copula after a modal are non-finite
match: h[]; m[xpos=MD, rel=aux, head=h]; x[rel=aux|aux:pass|cop, head=h, after=m]
require: not x[xpos=VBZ|VBP|VBD]
severity: error
source: Poutsma 1923 Infinitive §4, §60; https://universaldependencies.org/en/dep/aux_.html
```

```rule
rule: en.verbal.double-modal
what: two modals in one predicate — dialect or an attachment error
match: h[]; m[xpos=MD, rel=aux, head=h]; n[xpos=MD, rel=aux, head=h, after=m]
require: not n[xpos=MD]
severity: warn
source: Brown 1851 Part II Ch. VI (potential mood); https://universaldependencies.org/en/specific-syntax.html, Auxiliaries
```
