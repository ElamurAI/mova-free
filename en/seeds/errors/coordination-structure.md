# Coordination: the head is the first conjunct, cc is on the next one

**Gist.** In UD v2 a coordinated group "X, Y and Z" is headed by X: Y and Z hang on X as `conj`, and the conjunction *and* is `cc` of Z, i.e. of the conjunct following it. So `conj` always goes left to right, and `cc` stands before its head. In SD and UD v1, `cc` hung on the first conjunct. Old converters and models trained on old data repeat this scheme.

**Conditions and exceptions.**
- A conjunction at the start of a sentence (*But he left*) is `cc` of the root, also on the left.
- Paired conjunctions *either/both/neither* are `cc:preconj`.

**Examples.**
- *apples and pears* → conj(apples, pears), cc(pears, and).
- *He came, saw and conquered* → conj(came, saw), conj(came, conquered), cc(conquered, and).

**In UD.**
- The left-to-right direction of `conj` is checked by the UD validator.
- Moving `cc` to the next conjunct is the main source of non-projectivity in GUM after conversion.
- The structure itself is asymmetric: the head is the first conjunct. It is criticized: conjunct lengths in PTB are explained only by symmetric models.
- The Enhanced UD converter systematically errs when conjuncts differ in tense or mood.

**Check against gold.** EWT 2.18:
- `conj` to the right of the head — 9395 of 9395;
- `cc` before the head — 8292, violations 3.

**Sources.**
- UD `_en/dep/conj.md`, `_en/dep/cc.md`.
- english-banks §1.3 (v1 → v2: cc on the next conjunct).
- `W18-4918` (non-projectivity from cc).
- `2023.acl-long.864` (Przepiórkowski, Woźniak: asymmetric models of coordination do not explain conjunct lengths).
- `2021.eacl-main.67` (Grünewald et al.: 1417 EWT sentences, Enhanced UD converter mistakes on coordination).
- `maier-2012-annotating-coordination-penn-treebank` (in PTB a coordinating comma is not distinguished from an ordinary one; only ~14 % of commas are coordinating).

```rule
rule: en.errors.conj-rightward
what: conj goes right to left — the head of a coordinated group should be the first conjunct
match: h[]; c[rel=conj, head=h]
require: c[after=h]
severity: error
source: UD _en/dep/conj.md; UD validator (conj left to right)
```

```rule
rule: en.errors.cc-before-conjunct
what: cc stands after its head — in UD v2 the conjunction hangs on the next conjunct (the v1/SD scheme attached it to the first)
match: h[]; c[rel=cc, head=h]
require: c[before=h]
severity: warn
source: UD _en/dep/cc.md; english-banks §1.3 (UD v1 → v2)
```
