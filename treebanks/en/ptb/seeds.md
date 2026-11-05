# Seeds: treebanks/en/ptb

The seeds of this treebank, one section per seed. A section's anchor is the seed name: `seeds.md#errors`.

<a id="data"></a>

# PTB — Penn Treebank: the classic source of customs; we have no data

**Gist.** The Penn Treebank (Marcus et al. 1993) is the first large annotated corpus of American English. WSJ Treebank II has ≈ 1M words and ≈ 50k sentences; the annotation is semi-automatic: a machine annotates, a human corrects. Layers:
- POS following the Santorini 1990 guideline: 36 word tags and 12 punctuation tags;
- constituent trees following Bies et al. 1995: 14 phrasal labels, function tags (`-SBJ`, `-TMP`, `-PRD`…), empty elements (`*T*`, `*`, `*ICH*`…).

The data are paid (Treebank-3, LDC99T42). Only the guidelines are open: `$MOVA_DATA/raw/en-gram-ptb-guidelines/` (`tagguid1.pdf`, `prsguid1.pdf`). The NLTK sample (≈ 5% of WSJ) is for non-commercial use only. So for `mova` PTB is a source of customs, not a corpus.

**Conditions and exceptions.** PTB is the ancestor of almost all English UD:
- **EWT:** LDC annotated EWT (LDC2012T13) with PTB trees following the "new" guideline (Webtext addendum, Mott et al. 2012). They were then automatically converted to Stanford Dependencies (CoreNLP, de Marneffe et al. 2006), then manually to UD. EWT XPOS are the manual LDC tags.
- **ATIS:** annotators assigned PTB tags, and rules converted them to UPOS and FEATS; the tags were not included in the release (`../atis/seeds.md#pos-feats-from-ptb`).
- **ESLSpok:** XPOS follow the Santorini 1990 guideline, hence *to*/TO (`../eslspok/seeds.md#to-part`).
- **GUM, GENTLE, GUMReddit, PUD, LittlePrince, CHILDES** — XPOS in the PTB set.
- **`en`:** XPOS = PTB with EWT tags (`en/src/gram.rs`, `Tag`: 36 + 12 + `HYPH`, `NFP`, `ADD`, `AFX`, `GW`, `XX`).

**Examples.** PTB tokenizes *do n't*, *children 's*, *wo n't* (J93-2004, note 8). EWT splits the same way, but since 2.7 groups such words into MWT (*don't* → *do* + *n't*).

**In UD.** The PTB seeds are about tagging and tokenization (`tagset.md`) and about the constituents → dependencies path (`conversion.md`). PTB data are not converted into `mova`, because we do not have them. But "new PTB" is part of `mova`: its XPOS.

**Sources.** Marcus et al. 1993, `J93-2004` (papers/m/ma/marcus-1993-building-large-annotated-corpus-english, abstract only); Marcus et al. 1994, `H94-1020`; Santorini 1990 (`tagguid1.pdf`); Bies et al. 1995 (`prsguid1.pdf`); digest the UD survey digest «english-banks», §1.1, §1.3, §1.4; UD `docs/_en/tokenization.md`.

<a id="conversion"></a>

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

**In UD.** For `mova` this is the limit of the `convert` language: it rewrites labels and heads of a dependency tree, and does not read constituents (`docs/dialect.md`, "Not only UD"). Open PTB-style trees — the NLTK sample, MASC, OntoNotes, if it becomes available — need a separate reader and a "constituents → dependencies" transformer. Expected quality is ≈ 10% errors. So such data are fit as silver or as an XPOS test, not as gold.

**Sources.** de Marneffe et al. 2006, `L06-1260` (abstract only); de Marneffe & Manning 2008, `W08-1301`; Silveira et al. 2014, `L14-1067`; Peng & Zeldes 2018, `W18-4918` (papers/p/pe/peng-2018-all-roads-lead-ud-converting); Zeldes & Schneider 2023, `2023.udw-1.7`; Bies et al. 1995 (`prsguid1.pdf`); digest the UD survey digest «english-banks», §1.1, §1.3, §1.5.

<a id="tagset"></a>

# PTB — classic tagging versus the "new" PTB that EWT and `mova` follow

**Gist.** `mova` XPOS is not PTB 1990 but the "new" PTB from LDC (OntoNotes, BioMedical addendum 2004, Webtext addendum 2012). Differences:
1. ***to*.** Santorini 1990: the TO tag for every *to* — both particle and preposition. New PTB: prepositional *to* is IN. In EWT *to*: TO 3993, IN 2211.
2. **Hyphen.** WSJ keeps *long-term* as one token (JJ). New PTB splits it: *long* - *term*, the hyphen is `HYPH`, a prefix before a hyphen is `AFX`. In EWT `HYPH` 852, `AFX` 75. GUM switched to this in 2021.
3. **Webtext addendum web tags:**
   - `NFP` — emoticons, decorative punctuation (EWT 499);
   - `ADD` — addresses (475);
   - `GW` — part of a split word (345);
   - `XX` — unrecognized.

   PTB 1990 does not have them.
4. **Brackets.** WSJ has `-LRB-`/`-RRB-`, `-LCB-`/`-RCB-` and square `-LSB-`/`-RSB-`. EWT tags all brackets `-LRB-`/`-RRB-` and does not escape the form itself (*(*, not *-LRB-*). GUM merged square brackets with round ones in 2.8.
5. **Constituents not in the tag.** PTB has no separate tags for *be/do/have* (only VB*) and does not distinguish a preposition from a subordinating conjunction (IN). UD distinguishes them by UPOS according to role: AUX/VERB, ADP/SCONJ. In PTB this information is in the constituent tree.

**Conditions and exceptions.** The same in both:
- *n't* — RB;
- possessive *'s* — POS;
- *'s* = *is* — VBZ;
- quotes ``` `` ``` and `''`;
- relative *that* — WDT, conjunction *that* — IN.

It is UD treebanks that diverge, not PTB from EWT:
- CHILDES writes `?` and `!` instead of `.` (`../childes/seeds.md#punct`);
- ESLSpok — `"` instead of ``` `` ```/`''` (`../eslspok/seeds.md#data`).

**Examples.** *go to school*: PTB 1990 — *to*/TO; EWT — *to*/IN (ADP). *data-driven*: WSJ — one JJ token; EWT/GUM — *data*/NN *-*/HYPH *driven*/VBN.

**In UD.** ESLSpok (Santorini 1990 + TLE) is the only English UD treebank that keeps classic *to*/TO: 224 of 224 prepositional *to* have PART/TO. The rest are new PTB. `en` learns on EWT, so classic tags are a dialect for it that needs converting (`convert.md`).

**Sources.** Santorini 1990 (`tagguid1.pdf`), TO, IN, RB, POS; digest the UD survey digest «english-banks», §1.1 (BioMedical addendum: HYPH, AFX, NML), §1.4 (Webtext addendum: GW, NFP, ADD, AFX; `@`, `outta`, `dunno`), table §2 ("Contractions and clitics"); UD `docs/_en/tokenization.md`, `docs/_en/pos/X.md`, `pos/SYM.md`; Zeldes & Schneider 2023, `2023.udw-1.7`, sec. 4 (GUM v2.8–2.9: HYPH, `-LSB-`).
