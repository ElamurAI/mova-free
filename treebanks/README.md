# Treebanks: facts, seeds and converters

What Mova knows about Universal Dependencies treebanks: the facts of each release treebank, the papers about it, its annotation habits, and the rules for converting between its dialect and Mova's.

**Layout:**
- [`cards.tsv`](cards.tsv): one fact row per UD treebank — licence, genres, annotation layers, sizes per split, links, matched paper ids. Columns: [`tools/treebanks`](../tools/treebanks/README.md#cardstsv).
- [`INDEX.md`](INDEX.md): the index rendered from `cards.tsv`, with an anchor per treebank (`INDEX.md#en-ewt`).
- `<language>/<treebank>/`: Mova's own notes on a treebank, written by hand. The language is the ISO code, as in the UD files: `en`, `uk`, `de`…
  - `seeds.md`: seeds — the treebank's annotation conventions, how it differs from the Mova dialect, and known errors; one section per seed, anchored by its name (`en/ewt/seeds.md#errors`);
  - `convert.md`: the converter rules between the Mova dialect and the treebank's.
- [`en/seeds-overview.md`](en/seeds-overview.md): the English treebanks compared, with the counter rules shared by all of them.

The English treebanks have folders; `en/ptb/` holds the Penn Treebank's tagging customs (no UD data). [`docs/dialect.md`](../docs/dialect.md) explains why all this exists and how the converters work. Representations and UD versions (SUD, Enhanced UD, older UD, Stanford Dependencies…) live next door, in [`dialects/`](../dialects).

## Facts and papers (`tools/treebanks`)

`cards.tsv` and `INDEX.md` are generated; do not edit them by hand:

```
treebanks build $MOVA_DATA/raw/multi-ud/ud-treebanks-v2.18.tgz $MOVA_DATA/raw/ud-docs/docs/treebanks \
  $MOVA_DATA/raw/acl-anthology/anthology+abstracts.bib.gz $RESEARCH/papers treebanks $MOVA_DATA/runs/treebanks-2026-11-04
treebanks index treebanks     # INDEX.md only, after a folder gets seeds or a converter
```

The values come from the machine-readable metadata of each release README and from the counted sizes; the README text itself is not carried. The language and treebank codes come from the `.conllu` file names (`fr_gsd-ud-train.conllu` → `fr/gsd`). Readers: `tools/db` (licence, genres and authors of a loaded treebank; the size check of `db verify`).

**Gates.** Nothing is written unless both hold:
- the size, counted by streaming through the archive, equals the release's `stats.xml` for every part and in total;
- every README has its metadata: licence, genres, the five layers, authors, «Data available since».

**Papers** (column `papers`). References are taken from the release README and from the treebank's page in the UD docs: the References, Citation, How to cite and BibTeX sections, bibtex entries, and DOIs and ACL Anthology URLs anywhere. They are matched against the research library and the ACL Anthology, in this order:
1. an ACL id from a URL or a DOI `10.18653/v1/…`: the library first, then ACL;
2. another DOI;
3. bibtex: the title equals the paper's title;
4. free text: the paper's whole title stands in the reference, with word boundaries.

Steps 3 and 4 also need the year and an author's surname. A year off by one is accepted only for long titles (≥ 6 words, ≥ 4 in bibtex) and is marked `~y`: the TLT16 volume, for example, is dated 2017 while the conference took place in January 2018.

A reference with no match is counted in `unmatched_refs`; its raw line, with a hint when a title almost matched («similar paper … — not confirmed: year / authors / title boundaries»), is in the run's `unmatched.tsv`. Missing ACL papers go to the library by its usual pipeline, starting from the run's `missing-ids.txt` (`sources bib-ids`).

The run files are in `$MOVA_DATA/runs/treebanks-2026-11-04/`: `refs.jsonl` (every reference item with its result), `title-matches.tsv` (matches by title and bibtex, for review by eye), `unmatched.tsv`, `missing-ids.txt`.

**State (UD 2.18, run 2026-11-04):**
- 353 treebanks, 193 languages (codes);
- 737 reference items: 267 matched in the library, 55 in the ACL Anthology only, 415 without a match;
- 223 distinct papers, 291 treebank–paper pairs.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
