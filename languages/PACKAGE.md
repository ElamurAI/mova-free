# What every language gets

Mova takes on a language with the same package, so that every language reaches the same footing:

1. **A grammar manual** in `languages/<code>/` (a variety as `<code>-x-<variety>/` or as its own chapters), written in our own words from public-domain grammars and open research. Its examples come from the corpora Mova holds. Every chapter ends with how Mova handles the point and where it stops. `sources.md` lists the sources; works under share-alike or non-commercial terms are cited and paraphrased, never copied. The model is [hbo](hbo/README.md).
2. **Papers** on the language's linguistics and NLP, each with its card on [Elamur Research](https://research.elamur.ai/free/).
3. **Open annotated corpora**: treebanks, coreference, frames, named entities, discourse, with their licences (`licence/DATA.md`). Non-commercial and research-only data is for tests and measurements only.
4. **Wiktionary**, raw: the language's own Wiktionary and the English Wiktionary's entries for it. It is CC BY-SA, so it is for learning and tests only.
5. **Open parsers** as witnesses: Stanza, Trankit, UDPipe, spaCy and the language's own (on the GPU or not), set up as in [docs/parsers.md](../docs/parsers.md).
6. **A baseline of Mova's own parser** on the language's UD treebanks (`en xpos-fill`, then `en parse-eval`: train → dev), with LAS for its own tags and for gold tags, dated.

## Baselines (UD 2.18, train → dev, 2026-10-22)

| Language | Treebank | LAS, our tags | LAS, gold tags |
|---|---|---:|---:|
| French | GSD | 87.00 | 89.78 |
| Latin | LLCT | 87.35 | 87.80 |
| Ancient Greek | PTNK | 74.41 | 78.29 |
