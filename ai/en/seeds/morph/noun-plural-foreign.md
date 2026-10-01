# Foreign plurals: criteria, analyses, stimuli, appendices

**Gist.** Borrowings from Latin and Greek often keep their own plural: *criterion → criteria, phenomenon → phenomena, analysis → analyses, crisis → crises, stimulus → stimuli, formula → formulae, curriculum → curricula, appendix → appendices*. The general trend is replacement by ordinary *-s* (*formulas, forums, stadiums*), especially in colloquial speech; sometimes the two plurals diverge in meaning (*indexes* of books — *indices* in mathematics; *geniuses* people — *genii* spirits).

**Conditions and exceptions.**
- Types: *-on → -a* (*criterion, phenomenon*); *-is → -es* [iːz] (*analysis, basis, crisis, thesis, hypothesis, diagnosis, parenthesis, oasis, axis*); *-us → -i* (*stimulus, alumnus, fungus, nucleus, radius, cactus, syllabus, focus*); *-um → -a* (*curriculum, memorandum, stratum, erratum*); *-a → -ae* (*formula, larva, antenna, vertebra*); *-ex/-ix → -ices* (*index, appendix, matrix, vertex*); *-eau → -eaux* (*bureau, plateau*); *-im* (*cherubim, seraphim*).
- *Data, media, agenda* were once plural only; now *agenda* is an ordinary singular noun, and *data* and *media* are often used as singular collectives (*the data is*). EWT annotates them by agreement, most often NN, `Sing`, with lemma *data, media*.
- *Series, species* are the same in both numbers (see `noun-plural-zero`).
- Using *criteria, phenomena* as singular (*a criteria*) is an error in the text; the annotation then follows agreement, and the rule below has level `warn`.

**Examples.** *two **criteria**; the **analyses** show; **stimuli** of this kind; the **appendices**.*

**In UD.** NOUN, NNS, `Number=Plur`, lemma — the foreign singular (*criteria → criterion, analyses → analysis, stimuli → stimulus, appendices → appendix*).

**Sources.** Sweet NEG I §1007–1015 (text-1, p. 346–348: *formula–formulae, fungus–fungi, erratum–errata, analysis–analyses, appendix–appendices, phenomenon–phenomena, cherubim*; *data, agenda* plural only; *genii/geniuses*; general replacement by *-s*); Whitney §126 (text-1, p. 72–73: *phenomena, strata, genera, formulae, genii, analyses, beaux, indices, cherubim*; parallel *-s*); Santorini 1990, §4.2, p. 24 (*data* — NN or NNS by usage); Jespersen MEG II 2.6₁–2.6₉ (text-2, p. 67–72: *formulas, antennas; bonuses, circuses; memorandums; museums*); Kruisinga II.2 §§775–782 (text-3, p. 35–37: *agendas* — already singular); Mätzner I, p. 223–233 (text-1, p. 241–251).

```rule
rule: en.morph.foreign-plural-number
what: criteria, phenomena, analyses, stimuli, appendices etc. — plural, NNS
match: n[upos=NOUN, form=criteria|phenomena|analyses|crises|theses|hypotheses|diagnoses|parentheses|syntheses|oases|stimuli|alumni|cacti|fungi|nuclei|radii|syllabi|foci|curricula|memoranda|strata|errata|appendices|indices|matrices|vertices|formulae|antennae|larvae|vertebrae, !feats.Typo]
require: n[xpos=NNS, feats.Number=Plur]
severity: warn
source: Sweet NEG I §1007–1015; Whitney §126
```

```rule
rule: en.morph.foreign-plural-lemma
what: the lemma of a foreign plural is the foreign singular
match: n[upos=NOUN, form=criteria|phenomena|analyses|crises|theses|hypotheses|diagnoses|parentheses|syntheses|oases|stimuli|alumni|cacti|fungi|nuclei|radii|syllabi|foci|curricula|memoranda|strata|errata|appendices|indices|matrices|vertices|formulae|antennae|larvae|vertebrae, !feats.Typo]
require: n[lemma=criterion|phenomenon|analysis|crisis|thesis|hypothesis|diagnosis|parenthesis|synthesis|oasis|stimulus|alumnus|cactus|fungus|nucleus|radius|syllabus|focus|curriculum|memorandum|stratum|erratum|appendix|index|matrix|vertex|formula|antenna|larva|vertebra]
severity: warn
source: Sweet NEG I §1007–1015; Whitney §126
```
