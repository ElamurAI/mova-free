# Indirect object without a direct one and with xcomp

**Gist.** The English guideline still says that `iobj` occurs only next to `obj` or `ccomp`, and never with `xcomp`. The universal 2.12 amendment removed this condition, and EWT 2.18 follows the amendment.

**Guideline.**
- `2.18:https://universaldependencies.org/en/dep/iobj.html:10`: «It occurs only when there is a `obj` or `ccomp` in the clause.»
- `2.18:https://universaldependencies.org/en/specific-syntax.html:141`: «Only when another internal argument is present can `iobj` occur.»
- `2.18:https://universaldependencies.org/en/specific-syntax.html:157`: with `xcomp` the nominal object is «uniformly labeled `obj`, and never `iobj`».
- The opposite — `2.18:UD docs/changes.md` No. 7 (01.2023, 2.12, AMENDMENT) «Sole `iobj`»: the condition of an obligatory `obj` is «hereby removed» (#916).

**EWT 2.18 data** (train+dev+test, 16,622 sentences, engine, rules below):
- `iobj` in total — 795;
- without `obj` and `ccomp` — 243 (31%). Of them 93 are with `xcomp`, the remaining 150 have no other internal argument at all (*reminds me*, *asked Bush*);
- with `xcomp` the custom is lexical:
  - `iobj` — 93: *ask* 25, *allow* 21, *tell* 14, *convince* 7, *urge* 6, *cause* 6, *trust* 3, *persuade* 3, *teach*, *remind*, *warn*, *instruct*…;
  - `obj` — 39 with verbs of the same class: *encourage* 8, *enable* 8, *order* 5, *invite* 5, *force* 4, *require* 4, *beg*;
  - *permit* and *recommend* occur both with `iobj` and with `obj`;
  - ordinary `obj` + `xcomp` (*let, make, get, have, keep, find, want, help*) — 931.

**Where the discrepancy comes from.** The `_en` pages were not updated after the 2.12 amendment. The EWT data match the universal guideline, not the English one.

**Other 2.18 treebanks.** `iobj` is present in all manually annotated ones: GUM 482, LinES 101, ParTUT 33, GENTLE 31.

**Sources.** `2.18:https://universaldependencies.org/en/dep/iobj.html; `2.18:https://universaldependencies.org/en/specific-syntax.html; `2.18:UD docs/changes.md#sole-iobj`; seeds `en/seeds/lexicon/verb-iobj-licensors.md`, `en/seeds/verbal/indirect-object.md`; report `data/runs/bones-2026-09-25/verbal.md`, discrepancy 1.

Counters: "fired" — all `iobj`, "violations" — those that contradict the `_en` text.

```rule
rule: ud218.iobj-sole
what: iobj without obj and ccomp — _en forbids it, the 2.12 amendment allows it
match: v[]; i[rel=iobj, head=v]
require: exists o[rel=obj|ccomp, head=v]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/dep/iobj.html:10; UD docs/changes.md#sole-iobj (2.12)
```

```rule
rule: ud218.iobj-xcomp
what: iobj together with xcomp — specific-syntax.md requires obj
match: v[]; i[rel=iobj, head=v]
require: none x[rel=xcomp, head=v]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/specific-syntax.html:157
```

```rule
rule: ud218.iobj-alone
what: iobj without obj, ccomp and xcomp — the only internal argument
match: v[]; i[rel=iobj, head=v]
require: exists o[rel=obj|ccomp|xcomp, head=v]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/dep/iobj.html:10; UD docs/changes.md#sole-iobj (2.12)
```
