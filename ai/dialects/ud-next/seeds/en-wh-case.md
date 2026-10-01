# *who, whom, whose*: case and the lemma *who* (English)

**Gist.** After 2.18 the English `PRON.md` closed two TODOs. *Who* pronouns got the `Case` feature: *who* — `Nom` or `Acc` by context, *whom* — `Acc` with lemma *who*, dependent *whose* — `Case=Gen|Poss=Yes`, standalone *whose* — only `Poss=Yes`. Likewise *whoever*, *whomever* (lemma *whoever*) and the new form *whosever*.

**Level.** English, `_en/pos/PRON.md`. A guideline, not the validator: the feature registry for en did not change.

**Before (2.18).** `2.18:https://universaldependencies.org/en/pos/PRON.html:48` — «[PronType]() is the only feature except where shown below». TODO at `:66` (*whom(ever)*: case and lemma *who*) and `:68` (*whose*: `Poss=Yes`, possibly `Case=Gen`). Details — `dialects/ud-2.18/seeds/wh-pronouns.md`.

**Now (snapshot 24.09.2026).**
- https://universaldependencies.org/en/pos/PRON.html:48`: «`Case` is marked only for _who_ and variants»;
- `:50–54` — a table:

  | | `Case=Nom` | `Case=Acc` | dependent possessive: `Case=Gen`, `Poss=Yes` | independent possessive: `Poss=Yes` |
  |---|---|---|---|---|
  | `PronType=Int` or `Rel` | who | whom [who], who | whose | whose |
  | `PronType=Int` or `Rel` | whoever, *whosoever* | whomever [whoever], whoever | whosever | — |
  | XPOS | WP | WP | WP$ | WP$ |

- `:56`: «_who_ and _whoever_ may bear either value of `Case`, depending on context. The variant _whosoever_ receives `Style=Form`»;
- the TODOs about *whom* and *whose* were removed; TODOs remain about *whatsoever*, *whatever* as DET and exclamative *what* (`:68–72`).

**Evidence.** `diff` `2.18:https://universaldependencies.org/en/pos/PRON.html ↔ https://universaldependencies.org/en/pos/PRON.html: lines 48–69. Context — #517 «Lemmas of English personal pronouns» (closed 19.11.2025, milestone v2.18): «I am not sure why *whom*, *whomever* … aren't normalized to nominative». There is no separate issue about the change itself in the dump. Treebank branches are not in the snapshot, so whether EWT `dev` has already reworked this is unknown.

**What it means for English annotation.**
- lemma *who* for all forms, as in GUM, GENTLE, LinES and ParTUT. EWT 2.18 has lemma *whom* (23);
- case in FEATS for 447 EWT tokens: *who* 422 (one of them a typo with lemma *whoever*), *whom* 23, *whoever* 1, *whomever* 1;
- on *whose* (13) — `Case=Gen`: all 13 in EWT are `nmod:poss`, i.e. dependent;
- `Case` for *who* is taken from the syntactic function. This agrees with the 2.19 clarification on features from context (`features-context.md`). The form *whom* gives `Acc` even in subject position, because the form weighs more.

**Registry.** No new pairs needed: `feat PRON Case=Nom|Acc|Gen` and `feat PRON Poss=Yes` are already in `en/data/ud-registry-en.tsv`. Only `Style=Form` for *whosoever* is missing, but that was so in 2.18 too, and EWT has no such form.

**Sources.** https://universaldependencies.org/en/pos/PRON.html, `2.18:https://universaldependencies.org/en/pos/PRON.html; #517, #1009 (independent possessives); seed `en/seeds/morph/pron-wh.md` (`en.morph.whom-acc` is already written according to this guideline).

EWT 2.18 does not conform to these rules, and that is the negative control: the error rules give 24 (*whom*) + 423 (*who* without `Case`) + 13 (*whose*) violations, the warn rules 7 + 6 + 396. After the `mova → ud-next` converter (`../convert.md`) — 0, checked with the engine on 26.09.

```rule
rule: udnext.whom-who-acc
what: whom/whomever — lemma who/whoever and Case=Acc
match: w[upos=PRON, form=whom|whomever]
require: w[lemma=who|whoever, feats.Case=Acc]
severity: error
source: https://universaldependencies.org/en/pos/PRON.html:52–53 (snapshot 24.09.2026)
```

```rule
rule: udnext.who-case
what: who/whoever always has Case — Nom or Acc by context
match: w[upos=PRON, form=who|whoever]
require: w[feats.Case=Nom|Acc]
severity: error
source: https://universaldependencies.org/en/pos/PRON.html:48, :56 (snapshot 24.09.2026)
```

```rule
rule: udnext.who-object-acc
what: who/whoever as an object — Case=Acc
match: w[upos=PRON, form=who|whoever, rel=obj|iobj]
require: w[feats.Case=Acc]
severity: warn
source: https://universaldependencies.org/en/pos/PRON.html:52, :56 (snapshot 24.09.2026)
```

```rule
rule: udnext.who-oblique-acc
what: who/whoever as a prepositional object (who … to, who … with) — Case=Acc; the head of a free relative is left alone
match: w[upos=PRON, form=who|whoever, rel=obl|obl:agent|nmod]
require: w[feats.Case=Acc]
unless: exists r[rel=acl:relcl, head=w]
severity: warn
source: https://universaldependencies.org/en/pos/PRON.html:52, :56 (snapshot 24.09.2026)
```

```rule
rule: udnext.who-subject-nom
what: who/whoever as a subject — Case=Nom
match: w[upos=PRON, form=who|whoever, rel~nsubj]
require: w[feats.Case=Nom]
severity: warn
source: https://universaldependencies.org/en/pos/PRON.html:52, :56 (snapshot 24.09.2026)
```

```rule
rule: udnext.whose-dependent
what: whose with a noun — Case=Gen and Poss=Yes
match: w[upos=PRON, form=whose, rel=nmod:poss]
require: w[feats.Case=Gen, feats.Poss=Yes]
severity: error
source: https://universaldependencies.org/en/pos/PRON.html:52 (snapshot 24.09.2026)
```

```rule
rule: udnext.whose-independent
what: standalone whose — Poss=Yes without Case
match: w[upos=PRON, form=whose, rel!=nmod:poss]
require: w[feats.Poss=Yes, !feats.Case]
severity: warn
source: https://universaldependencies.org/en/pos/PRON.html:52 (snapshot 24.09.2026)
```
