# -y → -i- in inflection: try → tried, city → cities, happy → happier

**Gist.** Final *-y* after a consonant is written as *-i-* before the endings *-s, -ed, -er, -est, -ly, -ness*: *try → tries, tried; city → cities; happy → happier, happiest, happily, happiness*. Before *-ing* the *y* stays (*trying, studying*), since otherwise we would get *ii*. After a vowel *y* does not change: *play → plays, played; day → days; gay → gayer*. For lemmatization: *-ies, -ied, -ier, -iest, -ily* go back to *-y* (*studies → study*, not *studie*).

**Conditions and exceptions.**
- Vowel + *y*: *plays, played, keys, valleys, monkeys, boys, guys*; exceptions with old spelling — *laid, paid, said* (from *lay, pay, say*), *daily, gaily* (alongside *gayly*); *staid* is now an adjective.
- Proper names do not change *y*: *the Kennedys, two Marys, the Gregorys*.
- Monosyllabic adjectives vary: *drier/dryer, shyer, slyer*; adverbs *drily/dryly, shyly, slyly*.
- *-ey*: *monkeys, valleys, journeys* (old *vallies, monies* are obsolete); *money → moneys/monies*.
- The adjective *married*, *hurried* etc. has its own lemma as ADJ; as a verb — *marry, hurry*.
- *Supplies* in the sense "provisions" is annotated in EWT as plurale tantum (`Number=Ptan`, lemma *supplies*), so it is not in the noun rule.

**Examples.** *tried → try; studies → study; cities → city; carried → carry; happier → happy; happily → happily* (the adverb has its own lemma).

**In UD.** The lemma of a verb, noun and adjective is with *-y*: *applied → apply, countries → country, easiest → easy*.

**Sources.** Jespersen MEG VI 16.1₄ (text-5, p. 271: *ladies, armies, he marries*; *Marys, Henrys*; *days, valleys*; obsolete *vallies, monies*), 4.2₃ (pp. 46–47: *replied*, but *played*; *laid, paid, said*; *staid*), 22.8₂–22.8₆ (pp. 425–428: *happily; drily/dryly, shily/shyly; gaily*); Kruisinga I §§567, 572, 574 (text-1, pp. 254–256: *carried, but paid, laid*; *ladies* but *lady's*); Mätzner I, p. 224 (text-1, p. 242: *-ies/-ys*; *monies, attornies* "rejected as incorrect"), p. 337 (p. 355: *y → i*, except before *-ing*), p. 273 (p. 291: *happier* but *gayer*); Sweet NEG I §1021 (text-1, p. 350: *spies, cities; days*).

```rule
rule: en.morph.y-to-i-verb-lemma
what: tried, studies, carried, applied… as a verb — lemma in -y
match: v[upos=VERB, form=tried|tries|studied|studies|carried|carries|applied|applies|replied|replies|cried|cries|worried|worries|married|marries|copied|copies|denied|denies|relied|relies|supplied|supplies|identified|identifies|hurried|hurries|buried|buries|varied|varies|satisfied|satisfies, !feats.Typo]
require: v[lemma=try|study|carry|apply|reply|cry|worry|marry|copy|deny|rely|supply|identify|hurry|bury|vary|satisfy]
severity: error
source: Jespersen MEG VI 4.2₃, 16.1₄; Kruisinga I §567; Mätzner I, p. 337
```

```rule
rule: en.morph.y-to-i-noun-lemma
what: cities, countries, companies… as a noun — lemma in -y
match: n[upos=NOUN, form=cities|countries|companies|parties|families|babies|ladies|stories|studies|policies|activities|abilities|communities|universities|opportunities|categories|properties|technologies|strategies|facilities|difficulties|libraries|galleries|copies|bodies|armies|enemies|economies|industries|entries|replies|theories, !feats.Typo]
require: n[lemma=city|country|company|party|family|baby|lady|story|study|policy|activity|ability|community|university|opportunity|category|property|technology|strategy|facility|difficulty|library|gallery|copy|body|army|enemy|economy|industry|entry|reply|supply|theory]
severity: error
source: Jespersen MEG VI 16.1₄; Sweet NEG I §1021; Mätzner I, p. 224
```

```rule
rule: en.morph.y-to-i-adj-lemma
what: happier, easiest, earlier… as an adjective — lemma in -y
match: a[upos=ADJ, xpos=JJR|JJS, form=happier|happiest|easier|easiest|earlier|earliest|busier|busiest|heavier|heaviest|funnier|funniest|prettier|prettiest|luckier|luckiest|healthier|healthiest|wealthier|wealthiest|angrier|angriest|tinier|tiniest|dirtier|dirtiest|crazier|craziest|scarier|scariest, !feats.Typo]
require: a[lemma=happy|easy|early|busy|heavy|funny|pretty|lucky|healthy|wealthy|angry|tiny|dirty|crazy|scary]
severity: error
source: Mätzner I, p. 273 (happier but gayer); Jespersen MEG VI 22.8
```
