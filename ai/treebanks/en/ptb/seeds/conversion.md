# PTB — the constituents → Stanford Dependencies → UD path and what gets lost on it

**Gist.** The classic path from a PTB tree to dependencies is head rules plus CoreNLP tregex patterns (de Marneffe et al. 2006). It yields Stanford Dependencies, and then UD. The EWT base and the whole "Stanford" branch of English UD were made this way. What gets lost on the way is what PTB has and a dependency tree does not:
- empty categories and coindexation (`*T*`, `*`, `*ICH*`, `*RNR*`) — long-distance dependencies of wh-movement and control;
- function tags (`-TMP`, `-LOC`, `-CLR`, `-PRD`…). Some are recovered by the label (`-TMP` → the former `:tmod`), the rest disappear;
- constituent boundaries that do not exist in dependencies (NP inside NP, NML, UCP).

**Conditions and exceptions.**
- Quality of the automatic path (Peng & Zeldes 2018, W18-4918, on GUM), head / label error rate:
  - gold constituent trees → UD via CoreNLP — 10.6% / 10.4%;
  - SD → UD by rules — 1.73% / 1.38%;
  - the same with entity and coreference layers — 0.45% / 0.42%.
- The main causes of errors on the first path: no empty categories and function tags, fronted NPs not recognized, `obl`/`nmod` confusion, coordination via commas.
- EWT went through this path with manual correction, so its dependencies are clean. But traces of the conversion remain: a right head of `compound` in names (*Sri Lanka*), `compound` where GUM gives `amod` or `flat` (Zeldes & Schneider 2023, sec. 5).

**Examples.**
- *(NP-TMP last week)* → `obl:tmod` (UD ≤ 2.14) → `obl:unmarked` + `TemporalNPAdjunct=Yes` in EWT 2.15+. The trace of the `-TMP` function tag lives in MISC.
- *What did you see \*T\*?* → *What* — `obj` of *see*. The `*T*` trace disappears.

**In UD.** For `mova` this is the limit of the `convert` language: it rewrites labels and heads of a dependency tree, and does not read constituents (`train/dialects-and-converters.md`, "Not only UD"). Open PTB-style trees — the NLTK sample, MASC, OntoNotes, if it becomes available — need a separate reader and a "constituents → dependencies" transformer. Expected quality is ≈ 10% errors. So such data are fit as silver or as an XPOS test, not as gold.

**Sources.** de Marneffe et al. 2006, `L06-1260` (abstract only); de Marneffe & Manning 2008, `W08-1301`; Silveira et al. 2014, `L14-1067`; Peng & Zeldes 2018, `W18-4918` (papers/p/pe/peng-2018-all-roads-lead-ud-converting); Zeldes & Schneider 2023, `2023.udw-1.7`; Bies et al. 1995 (`prsguid1.pdf`); digest, §1.1, §1.3, §1.5.
