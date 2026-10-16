# flat — head on the left, compound — head on the right

**Gist.** In an English compound noun the head is on the right: a *phone book* is a "book", so compound(book, phone), and the dependent stands before the head. `flat` is for headless names (*Hillary Rodham Clinton*): all parts hang on the first word, so `flat` dependents are always on the right. If a `compound` stands after its head, it is either a flat name or a wrong head. `flat` to the left of its head means an inverted parse.

**Conditions and exceptions.**
- `compound:prt` (the particle in *made up*) stands after the verb — this is a different subtype.
- Participles with particles (*rusted out, grown up*) have `compound` after the head in EWT.
- A place name with the generic word in front is `flat` (*Lake Mead, Mount Everest*), with the generic word behind it is `compound` (*Mirror Lake*).

**Examples.**
- *oil price futures* → compound(price, oil), compound(futures, price).
- *Carl XVI Gustaf* → flat(Carl, XVI), flat(Carl, Gustaf).

**Treebank conventions.**
- *Sri Lanka, Hong Kong*: in EWT — `compound` with a right head, in GUM — `flat` with a left head.
- *Marvel Consultants, Inc.*: in EWT the head is *Inc.* with two `compound`s, in GUM the head is *Consultants*.
- Both differences are a legacy of the automatic conversion of PTB trees in EWT.
- In ParTUT `compound` goes to the right more often than in other treebanks: there are more `compound:prt` there, and proper names that others label `flat` are annotated as `compound`.

**Check against gold.** EWT 2.18: `flat` to the right — 2501 of 2501. `compound` between nouns to the left — 8416, violations 16. GUM — 6713/36.

**Sources.** UD `_en/dep/compound.md`, `_en/dep/nmod-desc.md` (Place names), `_en/dep/flat.md`; `2023.udw-1.7` (appendix: Sri Lanka, Marvel Consultants Inc.); `2021.udw-1.14` (Schneider, Zeldes); `2020.udw-1.8` (Dönicke et al.: English `compound` in ParTUT).

The rule is `en.nominal.flat-head-first` in `nominal/flat-names.md`.

The rule is `en.nominal.compound-head-final` in `nominal/compound.md`.
