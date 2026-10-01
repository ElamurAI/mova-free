# Verb in -s: 3rd person singular present (VBZ)

**Gist.** In the present tense an English verb has only one personal ending: *-s* in the 3rd person singular (*he works, she goes, it has*). The other persons use the bare base (*I/you/we/they work*, VBP). The PTB tag for the *-s* form is VBZ. This ending sounds and is spelled the same as the noun plural (*works* is both a verb and a noun), so syntax decides the part of speech.

**Conditions and exceptions.**
- Pronunciation: [ɪz] after sibilants (*kisses, judges*), [s] after voiceless sounds (*hopes*), [z] after the rest (*loves, plays*).
- Spelling: after *s, x, z, ch, sh* — *-es* (*passes, fixes, watches*); consonant + *y* → *-ies* (*tries, carries*), vowel + *y* → *-ys* (*plays*); several verbs in *-o* — *-oes* (*goes, does, echoes*).
- Irregular: *is* (be), *has* (have); *does* and *says* are spelled regularly but pronounced with a different vowel ([dʌz], [sez]).
- Modal verbs and *need, dare* in modal use have no *-s* (*he can, he need not*): they are MD, not VBZ.
- Archaic *-th/-eth* (*hath, doth, goeth*) is also 3rd person singular (see `verb-archaic-endings`); vernacular *I says, they says* extends *-s* to all persons.
- Contracted *'s* (= *is, has*) is also VBZ; colloquial *ai* (from *ain't*) in EWT is VBZ with lemma *be*.

**Examples.** *She **works** here. It **goes** well. He **has** left. It**'s** fine.*

**In UD.** UPOS VERB or AUX (*be, have, do, get* as auxiliaries); XPOS VBZ; FEATS `Mood=Ind|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin`; the lemma is the infinitive (*goes → go, is → be, has → have*).

**Sources.** Sweet NEG I §1290 (vol. 1 = text-1, p. 423: three endings of the regular verb *-s, -ed, -ing*; *-s* identical to the plural and possessive; vernacular *I says*), §1293 (p. 424: *says* [sez]), §1492–1493 (pp. 457–458: *has, does*), §1478, §1487 (pp. 451–455: *need* without *-s*); Whitney §243 (pp. 127–128: pronunciation and syllable of *-es*); https://universaldependencies.org/en/feat/Person.html (Person=3 for verbs — VBZ), `feat/Number.md`, `feat/Tense.md`; Santorini 1990, §2, p. 5 (VBZ), p. 3 (MD — verbs without *-s*); Jespersen MEG VI 3.1 (text-5, p. 30: three forms of the ending; *says, does, has*), 3.8–3.9 (p. 38: modals without *-s*; vernacular *says I*); Kruisinga II.1 §§5, 19–23 (text-2, pp. 42, 52–55); Mätzner I, p. 331 (text-1, p. 349: *-es* after *ss, z, x, sh, ch*; *goes, does*).

```rule
rule: en.morph.vbz-feats
what: VBZ — 3rd person singular present indicative verb
match: v[xpos=VBZ, upos=VERB|AUX]
require: v[feats.Number=Sing, feats.Person=3, feats.Tense=Pres, feats.VerbForm=Fin, feats.Mood=Ind]
severity: error
source: Santorini 1990 (PTB), VBZ; UD en: feats VERB
```

```rule
rule: en.morph.vbz-form-s
what: a form tagged VBZ ends in -s (exception — ai from ain't)
match: v[xpos=VBZ, upos=VERB|AUX, !feats.Typo, !feats.Abbr]
require: v[suffix=s|ai]
severity: warn
source: Sweet NEG I §1290; Whitney §243; EWT 2.18 practice (ai → be)
```
