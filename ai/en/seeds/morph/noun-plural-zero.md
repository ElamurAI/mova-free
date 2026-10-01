# Zero plural: sheep, deer, fish, species, aircraft

**Gist.** A few nouns have the same form in the singular and plural: *one sheep — two sheep*, *a deer — many deer*, *this species — these species*. The number of such a word is visible only from agreement: from the determiner (*this/these*, *a*), a numeral or the verb (*the sheep is/are*). So the tag (NN or NNS) and the `Number` feature are assigned by context.

**Conditions and exceptions.**
- Animals: *sheep, deer, swine, moose, bison*; fish — *fish, salmon, trout, cod* (but *fishes* — of different species); hunting language: *two brace of pheasants*.
- Latin words in *-es*: *species, series*; *means* ("way": *a means / all means*), *headquarters, crossroads, barracks* (EWT often gives them `Number=Ptan`).
- *Aircraft, spacecraft, craft, offspring*.
- Measure words after a numeral: *two dozen eggs, five hundred people, a ten-pound note* (but *dozens of eggs, hundreds of people*); in a compound number *hundred, thousand, million* do not take *-s*.
- Nationalities ending in a sibilant: *the Chinese, two Japanese, the Swiss* — the same form; names in *-man* have *-men*.
- The lemma is the same form (*sheep → sheep*).

**Examples.** ***These** species are rare* (NNS, `Plur`). ***A** species of bird* (NN, `Sing`). *Two **aircraft** landed* (NNS). *The **sheep** is lost* (NN).

**In UD.** NOUN; NN + `Number=Sing` or NNS + `Number=Plur` depending on agreement; the lemma equals the form.

**Sources.** Sweet NEG I §1004–1006 (text-1, p. 345–346: *sheep, deer*; measures after numerals *two dozen*; *swine*; *fish* collective vs *fishes*), §1007–1015 (p. 346–348: *series, species* invariable); Whitney §127 (text-1, p. 73: *sheep, deer, swine, fish, trout, salmon*; counting words *couple, brace, dozen, score, head*), §144c (p. 78–79: *the English, the Dutch*), §214 (p. 111–112: *two hundred*); Santorini 1990, §4.1, pp. 17–18 (NN/NNS by agreement); https://universaldependencies.org/en/feat/Number.html (*this sheep, these sheep, this species, these species*); Jespersen MEG II 3.1₁–3.5₂ (text-2, p. 81–90: *sheep, deer, swine; many fish, twenty snipe; two dozen, six brace*), VI 19.8₂ (text-5, p. 348: *Chinese, Japanese, Swiss*); Kruisinga II.2 §§783–794 (text-3, p. 37–43: *deer, sheep, grouse, cod, salmon, trout; craft, sail*).

```rule
rule: en.morph.zero-plural-with-plural-det
what: a noun with zero plural after these, those, several, many, few, both — NNS, plural
match: n[upos=NOUN, form=sheep|deer|swine|fish|salmon|trout|moose|bison|aircraft|spacecraft|craft|species|series|offspring]; d[head=n, form=these|those|several|many|few|both]
require: n[xpos=NNS, feats.Number=Plur]
severity: warn
source: UD en feat/Number (these sheep, these species); Sweet NEG I §1004–1006; Whitney §127
```

```rule
rule: en.morph.zero-plural-with-singular-det
what: a noun with zero plural after a, an, this, that, each, every — NN, singular
match: n[upos=NOUN, form=sheep|deer|swine|fish|salmon|trout|moose|bison|aircraft|spacecraft|craft|species|series|offspring]; d[head=n, rel=det, form=a|an|this|that|each|every]
require: n[xpos=NN, feats.Number=Sing]
severity: warn
source: UD en feat/Number (this sheep, this species); Santorini 1990, §4.1, pp. 17–18
```
