---
module: prag-data
licence: CC-BY-SA-4.0
requires: [attribution, share-alike, licence-notice, indicate-changes]
allows: [commercial-use, modification, redistribution]
sources: [Mova (CC-BY-SA-4.0), VerbNet 3.4 (VerbNet 3.0 License), Princeton WordNet 3.1 (WordNet 3.0 License)]
---

# Licence and sources of `prag/data`

Data, not code. **Licence:** Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0,
<https://creativecommons.org/licenses/by-sa/4.0/>) for the folder as distributed; the VerbNet and WordNet parts keep
their notices below. Mova's code is under the Elamur Cosmic License ([../../LICENSE.md](../../LICENSE.md)); that
licence does not cover this folder.

| files | what | sources |
|---|---|---|
| `verbnet.tsv` | VerbNet classes, members, roles and frames, converted by `prag lexicon convert` | VerbNet 3.4 (<https://github.com/cu-clear/verbnet>, commit `ae8e9cfd`); Karin Kipper Schuler. 2005. *VerbNet: A Broad-Coverage, Comprehensive Verb Lexicon.* PhD thesis, University of Pennsylvania. VerbNet 3.0 License: free to use, copy, modify and distribute, commercial use included, with the copyright kept: Copyright 2009 by the Regents of the University of Colorado / Martha Palmer |
| `nominals.tsv` | noun–verb derivational links, converted by `prag lexicon nominals` | Princeton WordNet 3.1 (George A. Miller. 1995. *WordNet: A Lexical Database for English.* CACM 38(11)). WordNet 3.0 License: permission to use, copy, modify and distribute for any purpose and without fee, with the notice kept: WordNet 3.1 Copyright 2011 by Princeton University. All rights reserved |
| `vnfreq.tsv` | the lessons' preference among a verb's VerbNet classes (`world frames vnfreq`) | counts over Mova's lessons |
| `cues.tsv`, `signatures.tsv`, `rules.tsv` | pragmatic cues (after Lakoff 1973 and the classics named in the header), implication signatures of verbs, frame rules | Mova's own |
