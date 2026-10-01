# Regular past in -ed and forms in -t

**Gist.** A regular verb forms both the past tense and the past participle with a single ending *-ed*: *call → called, want → wanted*. It is the only productive pattern: all new verbs (*google → googled, text → texted*) inflect exactly this way. The ending has three pronunciations: [ɪd] after *t, d* (*wanted, ended*), [d] after voiced sounds (*called, played*), [t] after voiceless ones (*looked, wished*).

**Conditions and exceptions.**
- Spelling: after silent *-e* or *-ee* only *-d* is added (*loved, agreed*); consonant + *y* → *-ied* (*try → tried*), but vowel + *y* → *-yed* (*play → played*; exceptions *laid, paid, said*); a final consonant after a short stressed vowel is doubled (*stop → stopped, prefer → preferred*, but *visit → visited, offer → offered*); British *l* is always doubled (*travelled*), American is not (*traveled*); *-c* → *-cked* (*panic → panicked*). More — `spell-consonant-doubling`, `spell-silent-e`, `spell-y-to-i`.
- Forms in *-t* alongside *-ed* (British; Americans more often have *-ed*): *burnt, learnt, dwelt, smelt, spelt, spilt, spoilt, dealt, dreamt, knelt, leant, leapt*; the lemma is the ordinary base (*learnt → learn*).
- The old pronunciation [ɪd] survives in adjectives: *a learned professor, my beloved wife, an aged man, blessed*. Such adjectives are JJ with their own lemma (*learned*), the verb forms are VBD/VBN with lemma *learn*.
- *Past* (adjective, preposition, adverb, noun) ≠ *passed* (form of the verb *pass*).

**Examples.** *She **walked** home. It was **stopped**. We **tried**. I **learnt** it* (VBD, lemma *learn*).

**In UD.** VBD (`Tense=Past|VerbForm=Fin|Mood=Ind`) or VBN (`Tense=Past|VerbForm=Part`); the lemma is the base without *-ed*, with restored spelling: *stopped → stop, tried → try, loved → love, panicked → panic*.

**Sources.** Jespersen MEG VI 4.2₁–4.2₃ (vol. 5 = text-5, pp. 44–47: pronunciation, adjectives in [ɪd], spelling *-ied, laid/paid/said*, doubling, *-ck-*, *past/passed*), 4.3₁, 4.5₁ (pp. 47–48, 54: forms in *-t*); Sweet NEG I §1286–1287 (text-1, p. 422: pronunciation; all new verbs are regular), §1295–1333 (pp. 425–430: *burnt, dwelt, learnt, spelt, spilt, spoilt, dealt, dreamt, knelt, leapt*); Whitney §244, §246–249 (text-1, pp. 128–129).

```rule
rule: en.morph.t-past-lemma
what: burnt, learnt, spelt, dreamt etc. as a verb — lemma on the base (burn, learn, spell, dream)
match: v[upos=VERB, xpos=VBD|VBN, form=burnt|learnt|dwelt|smelt|spelt|spilt|spoilt|dealt|dreamt|knelt|leant|leapt|blest, !feats.Typo]
require: v[lemma=burn|learn|dwell|smell|spell|spill|spoil|deal|dream|kneel|lean|leap|bless]
severity: error
source: Jespersen MEG VI 4.3₁, 4.5₁; Sweet NEG I §1295–1320; Whitney §247–249
```
