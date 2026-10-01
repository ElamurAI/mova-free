# The verb be: am, is, are, was, were, been, being

**Gist.** *Be* is the most irregular English verb: eight forms from different roots. Present: *am* (1st person singular), *is* (3rd singular), *are* (the rest). Past: *was* (1st and 3rd singular), *were* (the rest and unreal condition). Infinitive and subjunctive — *be*, participles — *been* and *being*. It is the only verb that distinguishes person or number in both present and past.

**Conditions and exceptions.**
- *Be* can be an auxiliary (AUX: *is going*, *was taken*), a copula (AUX, relation `cop`: *she is happy*) and lexical "exist, be located" (VERB is rare in EWT: *there are…*).
- *Were* with a singular subject (*if I were*) is subjunctive: VBD, `Mood=Sub`.
- Contracted *'m, 're, 's* are the same *am, are, is* (see `verb-clitics`); colloquial *ain't* → *ai* (lemma *be*).
- Non-standard *you was, they was, he were* are annotated by form: tag and lemma are the same.

**Examples.** *I **am** here. She **is** ready. We **were** late. It has **been** done. You are **being** silly.*

**In UD.** The lemma is always *be*. *Am* — VBP, `Person=1|Number=Sing`; *is* — VBZ; *are* — VBP; *was* — VBD, `Number=Sing`; *were* — VBD; *been* — VBN; *being* — VBG; *be* — VB.

**Sources.** https://universaldependencies.org/en/feat/Person.html (*am, was* — person), `feat/Number.md` (*am, is, was* — Sing; *are, were* — Plur), `feat/Mood.md` (*If I were rich* — Sub), https://universaldependencies.org/en/pos/AUX_.html; Santorini 1990, §2, p. 5 (VBD: *If I were rich*), §4.1, p. 17 (*be* is not MD); Sweet NEG I §1490–1491 (text-1, pp. 456–457: *you was* — vernacular; *if I was* instead of *were*; *ain't*); Jespersen MEG VI 5.6 (text-5, p. 91: three roots *be, is, was*; *were* — the only preterite with number); Kruisinga II.1 §§25, 29 (vol. 2 = text-2, pp. 56–58: *aren't I*; irrealis *were* with all persons, colloquial *was*); Whitney §273 (text-1, p. 133).

```rule
rule: en.morph.be-am
what: am — be, VBP, 1st person singular
match: v[form=am, upos=AUX|VERB, !feats.Typo]
require: v[lemma=be, xpos=VBP, feats.Person=1, feats.Number=Sing]
severity: error
source: UD en feat/Person, feat/Number
```

```rule
rule: en.morph.be-is
what: is — be, VBZ
match: v[form=is, upos=AUX|VERB, !feats.Typo]
require: v[lemma=be, xpos=VBZ]
severity: error
source: UD en feat/Person (she is: Person=3)
```

```rule
rule: en.morph.be-are
what: are — be, VBP
match: v[form=are, upos=AUX|VERB, !feats.Typo]
require: v[lemma=be, xpos=VBP]
severity: error
source: UD en feat/Number (are: Plur)
```

```rule
rule: en.morph.be-was
what: was — be, VBD, singular
match: v[form=was, upos=AUX|VERB, !feats.Typo]
require: v[lemma=be, xpos=VBD, feats.Number=Sing]
severity: error
source: UD en feat/Number (was: Sing)
```

```rule
rule: en.morph.be-were
what: were — be, VBD
match: v[form=were, upos=AUX|VERB, !feats.Typo]
require: v[lemma=be, xpos=VBD]
severity: error
source: UD en feat/Number, feat/Mood (If I were — Sub)
```

```rule
rule: en.morph.be-been
what: been — be, VBN
match: v[form=been, upos=AUX|VERB, !feats.Typo]
require: v[lemma=be, xpos=VBN]
severity: error
source: UD en feat/VerbForm (Part: been prepared)
```

```rule
rule: en.morph.be-being
what: being — be, VBG
match: v[form=being, upos=AUX|VERB, !feats.Typo]
require: v[lemma=be, xpos=VBG]
severity: error
source: UD en feat/VerbForm (They are being nasty)
```
