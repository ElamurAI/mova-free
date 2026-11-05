# The Mova dialect: a UD metadialect, treebanks and converters

This page covers `mova`, Mova's internal dialect of Universal Dependencies: why it exists, what it is built from, how treebanks and dialects are organized in the repository, the `convert` rule language and its commands, how converters are verified, and what training on it requires.

## What it is

UD treebanks are annotated in different dialects:
- **UD versions:** before 2.15 English had `:tmod`/`:npmod`; now it has `:unmarked`;
- **treebank conventions:** ATIS writes the imperative as `VerbForm=Inf`, ESLSpok has no lemmas or features, GUM uses `dep` more widely;
- **other representations:** SUD, Stanford Dependencies, classic PTB conversions.

Merged as they are, they would teach the model contradictions. So there is one internal dialect, **`mova`**, the English UD metadialect used by `en` and `annot`. Converter rules rewrite supported dependency-tree fields into it and back and can be checked in both directions; coverage is limited to the rules present in `dialects/` and `treebanks/`, and enhanced DEPS are carried through unchanged. UD labels are an export table, not internal identifiers; enrichments go in MISC (`Entity=`, `Cxn=`).

## The `mova` dialect

**Base:** UD 2.18 with EWT conventions. This is how `en` and `annot` annotate; the registry is `en/data/ud-registry-en.tsv`.

**Layers on top:**
- `ud-next`: accepted changes after the latest UD release, taken from a snapshot of the UD `docs` and `tools` repositories (`$MOVA_DATA/raw/ud-docs/`, `$MOVA_DATA/raw/multi-ud/`). Treebank branches are not in the snapshot.
- **Borrowings from alternative dialects**, only the ones we like: each decision is a seed in `dialects/mova/seeds.md` with a reason and evidence from data ([seeds/dialect-mova.md](seeds/dialect-mova.md)).
- **Own normalizations:** `det:poss` → `nmod:poss`, `:tmod`/`:npmod` → `:unmarked` (`annot::ann::ewt_rel`); FEATS from the subject (`en::ud::agreement`).

**Selection rule:** take whatever is more consistent and more informative and converts to the standard without loss. A lossy target is only an export target, never the base.

## Folders

```
treebanks/cards.tsv            — one fact row per UD treebank: licence, genres, layers, sizes, links, paper ids
treebanks/INDEX.md             — the index rendered from cards.tsv
treebanks/<lang>/<treebank>/   — Mova's notes on a treebank (lang is the ISO code, as in UD files)
  seeds.md                     — annotation conventions, differences from mova, known errors; one section per seed
  convert.md                   — converter rules between mova and the treebank's dialect, both directions
dialects/<id>/                 — representations and versions: mova, ud-2.18, ud-next, ud-2.14 (present);
                                 (other representations get a folder when they have rules)
  README.md, seeds/*.md, convert.md — the decisions, one seed per file, converter rules
```

Non-UD treebanks (PTB, OntoNotes, SUSANNE…) also go in `treebanks/`. Their data is often closed (LDC); we take the guidelines and seeds, and only open data.

## The converter rule language (`convert`)

It is the seed rule language ([english.md](english.md#rule-language)) plus `from:` and `to:` lines and `set` actions. The engine is `en::convert`; it reuses node and condition parsing and binding enumeration from `en::expert`.

```convert
rule: eslspok.tmod
what: :tmod → :unmarked (UD 2.15)
from: eslspok
to: mova
match: t[rel=obl:tmod|obl:npmod]
set: t[rel=obl:unmarked]
source: UD 2.15, #1028
```

**Fields:** required `rule`, `from`, `to`, `match`, `set`, `source`; optional `what`, `require`, `unless`. A rule applies to a binding where every `require` clause holds and no `unless` clause holds. An unknown or repeated field, or `from` = `to`, is a parse error. A block tagged ```` ```convert-todo ```` is not read: this disables a rule that cannot be written yet.

**`set` actions:** `node[action, action]; node[action]`, only on nodes from `match`; a node may be named twice.
- `rel=…`, `upos=…`, `xpos=…` (a PTB tag or `_`), `lemma=…`; `lemma=@form` sets the lowercased form.
- `feats+=X=V` sets or replaces; `feats-=X` removes; `feats+=X=@v` copies X from node `v` (nothing if `v` lacks it).
- `head=v` makes `v` the head; `head=0` makes the word the root; `head=@v` uses the head of `v`.
- `misc+=K=V` sets or replaces; `misc-=K` removes.

A comma in the value of `feats+=`, `lemma=`, `misc+=` is part of the value (`feats+=PronType=Int,Rel`); a piece without `=` after a comma joins the previous action, after `;` it is an error. A comma in a `match` condition value is not supported.

**Fail-fast values:** `rel`, `upos`, `xpos` are checked against the `Rel`, `UPos`, `Tag` enumerations; `feats+=X=V` against `Feat`; in `feats-=X` and `feats+=X=@v` the feature must be known; an unknown action, a node not from `match`, or `head=` pointing to itself is an error.

**How rules are applied:**
- **Order.** Rules for a `from → to` pair come from all `convert.md` files under the folder in alphabetical path order, then block order.
- **Bindings.** All bindings of a rule are found on the current sentence, `@form` and `@v` read from that state, then all actions run together. The next rule sees the result.
- **Conflict.** Two actions setting different values of the same field on one word are an error naming the rule, word and field. Each FEATS feature and MISC key is a separate field. Identical values are not a conflict.
- **FEATS and MISC.** A new feature is placed in case-insensitive alphabetical order; an existing one is replaced in place. A new MISC key goes at the end.
- **Tree gate after all rules:** heads within the sentence, exactly one root (HEAD 0, the only `root`), no cycles. There is no target-registry gate yet.
- **Everything else** (comments, multiword tokens, empty nodes, DEPS, MISC) is carried over byte for byte unless an action touches it. There are no DEPS actions: the enhanced graph stays in the source dialect.

**Lossless reading** (`en::conllu::Doc`) keeps comments and all raw columns, with a `Sentence` view for matching. Writing without edits gives identical bytes on every English UD treebank (test `full_round_trip_all_english_treebanks`). Format violations are errors with a line number: CR, a comment inside a sentence, not 10 columns, non-consecutive word numbers, an unknown relation.

**Commands:**
- `en convert <rules folder> <from> <to> <in.conllu>...`: CoNLL-U to stdout, summary to stderr.
- `en convert-check <rules folder> <dialect> <in.conllu>...`: an X → mova → X run. It reports forward changes per column with the most frequent replacements; reverse differences against the original per column (UPOS, XPOS, FEATS, LEMMA, HEAD, DEPREL, DEPS, MISC) with examples and the share of forward changes that returned exactly; and per rule, how many words it changed and, for reverse rules, how many match the original.

The rules folder can be anything: `dialects/`, `treebanks/`, the whole repo or one file. A parse error in any `convert.md` inside it stops both commands. Hidden folders and `target` are skipped.

## Verifying converters

- **Round trip:** X → mova → X on the whole treebank. Lossless pairs give zero differences; lossy ones get a per-column report with an explanation in `convert.md`.
- **Negative control:** a broken rule must produce differences (test `dialect_ud_2_14` breaks the "time → tmod" rule and expects one).
- **Repo gate:** test `repo_rules_all_parse` parses every `convert.md` in `dialects/` and `treebanks/`.

The converter in `dialects/ud-2.14` maps `:tmod`/`:npmod` ↔ `:unmarked`. The direction mova → ud-2.14 is lossy: the choice between `:tmod` and `:npmod` is reconstructed by ordered rules (measure, time lexemes, dates, the rest) and is sometimes wrong. All other columns return exactly; the analysis is in `dialects/ud-2.14/convert.md`.

## Training on the metadialect

Training inputs must pass the licence gate ([training data and licences](../en/README.md#layers)). `en convert-check`
reports round-trip differences for the configured converters; a passing round trip does not establish annotator
accuracy or full equivalence of representations. The converter works on CoNLL-U dependency trees; training a union of
every treebank, generalizing the parser beyond English, constituency, CCG, HPSG and meaning-graph conversion, and an
annotation more abstract than the UD layer are not implemented by these commands.
