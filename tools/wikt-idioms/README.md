# wikt-idioms — multiword expressions from English Wiktionary

Extracts idioms, proverbs, phrases, phrasal and multiword verbs, and register- or region-labelled words from an English Wiktionary XML dump read from stdin. Its output feeds the expressions base described in [docs/pragmatics.md](../../docs/pragmatics.md) and the idiom level of [docs/knowledge.md](../../docs/knowledge.md).

## Run

```bash
cargo build --release -p wikt-idioms
lbzip2 -dc $MOVA_DATA/raw/wiktionary/enwiktionary/<date>/*-multistream*.xml*.bz2 | target/release/wikt-idioms > idioms.tsv
```

A page counts if its title has a space (or a hyphen between letters) and its `==English==` section has a multiword part of speech (Phrase, Proverb, Idiom, Verb, Noun, Adjective, Adverb, Interjection, Prepositional phrase) with an idiomatic label, an idiom or phrasal-verb category, the POS Phrase/Proverb/Idiom, or a verb + particle/object title. A single word counts only with a register label (slang, informal, formal, humorous, euphemistic, archaic …) or a regional variety label.

## Output (TSV, no header)

| Column | Content |
|---|---|
| 1 | phrase |
| 2 | kind: `idiom`, `phrasal-verb`, `verb-phrase`, `phrase`, `proverb`, `multiword`, `word` |
| 3 | part of speech |
| 4 | first gloss |
| 5 | register labels with the gloss of the labelled sense (`slang: …; humorous: …`) |
| 6 | regional varieties with the gloss of the labelled sense (`UK: …; Australia: …`) |

## Limits

Line-based parsing of export-0.11; glosses are crude wikitext-to-text (links kept as text, most templates dropped). The output is CC BY-SA (Wiktionary): attribution and share-alike apply, so it is for learning and tests, not for shipping; see [licence/DATA.md](../../licence/DATA.md).
