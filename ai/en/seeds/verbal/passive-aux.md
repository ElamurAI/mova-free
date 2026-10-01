# Passive: be or get + past participle

**Gist.** The passive voice shows that the subject undergoes the action rather than performs it. It is built with *be* (more rarely colloquial *get*) and the past participle: *was killed*, *is being built*, *got stolen*. The passive is possible only from verbs that take an object (transitive): *Caesar was slain*, but not *\*He was arrived*.

**Conditions and exceptions.** Other verbs with similar meaning (*become, seem*) are never passive auxiliaries: *He became known* — `xcomp`. Passive infinitive — *to be given*, passive perfect — *has been given*, passive gerund — *being punished*. A prepositional passive (*That matter was talked about*) is also a passive; the preposition is left without a noun.

**Examples.** *Kennedy was killed.* — *Kennedy got killed.* — *He was to be released before dawn* (be — aux:pass, was — aux).

**In UD.** The passive auxiliary is `aux:pass`, lemma *be* or *get*, UPOS `AUX`. Its head is `VERB`, XPOS `VBN`, `Voice=Pass`.

**Sources.** Brown 1851, Part II, Ch. VI, "III. Form of Passive Verbs" ("made from active-transitive verbs"; ); Reed & Kellogg, Higher Lessons, Lesson 129 (Voice; ); Poutsma 1923, *The Infinitive…*, §58 (passive infinitive: to be given; vol. 2, p. 71); https://universaldependencies.org/en/dep/aux-pass.html, https://universaldependencies.org/en/feat/Voice.html.

```rule
rule: en.verbal.passive-aux-lemma
what: the passive auxiliary is only be or get
match: a[rel=aux:pass]
require: a[lemma=be|get]
severity: error
source: https://universaldependencies.org/en/dep/aux-pass.html; https://universaldependencies.org/en/pos/AUX_.html
```

```rule
rule: en.verbal.passive-aux-head
what: the head of aux:pass is a past participle with Voice=Pass
match: h[]; a[rel=aux:pass, head=h]
require: h[upos=VERB, xpos=VBN, feats.Voice=Pass]
severity: error
source: https://universaldependencies.org/en/feat/Voice.html; Brown 1851 Part II Ch. VI, Form of Passive Verbs
```
