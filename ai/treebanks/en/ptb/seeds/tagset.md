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
- CHILDES writes `?` and `!` instead of `.` (`../../childes/seeds/punct.md`);
- ESLSpok — `"` instead of ``` `` ```/`''` (`../../eslspok/seeds/data.md`).

**Examples.** *go to school*: PTB 1990 — *to*/TO; EWT — *to*/IN (ADP). *data-driven*: WSJ — one JJ token; EWT/GUM — *data*/NN *-*/HYPH *driven*/VBN.

**In UD.** ESLSpok (Santorini 1990 + TLE) is the only English UD treebank that keeps classic *to*/TO: 224 of 224 prepositional *to* have PART/TO. The rest are new PTB. `en` learns on EWT, so classic tags are a dialect for it that needs converting (`../convert.md`).

**Sources.** Santorini 1990 (`tagguid1.pdf`), TO, IN, RB, POS; digest, §1.1 (BioMedical addendum: HYPH, AFX, NML), §1.4 (Webtext addendum: GW, NFP, ADD, AFX; `@`, `outta`, `dunno`), table §2 ("Contractions and clitics"); https://universaldependencies.org/en/tokenization.html, https://universaldependencies.org/en/pos/X.html, `pos/SYM.md`; Zeldes & Schneider 2023, `2023.udw-1.7`, sec. 4 (GUM v2.8–2.9: HYPH, `-LSB-`).
