# PTB — Penn Treebank: the classic source of customs; we have no data

**Gist.** The Penn Treebank (Marcus et al. 1993) is the first large annotated corpus of American English. WSJ Treebank II has ≈ 1M words and ≈ 50k sentences; the annotation is semi-automatic: a machine annotates, a human corrects. Layers:
- POS following the Santorini 1990 guideline: 36 word tags and 12 punctuation tags;
- constituent trees following Bies et al. 1995: 14 phrasal labels, function tags (`-SBJ`, `-TMP`, `-PRD`…), empty elements (`*T*`, `*`, `*ICH*`…).

The data are paid (Treebank-3, LDC99T42). Only the guidelines are open: `data/raw/en-gram-ptb-guidelines/` (`tagguid1.pdf`, `prsguid1.pdf`). The NLTK sample (≈ 5% of WSJ) is for non-commercial use only. So for `mova` PTB is a source of customs, not a corpus.

**Conditions and exceptions.** PTB is the ancestor of almost all English UD:
- **EWT:** LDC annotated EWT (LDC2012T13) with PTB trees following the "new" guideline (Webtext addendum, Mott et al. 2012). They were then automatically converted to Stanford Dependencies (CoreNLP, de Marneffe et al. 2006), then manually to UD. EWT XPOS are the manual LDC tags.
- **ATIS:** annotators assigned PTB tags, and rules converted them to UPOS and FEATS; the tags were not included in the release (`../../atis/seeds/pos-feats-from-ptb.md`).
- **ESLSpok:** XPOS follow the Santorini 1990 guideline, hence *to*/TO (`../../eslspok/seeds/to-part.md`).
- **GUM, GENTLE, GUMReddit, PUD, LittlePrince, CHILDES** — XPOS in the PTB set.
- **`en`:** XPOS = PTB with EWT tags (`en/src/gram.rs`, `Tag`: 36 + 12 + `HYPH`, `NFP`, `ADD`, `AFX`, `GW`, `XX`).

**Examples.** PTB tokenizes *do n't*, *children 's*, *wo n't* (J93-2004, note 8). EWT splits the same way, but since 2.7 groups such words into MWT (*don't* → *do* + *n't*).

**In UD.** The PTB seeds are about tagging and tokenization (`tagset.md`) and about the constituents → dependencies path (`conversion.md`). PTB data are not converted into `mova`, because we do not have them. But "new PTB" is part of `mova`: its XPOS.

**Sources.** Marcus et al. 1993, `J93-2004` (papers/m/ma/marcus-1993-building-large-annotated-corpus-english, abstract only); Marcus et al. 1994, `H94-1020`; Santorini 1990 (`tagguid1.pdf`); Bies et al. 1995 (`prsguid1.pdf`); digest, §1.1, §1.3, §1.4; https://universaldependencies.org/en/tokenization.html.
