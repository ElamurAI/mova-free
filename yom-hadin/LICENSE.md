---
module: yom-hadin-data
licence: CC-BY-SA-4.0
requires: [attribution, share-alike, licence-notice, indicate-changes]
allows: [commercial-use, modification, redistribution]
sources: [Mova (CC-BY-SA-4.0), The Holy Scriptures JPS 1917 (public domain), Guggenheimer The Jerusalem Talmud via Sefaria (CC-BY), L. W. King Code of Hammurabi 1910 (public domain), Ukrainian official legal texts (not subject to copyright), U.S. Code and Code of Federal Regulations (U.S. Government works, public domain)]
---

# Licence and sources of the `yom-hadin` data

Data, not code: `jurisdictions/**` and `data/**`. **Licence:** Creative Commons Attribution-ShareAlike 4.0
International (CC BY-SA 4.0, <https://creativecommons.org/licenses/by-sa/4.0/>), with the sources below. The crate's
code (`src/`, `tests/`) is under the Elamur Cosmic License ([../LICENSE.md](../LICENSE.md)); that licence does not
cover this data.

| files | what | sources |
|---|---|---|
| `data/**`, `jurisdictions/_common/**`, every `jurisdiction.tsv` and `authorities.tsv` | cases, gold verdicts, scales, terms and jurisdiction metadata | written for Mova |
| `jurisdictions/torah/**` | rules read from the Torah | *The Holy Scriptures*, Jewish Publication Society, 1917; public domain |
| `jurisdictions/yerushalmi/**` | rules read from the Jerusalem Talmud; the `text` column of `rules.tsv` quotes the translation, `text.tsv` holds our paraphrased base laws for the legal devices (`devices.tsv`) | Heinrich W. Guggenheimer, *The Jerusalem Talmud*, translation and commentary (Berlin, De Gruyter, 1999–2015), via the Sefaria export, which states the licence CC BY (version source: National Library of Israel, NNL_ALEPH001901012). Changes: segmented into sentences and rules, with Mova's parses beside the quoted text |
| `jurisdictions/hammurabi/**` | Mova's paraphrase of the Code of Hammurabi | after L. W. King's translation (1910), public domain |
| `jurisdictions/ukraine/**` | Mova's English of Ukrainian legal provisions | official texts, which Ukrainian law leaves outside copyright |
| `jurisdictions/us-federal/**` | plain sentences after U.S. federal law | the U.S. Code and the Code of Federal Regulations, works of the U.S. Government |

No non-commercial, research-only or no-derivatives text is included.
