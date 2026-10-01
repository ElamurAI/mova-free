# Archaic verb endings: -th/-eth (hath) and -st/-est (thou hast)

**Gist.** Until the 17th–18th c. the English verb had two more personal endings: *-th/-eth* for the 3rd person singular (*he goeth, hath, doth, saith*) and *-st/-est* for the 2nd person singular with *thou* (*thou goest, hast, art, canst, wilt*). In the modern language they live only in the Bible, prayers, proverbs, solemn and stylized text. For annotating old texts it matters: *-eth* is the same as modern *-s* (VBZ), and forms in *-st* are 2nd person singular.

**Conditions and exceptions.**
- *Hath, doth, saith* held on until the mid-18th c., were revived in the 19th c. in solemn style; at the same time the split *doth* (auxiliary) — *doeth* (lexical) appeared.
- 2nd person: *-est* is shortened, except after sibilants; in *-t*: *art, shalt, wilt*; in *-st*: *canst, mayst, darest, hast, dost, didst, hadst, wouldst, shouldst, couldst*; *wast* — indicative, *wert* — subjunctive and poetry; *thou must* without an ending.
- Lemmas: *hath, hast → have; doth, dost, doeth, didst → do; art, wast, wert → be; saith → say; shalt → shall; wilt → will; canst → can*.
- *Wilt* is also a modern verb "to wither" (*the flowers wilt*): as MD — *will*, as VERB — *wilt*.
- In Early Modern English (1540–1640) forms in *-s/-th* also occurred with a plural subject (*they hath*).
- In UD such forms may have `Style=Arch`.

**Examples.** *He **hath** spoken* (VBZ, *have*). *Thou **art** mine* (VBP, *be*, `Person=2|Number=Sing`). *Thou **shalt** not kill* (MD, *shall*). *The Lord **giveth*** (VBZ, *give*).

**In UD.** Forms in *-th/-eth*: VBZ, `Person=3|Number=Sing|Tense=Pres`. 2nd person forms with *thou*: `Person=2|Number=Sing`; modals — MD with the modern lemma.

**Sources.** Jespersen MEG VI 2.3₁–2.7₂ (text-5, pp. 23–29: 2nd person *-est*, *art, shalt, wilt, canst, mayst, dost/doest, didst, wouldst, wast/wert*), 3.6–3.7 (pp. 36–37: *hath, doth, saith* until the mid-18th c.; *doth/doeth*), II 2.24₂ (text-2, pp. 51–52: *they hath*); Sweet NEG I §1283 (text-1, p. 421: *-st, -th* only in proverbs and high style), §1479–1490 (pp. 451–456: *canst, couldst, mayst, shalt, wilt, wouldst, art, wast/wert, hast, hadst*); Whitney §243, §256, §273, §277–278 (text-1, pp. 127–135).

```rule
rule: en.morph.archaic-eth-vbz
what: verb in -eth (goeth, giveth) — 3rd person singular present
match: v[upos=VERB|AUX, suffix=eth, !feats.Typo]
require: v[xpos=VBZ, feats.Person=3, feats.Number=Sing, feats.Tense=Pres]
severity: warn
source: Jespersen MEG VI 3.6–3.7; Sweet NEG I §1283; Whitney §243
```

```rule
rule: en.morph.archaic-verb-lemma
what: hath, doth, saith, art, hast, dost, didst, wast, wert — modern lemmas have, do, say, be
match: v[upos=VERB|AUX, form=hath|hast|hadst|doth|dost|doeth|didst|saith|art|wast|wert, !feats.Typo]
require: v[lemma=have|do|say|be]
severity: error
source: Jespersen MEG VI 2.3₁–2.7₂, 3.6–3.7; Sweet NEG I §1479–1493
```

```rule
rule: en.morph.archaic-modal-lemma
what: shalt, wilt, canst, couldst, wouldst, shouldst, mayst as modals — modern lemmas
match: m[xpos=MD, form=shalt|wilt|canst|couldst|wouldst|shouldst|mayst|mightst, !feats.Typo]
require: m[lemma=shall|will|can|could|would|should|may|might]
severity: error
source: Jespersen MEG VI 2.3₁–2.7₂; Sweet NEG I §1479–1490; Whitney §277–278
```
