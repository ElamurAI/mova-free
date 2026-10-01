# Contracted forms: 's, 're, 'm, 've, 'd, 'll, n't

**Gist.** In speech and informal writing, auxiliaries and *not* are contracted and attach to the preceding word: *it's, we're, I'm, they've, she'd, you'll, isn't*. Penn Treebank and UD tokenization split the contraction off as a separate word, and it gets the same lemma and features as the full form: *'re* is *are*, *n't* is *not*.

**Conditions and exceptions.**
- *'s* has three meanings: *is* (*it's cold*), *has* (*it's been cold*) — both VBZ with lemma *be* or *have*; possessive *'s* (*John's*) — POS (see `noun-genitive-s`); *us* in *let's* — PRON with lemma *we*.
- *'d* — *had* (VBD, lemma *have*) or *would* (MD, lemma *would*); the next word decides: a participle (*I'd gone*) → *had*, a base form (*I'd go*) → *would*.
- *'ll* — *will* (MD).
- *N't* is split off together with *n*: *is|n't*, *do|n't*. Several verbs change their stem in the process, and the stump gets the lemma of the full form: *can't* → *ca* + *n't* (lemma *can*), *won't* → *wo* + *n't* (lemma *will*), *shan't* → *sha* + *n't* (lemma *shall*), *ain't* → *ai* + *n't* (lemma *be* or *have*).
- *Cannot* is split into *can* + *not*.
- Colloquial fusions are split too: *gonna* → *gon* (VBG, lemma *go*) + *na* (TO, lemma *to*); *wanna* → *wan* (lemma *want*) + *na*; both parts have `Abbr=Yes`.

**Examples.** *It**'s** fine. We**'re** late. I**'d** go. You**'ll** see. It is**n't**. I ca**n't**. I'm gon**na** try.*

**In UD.** *n't*: PART, XPOS RB, lemma *not*, `Polarity=Neg`. *'m*: lemma *be*, VBP, `Person=1|Number=Sing`. *'re*: lemma *be*, VBP. *'ve*: lemma *have*. *'ll*, *wo*: lemma *will*, MD. *ca*: lemma *can*, MD. *'d*: *would* (MD) or *have* (VBD).

**Sources.** https://universaldependencies.org/en/tokenization.html (PTB tokenization), https://universaldependencies.org/en/pos/PRON.html (*'s* in *let's* — as *us*, "like other standard contractions (e.g. n't)"), `feat/Polarity.md` (*not* — `Polarity=Neg`); Santorini 1990, §2, p. 2 (RB: *not, n’t*), p. 3 (MD), p. 4 (POS); Sweet NEG I §78 (text-1, pp. 59–60: *John's book* vs. *John's here*), §1478–1493 (pp. 451–458: *can't, shan't, won't, mustn't, ain't, haven't, don't*); Jespersen MEG V 23.1₄–23.2₅ (vol. 1 = text-1, pp. 442–449: *n't* written since about 1660; *aren't I* / *ain't I*; *don't* in the 3rd person); Kruisinga II.1 §§20–25 (text-2, pp. 53–56: weak *'s, 'd* only with an auxiliary; *mayn't, shan't, won't*).

```rule
rule: en.morph.clitic-nt
what: n't — negation particle with lemma not
match: n[form=n't|n’t, !feats.Typo]
require: n[upos=PART, xpos=RB, lemma=not, feats.Polarity=Neg]
severity: error
source: UD en feat/Polarity (not: Neg), pos/PART; Santorini 1990, RB
```

```rule
rule: en.morph.clitic-s-verb
what: 's as a verb — is or has (VBZ, lemma be or have)
match: s[form='s|’s, upos=AUX|VERB, !feats.Typo]
require: s[xpos=VBZ, lemma=be|have]
severity: error
source: UD en tokenization (PTB contractions); Santorini 1990, VBZ
```

```rule
rule: en.morph.clitic-s-us
what: 's in let's — pronoun us with lemma we
match: s[form='s|’s, upos=PRON, !feats.Typo]
require: s[lemma=we, feats.Case=Acc, feats.Person=1, feats.Number=Plur]
severity: error
source: UD en pos/PRON (’s [we])
```

```rule
rule: en.morph.clitic-am
what: 'm — am (VBP, lemma be, 1st person singular)
match: v[form='m|’m, upos=AUX|VERB, !feats.Typo]
require: v[lemma=be, xpos=VBP, feats.Person=1, feats.Number=Sing]
severity: error
source: UD en feat/Person (am: 1st person); tokenization
```

```rule
rule: en.morph.clitic-re
what: 're — are (VBP, lemma be)
match: v[form='re|’re, upos=AUX|VERB, !feats.Typo]
require: v[lemma=be, xpos=VBP]
severity: error
source: UD en tokenization; Santorini 1990, VBP
```

```rule
rule: en.morph.clitic-ve
what: 've — have
match: v[form='ve|’ve, upos=AUX|VERB, !feats.Typo]
require: v[lemma=have]
severity: error
source: UD en tokenization
```

```rule
rule: en.morph.clitic-will
what: 'll and wo (from won't) — modal will
match: m[form='ll|’ll|wo, xpos=MD, !feats.Typo]
require: m[lemma=will]
severity: error
source: UD en tokenization; EWT 2.18 practice (wo → will)
```

```rule
rule: en.morph.clitic-can
what: ca (from can't) — modal can
match: m[form=ca, xpos=MD, !feats.Typo]
require: m[lemma=can]
severity: error
source: UD en tokenization; EWT 2.18 practice (ca → can)
```

```rule
rule: en.morph.clitic-d
what: 'd as a modal — would; as VBD — had
match: m[form='d|’d, xpos=MD, !feats.Typo]
require: m[lemma=would]
severity: error
source: UD en tokenization; EWT 2.18 practice
```

```rule
rule: en.morph.clitic-d-had
what: 'd tagged VBD — had (lemma have)
match: v[form='d|’d, xpos=VBD, !feats.Typo]
require: v[lemma=have]
severity: error
source: UD en tokenization; EWT 2.18 practice
```

```rule
rule: en.morph.clitic-gonna
what: gon from gonna — VBG of the verb go
match: g[form=gon, upos=VERB|AUX]
require: g[lemma=go, xpos=VBG]
severity: error
source: EWT 2.18 practice (gonna → gon + na, Abbr=Yes)
```
