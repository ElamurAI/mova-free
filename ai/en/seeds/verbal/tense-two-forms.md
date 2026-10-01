# Only two morphological tenses

**Gist.** The English verb inflects for only two tenses: present (*walks*) and past (*walked*). The future (*will walk*), perfect (*has walked*), progressive (*is walking*) and pluperfect (*had walked*) are combinations with auxiliaries, not separate word forms. So a feature "future tense" or "aspect" on the verb itself does not exist.

**Conditions and exceptions.** The past participle has `Tense=Past`, the *-ing* participle in progressive forms has `Tense=Pres`, although they do not express tense as such (Poutsma considered the names "present/past participle" unfortunate). Modals have no tense at all (see `morph/verb-md-modals.md`).

**Examples.** *I had been there* — *had*: `Tense=Past`, *been*: `VerbForm=Part|Tense=Past`, and no `Tense=Pqp`. — *She will go* — *go* has no `Tense=Fut`.

**In UD.** `Tense` is only `Pres` or `Past`; English verbs have no `Aspect` feature (it is absent from the UD 2.18 registry for `VERB`/`AUX`).

**Sources.** https://universaldependencies.org/en/feat/Tense.html ("pluperfect and future tenses in English are constructed periphrastically"); UD 2.18 feature registry (`ud-registry-en.tsv`); Jespersen, MEG IV (Time and Tense), ch. I–II (text-4); Poutsma 1923, *Participles*, §2 (on the names present/past; vol. 2, p. 184); Brown 1851, Part II, Ch. VI "Tenses".

```rule
rule: en.verbal.tense-pres-past
what: the Tense feature has only the values Pres or Past
match: v[feats.Tense]
require: v[feats.Tense=Pres|Past]
severity: error
source: https://universaldependencies.org/en/feat/Tense.html
```

There is no rule about Aspect: this feature is absent from the English UD registry, so an `Aspect=…` pair is already rejected by the FEATS parsing gate (`Feats::parse`, unknown pair). An engine rule would never fire (26.09).
