# Indefinite pronouns some-, any-, every-, no- + one, body, thing

**Gist.** English has a series of compound pronouns: the first part says "which" (*some-* — some, *any-* — any, *every-* — each, *no-* — none), the second "who or what" (*-one, -body* — a person, *-thing* — a thing). They are written as one word (*someone, nothing*), except *no one* (two words). They are always singular: *everyone **is***, *nothing **has** changed*.

**Conditions and exceptions.**
- Penn Treebank tags them as nouns (NN), but in UD they are PRON: they take no articles or modifiers before them.
- *None* is a separate word: a negative pronoun (`PronType=Neg`) without number, because it agrees with both singular and plural verbs (*none of them is/are*).
- *No one* is written separately: *no* — DET, *one* — PRON with `PronType=Neg`.
- Adverbs of the same series (*somewhere, anywhere, nowhere, everywhere*) are ADV with `PronType`, not PRON.
- The lemma is the form itself: *somebody → somebody*, not *body*.

**Examples.** ***Someone** called. I didn't see **anything**. **Everybody** knows. **Nothing** happened. **None** of them came.*

**In UD.** UPOS PRON; XPOS NN; `Number=Sing` (except *none*); `PronType=Ind` (*some-, any-*), `PronType=Tot` (*every-*), `PronType=Neg` (*no-, none*).

**Sources.** https://universaldependencies.org/en/pos/PRON.html (Indefinite pronouns: table and *no one*), `feat/PronType.md` (Ind, Tot, Neg), `pos/ADV.md` (*somewhere* etc. — ADV); Santorini 1990, §4.1, p. 18 (*none* and *some-/any-/every-/no-* + *-one/-thing* — NN, not PRP); Sweet NEG I §1148–1157 (text-1, p. 387–389: *someone, somebody, something*; *any, every* — adjectival only), §1136 (p. 382–383: *none* standalone, *no* adnominal); Jespersen MEG II 16.6₁, 17.2₁–17.3₁ (text-2, p. 450, 475–478); Kruisinga II.2 §§1339–1356 (text-3, p. 350–360: *every, no* attributive only; *none* more often with the plural).

```rule
rule: en.morph.indef-pron-upos
what: someone, anything, everybody, nothing etc. tagged NN — a pronoun in UD
match: p[xpos=NN, form=someone|somebody|something|anyone|anybody|anything|everyone|everybody|everything|nobody|nothing|none, !feats.Typo]
require: p[upos=PRON]
severity: error
source: UD en pos/PRON (These are NN in PTB but PRON in UD)
```

```rule
rule: en.morph.indef-some-any
what: some-/any- + one/body/thing — PronType=Ind, singular
match: p[upos=PRON, form=someone|somebody|something|anyone|anybody|anything, !feats.Typo]
require: p[feats.PronType=Ind, feats.Number=Sing]
severity: error
source: UD en pos/PRON, feat/PronType (Ind)
```

```rule
rule: en.morph.indef-every
what: every- + one/body/thing — PronType=Tot, singular
match: p[upos=PRON, form=everyone|everybody|everything, !feats.Typo]
require: p[feats.PronType=Tot, feats.Number=Sing]
severity: error
source: UD en pos/PRON, feat/PronType (Tot)
```

```rule
rule: en.morph.indef-no
what: nobody, nothing — PronType=Neg, singular; none — PronType=Neg without number
match: p[upos=PRON, form=nobody|nothing|none, !feats.Typo]
require: p[feats.PronType=Neg]
severity: error
source: UD en pos/PRON, feat/PronType (Neg)
```

```rule
rule: en.morph.indef-lemma
what: the lemma of a compound pronoun is the whole form
match: p[upos=PRON, form=someone|somebody|something|anyone|anybody|anything|everyone|everybody|everything|nobody|nothing|none, !feats.Typo]
require: p[lemma=someone|somebody|something|anyone|anybody|anything|everyone|everybody|everything|nobody|nothing|none]
severity: error
source: UD en pos/PRON (table of indefinite pronouns)
```
