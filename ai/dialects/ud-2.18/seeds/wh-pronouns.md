# The pronouns *who, whom, whose* in 2.18: TODO in the guideline

**Gist.** In 2.18 `PRON.md` does not yet have a decision on the case of *who* pronouns: there are two TODOs. The first is to mark the case of *whom(ever)* and lemmatize *whom* as *who*. The second is to add *whose* `Poss=Yes` and possibly `Case=Gen`. The EWT 2.18 data already partly fulfil the second TODO (`Poss=Yes`), but not the first. After 2.18 the guideline was completed: `dialects/ud-next/seeds/en-wh-case.md`.

**2.18 guideline.**
- `2.18:https://universaldependencies.org/en/pos/PRON.html:48`: «[PronType]() is the only feature except where shown below»;
- `:52`: *who, whom* — WP; *whoever, whosoever, whomever* — WP; *whose* — WP$;
- `:66`: «TODO: tag *whom(ever)* for case; lemmatize *whom* as *who* and *whomever* as *whoever*»;
- `:68`: «TODO: add Poss=Yes and possibly Case=Gen for *whose*»;
- `:60`: *whosoever* has `Style=Form`. But in the validator registry for en the `Style` values are only `Arch, Coll, Expr, Slng, Vrnc`, so the validator will not accept `Style=Form`. EWT has no *whosoever*.

**EWT 2.18 data** (engine and grep):
- *who*: 421 (Rel 361, Int 60), without `Case`;
- *whom*: 23 with lemma *whom* (Rel 20, Int 3), without `Case`; *whomever*: 1 with lemma *whomever*;
- *whose*: 13, all `Poss=Yes|PronType=Rel`, without `Case=Gen`;
- in total *who, whom, whoever, whomever* without `Case` — 447 of 447.

**Other 2.18 treebanks.** The lemma *who* for *whom* is already written by GUM (16), GENTLE (6), LinES (15) and ParTUT (4). EWT, PUD and LittlePrince keep the lemma *whom*. No English treebank writes `Case` on *who* pronouns.

**Where the discrepancy comes from.** The 2.18 guideline acknowledged an open question (TODO, see #517 «Lemmas of English personal pronouns», closed 19.11.2025). EWT 2.18 reflects the state before the decision.

**Sources.** `2.18:https://universaldependencies.org/en/pos/PRON.html; `tools/data/feats.json` (en, Style); issue #517; seeds `en/seeds/morph/pron-wh.md` (`en.morph.whom-acc`), `en/seeds/nominal/who-whom-whose.md`; report `data/runs/bones-2026-09-25/morph.md`.

```rule
rule: ud218.whom-lemma
what: whom/whomever with its own lemma — the 2.18 TODO advises the lemma who/whoever
match: w[upos=PRON, form=whom|whomever]
require: w[lemma=who|whoever]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/pos/PRON.html:66
```

```rule
rule: ud218.who-case
what: who pronoun without Case — 2.18 TODO
match: w[upos=PRON, form=who|whom|whoever|whomever]
require: w[feats.Case]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/pos/PRON.html:66
```

```rule
rule: ud218.whose-poss
what: whose with Poss=Yes — 2.18 TODO, already present in the data
match: w[upos=PRON, form=whose]
require: w[feats.Poss=Yes]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/pos/PRON.html:68
```
