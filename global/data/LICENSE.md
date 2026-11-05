---
module: global-data
licence: CC-BY-SA-4.0
requires: [attribution, share-alike, licence-notice, indicate-changes]
allows: [commercial-use, modification, redistribution]
sources: [Mova (CC-BY-SA-4.0), ATOMIC-2020 (CC-BY-4.0), Wikidata (CC0-1.0), Project Gutenberg books (public domain)]
notes: [absurdity ratings written with Anthropic Claude models; Anthropic's terms restrict using model outputs to develop competing models]
---

# Licence and sources of `global/data`

Data, not code. **Licence:** Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0,
<https://creativecommons.org/licenses/by-sa/4.0/>). Attribution: "Mova, the Elamur team". Mova's code is under the
Elamur Cosmic License ([../../LICENSE.md](../../LICENSE.md)); that licence does not cover this folder.

| files | what | sources |
|---|---|---|
| `absurdity-roles.tsv`, `absurdity-roles-tale.tsv` | the absurdity matrix: plausibility scores | rated for Mova by Anthropic's Claude Opus (2026-10-02); their terms restrict using model outputs to develop competing models |
| `absurdity-roles-known.tsv` | a layer Mova derived from its own knowledge (`world absurd-learn`) | Mova's own |
| `atomic-feelings.sql`, `atomic-feelings-o.sql`, `atomic-uses.sql` | queries over ATOMIC-2020 (the data stays in `$MOVA_DATA`) | Jena D. Hwang, Chandra Bhagavatula, Ronan Le Bras, Jeff Da, Keisuke Sakaguchi, Antoine Bosselut and Yejin Choi. 2021. *(Comet-)Atomic 2020: On Symbolic and Neural Commonsense Knowledge Graphs.* AAAI 2021. CC BY 4.0 |
| `wikidata-core.sql` | a query over Wikidata (the data stays in `$MOVA_DATA`) | Wikidata, <https://www.wikidata.org/>, CC0 1.0 |
| `register-children.tsv`, `register-legal.tsv`, `register-tale.tsv` | register lexicons: log-odds counts (Burt L. Monroe, Michael P. Colaresi and Kevin M. Quinn. 2008. *Fightin' Words.* Political Analysis 16(4)) | counts over public-domain Project Gutenberg books |
