# The ERRANT error taxonomy and what of it is visible in UD

**Gist.** ERRANT classifies every text edit deterministically, without training: by part-of-speech tags, lemmas and a dictionary on both sides of the edit. It has 25 main types:

ADJ, ADJ:FORM, ADV, CONJ, CONTR, DET, MORPH, NOUN, NOUN:INFL, NOUN:NUM, NOUN:POSS, ORTH, OTHER, PART, PREP, PRON, PUNCT, SPELL, UNK, VERB, VERB:FORM, VERB:INFL, VERB:SVA, VERB:TENSE, WO.

Each type combines with one of three operations: M (missing), R (replaced), U (unnecessary). For verbs the rules are ordered:
1. both sides are equal in lower case → ORTH;
2. the original is not a word → SPELL;
3. the same lemma → go on to the form;
4. one side is VBG or VBN → FORM;
5. one side is VBD → TENSE;
6. one side is VBZ → SVA.

Independent experts rated 95 % of the types "good" or "acceptable". When a type turned out bad, the cause was mostly a wrong part of speech or parse (*ring → rings*: NOUN:NUM or VERB:SVA).

**Frequencies.**

| corpus | most frequent edit types |
|---|---|
| W&I+LOCNESS (BEA-2019, train) | PUNCT 17 %, OTHER 13 %, DET 11 %, PREP 10 %, VERB:TENSE 6 %, VERB 6 %, ORTH 5 %, NOUN 4 %, NOUN:NUM 4 %, SPELL 4 %, VERB:FORM 3.6 %, PRON 2.6 %, VERB:SVA 2.2 % |
| FCE | OTHER 13 %, PREP 11 %, DET 11 %, PUNCT 10 %, SPELL 10 % |
| NUCLE | OTHER 26 %, DET 16 %, NOUN:NUM 8 % |

**For the expert system.** Types that leave a trace in the UD tree are separate seeds of this section:
- VERB:SVA — agreement;
- VERB:FORM — the auxiliary chain;
- DET and NOUN:NUM — determiner and number;
- ADJ:FORM — double comparison;
- VERB:INFL — *goed*;
- PREP — verb prepositions;
- WO — adverb between verb and object;
- PRON — pronoun case.

Error correction systems do worst on content words (BEA-2019) and syntactic errors. SERCL syntactically classifies about 60 % of what ERRANT assigns to OTHER.

**Examples** (verb rules from the paper):
- [iss → is] SPELL;
- [IS → is] ORTH;
- [has → is] VERB;
- [being → is] VERB:FORM;
- [was → is] VERB:TENSE;
- [are → is] VERB:SVA.

**In UD.** An error in the text is not corrected in UD: it stays in the tree (see `literal-annotation-learner.md`). So the agreement and form rules of this section are at the same time detectors of grammatical errors.

**Sources.** `P17-1074` (Bryant et al. 2017: table 2, §3.1–3.5); `W19-4406` (Bryant et al. 2019: table 4, conclusions); `2020.conll-1.7` (Choshen et al.: SERCL); `2025.acl-long.1026` (Koyama et al.: CTSEG — minimal pairs by CEFR, the "verb tense" type split into 15 subtypes). Raw data locally: `data/raw/en-gram-errant`, `en-gram-wi-locness`, `en-gram-fce`.
