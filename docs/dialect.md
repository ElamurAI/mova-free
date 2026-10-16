# The Mova dialect: a UD metadialect, treebanks, and converters

Mova has its own dialect of Universal Dependencies, called `mova`, set up on 2026-09-26:

- **It follows the latest UD**, and can adopt upcoming changes, such as the 2.19 fixes, before they are released. It also takes changes we like from alternative UD dialects.
- **Converters go both ways.** To produce EWT, any classic or other treebank, the newest standard UD or a specific UD variant, Mova converts from its dialect into the target and from the target back into the dialect.
- **Every treebank has a folder.** Each language gets a folder under [`treebanks/`](../treebanks), with a subfolder per treebank that describes its seeds and lists the papers about it.
- **The dialect learns from every treebank.** Once the converters exist, it is trained further on all their corpora.

## Why

UD treebanks are annotated in different dialects:
- **UD versions:** before 2.15, English had `:tmod`/`:npmod`; now it has `:unmarked`;
- **conventions of individual treebanks:** ATIS writes the imperative as `VerbForm=Inf`, ESLSpok has no lemmas or features, GUM uses `dep` more widely;
- **other representations:** SUD, Stanford Dependencies, classic PTB conversions.

They cannot be merged as-is for training: the model would learn contradictions. So there is one internal dialect, **`mova`**, plus converters from every treebank and representation into it and back. Then:
1. all corpora are converted to `mova` and train the Rust tagger/parser `en` (and later other languages);
2. `mova` output is converted to any target: EWT 2.18, the newest UD, older UD, SUD, a treebank with its own conventions;
3. quality is measured on each treebank's gold set in its own dialect, via the reverse converter.

Background: the digest the UD survey digest «future», section "For Mova":
- UD labels are an export table, not internal identifiers;
- UD, SUD, and UCxn are equal "translations";
- EUD is a derived layer;
- enrichments go in MISC (`Entity=`, `Cxn=`).

## Not just UD

Design note (2026-09-26): the idea is abstract and works with any trees, even non-UD ones.

The "treebank → meta → treebank" cycle and the "two roads to meta" do not depend on the annotation scheme. All that is needed is a reader and writer for the format and a converter into meta and back. The same approach will work for:
- **PTB and OntoNotes constituency trees:** head rules give dependencies, and projection rules give the way back;
- **SUD**: function words as heads;
- **CCG** (CCGbank), **HPSG**;
- **Prague tectogrammatics (PDT)** and **Paninian karaka relations**;
- **AMR/UMR**: meaning graphs.

`mova` is a UD dialect at first. That is the first step, because UD has the most data. The later goal is the abstract annotation of the Mova School (`docs/school.md`): a predicate, participants with roles, universal features. UD then becomes one of its projections, alongside other schemes.

The `convert` rule language currently rewrites labels and heads on dependency trees. Constituency, CCG, or meaning graphs will need their own readers and structural transformations: we add them when that scheme's turn comes.

## Translation via meta

The metas of different languages and the transformers between them are planned separately.

## The `mova` dialect

**Base**: UD 2.18 with EWT conventions. This is how `en` and `annot` annotate today; the registry is `en/data/ud-registry-en.tsv`.

**Layers on top of the base:**
- `ud-next`: changes after the 2.18 release that will go into 2.19. A snapshot of the `docs` and `tools` repos from 2026-09-24 against the 2.18 release archives: `raw/ud-docs/` in the data directory and `$MOVA_DATA/raw/multi-ud/ud-{documentation,tools}-v2.18.tgz`. Treebank branches from GitHub are not in the snapshot.
- **Borrowings from alternative dialects**: only the ones we like. Each decision is a separate seed in `dialects/mova/seeds/` with a reason and evidence from data.
- **Our own normalizations:** `det:poss` → `nmod:poss`, `:tmod`/`:npmod` → `:unmarked` (`annot::ann::ewt_rel`); FEATS from the subject (`en::ud::agreement`).

**Selection rule:** take whatever is more consistent and more informative and can be converted to the standard without loss. A lossy target is only an export target, never the base.

## Folders

```
treebanks/<lang>/<treebank>/   — a treebank as a dataset (lang is the ISO code, as in UD files: en, uk, de…)
  README.md                    — card: metadata from the UD release README (license, genres, size, authors), description
  papers.md                    — papers about the treebank, linking to the papers/ library
  seeds/*.md                   — seeds: annotation conventions, differences from mova, known errors
  convert.md                   — converter rules between mova and the treebank's dialect (both directions)
dialects/<id>/                 — representations and versions: mova, ud-2.18, ud-next, ud-2.14, ud-1, eud, sud, sd, conllx-ptb, ud-v3-draft
  README.md, papers.md, seeds/*.md, convert.md — same as above
```

Non-UD treebanks (PTB, OntoNotes, SUSANNE…) also go in `treebanks/`. Their data is often closed (LDC), so we take the card, papers, and guidelines, but only open data.

## The converter rule language (`convert`)

This is the seed rule language (`docs/expert.md`, v1) plus `from:` and `to:` lines and `set` actions. The engine is `en::convert`. It reuses node and condition parsing and binding enumeration from `en::expert`, without copies.

```convert
rule: eslspok.tmod
what: :tmod → :unmarked (UD 2.15)
from: eslspok
to: mova
match: t[rel=obl:tmod|obl:npmod]
set: t[rel=obl:unmarked]
source: UD 2.15, #1028
```

**Fields:**
- required: `rule`, `from`, `to`, `match`, `set`, `source`;
- optional: `what`, `require`, `unless`.

`require` and `unless` filter bindings; the clause forms are the same as in seeds. A rule applies to a binding where every `require` clause holds (one ` or ` alternative is enough within a clause) and no `unless` clause holds. An unknown field, a repeated field, or `from` = `to` is a parse error.

A block with a different tag (```` ```convert-todo ````) is not read by the engine: this is how to disable a rule that cannot be written yet.

**`set` actions:** `node[action, action]; node[action]`, only on nodes from `match`. Nodes are separated by `;`, actions in brackets by `,` or `;`. A node may be named twice.
- `rel=…`, `upos=…`, `xpos=…` (a PTB tag or `_`), `lemma=…`; `lemma=@form` sets the lowercased form.
- `feats+=X=V` sets or replaces; `feats-=X` removes; `feats+=X=@v` copies feature X from node `v`. If `v` lacks it, nothing happens.
- `head=v` makes node `v` the head; `head=0` makes it the root; `head=@v` uses the head of node `v`.
- `misc+=K=V` sets or replaces; `misc-=K` removes.

A comma in the value of `feats+=`, `lemma=`, `misc+=` is part of the value: `feats+=PronType=Int,Rel`. A piece without "=" after a comma is appended to the previous action; after `;` it is an error. A comma in a `match` condition value (`feats.PronType=Int,Rel`) is not supported, just as in seeds.

**Values are checked at parse time (fail-fast):**
- `rel`, `upos`, `xpos` against the `Rel`, `UPos`, `Tag` enumerations;
- `feats+=X=V` against the `Feat` enumeration (all pairs of English UD 2.18); in `feats-=X` and `feats+=X=@v` feature X must be known;
- an unknown action, a node not from `match`, or `head=` pointing to itself is an error.

**How rules are applied:**
- **Order.** Rules for a `from → to` pair are taken from all `convert.md` files under the folder, in alphabetical path order; within a file, in block order.
- **Bindings.** For a rule, all bindings are found on the current state of the sentence. The values of `@form` and `@v` come from that same state. Then all actions are executed together. The next rule sees the result.
- **Conflict.** Two actions that set different values of the same field on one word are an error, reported with the rule id, word, and field. Each FEATS feature and each MISC key is a separate field. Identical values from two bindings are not a conflict.
- **FEATS and MISC.** A new FEATS feature is placed in case-insensitive alphabetical order, as in CoNLL-U; an existing one is replaced in place. A new MISC key goes at the end. Everything else, including features outside `Feat`, is unchanged.
- **Tree gate after all rules:**
  - the head is a number within the sentence;
  - exactly one root (HEAD 0), and only it has `root`;
  - no cycles.
  There is no target registry as a gate yet.
- **The rest of CoNLL-U** (comments, multiword tokens, empty nodes, DEPS, MISC) is carried over byte for byte unless an action touches it. There are no actions for DEPS: the enhanced graph stays in the source dialect.

**Lossless reading**: `en::conllu::Doc`:
- comments and all lines with raw columns;
- for words, a `Sentence` view, on which matching runs.

Writing without edits yields identical bytes on all 29 files of English UD 2.18, and the view matches `conllu::read` (test `full_round_trip_all_english_treebanks`). Format violations are errors with a line number:
- CR;
- a comment in the middle of a sentence;
- not 10 columns;
- non-consecutive word numbers;
- unknown relation.

**Commands:**
- `en convert <rules folder> <from> <to> <in.conllu>...`: CoNLL-U to stdout, summary to stderr.
- `en convert-check <rules folder> <dialect> <in.conllu>...`: an X → mova → X run. Report:
  - forward: how many words and sentences changed, per column, with the most frequent replacements;
  - reverse against the original: UPOS, XPOS, FEATS, LEMMA, HEAD, DEPREL, DEPS, MISC: words, sentences, "original → after" pairs, up to three examples; the share of forward-changed items that returned exactly;
  - per rule: how many words each changed, and for reverse rules, how many of them match the original.

The rules folder can be anything: `dialects/`, `treebanks/`, the whole repo, or a single file. A parse error in any `convert.md` inside it stops both commands. Hidden folders and `target` are skipped.

**Converter verification:**
- **Round trip:** X → mova → X on the whole treebank. For lossless pairs, 0 differences. For lossy ones, a per-column report with an explanation in `convert.md`.
- **Negative control:** a broken rule must produce differences. Test `dialect_ud_2_14` breaks the "time → tmod" rule and expects a difference.
- **Repo gate:** test `repo_rules_all_parse` parses every `convert.md` in `dialects/` and `treebanks/`.

**The first converter is `dialects/ud-2.14`:** `:tmod`/`:npmod` ↔ `:unmarked`. The round trip returns exactly:
- ESLSpok: 90.7% of 151 labels;
- EWT r2.14 dev + test: 93.8%.

All other columns: 0 differences. Analysis is in `dialects/ud-2.14/convert.md`.

## Training on the metadialect

Design note (2026-09-26): take a French corpus, run it through our annotator for that corpus to get an annotator with their rules, build a converter to our meta, then convert from meta back to the French treebank, where it can be checked against the original.

**The cycle for each treebank:**
1. **An annotator "with their rules".** A tagger/parser trained on the treebank's corpus in its own dialect. This is the baseline.
2. **Converters** treebank → `mova` and `mova` → treebank.
3. **Converter verification.** Original → `mova` → back must equal the original. Differences are converter losses; we explain them in `convert.md` or fix them.
4. **Meta-annotator.** Trained on `mova` from all treebanks of the language. Its output → reverse converter → comparison with the treebank's gold. Compared against the annotator "with their rules": the metadialect must be no worse, and better on small treebanks.
5. **Two roads to meta.** Take their sentences, run them through our annotator with their rules and the converter into our UD meta; in parallel, train our direct UD annotator; then compare the two parses in our meta style, which should agree.
   - Road A: annotator "with their rules" → converter → `mova`.
   - Road B: direct meta-annotator → `mova`.
   - The parses from the two roads must match. A difference is a signal: a converter error, an error in one of the annotators, or a dialect contradiction. Such sentences are analyzed first, and agreement between two independent roads is a sign of quality.
   - This also works on raw text without gold. Use it to select sentences for silver, and send disagreeing ones to the Opus correction mode.

For languages other than English, the tagger/parser must work on UPOS and string XPOS: currently `en` holds PTB tags. This is a step in generalizing `en` into a language-independent crate.

**For English:**

1. Convert all English UD treebanks to `mova`.
2. Train `en` on the union. Layers a treebank lacks (lemmas or FEATS in ESLSpok) do not train, as already done in `en::ud`.
3. Measure on each treebank's test set: `en` output in `mova` → reverse converter → the treebank's gold.
4. Negative control:
   - training on converted data must not worsen EWT;
   - a converter with a deliberate error must worsen the result.
