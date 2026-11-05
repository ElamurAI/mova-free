# 13. Concrete improvements for Mova's parser and lexicon

Each item says what to change, where, how many sentences it touches (with the date and the set it was counted
on), and its negative control: the case that must stay unchanged, so that the change can show red. Counts:
**SLM now** = `en-agreed.bin` over 6 013 Torah sentences aligned with the db; **db** = 6 065 JPS sentences of
`parsedb/current.conllu`; all 2026-10-17. Items are ordered by expected gain per line of code.

The items are written so that each can be a separate commit with its own measurement (two runs identical, the
Torah and tales cycles as the check, the held-out sets untouched). The «Status» section says which are in the code.

## Status (2026-10-18)

Done: T1, T2, T3, T4, L1, L2, L3, L5, L6, F1, S1; F2 and F3 for the words the tables cover. The rest are proposals.

| # | Where | What the code does now |
|---|---|---|
| L1–L3 | `en/src/morph.rs` (`archaic`, `ending_verb`, `known_form`) | spellings of the stem ranked (CVC stem → «e» spelling, else the most frequent verb in UD); AGID-only superlatives are not dictionary forms; «-est»/«-st» after a past form → that verb, VBD |
| T1, T2, T4 | `en/src/emode.rs` (`CLOSED`, `force`, `fix`) | 78 entries (pronouns, modals, have/do/be forms, saith, lest, unto, behold, lo, aught, nought, whosoever, yea, nay, 35 adverbs, LORD/GOD in capitals, a second reading of «thine» as PRP) with context conditions; the tag is forced in the tagger's beam (`PTagger::tag_forced`), lemma/UPOS/FEATS set after the parse, FEATS filtered by the UD registry |
| T3 | `en/src/emode.rs` (`ending`) | an unknown lower-case word in «-eth»/«-est»/«-st» over a known verb is forced VBZ/VBP/VBD; lemma the verb; FEATS person, number, tense, `Style=Arch` |
| S1 | `en::emode::postverbal_not`, called in `en::annotate` and `gparse annotate` | «not» after a lexical verb (or its postposed pronoun) → the verb's `advmod`; not after let/be/have/do, not before VBG/VBN/TO/JJ/«only» |
| L5 | `gparse/src/data.rs` (`Lex::fill`) | a retagged word unknown to `GP_LEX` is lemmatized for its new tag; a verb keeps an informative old lemma when the new one only repeats the form |
| F1 | `gparse/src/data.rs` (`Lex::table`), `gparse annotate` | every word the EModE table knows with its tags (retagged or not) takes lemma and FEATS from the table |
| L6 | `annot/src/repair.rs`; `annot db-repair <in> <out>`; `drafts_from` | rules `table`, `ending`, `verb-lemma`, `lower-lemma`, `not` over CoNLL-U lines; idempotent; the drafts get it before the gates |

Annotation path: `annot`'s Rust draft goes through `en::annotate::annotate` (one path with `en annotate`).
Measurement tools: `en compare <ref> <pred>` (UD scores and agreement by class) and `annot gate-check <drafts>`.

Measured 2026-10-18, `en-agreed.bin`, graph parser `live-0707` with `GP_LEX` = school-agreed, against the 6 065 JPS
sentences of `corpora/judaism/parsedb/current.conllu` (a copy taken 2026-10-17 23:47); every run twice, identical:

| | main before (2026-10-17) | now |
|---|---:|---:|
| SLM draft: UPOS / XPOS / UFeats | 94.25 / 92.20 / 92.36 | 97.00 / 95.37 / 95.54 |
| SLM draft: lemma / UAS / LAS / MLAS | 98.22 / 84.10 / 78.20 / 62.18 | 98.53 / 85.83 / 81.03 / 68.29 |
| SLM draft: closed EModE words UPOS / UFeats | 64.57 / 49.93 | 96.69 / 95.03 |
| SLM draft: -eth verbs UPOS; -est/-st verbs UPOS | 9.30; 5.48 | 100.00; 97.26 |
| SLM draft: sentences without a UPOS disagreement | 1 873 | 2 834 |
| SLM draft: refused by the gates (of 6 105) | 29 | 5 |
| graph parser: UFeats / MLAS | 95.88 / 83.55 | 96.90 / 85.41 |
| graph parser: verbs with lemma = form (-eth) | 14 626 (546) | 13 994 (3) |
| graph parser: archaic pronouns with Person=3 | 1 981 (old drafts) | 0 |
| graph parser vs the db repaired by L6: lemma / Feats | 98.51 / 94.24 | 99.53 / 97.06 |
| UD English EWT dev LAS (en-agreed.bin; en.bin) | 80.49; 80.29 | 80.50; 80.30 |
| UD English EWT test LAS (en-agreed.bin; en.bin) | 81.25; 81.04 | 81.28; 81.09 |

Lemma agreement with the db itself falls (SLM -eth 67.94 → 38.87, graph parser 99.18 → 98.78) because the db keeps
D1, D3 and D6: `giveth` → giveth, `thee` → thee, `Lord` for LORD. `annot db-repair` on a copy of the judaism db
(7 982 sentences) changes 3 665 words by `table`, 668 by `ending`, 51 by `not`, 19 by `lower-lemma`, 3 by
`verb-lemma`; on its JPS sentences D1 375 → 0, D2 21 → 0, D3 243 → 2, D4 141 → 0, archaic forms without
`Style=Arch` 2 817 → 27 tokens, D6 611 → 36, D9 72 → 7; the gates refuse nothing before or after. The shared db is
not rewritten: that is the operator's step.

Not done, with the reason: a rule letting a dictionary adjective under a verb tag take the ending («thou lightest»)
turned *honest* (VB) into *hone*; `never` and modern `not` in S1 cost 0.01 LAS on EWT test; both left out.

## Summary table

| # | Change | Where | Sentences affected | Kind |
|---|---|---|---:|---|
| T1 | closed-class EModE table before the tagger: pronouns, auxiliaries, `lest`, `unto`, `behold`, `aught` … | `en` (tagger input or a forced-tag pass) | **1 637** (SLM now) | tag |
| T2 | the same for 27 pro-adverbs (`thereof`, `wherewith`, `whither` …) | same; and `archaic_adverbs` selection | **479** (SLM now) | tag |
| T3 | `-eth` / 2nd person `-est` guesser: unknown form = known verb stem + ending → verb tag | `en` tagger features or a pass after it | **573** (SLM now) | tag |
| — | T1–T3 together | | **2 242** of 6 013 (37 %); 704 would have no UPOS error left | |
| T4 | `LORD`, `GOD` in capitals → PROPN, lemma `LORD` | tagger pass | 705 (SLM now); 611 db lemmas | tag, lemma |
| T5 | capitalised divine pronouns (`Me`, `Thy`, `Thou`, `He` mid-sentence) → PRON | tagger pass | 110 (SLM now) | tag |
| L1 | `archaic()`: rank candidates by «is a verb lemma» then UD frequency | `en/src/morph.rs` | 55 tokens wrong today (`com`, `se`, `hat`, `li`, `di`, `putt`, `marri` …) | lemma |
| L2 | ignore AGID-only `JJS` analyses for verb tags; drop AGID superlatives of non-adjectives | `en/src/morph.rs` or `en/build.rs` | `goest` 27, `doest` 8, `seest` 7, `shouldest` 8, `liest` 3, … ≈ 60 tokens; plus the SLM's `JJS` tagging of 126 sentences | lemma, tag |
| L3 | 2nd person **past**: `-est`/`-st` after a past form → that form's lemma | `en/src/morph.rs` | `saidst` 7, `broughtest` 3, `stoodest` 2, `knewest`, `drankest`, `sentest`, `defiledst`, `stretchedst`, `badest`, `sawest`, `smotest` ≈ 20 tokens | lemma |
| L4 | strong/old forms table (≈ 50 entries) and old plurals | `en/data/archaic.tsv` via `build.rs` | db: 67 tokens / 62 sentences (D7) + `kine`/`brethren` 137 tokens | lemma |
| L5 | `gparse Lex::fill`: when the tag changes, re-lemmatize with `en::morph` instead of keeping the old lemma | `gparse/src/data.rs` | db: 400 tokens / 328 sentences (D1, D2) | lemma |
| L6 | `annot drafts_from`: extend the AUX lemma repair to VERB (any verb tag) and to `thee` → `thou` | `annot/src/main.rs` | 328 + 202 sentences | lemma |
| F1 | FEATS of the archaic pronouns from a table, never from the tag fallback | `gparse/src/data.rs` (`Lex::fill`) and the table of T1 | db: 98 sentences with `Person=3` on `thou`/`thy`/`thine` (D4) | feats |
| F2 | `Style=Arch` on every archaic form, by the table | T1 table, L4 table | db: 1 681 sentences lack it somewhere (D5) | feats |
| F3 | verb person/number from the ending (`-est` → 2 Sing, `-eth` → 3 Sing) | post-tag pass | db: 3 stray `Plur|Person=3` on `-est` verbs; future drafts | feats |
| S1 | re-attach postverbal `not` to its verb | post-parse rule | db: 72 tokens, about 70 sentences | tree |
| S2 | «it came to pass»: one analysis (`it` expl, `pass` xcomp, event `csubj`/`conj`) | post-parse rule | db: 106 | tree |
| S3 | subjunctive: VB with a subject and a conditional `mark` → `Mood=Sub|Tense=Pres|VerbForm=Fin` | post-parse rule | db: ≥ 53 tokens (about 50 sentences) + 191 candidate patterns | feats |
| S4 | `for ever`, `to-day`, `to-morrow`, `to-night`: one analysis each | post-parse rule | db: 67 + 14 + 9 + 1 | tree |
| S5 | NP + comma + resumptive pronoun → `dislocated` | post-parse rule | a few dozen («And Abel, he also…», «the LORD, He is God») | tree |
| S6 | `ye` + noun: `ye` head, noun `nmod:unmarked` (EWT) | post-parse rule | db: 4 | tree |
| S7 | lower-case lemma for non-PROPN (`Behold`, `Whosoever`, `Thy`, `Howbeit`, `Amen`, `Nay`) | lemma pass | db: 91 tokens / 87 sentences (D9) | lemma |
| P1 | negators: add `nought`, `naught`, `nay`; `lest` as negative purpose | `prag/src/feats.rs` | 66 `lest` + 8 `nought` + 9 `nay` sentences at most | pragmatics |
| P2 | modal feature from the lemma, or map `shalt`→shall, `wilt`→will, `mayest`→may, `canst`→can, `shouldest`→should … in `norm_word` | `prag/src/feats.rs` | 750 Torah sentences with an archaic modal | pragmatics |
| P3 | subject person: fall back to the pronoun's form (`thou`, `thee`, `ye` → 2) when FEATS disagree | `prag/src/feats.rs` | 98 (D4) | pragmatics |
| P4 | add `thyself` to the second-person list | `prag/src/feats.rs` | 32 `thyself` tokens | pragmatics |
| W1 | extend `archaic_adverbs` with the 19 missing pro-adverbs | `global/seeds/shortcuts/selection.md` | the reranker's lexical repair on 473 sentences (SLM now) | world |
| W2 | register senses for false friends (`even`, `corn`, `suffer`, `quick`, `sore`, `seed`, `stranger`, `know` …) | world dictionary, «formal»/archaic domain (`world/src/domains.rs`) | `at even` 11, `corn` 32, `seed` 93, `stranger` 53 … | world |
| W3 | biblical formulas into the expression base (`found grace in the eyes of`, `lifted up his eyes`, `be cut off from`, `surely die`, `knew his wife`) | `docs/pragmatics.md` base | 7 + 9 + 21 + 17 + 2 … | world |
| W4 | partitive `of` after *eat, take, drink, give, offer, bring* | frame reader | `eat of` 34, `take of` 26, … ≈ 80 | world |
| K1 | tokenizer: `'tis`, `'twas`, `'twould`, `may'st`, `could'st`, `ne'er`, `e'er`, `o'er`, `e'en`, `th'` | `en` tokenizer | tales: 112 lines with `'tis`-type, 13 with `-'st`, 44 with `ne'er`-type | token |
| K2 | `an`/`and` + nominative pronoun + finite verb = SCONJ *if* | post-tag rule | tales (`book:pg29672:104` …) | tag |
| R1 | retrain the SLM with the repaired Torah db | `en train` | all | model |
| R2 | add the repaired Torah db to `GP_LEX` (the graph parser's lexicon) | cycle config `CYCLE_LEX` | all retagged archaic tokens | lexicon |

## Details

### T1. A closed-class EModE table before the tagger

**What.** A table of forms whose analysis in the biblical register does not depend on context:

| Form | UPOS | XPOS | Lemma | FEATS |
|---|---|---|---|---|
| thou | PRON | PRP | thou | `Case=Nom|Number=Sing|Person=2|PronType=Prs|Style=Arch` |
| thee | PRON | PRP | thou | `Case=Acc|Number=Sing|Person=2|PronType=Prs|Style=Arch` |
| thy | PRON | PRP$ | thy | `Case=Gen|Number=Sing|Person=2|Poss=Yes|PronType=Prs|Style=Arch` |
| thine | PRON | PRP$ | thy | same |
| thyself | PRON | PRP | thyself | `Case=Acc|Number=Sing|Person=2|PronType=Prs|Reflex=Yes|Style=Arch` |
| ye | PRON | PRP | ye | `Case=Nom|Number=Plur|Person=2|PronType=Prs|Style=Arch` |
| shalt, wilt, canst, mayest, mayst, mightest, shouldest, shouldst, wouldest, wouldst, couldest, couldst | AUX | MD | shall, will, can, may, may, might, should, should, would, would, could, could | `Number=Sing|Person=2|VerbForm=Fin|Style=Arch` |
| hast | AUX | VBP | have | `Mood=Ind|Number=Sing|Person=2|Tense=Pres|VerbForm=Fin|Style=Arch` |
| hath | AUX | VBZ | have | `Mood=Ind|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin|Style=Arch` |
| hadst | AUX | VBD | have | `Mood=Ind|Number=Sing|Person=2|Tense=Past|VerbForm=Fin|Style=Arch` |
| dost, doth, didst | AUX | VBP, VBZ, VBD | do | person, tense, `Style=Arch` |
| wast, wert | AUX | VBD | be | `…|Person=2|Tense=Past…|Style=Arch` (`wert` may be `Mood=Sub`) |
| saith, sayest | VERB | VBZ, VBP | say | person, `Style=Arch` |
| lest | SCONJ | IN | lest | — |
| unto | ADP | IN | unto | — |
| behold (sentence-initial or before a comma), lo | INTJ | UH | behold, lo | — |
| aught, nought, naught | PRON | NN | aught, nought, naught | `PronType=Ind` / `PronType=Neg` |
| whosoever, whoso | PRON | WP | whosoever, whoso | `PronType=Rel` |

The UPOS of `hath`, `hast`, `hadst` is AUX before a past participle and VERB otherwise; the parser decides
(the table gives the lemma and FEATS and the XPOS; the UPOS between AUX and VERB comes from the context, as for
modern *have*). `art` needs one context condition: AUX when `thou` is within one word before or after it,
otherwise left to the tagger (it is mostly the noun in the tales).

**Where.** A forced-tag pass: the table is `CLOSED` in `en/src/emode.rs` (a Rust constant; each entry carries its
«why»), `force` gives the tags to `PTagger::tag_forced` before the tagger decides the rest, `fix` sets lemma, UPOS and
FEATS after the parse. The modals carry `Style=Arch|VerbForm=Fin` (EWT gives a modal `VerbForm=Fin` only, no person);
`thine` has the lemma *thy*; `art`, `wilt` need `thou` up to two words before or one after; `behold`, `lo` are UH
first in the clause, after a punctuation mark or *and*, or before a punctuation mark. `hath`, `hast`, `hadst`, `dost`,
`doth`, `didst` take AUX or VERB from the parse; a copula among them becomes `aux`.

**Count.** SLM now: 1 637 sentences have a UPOS error on a T1 form. The table does not need a model.

**Negative control.** «the art of the perfumer» (db `art|NOUN`), «Ye Olde» (not in our texts; the table must not
fire on a capitalised `Ye` before a capitalised noun in a modern text), *wilt* the verb (to droop) in a modern
text: `wilt` is AUX only before a verb or after `thou`.

### T2. Pro-adverbs

Add to the same table, as ADV with `advmod`: thereof, therein, thereon, thereby, thereto, therewith, whereof,
wherein, whereon, wherewith, whereby, whither, whence, thither, hither, thence, hence, wherefore, peradventure,
howbeit, yonder, nigh, betimes, hitherto, henceforth, thenceforth, yesternight. SLM now: **479** sentences.
`whither`, `whereof`, `wherein`, `wherewith`, `whereon`, `whereby` get `PronType=Rel` when they open a relative
clause. Negative control: `hence` and `thence` in modern prose («hence the name») are also ADV — no conflict;
`nigh` as an adjective («the time is nigh») → let the parser choose ADJ when it is a predicate after `be`.

**Now.** In `CLOSED`: the 24 *there-*/*thither*/… words as RB/ADV with no FEATS, the 11 *where-* words as WRB/ADV
with `PronType` from the annotation, `nigh` only when not after a form of *be*; `yesternight` is left out (the db has
it as NOUN). Also `yea`, `nay` as UH/INTJ.

### T3. The `-eth` / `-est` guesser

A form not in the dictionary (or in it only as an AGID superlative, L2) that ends in `-eth`/`-th` or `-est`/`-st`
and whose stem (with the spelling rules of chapter 4.3) is a known **verb** lemma is a verb: VBZ with
`Number=Sing|Person=3|Style=Arch` for `-eth`; VBP (or VBD after a past stem) with `Number=Sing|Person=2|Style=Arch`
for `-est`. SLM now: 454 sentences with an `-eth` error and 126 with an `-est` error, 573 together.

Negative control (all from the Torah): *priest*, *against*, *beast*, *feast*, *harvest*, *interest*, *youngest*,
*greatest*, *earth*, *death*, *teeth*, *Beth*, *Heth*, *Japheth*, *twentieth* — every one of these is in the
dictionary as a noun, adjective or proper noun, or has a stem that is not a verb.

**Now.** `en::emode::ending` with `en::morph::ending_verb`; the word must be all letters and not capitalised inside
the sentence (after a punctuation mark it may be). A lexical verb the parser made AUX is VERB unless its lemma is a
registry auxiliary. The negative controls above, with *first*, *must*, *best*, *lest*, *honest*, *forest*, are tests.

### L1–L3. Fixes inside `archaic()`

```text
candidates(form):
  for ending in [eth, est, th, st, edst, dst]:
    stem = form - ending
    try: stem, stem+e, undoubled(stem), stem[-i]+y          # spelling rules
    keep candidates c with dict::lemma(c) having a verb paradigm (VB/VBZ/VBD forms)
    and, for -est/-st/-edst/-dst, also stems that are a VBD form → that form's lemma
  pick the kept candidate with the highest UD frequency
```

Unit tests to add (each from the Torah, with the right answer): `cometh`→come, `seeth`→see, `hateth`→hate,
`lieth`→lie, `dieth`→die, `putteth`→put, `marrieth`→marry, `biteth`→bite, `rideth`→ride, `writeth`→write,
`goest`→go, `doest`→do, `seest`→see, `shouldest`→should, `liest`→lie, `saidst`→say, `broughtest`→bring,
`stoodest`→stand, `knewest`→know, `sentest`→send, `badest`→bid. Negative controls: `digest`, `interest`,
`harvest`, `greatest`, `rest` with their usual tags (already in `archaic_verbs`), plus `hat`, `com`, `putt` as
nouns.

**Now.** All the forms above are tests in `en/src/morph.rs` (`archaic_endings`) and pass. The ranking differs from
the sketch: a stem ending consonant–vowel–consonant takes its «e» spelling first (*biteth* → bite, not *bit*), the
others go by the verb's UD frequency, then the lemma's. A form is «known» unless AGID alone lists it, as a
superlative or comparative (`known_form`). For the past, the VBD reading must be at least as frequent in UD as the
form's own present reading (*sawest* → see; *layest* → lay). *badest* is in the verb-only list (AGID has no *bade*).

### L4. The old-forms table

Proposed entries (verb tag required unless marked *):

| Form | Lemma | Form | Lemma | Form | Lemma |
|---|---|---|---|---|---|
| spake* | speak | brake | break | bare | bear |
| sware* | swear | clave* | cleave | gat | get |
| begat* | beget | drave* | drive | sate | sit |
| bade | bid | trode* | tread | clomb* | climb |
| holp*, holpen* | help | durst* | dare | wist* | wit |
| wot* | wit | wrought | work | smote | smite |
| abode | abide | builded* | build | digged* | dig |
| sod | seethe | sodden | seethe | waxen | wax |
| hewn | hew | molten | melt | graven | grave |
| shapen | shape | stricken | strike | laden | lade |
| gotten | get | shew*, shewed*, shewn* | show | quoth* | quoth |
| kine (NNS)* | cow | brethren (NNS)* | brother | | |

`wrought` must stay ADJ with lemma *wrought* in «wrought iron»; `sod`, `bare`, `abode`, `laden`, `bade` keep their
noun/adjective readings without a verb tag.

### L5. Re-lemmatize after a re-tag

In `gparse/src/data.rs`, `Lex::fill` returns «the most frequent FEATS of the tag, with the old lemma» when the
corpus does not know the (form, UPOS, XPOS) triple. Replace the old lemma by `en::morph::Morph.lemmatize(form,
Tag::parse(xpos))` (and the FEATS by those of T1/T3 when the form is in the tables). This is the step that turns
`giveth` NOUN into `giveth` VERB today. db: **400 tokens in 328 sentences** carry the result (D1 + D2). The same
change keeps future rounds clean.

Negative control: a form whose tag does not change keeps its lemma (the current behaviour, `annotate` in
`gparse/src/main.rs` already guards this).

**Now.** Done in `Lex::fill`: the EModE table first (F1), then the corpus, then the lemma for the new tag. A verb
whose new lemma would only repeat its form keeps an old lemma that does not (*smote* VBD → VBP stays *smite*).

### L6. A deterministic repair pass over the db

The current db can be repaired without a teacher: re-lemmatize every VERB/AUX whose lemma equals an archaic form
(L1–L5), set `thee` → `thou`, `thine` → `thy`, `mine` → `my`, lower-case non-PROPN lemmas (S7), and apply F1/F2.
Write the result as a new db version (the db keeps `history.conllu` and `log.tsv`), and report the changed token
counts per rule. Then re-run the world and the course on it; Torah ★ (252 on 2026-10-17) is the check.

**Now.** `annot db-repair <in.conllu> <out.conllu>` writes the repaired copy and prints the counts per rule (see the
status section); `drafts_from` applies the same rules to every draft before the gates. `mine` → *my* is not done
(not in the table). Writing it into the shared db, and the world and course re-run, are left to the operator.

### F1–F3. Features

- F1: archaic pronoun FEATS only from the T1 table (db: 98 sentences with `Person=3`).
- F2: `Style=Arch` everywhere the tables fire (db: 1 681 sentences lack it on at least one archaic form). UD
  English EWT uses it on its single `thou`, `thy`, `ye` and `art` tokens; the registry allows it.
- F3: verb person and number from the ending; remove `Number=Plur|Person=3` from `-est` verbs (3 in the db).

**Now.** F1 in `en::emode::fix` (the SLM draft), `Lex::table` (the graph parser, every word of the table) and
`annot db-repair`. F2 for every word of the tables, filtered by the registry (no `Style=Arch` on SCONJ or INTJ). F3
for the words T3 covers.

### S1. Postverbal `not`

Rule: `not` (or `never`) directly after a lexical VERB tagged VB/VBD/VBP/VBZ, or after that verb's postposed
pronoun subject, and attached to a word to its right → re-attach to the verb as `advmod`. db: **72** tokens
attached elsewhere today. Negative controls: «not the sinew but the bone» (no verb before `not`); «I will not eat»
(AUX before `not`); «let not the Lord be angry» (keep: `not` belongs to the lower clause — exception for *let*).

**Now.** `en::emode::postverbal_not`, after the parse in `en::annotate`, `gparse annotate` and `annot db-repair`. Only
`not`: `never` and modern `not` («recommend not using», «know not to eat», «has not a Tom») moved EWT words, so
the rule skips *be*, *have*, *do* and a `not` before VBG, VBN, TO, JJ or *only*. Torah SLM draft: 114 moved, their
head agreeing with the db 33 → 73; the rest are the db's own D10 errors.

### S2. «it came to pass»

Rewrite to: `came` root, `it` `expl`, `to` `mark` of `pass`, `pass` `xcomp`, the `that`-clause `csubj` of `came`, a
temporal `as`/`when`-clause `advcl`. db: 106 occurrences, five analyses today. Negative control: «came to the
place» (no `pass`), «come to pass» inside a quoted prophecy («it shall come to pass») — same rule, future tense.

### S3. Subjunctive

A VB with a subject, under a `mark` in {if, though, lest, until, till, ere, whether, except, unless}, or a VB root
with a third-person subject before it and no auxiliary («God give thee», «cursed be») → `Mood=Sub|Tense=Pres|
VerbForm=Fin` plus the subject's person and number (EWT's convention). db: 53 VB tokens with `VerbForm=Inf` in
this position, plus 35 `be` tokens (the two overlap). Negative controls: «that thou mayest know» (modal + infinitive);
«if ye will do this» (indicative AUX); imperatives (no subject).

### P1–P4. The pragmatic layer

All in `prag/src/feats.rs`:

- `negw`: add *nought*, *naught*, *nay*; treat `lest` as `neg = "purpose"` (or as a separate feature) so that «lest
  ye die» is not a statement that you die.
- `norm_word`: add `"shalt" => "shall"`, `"wilt" => "will"`, `"canst" => "can"`, `"mayest" | "mayst" => "may"`,
  `"mightest" => "might"`, `"shouldest" | "shouldst" => "should"`, `"wouldest" | "wouldst" => "would"`,
  `"couldest" | "couldst" => "could"` — as it already maps `"sha" => "shall"`. 750 Torah sentences.
- subject person: when `w.form` is *thou*, *thee*, *ye*, use 2 whatever FEATS say (D4: 98 sentences).
- the second-person list: add *thyself*.

### W1–W4. The world layer

- W1: `archaic_adverbs` in `global/seeds/shortcuts/selection.md`: add thereof, therein, thereon, thereby,
  thereto, therewith, whereof, wherein, whereon, wherewith, whereby, peradventure, howbeit, nigh, betimes,
  hitherto, henceforth, thenceforth, yesternight.
- W2: false friends as register senses (chapter 10.3).
- W3: biblical formulas as expressions (chapter 11.4).
- W4: partitive `of` as an object role in frames (chapter 8.3).

### K1–K2. Tokenizer and the conditional `an`

- K1: split `'tis` → `'t` + `is`, `'twas`, `'twere`, `'twill`, `'twould` alike; keep `may'st`, `could'st`,
  `would'st`, `should'st`, `ne'er`, `e'er`, `o'er`, `e'en`, `th'` as one token each and give them lemmas
  (*may, could, would, should, never, ever, over, even, the*). Tales: 112 lines with `'tis`-type forms, 13 with
  `-'st`, 44 with `ne'er`-type forms (pool, 2026-10-17); round 121 refused drafts on the fragments `St`, `st`, `e`,
  `er`, `lt`, `k`.
- K2: `an`/`and` + nominative pronoun + finite verb → SCONJ, lemma *an*, gloss *if*.

### R1–R2. Training data

- R1: once L6 has repaired the db, add the Torah db to the SLM's training set (JPS 1917 is public domain;
  the annotation is ours). One word in 32 of the Torah is an archaic pronoun; EWT train has 6 such tokens in all.
  Follow «examples are not truth»: retrain on the repaired db, measure on the tales and on EWT dev, and keep the
  Torah test fold out of the training.
- R2: `CYCLE_LEX` (the graph parser's and `treeagree`'s lexicon) is `corpus/school/school-agreed.conllu`, which
  has no biblical forms; adding the repaired Torah db makes `Lex::fill` find (form, UPOS, XPOS) for every archaic
  form and stops the `it`-features fallback at the source.

## What not to do

- **Do not normalise the text** (respell `thou` → `you`, `hath` → `has`) before parsing. It would destroy the
  person and number information (`thou` is singular, `you` is not), break the alignment with the KJV and the
  Hebrew, and make the db disagree with the text it claims to annotate. Normalisation is for spelling (chapter 2),
  not for grammar.
- **Do not extend the registry's auxiliary list** with `shalt`, `hath`, `art`: the registry is right, the lemma
  is wrong.
- **Do not ask the teacher to fix lemmas the tables can fix**: it costs money, it is not deterministic, and the
  db shows it does not do it reliably (D1–D3).
