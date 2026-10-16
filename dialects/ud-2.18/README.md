# UD 2.18 — the standard

Release 2.18 of 15.05.2026: 353 treebanks, 193 languages. English has 14 treebanks in it, the main one is EWT. This is the base of the `mova` dialect (`docs/dialect.md`): "UD 2.18 with EWT conventions".

**What counts as the standard.** Three layers, and they do not always agree:
1. **Guidelines:** `markdown-source/` from `$MOVA_DATA/raw/multi-ud/ud-documentation-v2.18.tgz`. Universal — `_u-*`, `u/`; English — `_en/`.
2. **The validator and its data:** `$MOVA_DATA/raw/multi-ud/ud-tools-v2.18.tgz`, `validate.py`, `udtools` 0.2.7, `data/*.json`. For English they are collected in `en/data/ud-registry-en.tsv`: 251 "UPOS × feature" pairs, 58 relations, 16 auxiliaries. The registry matches the 2.18 release `data/*.json` line for line.
3. **EWT 2.18 data:** `$MOVA_DATA/en/ud/ud-treebanks-v2.18/UD_English-EWT/`, 16,622 sentences.

Where the `_en` text diverges from the data, `mova` takes the EWT data if they match the universal guideline. Below is each such discrepancy separately.

## Discrepancies between the `_en` guidelines and the EWT 2.18 data

Numbers are train+dev+test. Each seed has counter rules, so one command reproduces the numbers:

```
target/release/en expert-check dialects/ud-2.18/seeds \
  $MOVA_DATA/en/ud/ud-treebanks-v2.18/UD_English-EWT/en_ewt-ud-{train,dev,test}.conllu
```

| seed | the guideline says | EWT 2.18 | why |
|---|---|---|---|
| [iobj-sole](seeds/iobj-sole.md) | `iobj` only with `obj`/`ccomp`; with `xcomp` — `obj` | 795 `iobj`: 243 without `obj`/`ccomp`, of them 93 with `xcomp` | 2.12 amendment "Sole iobj" |
| [iobj-to-phrase](seeds/iobj-to-phrase.md) | *to me* — `nmod` | *to* phrase on a verb: `obl` 1285, `nmod` 0 | v1 leftover |
| [imperative-do](seeds/imperative-do.md) | VB with an auxiliary — `Inf` | with imperative *do*: `Mood=Imp\|Fin` 98, `Inf` 8 | EWT annotates by sentence type |
| [subjunctive](seeds/subjunctive.md) | subjunctive → `Inf` / `Ind` | `Mood=Sub` 57: VB 49, *were* 8; ≥5 subjunctives remained `Inf` | `Mood.md` not updated; `Tense.md` already mentions the subjunctive |
| [modal-mood](seeds/modal-mood.md) | every finite form has `Mood` | MD with `Fin` without `Mood` — 4049; validator warning | open question, #1118 |
| [depictives](seeds/depictives.md) | `acl` (:612) and `advmod` (:878) | ADJ depictive `advcl` 22, `acl` 0, `advmod` 0 | 2.10 amendment |
| [clausal-predicate](seeds/clausal-predicate.md) | the copula is the head, the clause is `ccomp` | `nsubj:outer`/`csubj:outer` 257, old analysis 0 | 2.10 amendment |
| [wh-pronouns](seeds/wh-pronouns.md) | TODO: `Case` and the lemma *who* for *whom*; `Poss` for *whose* | *whom* with lemma *whom* 23; `Case` 0 of 447; *whose* `Poss=Yes` 13 of 13 | the decision was made after 2.18 |
| [discourse-adverbs](seeds/discourse-adverbs.md) | only non-adverbial markers | ADV with `discourse` — 9 (in GUM — 78) | boundary by meaning, not by word class |
| [v1-labels](seeds/v1-labels.md) | examples with `dobj`, `nsubjpass`, `neg`, `mwe`… (36 lines) | 0 | the text has not been updated since v1 |

Conclusion: mostly the `_en` text is out of date, while the EWT data follow later universal amendments. Genuine inconsistencies in the data themselves:
- 8 imperatives with `Inf`;
- ≥5 subjunctives with `Inf`;
- 9 adverbs with `discourse`;
- the lexical `iobj`/`obj` split with `xcomp`: *permit* and *recommend* have both variants.

Decisions for `mova` are in `dialects/mova/`.

**Path notation.** `2.18:docs/…` — `markdown-source/…` from the 2.18 documentation archive. `2.18:tools/…` — the 2.18 tools archive. `docs/…` without a prefix — the post-2.18 snapshot (`$MOVA_DATA/raw/ud-docs/docs`). For all `_en/` files except `index.md` and `pos/PRON.md`, line numbers are the same in both. `#N` — an issue in `$MOVA_DATA/raw/ud-docs/issues/`; it holds only issue bodies, without comments.
