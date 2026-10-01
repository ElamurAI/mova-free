# Deliberate non-standard — Style, not Typo

**Gist.** EWT distinguishes an error (Typo=Yes) from a deliberate colloquial or dialect form (Style=Vrnc, Coll, Slng). Examples from EWT:
- *should of took*: *of* — AUX with lemma *have*, VB, Style=Vrnc;
- *them boys*: *them* — DT (`det`), Style=Vrnc;
- *ain't*: *ai* — lemma *be*, Style=Vrnc;
- *cos, coz, cus*: lemma *because*, Abbr=Yes, Style=Vrnc;
- *ya, 'em*: Style=Coll.

An annotator that "corrects" such forms (tags *of* as ADP or sets Typo=Yes) loses register information. An annotator that does not see the substitution *have → of* breaks the tree: *of* becomes a preposition without a noun.

**Conditions and exceptions.** The boundary between a misspelling and a deliberate form is blurry: *walkin, goin* in EWT have both Style=Vrnc and Typo=Yes.

**Examples.**
- *he should of called back* → of: AUX, lemma have, VB, Style=Vrnc; aux(called, of).
- *them apples* → them: DT, det, Style=Vrnc.

**Check against gold.** EWT 2.18: of/AUX — 3/0; them/det — 1/0. Total Style in EWT: Vrnc — 19, Coll — 4, Slng — 12, Expr — 49.

**Sources.** EWT 2.18 practice (the Style feature); `2025.udw-1.17` (Masciolini et al.: non-standard in L2 annotation, code-switching, borrowings); english-banks §1.4 (Webtext addendum: GW, NFP for web text).

```rule
rule: en.errors.of-have-vernacular
what: of in the role of have (should of called) — AUX with lemma have and Style=Vrnc
match: w[form=of, upos=AUX]
require: w[feats.Style=Vrnc, lemma=have]
severity: warn
source: EWT 2.18 practice (Style=Vrnc); 2025.udw-1.17
```

```rule
rule: en.errors.them-det-vernacular
what: them as a determiner (them boys) — dialectal, Style=Vrnc
match: d[form=them, rel=det]
require: d[feats.Style=Vrnc]
severity: warn
source: EWT 2.18 practice (Style=Vrnc)
```
