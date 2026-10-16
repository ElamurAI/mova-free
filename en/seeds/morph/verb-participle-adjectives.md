# Former participles that became adjectives: drunken, molten, sunken

**Gist.** When a verb develops two participle forms, the old form in *-en* often drifts away from the verb and becomes an adjective used before a noun: *a drunken sailor* (but *he has drunk*), *a sunken ship*, *molten lava*, *swollen feet*, *a laden cart*, *clean-shaven*. Likewise *wrought (iron), gilt, roast (beef), dread (moment)* became adjectives. Such words are ADJ with their own lemma, not verb forms.

**Conditions and exceptions.**
- Only adjectives (no verbal use): *drunken, sunken, shrunken, molten, graven, bounden (duty), rotten, misshapen*.
- Mostly adjectives, but verbal use is possible: *laden, hewn, strewn, mown (new-mown), swollen, shaven, stricken*.
- Adjectives in [ɪd] with the old pronunciation: *learned, beloved, blessed, aged, crooked, wicked* (see `verb-ed-regular`, `wf-ed-adjectives`).
- *Born* is the participle of *bear* only in the sense of birth in the passive (*he was born*): VBN with lemma *bear*; *borne* — in other senses (*she has borne five children*, *borne by*).
- *Proven* is mostly an adjective (*proven abilities*) and the Scottish legal *not proven*; as a participle — VBN with lemma *prove*.
- Adverbs are built from the *-en* form: *brokenly, mistakenly*.

**Examples.** *a **drunken** brawl* (JJ); *he was **drunk*** (JJ, as a state); *he has **drunk** it* (VBN); *a **sunken** ship* (JJ); *he was **born** in 1990* (VBN, lemma *bear*).

**In UD.** ADJ, JJ, `Degree=Pos`, the lemma is the form itself (*drunken, molten*). Verbal use — VBN with the infinitive as lemma.

**Sources.** Jespersen MEG VI 5.5, 5.7 (text-5, pp. 87–98: *drunk/drunken, sunken, shrunken, mown, hewn, strewn, laden, molten, graven, clean-shaven*; the *-n* form as a "buffer syllable" before a noun; *brokenly*), 5.3₃ (p. 75: *born/borne*), 5.5₃ (p. 89: *proven*), 4.4₂, 4.9₁ (pp. 52, 59–60: *dread, roast; wrought iron*); Sweet NEG I §1386, §1390, §1462, §1439 (text-1, pp. 438–439, 445–449: *drunken, sunken, molten* — only adjectives; *born/borne*); Whitney §275, §455 (text-1, pp. 134, 238–239: *molten, shapen, graven, shaven, laden, riven, rotten, swollen, hewn, mown, sawn, bounden*; *drunken man* vs. *has drunk*); Santorini 1990, §4.1, pp. 15–17 (JJ or VBN).

```rule
rule: en.morph.participle-adjective-only
what: drunken, sunken, shrunken, molten, graven, bounden, rotten, misshapen — only adjectives
match: w[form=drunken|sunken|shrunken|molten|graven|bounden|rotten|misshapen, !feats.Typo]
require: w[upos=ADJ]
severity: warn
source: Jespersen MEG VI 5.5, 5.7; Sweet NEG I §1386, §1390, §1462; Whitney §275
```

```rule
rule: en.morph.born-lemma
what: born as a verb — participle of bear
match: v[upos=VERB, form=born, !feats.Typo]
require: v[xpos=VBN, lemma=bear]
severity: error
source: Jespersen MEG VI 5.3₃ (born — passive of birth); Sweet NEG I §1439; EWT 2.18 practice
```
