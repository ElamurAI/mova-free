# Treebanks: seeds, papers and converters

Every UD treebank has a folder here, with what Mova knows about it: its annotation habits, the papers about it, and the rules for converting between its dialect and Mova's.

**Layout:** `treebanks/<language>/<treebank>/`. The language is the ISO code, as in the UD files: `en`, `uk`, `de`…

Each treebank folder holds:
- `README.md`: a card from the UD release metadata — licence, genres, size, authors;
- `papers.md`: the papers about the treebank, linked to the library;
- `seeds/*.md`: seeds — the treebank's annotation conventions, how it differs from the Mova dialect, and known errors;
- `convert.md`: the converter rules between the Mova dialect and the treebank's.

[`docs/dialect.md`](../docs/dialect.md) explains why all this exists and how the converters work. Representations and UD versions (SUD, Enhanced UD, older UD, Stanford Dependencies…) live next door, in [`dialects/`](../dialects).

Code generates the cards and the paper lists from the UD release; `INDEX.md` is the index. The seeds are written by hand. Missing papers were fetched into the library by its usual pipeline.

## Cards, papers and indexes (`tools/treebanks`, 2026-09-26)

**Generated** by `tools/treebanks build`:

```
treebanks build raw/multi-ud/ud-treebanks-v2.18.tgz raw/ud-docs/docs/treebanks \
  raw/acl-anthology/anthology+abstracts.bib.gz papers treebanks runs/treebanks-2026-09-26
```

(The `raw/` and `runs/` paths are in the data directory.) The command writes only the treebanks' `README.md` and `papers.md`, plus `<language>/README.md` and `INDEX.md`. It never touches `seeds/` or `convert.md`. Do not edit generated files by hand: regenerate them.

**A card** (`<language>/<treebank>/README.md`) has:
- the UD name, the language and its family, licence, genres, authors and contact, and «in UD since», all from the machine-readable metadata of the release README;
- the layers Lemmas, UPOS, XPOS, Features and Relations, with how each was annotated (manual native, converted, automatic…);
- the size per train/dev/test: sentences, tokens, words, multiword tokens, empty nodes;
- the Summary and Introduction of the README, quoted.

The language and treebank codes come from the `.conllu` file names (`fr_gsd-ud-train.conllu` → `fr/gsd`).

**Gates.** Nothing is written unless both hold:
- the size, counted by streaming through the archive, equals the release's `stats.xml` for every part and in total;
- every README has its metadata: licence, genres, the five layers, authors, «Data available since».

**Papers** (`papers.md`). References are taken from the release README and from the treebank's page in the UD docs: the References, Citation, How to cite and BibTeX sections, bibtex entries, and DOIs and ACL Anthology URLs anywhere. They are matched in this order:
1. an ACL id from a URL or a DOI `10.18653/v1/…`: the library first, then ACL;
2. another DOI;
3. bibtex: the title equals the paper's title;
4. free text: the paper's whole title stands in the reference, with word boundaries.

Steps 3 and 4 also need the year and an author's surname.

A year off by one is accepted only for long titles (≥ 6 words, ≥ 4 in bibtex) and is marked «year in README ±1» in the list. The reason: the TLT16 volume is dated 2017, while the conference took place in January 2018.

Whatever is not found goes, as a raw line, under «Outside the ACL Anthology — not fetched». When a title almost matched but failed one check, a hint stands next to it: «a similar paper … — not confirmed: year / authors / title boundaries».

**Missing papers** were fetched by the library's usual pipeline:
1. `sources bib-ids`: the manifest `treebanks.jsonl`, topic `treebanks`;
2. `sources plan`, then curl at ≤ 3 requests a second, then `sources verify`;
3. `papers import`, `notes`, `index`, `html`.

The working files of the run are in `runs/treebanks-2026-09-26/`:
- `refs.jsonl`: every reference item with its result;
- `title-matches.tsv`: matches by title and bibtex, for review by eye;
- `treebanks-ids.txt`, `treebanks.jsonl`: what was fetched.

**State (UD 2.18):**
- 353 treebanks, 193 languages (codes);
- 734 reference items: 312 matched to the library, 422 outside the ACL Anthology;
- 217 distinct papers, 283 treebank–paper pairs;
- 86 papers fetched: 59 with text in the library, 26 under NC kept as a card only, and W17-8102 with no PDF on the Anthology.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
