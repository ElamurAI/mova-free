# The -ing form: VBG — participle and gerund

**Gist.** Modern English has one *-ing* form, but it has several functions. It can be a participle (*she is **reading**, a **sleeping** child*), a gerund — a "verbal noun" that keeps verbal structure (*I enjoy **reading** books*), and an ordinary deverbal noun (*the **reading** of the will, **buildings***). Penn Treebank gives all verbal uses one tag VBG; UD distinguishes participle and gerund with the feature `VerbForm`, and a noun in *-ing* is NOUN.

**Conditions and exceptions.**
- Test "verb or noun": has a direct object or an adverb (*closing **the plant***, *cooking **well***) — VBG; has an *of*-complement, an adjective, an article or a plural (*the closing **of** the plant, **good** cooking, **buildings***) — NN/NNS.
- Test "verb or adjective": *very*, degrees or *un-* can be added (*very interesting, uninteresting*), or it stands after *seem, become* — JJ.
- `VerbForm=Part` + `Tense=Pres`: in the progressive with an auxiliary (*is going*), in an adverbial participle clause (*…, saying there were…*), as a modifier. `VerbForm=Ger` (without `Tense`): in "nominal" positions without an auxiliary — subject, object, after a preposition (*I enjoyed working with you*).
- Spelling: silent *-e* drops (*make → making*), but not in *-ee, -ye, -oe* (*seeing, dyeing, hoeing*) and in *singeing* (to distinguish it from *singing*); *-ie* → *-ying* (*lie → lying, die → dying*); doubling after a short stressed vowel (*run → running*); see `spell-silent-e`, `spell-consonant-doubling`.
- Colloquial forms *goin', walkin* and the stump *gon* from *gonna* are also VBG with the full lemma.

**Examples.** *He is **sleeping*** (Part). ***Swimming** is fun* (Ger). *a **sleeping** bag* (NN in a compound). *The **meeting** starts at five* (NN).

**In UD.** UPOS VERB or AUX (*being*); XPOS VBG; FEATS `Tense=Pres|VerbForm=Part` or only `VerbForm=Ger`; no person, number or mood. The lemma is the infinitive (*lying → lie*). A noun in *-ing*: NOUN, NN/NNS, the lemma is the noun form (*building → building*), without `VerbForm`.

**Sources.** Sweet NEG I §101, §324–329 (text-1, pp. 66–67, 146–147: one form — two functions; the gerund keeps verbal constructions; *such doings*, *wire netting* — nouns), §335 (pp. 147–148: participles as adjectives: *a charming view*), §1600 (p. 490: concrete nouns in *-ing*: *building, clothing, shipping*); Whitney §237–238 (text-1, p. 124), §447 (pp. 233–234: *Caesar's passing the Rubicon* vs. *the passing of the Rubicon*), §455 (pp. 238–239: *charming, interesting* — adjectives); Santorini 1990, §4.1, pp. 14–15 (JJ or VBG), pp. 19–20 (NN or VBG); https://universaldependencies.org/en/feat/VerbForm.html (Part, Ger; VBG rules since v2.14); Jespersen MEG VI 21.9₁–21.9₃ (text-5, pp. 393–396: a noun in *-ing* from any verb; *morning, evening* without a verb; *-ing* spelling); Kruisinga II.1 §§139–151 (text-2, pp. 154–163: with an object or adverb — verbal; with plural *-s* — noun), I §§567, 569, 573 (text-1, pp. 254–255); Mätzner I, p. 453 (text-1, p. 471).

```rule
rule: en.morph.vbg-verbform
what: VBG as a verb — participle or gerund
match: v[xpos=VBG, upos=VERB|AUX]
require: v[feats.VerbForm=Part|Ger, !feats.Mood, !feats.Person, !feats.Number]
severity: error
source: UD en feat/VerbForm (Part, Ger for VBG)
```

```rule
rule: en.morph.vbg-participle-tense
what: -ing participle — Tense=Pres
match: v[xpos=VBG, feats.VerbForm=Part]
require: v[feats.Tense=Pres]
severity: error
source: UD en feat/VerbForm (Part: VBG along with Tense=Pres)
```

```rule
rule: en.morph.vbg-gerund-no-tense
what: -ing gerund — no Tense
match: v[xpos=VBG, feats.VerbForm=Ger]
require: v[!feats.Tense]
severity: error
source: UD en feat/VerbForm (Ger: with no Tense)
```

```rule
rule: en.morph.vbg-form-ing
what: a form tagged VBG ends in -ing (colloquial -in, -in')
match: v[xpos=VBG, !feats.Typo, !feats.Abbr]
require: v[suffix=ing|in|in'|in’]
severity: warn
source: Sweet NEG I §1290 (-ing); EWT 2.18 practice (goin, walkin; gon with Abbr=Yes)
```
