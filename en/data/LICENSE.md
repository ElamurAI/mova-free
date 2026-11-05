---
module: en-dictionary
licence: CC-BY-SA-4.0
requires: [attribution, share-alike, licence-notice, indicate-changes]
allows: [commercial-use, modification, redistribution]
sources: [UD_English-EWT 2.18 (CC-BY-SA-4.0), UD_English-ESLSpok 2.18 (CC-BY-SA-4.0), AGID 2016.01.19 (AGID notice), Open English WordNet 2025 (CC-BY-4.0), SPECIALIST Lexicon 2026 (NLM terms), 2010 U.S. Census surnames (public domain), Names Corpus 1.3 (credit, README), Mova (CC-BY-SA-4.0)]
---

# Licence and sources of `en/data`: the in-code English dictionary

The files of this folder are data, not code. `en/build.rs` compiles `lemmas.tsv`, `forms.tsv`, `consts.txt`,
`lexicon-ext.tsv` and `acronyms.tsv` into every binary that uses the `en` crate. `en licence` prints this file, and
the text is compiled into the `en` library as `en::dict::LICENCE`.

**Licence:** the in-code English dictionary (`lemmas.tsv`, `forms.tsv` and the lexicon extension `lexicon-ext.tsv`,
`acronyms.tsv`), and the other files of this folder, are distributed under the **Creative Commons
Attribution-ShareAlike 4.0 International** licence (CC BY-SA 4.0, <https://creativecommons.org/licenses/by-sa/4.0/>).
Share-alike follows from the forms, lemmas and counts taken from Universal Dependencies English EWT. Each source
below keeps its own notice, and its terms apply to the part taken from it. Mova's code is under the Elamur Cosmic
License ([../../LICENSE.md](../../LICENSE.md)); that licence does not cover this folder.

## The dictionary: `lemmas.tsv`, `forms.tsv`

Made by `en lexicon <AGID infl.txt> <train.conllu>...` (`en/src/lexgen.rs`), 2026-09-24, from the sources below.
2026-11-04: `en lexicon-redact` replaced 415 lemmas that were personal tokens of the EWT e-mail and web genres
(e-mail addresses, links, handles, telephone numbers) by `<redacted-N>` placeholders and dropped their 321 form rows.
Lemma numbers are append-only. `forms.tsv` columns: form, PTB tag, lemma number, UD count, source letters (`u` UD;
`a`, `r`, `b` AGID).

1. **UD English EWT**, Universal Dependencies 2.18, the training part (`en_ewt-ud-train.conllu`): the forms, tags,
   lemmas and counts of rows with source `u`, the count column, and the lemma lines that come only from UD.
   Natalia Silveira, Timothy Dozat, Marie-Catherine de Marneffe, Samuel Bowman, Miriam Connor, John Bauer and
   Christopher D. Manning. 2014. *A Gold Standard Dependency Corpus for English.* In Proceedings of the Ninth
   International Conference on Language Resources and Evaluation (LREC 2014). Annotators: Natalia Silveira,
   Timothy Dozat, Sebastian Schuster, Miriam Connor, Marie-Catherine de Marneffe, Nathan Schneider, Ethan Chi,
   Samuel Bowman, Hanzhi Zhu, Daniel Galbraith, John Bauer. Built over the source text of the English Web Treebank,
   LDC2012T13 (Ann Bies, Justin Mott, Colin Warner and Seth Kulick, 2012). Licence: CC BY-SA 4.0.
   <https://github.com/UniversalDependencies/UD_English-EWT>
2. **UD English ESLSpok**, Universal Dependencies 2.18, the training part (`en_eslspok-ud-train.conllu`), read by the
   same run. It annotates no lemmas (column 3 is `_`), and `en lexicon` skips a token without a lemma, so no row of
   the dictionary comes from it; it is cited because the generator reads it.
   Kristopher Kyle, Masaki Eguchi, Aaron Miller and Theodore Sither. 2022. *A Dependency Treebank of Spoken Second
   Language English.* In Proceedings of the 17th Workshop on Innovative Use of NLP for Building Educational
   Applications (BEA 2022), pages 39–45, Seattle. Association for Computational Linguistics. Source corpus: Emi
   Izumi, Kiyotaka Uchimoto and Hitoshi Isahara. 2004. *The NICT JLE Corpus.* Licence: CC BY-SA 4.0.
   <https://github.com/UniversalDependencies/UD_English-ESLSpok>
3. **Universal Dependencies** (the annotation scheme of both treebanks): Marie-Catherine de Marneffe, Christopher D.
   Manning, Joakim Nivre and Daniel Zeman. 2021. *Universal Dependencies.* Computational Linguistics 47(2):
   255–308. <https://universaldependencies.org>
4. **AGID**, the Automatically Generated Inflection Database, version 2016.01.19, by Kevin Atkinson
   (<http://wordlist.aspell.net>): rows with source `a` (the main form of a paradigm cell), `r` (a clean form for
   reverse lookup) and `b` (a stem), and the lemma lines that come only from AGID. Its notice, as its terms ask:

   > Copyright 2000-2014 by Kevin Atkinson
   >
   > Permission to use, copy, modify, distribute and sell this database, the associated scripts, the output created
   > form the scripts and its documentation for any purpose is hereby granted without fee, provided that the above
   > copyright notice appears in all copies and that both that copyright notice and this permission notice appear in
   > supporting documentation. Kevin Atkinson makes no representations about the suitability of this array for any
   > purpose. It is provided "as is" without express or implied warranty.

   AGID's part-of-speech data comes from Alan Beale's 2of12id and from WordNet 1.6, and its word list from ENABLE2K
   and its supplement, YAWL, the Moby Words package and the Jargon File word list (public domain) and the UK Advanced
   Cryptics Dictionary. Their notices, as AGID's README carries them:

   > WordNet 1.6 Copyright 1997 by Princeton University. All rights reserved.
   >
   > THIS SOFTWARE AND DATABASE IS PROVIDED "AS IS" AND PRINCETON UNIVERSITY MAKES NO REPRESENTATIONS OR WARRANTIES,
   > EXPRESS OR IMPLIED. BY WAY OF EXAMPLE, BUT NOT LIMITATION, PRINCETON UNIVERSITY MAKES NO REPRESENTATIONS OR
   > WARRANTIES OF MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF THE LICENSED SOFTWARE,
   > DATABASE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
   >
   > The name of Princeton University or Princeton may not be used in advertising or publicity pertaining to
   > distribution of the software and/or database. Title to copyright in this software, database and any associated
   > documentation shall at all times remain with Princeton University and LICENSEE agrees to preserve same.

   > Copyright (c) J Ross Beresford 1993-1999. All Rights Reserved.
   >
   > The following restriction is placed on the use of this publication: if The UK Advanced Cryptics Dictionary is
   > used in a software package or redistributed in any form, the copyright notice must be prominently displayed and
   > the text of this document must be included verbatim.
   >
   > There are no other restrictions: I would like to see the list distributed as widely as possible.

5. **Mova's own data:** the merging, numbering, tags and source letters, and `consts.txt` (words that get a constant
   in code).

## The lexicon extension: `lexicon-ext.tsv`, `acronyms.tsv`

Made by `en lexgrow` (`en/src/lexgrow.rs`). A blob of its own, outside the dictionary's fingerprint. Each file's
header names its sources; the Names Corpus README is in the header of `lexicon-ext.tsv`, as its terms ask.

6. **Open English WordNet 2025** (rows `o`, `o:<synset>`): John P. McCrae, Alexandre Rademaker, Francis Bond, Ewa
   Rudnicka and Christiane Fellbaum. 2019. *English WordNet 2019 — An Open-Source WordNet for English.* In
   Proceedings of the 10th Global WordNet Conference. Based on Princeton WordNet (George A. Miller. 1995. *WordNet: A
   Lexical Database for English.* Communications of the ACM 38(11): 39–41). Licence: CC BY 4.0.
   <https://github.com/globalwordnet/english-wordnet>
7. **The SPECIALIST Lexicon**, 2026 release (rows `s:<EUI>`), Lexical Systems Group, U.S. National Library of
   Medicine, under the SPECIALIST NLP Tools Terms and Conditions; no endorsement by HHS, NIH or NLM is implied.
   <https://lhncbc.nlm.nih.gov/LSG/Projects/lexicon/>
8. **2010 U.S. Census surnames** (rows `c`), U.S. Census Bureau: a U.S. Government work, public domain.
9. **Names Corpus**, version 1.3 (rows `k`), Mark Kantrowitz, additions by Bill Ross: free for any purpose with
   credit, redistributable with its README.

## The other files

| file | source |
|---|---|
| `train-licenses.tsv` | the registry of training sources and their licences, Mova's own |
| `ud-registry-en.tsv` | facts from the UD validator's data (UniversalDependencies/tools, `data/*.json`, UD 2.18): the allowed features, relations and auxiliaries of English |
| `constructions.tsv` | Mova's own construction sentences, after the method of Laura Rimell, Stephen Clark and Mark Steedman. 2009. *Unbounded Dependency Recovery for Parser Evaluation.* EMNLP 2009 |
| `constructions-floor-ewt.tsv`, `constructions-floor-en-bin.tsv` | recall floors measured by Mova on `constructions.tsv` |
