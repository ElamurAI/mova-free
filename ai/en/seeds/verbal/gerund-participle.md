# The -ing form: gerund or participle

**Gist.** One *-ing* form has two roles. The gerund is a verb in a noun role: subject, object, after a preposition (*Swimming is fun*, *I enjoyed working with you*, *for keeping money*). The participle is a verb in a modifier or adverbial role, and also in progressive tenses (*a sleeping child*, *He left, saying nothing*, *is sleeping*). Modern grammars (CGEL) see a single "gerund-participial" form; UD approximately reproduces the traditional split.

**Conditions and exceptions.** An *-ing* word that has become a noun (*the opening of the store*, *a building*) is `NOUN` without verbal features. Poutsma showed that the boundary between the gerund and the verbal noun is blurred (§§53–55); the criterion is whether the word keeps verbal government (direct object, adverb).

**Examples.** *I enjoyed working with you* (Ger). — *I will be driving home* (Part). — *The opening was delayed* (NOUN).

**In UD.** `VBG`: `VerbForm=Ger` (without `Tense`) in nominal positions and without `aux`; `VerbForm=Part|Tense=Pres` — with `aux` and in modifier and adverbial roles. A gerund never has `aux`.

**Sources.** https://universaldependencies.org/en/feat/VerbForm.html (Ger, Part; VBG rules since v2.14, EWT issue 305); Poutsma 1923, *The Gerund*, §1 (the gerund is between the infinitive and the noun of action; vol. 2, p. 111), §§49–55 (gerund vs participle and verbal noun; p. 152 ff.); Reed & Kellogg, Higher Lessons, Lesson 37 (participles: "verbs as adjectives and as nouns"); Jespersen, MEG V, ch. VIII–IX "The Gerund" (text-1, p. 120).

```rule
rule: en.verbal.gerund-no-aux
what: a gerund (VerbForm=Ger) has no auxiliaries (absence of Tense — en.morph.vbg-gerund-no-tense)
match: v[feats.VerbForm=Ger]
require: none a[rel=aux|aux:pass, head=v]
severity: error
source: https://universaldependencies.org/en/feat/VerbForm.html, Ger
```
