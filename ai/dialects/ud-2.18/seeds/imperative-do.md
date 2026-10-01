# Main verb with imperative *do*: Inf in the text, Imp in the data

**Gist.** By the rule of `VerbForm.md`, a VB with an auxiliary has `VerbForm=Inf`. In *Don't go* there is an auxiliary *do*, so *go* should be Inf. In 98 cases of 106 EWT gives the main verb `Mood=Imp|VerbForm=Fin`, just like *Do* itself.

**Guideline.**
- `2.18:https://universaldependencies.org/en/feat/VerbForm.html:12`: VB has `Fin` «if they don't have an auxiliary or modal verb attached to it»;
- `2.18:https://universaldependencies.org/en/feat/VerbForm.html:21`: VB has `Inf` «if they have an auxiliary or modal verb or the inifinitval _to_ attached to it».

There is no exception for imperative *do*.

**EWT 2.18 data** (engine):
- VB with `aux` *do* that has `Mood=Imp` — 106;
- of them `Mood=Imp|VerbForm=Fin` — 98 (*Don't worry*, *Do not hesitate*, *DO NOT GO HERE!!!*);
- `VerbForm=Inf` — 8. These are the same constructions: *Don't cling* (newsgroup-groups.google.com_magicworld_04c89d43ff4fd6ea_ENG_20050104_152000-0065), *Don't go!* (reviews-263630-0009), *don't let that fool you* (reviews-280663-0008), *Do not go there* (reviews-020992-0012) and others.

Separately: *Don't* with `Mood=Ind` in imperative sentences — 7 (report `verbal.md`, `vb-aux-infinitive`). These are errors on *do* itself, not on the main verb.

**Where the discrepancy comes from.** The text describes a rule by form (an auxiliary is present → Inf). EWT annotates by sentence type: the whole sentence is imperative, so the main verb is too. The 8 cases with Inf are a data inconsistency.

**Sources.** `2.18:https://universaldependencies.org/en/feat/VerbForm.html; seed `en/seeds/verbal/imperative.md` (`en.verbal.imperative-do-main-verb`); report `data/runs/bones-2026-09-25/verbal.md`, discrepancy 2.

"Fired" — all VB with imperative *do*; "violations" — those that are not Inf, as the text requires.

```rule
rule: ud218.imperative-do-inf
what: VB with imperative do — VerbForm.md requires Inf, EWT uses Mood=Imp|VerbForm=Fin
match: v[xpos=VB]; d[lemma=do, rel=aux, head=v, feats.Mood=Imp]
require: v[feats.VerbForm=Inf]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/feat/VerbForm.html:12, :21
```
