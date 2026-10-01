# Regular endings on irregular verbs: goed, buyed

**Gist.** Learners and children "regularize" irregular verbs by the -ed rule: *goed, buyed, thinked, catched*. ERRANT calls this VERB:INFL (*getted → got*); BLiMP separately tests irregular participle forms. Under the universal UD guidelines and EWT practice the erroneous form is kept as is. It gets Typo=Yes, the lemma and tag of the intended word (VBD or VBN), and the correct form goes into MISC as CorrectForm.

**Conditions and exceptions.** Real words that coincide with such forms (*seed, leaved, payed*) are not in the list.

**Examples.**
- *I buyed it* → buyed: lemma buy, VBD, Typo=Yes, CorrectForm=bought.
- ✗ *We goed home* without Typo=Yes.

**Check against gold.** EWT 2.18: 0 such forms, so no false alarms. The spoken L2 English treebank ESLSpok: 2 — *I just drived five minutes*, *British is runned*. There is no Typo=Yes there, because ESLSpok has no FEATS.

**Sources.** `P17-1074` (VERB:INFL); `2020.tacl-1.25` (BLiMP: IRREGULAR FORMS); `2025.udw-1.17` (Masciolini et al.: in UD the form is not changed, Typo=Yes is set, the lemma comes from the normalized spelling, CorrectForm goes into MISC); EWT 2.18 (1439 tokens with Typo=Yes).

```rule
rule: en.errors.overregularized-typo
what: regularized form of an irregular verb (goed, buyed) without Typo=Yes
match: v[form=goed|comed|buyed|bringed|thinked|teached|catched|taked|eated|runned|swimmed|writed|maked|gived|knowed|drinked|speaked|breaked|falled|feeled|finded|getted|losed|meeted|selled|sended|sitted|spended|standed|telled|understanded|winned|choosed|drived|flied|forgetted|growed|hitted|holded|keeped|putted|rided|shaked|singed|sleeped|stealed|throwed|weared]
require: v[feats.Typo=Yes]
severity: warn
source: P17-1074 (VERB:INFL); 2020.tacl-1.25 (BLiMP irregular forms); 2025.udw-1.17 (Typo=Yes)
```
