# Imperative mood

**Gist.** A command or request is expressed by the form that coincides with the infinitive without *to*: *Make a sandwich!*, *Be careful*. The subject is usually omitted; if present, it stands before the verb (*You go first*, *Somebody help me*). Negative and emphatic imperatives use *do*: *Don't go*, *Do come*. An imperative verb has no other auxiliaries (modals, *have*, passive *be*).

**Conditions and exceptions.** *Let's go* is an imperative of *let* (VERB) with the object *us* and `xcomp`. *Don't be silly* — the imperative here is *Do* (AUX), and *be* is a copula. On the main verb with imperative *do*, the UD documentation and EWT diverge: `VerbForm.md` says to use `Inf` ("if there is an auxiliary"), while in EWT 2.18 in ~98 of ~106 cases it has `Mood=Imp|VerbForm=Fin`. The rule below follows EWT as a warning — the decision is Mova's.

**Examples.** *Read the book!* — *Don't take that deal.* — *\*Must try!* (error: a modal with an imperative).

**In UD.** Imperative: XPOS `VB`, `Mood=Imp`, `VerbForm=Fin`, no `Tense`. Among its `aux` there can only be *do*.

**Sources.** https://universaldependencies.org/en/feat/Mood.html (Imp); https://universaldependencies.org/en/feat/VerbForm.html (Fin for VB without an auxiliary); Reed & Kellogg, Higher Lessons, Lesson 56 (Imperative sentences: the subject is usually omitted) and Lesson 131 (Imperative Mode); Brown 1851, Part II, Ch. VI, Form of Negation (Love not, or Do not love); Jespersen, MEG V, ch. XXIV "Requests" (text-1).

```rule
rule: en.verbal.imperative-form
what: imperative — VB, VerbForm=Fin, no Tense
match: v[feats.Mood=Imp]
require: v[xpos=VB, feats.VerbForm=Fin]; not v[feats.Tense]
severity: error
source: https://universaldependencies.org/en/feat/Mood.html; https://universaldependencies.org/en/feat/VerbForm.html
```

```rule
rule: en.verbal.imperative-aux-do-only
what: with an imperative verb the only possible auxiliary is do
match: v[feats.Mood=Imp, upos=VERB]
require: none a[rel=aux|aux:pass, head=v, lemma=be|have|get|can|could|may|might|must|shall|should|will|would|ought|need|dare]
severity: error
source: Brown 1851 Part II Ch. VI (Do not love); https://universaldependencies.org/en/feat/Mood.html
```

```rule
rule: en.verbal.imperative-do-main-verb
what: the main verb with imperative do is also Imp/Fin (EWT practice; contradicts VerbForm.md)
match: v[xpos=VB, upos=VERB]; a[rel=aux, lemma=do, head=v, feats.Mood=Imp]
require: v[feats.VerbForm=Fin, feats.Mood=Imp]
severity: warn
source: UD_English-EWT 2.18 (annotation practice); divergence from https://universaldependencies.org/en/feat/VerbForm.html
```
