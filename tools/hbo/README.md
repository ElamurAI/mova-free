# hbo

Mova's own parser for Ancient (Biblical) Hebrew: raw text → words → trees. `macula` gives trees only for the text MACULA analysed; `hbo` reads Hebrew that MACULA never saw (quoted in papers, commentaries, other books).

```
raw text → tokens (spaces, maqaf, sof pasuq) → segmenter (seg.tsv) → gparse (GPU) → CoNLL-U, hbo
```

## Data

`macula` converts the whole Hebrew Bible (all 39 books of MACULA's WLC lowfat files). The Aramaic chapters (Daniel 2:4b–7:28, Ezra 4:8–6:18 and 7:12–26, Jeremiah 10:11) go to `arc/` and are not read here. A sentence goes by the language of most of its words, so Genesis 31:47 (two Aramaic words in a Hebrew verse) stays Hebrew.

`hbo split` holds books out whole:

| part | books |
|---|---|
| train | the rest |
| dev | Ruth, Jonah |
| test | Deuteronomy, Genesis 19–30 |

The test covers all of PTNK's test split (Genesis 19–30 and its verses of Deuteronomy), so PTNK test can be read through MACULA ids. FORM is normalized to a level of pointing: `pointed` drops the accents (teʿamim), meteg and rafe and keeps the vowels; `unpointed` keeps letters only. XPOS is MACULA's `Type` (the verb form: wayyiqtol, qatal, yiqtol, weqatal, imperative …; the kind of a particle), or the UPOS where it has none.

## The segmenter

A written word is cut into MACULA's words: prefixes (ו ה ב כ ל מ ש …), the host, suffixes (pronoun suffixes, the directional and paragogic ה and נ). A split is a list of pieces, each a number of letters (a letter with its marks) or `H`, the article a preposition swallowed (בָּאָרֶץ = בְּ + ה + אָרֶץ, the ה not written). A suffix can take the vowel of the letter before it: MACULA cuts כֻּלָּם as כֻּלּ + ָם, so a piece marked `^` takes the vowels of the letter before it and leaves the dagesh.

The model is learned from the training trees (`seg.tsv`, plain TSV):

1. `lex`: every written word seen in training, with its splits and their counts. A seen word takes its commonest split.
2. `pre`: the first k letters of a word (k = 1 … `max_k`) → how its prefixes split, counted over the rarer words (seen at most `rare` times), which behave like unseen ones. An unseen word takes the split of its longest beginning seen at least `min` times.
3. `rest`: what follows the prefixes, as seen in training → its suffixes; if that was never seen, `suf`: the last k letters of it, as for `pre`.
4. `chain`, `sufx`, `host`: the prefix chains (the letters of the prefixes: ו, ו.ב, ב.H …), suffixes and hosts of training, with counts.
5. `ctx`: for the written words the lexicon splits more than one way, their splits by the word before and the word after.

From these tables a word gets a list of **candidate splits**, best first (`Model::candidates`); the segmenter takes the first. Every candidate must pass the **consonant check**: its words, the swallowed article left out, spell the written word's letters back (Naaijer 2023); a word no candidate passes stays whole. The switches, written in the header of `seg.tsv` (`with=`) or given with `--with`:

| switch | what it does | used |
|---|---|---|
| `hostlex:b` | an unseen word's candidate whose host was never a host in training has its share multiplied by b, and the candidates are ranked by share (Rubinstein 2021); `hostlex` alone is a hard filter | pointed, b = 0.3 |
| `partial` | a pointed word not in the lexicon takes the splits of the lexicon's forms whose points include its own: a missing point is unspecified, not absent (Habash 2016); a word with a bare letter inside that is still unknown goes to the unpointed segmenter (`--fallback`, or `hbo parse` with both model dirs) | pointed |
| `ctx:K` | a word the lexicon splits more than one way: each split's share replaced, first by the word before and then by the word after, with q = (count with that neighbour + K · share) / (neighbour's count + K) | unpointed, K = 0.5 |
| `chain:N` | drop a candidate whose prefix chain or suffix was seen fewer than N times | off |
| `joint` | rank an unseen word's candidates by the product of the prefix and suffix shares, not prefixes first | off |
| `no-check` | turn the consonant check off (negative control) | off |

Every word says why it was cut so, in MISC: `Seg=lex:1772/1772` (seen 1 772 times, always so), `Seg=pre:וּבְ→1.1:31/33,suf:יו→^2:40/52`, `Seg=lex:30/41,p:אל:6/6` (the word before decided) and the like.

Parameters are chosen on dev and on the two-fold check of train (`hbo seg cv`, books alternating between the halves): pointed `max_k 4, min 10, rare 2, with=hostlex:0.3,partial`, unpointed `max_k 3, min 3, rare 5, with=ctx:0.5`. Learning twice gives the same file.

Without points the lexicon itself is ambiguous (בבית: בְּ + בַיִת or בַּ + ה + בַּיִת); `ctx` decides such words by their neighbours. `hbo seg eval` scores the share of written words split exactly right, separately for words not in the lexicon; `--without` turns tables off (ablation), and `--thin X` drops a share X of the points deterministically to evaluate partly pointed text (scored on the letters cut, as a dropped vowel hides which suffix takes it).

### Several segmentations (lattice)

`hbo lattice` writes each sentence in several readings: the segmenter's best, and per written word each other candidate among the first k (3) with at least `ratio` (0.05) of the best's share. `gparse annotate` with `GP_SCORE=1` adds each reading's tree score (`# gp_score`: the log-probabilities of head and relation and half those of the two tags). `hbo choose` gives each word the candidate whose reading gains most, tree gain + λ · log share ratio + μ · words added (`--study` writes every reading's gains against gold), and the text is parsed again.

Nothing uses the lattice by default: the parser, trained on gold words only, scores a wrong segmentation nearly as confidently as a right one, so choosing among readings does not help it. The commands stay for study.

## The parser

`gparse` (the GPU parser, `gparse/README.md`) is trained on the normalized trees as they are; its predicted XPOS carries the verb form. `GP_LEX` (the training trees, `lex.conllu`) gives LEMMA and FEATS for a word whose form and tags were seen, the commonest FEATS of its tags otherwise.

```
GP_LR=0.002 GP_TOKENS=1000 GP_EPOCHS=30 GP_SEED=42 gparse train pointed/gparse data-pointed/train.conllu -- data-pointed/dev-RUT.conllu data-pointed/dev-JON.conllu
```

`hbo blank` clears LEMMA … DEPREL so the parser can be evaluated on gold words; `hbo resegment` cuts the written words with the segmenter for end-to-end evaluation, scored as F1 with words aligned within each written word. The unpointed model uses the same settings.

**Layers of the language.** Biblical Hebrew was written over a thousand years in different grammars. The models `layers-pointed` and `layers-unpointed` (the same settings, their own segmenters) are trained without the archaic poems (Exodus 15, Judges 5, Deuteronomy 32–33, Genesis 49, Psalm 18, 2 Samuel 22), without 1–2 Chronicles, without Amos and Micah, and without Genesis 19–30, so that each layer can be read on held-out text. PTNK test (UD 2.18, CC BY-NC) is read through MACULA ids (`macula compare`, `tools/macula/README.md`).

Two runs of `annotate` give the same files.

## Run

```
macula <lowfat-dir> $MOVA_DATA/models/hbo/trees
hbo split $MOVA_DATA/models/hbo/trees $MOVA_DATA/models/hbo/data-pointed pointed --dev RUT,JON --test DEU,GEN.19-30
hbo seg learn pointed $MOVA_DATA/models/hbo/pointed/seg.tsv data-pointed/train.conllu --max-k 4 --min 10 --rare 2 --with hostlex:0.3,partial
hbo seg learn unpointed $MOVA_DATA/models/hbo/unpointed/seg.tsv data-unpointed/train.conllu --max-k 3 --min 3 --rare 5 --with ctx:0.5
hbo seg cv unpointed data-unpointed/train.conllu --max-k 3 --min 3 --rare 5 --with ctx:0.5
hbo seg eval pointed/seg.tsv data-pointed/test-DEU.conllu [--without lex,rest] [--with …] [--fallback unpointed/seg.tsv] [--thin 0.5]
HBO_GPARSE=<path to gparse> hbo parse $MOVA_DATA/models/hbo in.txt out.conllu
```

`hbo parse` takes one or more sentences a line; a sentence ends at sof pasuq (׃) or «.», «?», «!». A model dir holds `seg.tsv`, `gparse/` and `lex.conllu`; given the dir above both, it takes `pointed/` when the text has vowel points and `unpointed/` otherwise. The output keeps the written words as multiword tokens (forms as written, accents included), with `Seg=` and `Swallowed=ה` in MISC.

The crate is also a library (`hbo::load_seg`, `hbo::segment_line`, `hbo::annotate`, `hbo::lattice`, `hbo::text`, `hbo::seg`, `hbo::conllu`): `roundtrip mixed` parses its Hebrew pieces through it, with the same segmenter switches and fallback as `hbo parse`.

Other commands: `hbo segment` (words only), `hbo resegment` (gold sentences cut by the segmenter; `--with`, `--fallback`), `hbo lattice` and `hbo choose` (several segmentations, above), `hbo blank`, `hbo merge` (a parser's columns back onto a file with multiword tokens and MISC), `hbo score`, `hbo pick` (the sentences of some books or chapters).

Models and data live in `$MOVA_DATA/models/hbo/` (`trees/`, `data-*/`, `pointed/`, `unpointed/`, `layers-pointed/`, `layers-unpointed/`, `runs/`, `log/`), not in git.

## Limits

- **Gold is our conversion.** The trees are read off MACULA's groups by `macula`'s rules; the parser learns those rules' habits along with the language.
- **Sentences are MACULA's.** Raw text is cut at sof pasuq; a verse MACULA splits into two sentences is one here.
- **Unseen words.** A written word not in the training lexicon is cut by the prefix and suffix tables, and the parser has only hashed prefixes and suffixes for it.
- **FEATS by lookup.** FEATS are those of the commonest analysis of the form with its predicted tags; an unseen form gets the commonest FEATS of its tags, a guess.
- **Pointing.** The pointed model expects Tiberian vowels. Partly pointed words are segmented by `partial` and the unpointed fallback, but the parser still sees their forms as written: a form whose points the pointed parser never saw is an unseen word to it. A text pointed by other conventions (modern Hebrew) is not handled.
- **One reading.** The segmenter commits to one split per written word; the lattice commands are for study only.
- **Licence.** The trees come from MACULA Hebrew (CC BY 4.0); PTNK (CC BY-NC) is only a measure.

## Licence

Elamur Cosmic License ([LICENSE.md](../../LICENSE.md)). MACULA Hebrew: © Biblica, Inc., CC BY 4.0.
