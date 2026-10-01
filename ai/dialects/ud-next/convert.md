# Converter `mova` ⇄ `ud-next`

The rules are written in the `convert` language from `train/dialects-and-converters.md`: the v1 seed rule language plus `set` actions. They were run with the real engine `en convert` / `en convert-check` (`en::convert`, commit b23ef218 of 26.09.2026). Results are in the "Check" section. Rule ids follow the repo custom: `mova.ud-next.*` for the `mova → ud-next` direction, `ud-next.*` for the reverse.

**What is converted to what.**
- `mova` — the state as of 26.09.2026: UD 2.18 with EWT conventions and our own normalizations. It does not yet contain the `ud-next` changes: they are only proposed in `dialects/mova/`.
- `ud-next` — the same EWT conventions plus guideline changes after 2.18 (`seeds/`). This is how EWT 2.19 would look if it follows the new guidelines. EWT `dev` branches are not in the snapshot, so there is no real EWT 2.19 to compare with yet.
- If Mova accepts the proposals, `mova` itself will become like `ud-next` at these points. Then these rules, with `from`/`to` swapped, will become part of the `mova → ud-2.18` converter.

**Two changes affect annotation:**
- *who* pronouns (`seeds/en-wh-case.md`);
- `discourse` on adverbs (`seeds/discourse-scope.md`).

The other changes do not touch annotation: clarification No. 18 legitimizes what EWT already does; XPOS is a format gate; layers, `expl:*` and validator data did not change for en.

## mova → ud-next

Order matters: the free-relative rule must fire before the general rules about *who*. All rules about `Case` have the condition `!feats.Case`, so a repeated run changes nothing.

```convert
rule: mova.ud-next.whom-lemma
what: whom → lemma who
from: mova
to: ud-next
match: t[upos=PRON, form=whom]
set: t[lemma=who]
source: https://universaldependencies.org/en/pos/PRON.html:52 (snapshot 24.09.2026)
```

```convert
rule: mova.ud-next.whomever-lemma
what: whomever → lemma whoever
from: mova
to: ud-next
match: t[upos=PRON, form=whomever]
set: t[lemma=whoever]
source: https://universaldependencies.org/en/pos/PRON.html:53
```

```convert
rule: mova.ud-next.whom-acc
what: whom, whomever — Case=Acc by form (the form wins over context, changes.md No. 18)
from: mova
to: ud-next
match: t[upos=PRON, form=whom|whomever, !feats.Case]
set: t[feats+=Case=Acc]
source: https://universaldependencies.org/en/pos/PRON.html:52–53; UD docs/changes.md:55–56
```

```convert
rule: mova.ud-next.who-freerel-acc
what: who/whoever — head of a free relative in object position; the subordinate clause has its own subject, so who is an object there — Acc
from: mova
to: ud-next
match: t[upos=PRON, form=who|whoever, !feats.Case, rel=obj|iobj|obl|obl:agent|nmod]; r[rel=acl:relcl, head=t]; s[rel~nsubj, head=r]
set: t[feats+=Case=Acc]
source: https://universaldependencies.org/en/pos/PRON.html:52–56
```

```convert
rule: mova.ud-next.who-freerel-nom
what: who/whoever — head of a free relative in object position without its own subject (for who wants…, of whoever wrote it), so who is the subject of the subordinate clause, Nom
from: mova
to: ud-next
match: t[upos=PRON, form=who|whoever, !feats.Case, rel=obj|iobj|obl|obl:agent|nmod]; r[rel=acl:relcl, head=t]
set: t[feats+=Case=Nom]
source: https://universaldependencies.org/en/pos/PRON.html:52–56
```

```convert
rule: mova.ud-next.who-acc
what: who/whoever as an object, direct or prepositional (who … to, who … with) — Acc
from: mova
to: ud-next
match: t[upos=PRON, form=who|whoever, !feats.Case, rel=obj|iobj|obl|obl:agent|nmod]
set: t[feats+=Case=Acc]
source: https://universaldependencies.org/en/pos/PRON.html:52, :56
```

```convert
rule: mova.ud-next.who-nom
what: who/whoever in all other positions (subject, question predicate, root) — Nom
from: mova
to: ud-next
match: t[upos=PRON, form=who|whoever, !feats.Case]
set: t[feats+=Case=Nom]
source: https://universaldependencies.org/en/pos/PRON.html:52, :56
```

```convert
rule: mova.ud-next.whose-dependent
what: whose with a noun — Case=Gen and Poss=Yes
from: mova
to: ud-next
match: t[upos=PRON, form=whose, rel=nmod:poss, !feats.Case]
set: t[feats+=Case=Gen; feats+=Poss=Yes]
source: https://universaldependencies.org/en/pos/PRON.html:52
```

```convert
rule: mova.ud-next.whose-independent
what: standalone whose — Poss=Yes without Case
from: mova
to: ud-next
match: t[upos=PRON, form=whose, rel!=nmod:poss]
set: t[feats+=Poss=Yes]
source: https://universaldependencies.org/en/pos/PRON.html:52
```

```convert
rule: mova.ud-next.discourse-adv
what: adverb with discourse → advmod (in English discourse is not for the ADV class)
from: mova
to: ud-next
match: t[rel=discourse, upos=ADV]
set: t[rel=advmod]
source: https://universaldependencies.org/u/dep/discourse.html:25–31
```

**About `feats+=`.** In the engine each FEATS feature is a separate field. `feats+=X=V` sets a value, replacing the existing one, and `feats-=X` together with `feats+=X=…` in one rule is a conflict. That never happens here: the condition `!feats.Case` is everywhere. `Poss=Yes` on *whose* is already in EWT, so the action changes nothing.

**DEPS.** The engine carries DEPS over unchanged. For `mova.ud-next.discourse-adv` this is a desync: `N:discourse` will remain in the EWT DEPS. Either a DEPS action or an EUD rebuild from the basic tree is needed. This is a question for the engine author.

**Changes on EWT 2.18** (train+dev+test; engine: 16,622 sentences, 254,820 words, 469 words changed; distribution by rule — from counters):

| rule | tokens |
|---|---:|
| `mova.ud-next.whom-lemma` + `mova.ud-next.whomever-lemma` | 24 |
| `mova.ud-next.whom-acc` | 24 |
| `mova.ud-next.who-freerel-acc` | 0 |
| `mova.ud-next.who-freerel-nom` | 2 (*for who wants*, *of whoever wrote it*) |
| `mova.ud-next.who-acc` | 13: `obj` 7, `obl` 6 (*who I need to send it to*, *who she worked with*…) |
| `mova.ud-next.who-nom` | 408: subject 396, the rest (12) — root, `ccomp`, `advcl`, `parataxis`, `conj`, `csubj` |
| `mova.ud-next.whose-dependent` | 13 |
| `mova.ud-next.whose-independent` | 0 |
| `mova.ud-next.discourse-adv` | 9 (train 4, dev 1, test 4) |

In total, *who* pronouns with `Case` after the run — 447 of 447: `Acc` 37 (*whom* forms 24, objects 13), `Nom` 410. This is what `udnext.who-case` requires.

**Edge cases.**
- A free relative stands with a copula, i.e. in predicate position: *that's who the photographers took pictures of*. The rule gives `Nom`, as for a predicate; traditional grammar would give `Acc` by the role inside. EWT has one such case (answers-20111104175256AAL21Ha_ans-0007).
- The free-relative rules are deliberately restricted to object position. On *who* in the role of subject or predicate they gave false `Acc`: *who would handle this … that we can speak to* (email-enronsent29_01-0006) and *they are NOT who … they are claiming to be* (answers-20111108101024AANYJpw_ans-0022).
- The role inside the free relative is in the EWT DEPS (for example, `5:nsubj`), but the rule language does not see DEPS.

## ud-next → mova

```convert
rule: ud-next.who-case-drop
what: drop Case from who pronouns (2.18 and EWT do not have it)
from: ud-next
to: mova
match: t[upos=PRON, lemma=who|whoever, feats.Case]
set: t[feats-=Case]
source: UD 2.18 https://universaldependencies.org/en/pos/PRON.html:48
```

```convert
rule: ud-next.whose-case-drop
what: drop Case=Gen from whose, keep Poss=Yes
from: ud-next
to: mova
match: t[upos=PRON, form=whose, feats.Case]
set: t[feats-=Case]
source: UD 2.18 https://universaldependencies.org/en/pos/PRON.html:68
```

```convert
rule: ud-next.whom-lemma
what: whom, whomever — lemma equals form (EWT 2.18 custom)
from: ud-next
to: mova
match: t[upos=PRON, form=whom|whomever]
set: t[lemma=@form]
source: EWT 2.18 (23 whom, 1 whomever)
```

There is no reverse rule for `discourse`: which `advmod` were `discourse` is not visible from the tree itself.

## Check

Engine `en convert` / `en convert-check`, 26.09.2026, EWT 2.18 train+dev+test (`mova` = EWT 2.18 as is).

| run | result |
|---|---|
| `en convert dialects/ud-next mova ud-next` | 10 rules; 469 words changed (FEATS 469, LEMMA 24, DEPREL 9) |
| `en expert-check dialects/ud-next/seeds` on the output | 0 violations of all `udnext.*` about *who* and `discourse`. 1 `udnext.vbz-3sg` remains: a token with empty FEATS, unrelated to the converter. `Case`: `Nom` 410, `Acc` 37, `Gen` 13 |
| `en convert dialects/ud-next ud-next mova` on the output, then `diff` with the original EWT | 3 rules; 460 words changed. Exactly 9 lines differ from the original — adverbs with `discourse` (*so* ×2, *though* ×3, *also*, *maybe*, *btw*, *FTW*). The rest matches byte for byte |
| `en convert-check dialects/ud-next ud-next` on the forward output | `ud-next → mova → ud-next`: 0 divergences; of the words changed by the forward run, 460 of 460 came back |

**What this means.**
- The forward direction makes EWT 2.18 conform to `ud-next`.
- The reverse one restores EWT except for the 9 deliberately lost `discourse`. These 9 already contradict the 2.18 guideline (`dialects/ud-2.18/seeds/discourse-adverbs.md`), so no MISC marks are introduced for them (`dialects/mova/seeds/discourse-adverbs.md`).
- For other English treebanks the reverse *whom* → lemma *whom* does not give identity: GUM, GENTLE, LinES and ParTUT already have the lemma *who*. That is handled by the treebank converters (`treebanks/en/*/convert.md`), not by this file.
- If EWT 2.19 annotates `Case` manually differently than by function, `convert-check ud-next` on it will show where our rule disagrees with theirs. That will be a useful report, not an error.

**Negative control** (by the engine, with a copy of the rules in a temporary folder):
- a reverse run will not notice a wrong `Case`, because the reverse drops `Case` entirely. So the control is the `udnext.*` rules from `seeds/`, run after the forward converter;
- `mova.ud-next.who-acc` with `Nom` instead of `Acc` and `mova.ud-next.whose-dependent` without `Case=Gen`: `udnext.who-object-acc` — 7 violations, `udnext.who-oblique-acc` — 6, `udnext.whose-dependent` — 13;
- on EWT 2.18 itself, without the converter, the `udnext.*` error rules give 24 + 423 + 13 + 9 violations. On a manual example with correct annotation — 0.

**Target `ud-next` gates:**
- tree;
- en registry: unchanged, `Case` on PRON is allowed;
- XPOS is not an empty string (validator 0.2.8, `seeds/xpos-not-empty.md`). This gate is in the CoNLL-U writer.
