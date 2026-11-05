---
module: onto-data
licence: CC-BY-SA-4.0
requires: [attribution, share-alike, licence-notice, indicate-changes]
allows: [commercial-use, modification, redistribution]
sources: [Mova (CC-BY-SA-4.0), Open English WordNet 2025 (CC-BY-4.0), Wikidata (CC0-1.0), Harmonized System 2022 codes (facts)]
notes: [sense confirmations in hs/sense-confirm.tsv by Anthropic Claude Opus]
---

# Licence and sources of `onto/data`

Data, not code. **Licence:** Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0,
<https://creativecommons.org/licenses/by-sa/4.0/>). Mova's code is under the Elamur Cosmic License
([../../LICENSE.md](../../LICENSE.md)); that licence does not cover this folder.

| files | what | sources |
|---|---|---|
| `core.tsv`, `core-props.tsv` | the core of the world model | Mova's own, keyed by Open English WordNet 2025 lemmas and synsets (McCrae et al. 2019, CC BY 4.0) |
| `align-gold.tsv` | gold alignment WordNet synset ↔ Wikidata item, checked by hand | Open English WordNet (CC BY 4.0), Wikidata (CC0 1.0) |
| `frozen.tsv`, `wsd-frozen.tsv`, `fable-questions.tsv` and their `.sha256` | frozen check sets, written before the code | Mova's own sentences and questions |
| `hs/**` | the Harmonized System 2022 (HS22) code tree mapped to the world model | the codes are facts (World Customs Organization); labels and mappings Mova's own; other-language labels from Wikidata (CC0 1.0); sense confirmations by Anthropic's Claude Opus |
