# roundtrip — where each layer of Mova loses information

`roundtrip` sends a text down Mova's layers and back up: words → tokens → UD tree → frames → world, then world → frames → tree → tokens → words. At every layer boundary it records what does not come back, puts every loss in a class, and counts what would have to be stored beside the layer's representation for the round trip to be lossless (the layer's residual). The idea, the exact and normal modes, the residual classes, the bit rules, blocks and the gates are explained in [docs/english.md, «The round trip»](../../docs/english.md#the-round-trip-what-each-layer-keeps-and-loses); this README holds the crate's own detail: its decoders, commands, inputs and outputs.

Subcommands:
- `probe` shows every layer of one text, both ways.
- `run` is the round trip over named sets, in the exact mode (`--exact`, the default gate) or the normal mode (`--normal`).
- `compare` puts two runs side by side.
- `mixed` is the meta layer test: many languages and styles in one stream, each piece read by its own parser, one world, back byte for byte.
- `blocks` is the training gate: every lesson, chapter or run of chapters of every corpus of the training mix as one block with one discourse and one world, back byte for byte. With `--site-md` it takes the site's Markdown pages as blocks too.
- `site-learn` learns what the site's train pages teach (markup dictionary, Hebrew trees' lexicon, adapted word order).

## How it works

### The layers, and what goes up and down

| boundary | up (encoder) | down (decoder) | residual of the exact mode |
|---|---|---|---|
| text ↔ tokens | `en::tok` on each block (a paragraph, a line, a treebank sentence); the literal before every token is kept as a gap; blocks are cut into sentences at . ! ? (with closing quotes) before a space and a capital or an opening quote | the forms joined by their space bit, one space between sentences | the layout pieces (blank lines, line ids, lesson keys), every gap that is not "" or " ", a block's tail |
| tokens ↔ tree | the `en` annotator (`en.bin`): lemma, PTB tag, head, relation | `en`'s generator: `lin.rs` gives the order, `morph.rs` the form from lemma, tag and the FEATS agreement reads from the tree (`tree_feats`: pronoun case, the person and number of «be», these/those; `ROUNDTRIP_OFF=feats`: lemma and tag alone), positional rules a/an and the first capital, the spacing rules of `en::text::detok` | the local order of each head the generator gets wrong (the whole order of a non-projective tree), a case beyond the rules, a form lemma and tag do not give back, a space bit the rules miss |
| tree ↔ frames | `prag::frames::frames_of_tree`, the discourse carried through the volume | a realizer in this crate (below), then the tree decoder | an edit script from the realized tree to the parsed one: words to insert, lemma, tag, head or relation of a realized word to patch, realized words to drop |
| frames ↔ world | the story's state: entities by id (number, name or not, every modifier said of them, the pronoun class the text uses for them) and the line of events, statements and phrases without a predicate (headings, titles, noun phrases; `ROUNDTRIP_OFF=world-bare` drops them) with roles as ids (each mention with what it says beside the entity: determiner, preposition, possessor, modifiers), the clause's own words, and the links «what was said», «because» and where each clause hangs | one frame per event, modifiers at the first mention, a new sentence at an event not linked to the current one; an entity by name, or by its pronoun where the referring-expression generator says so (below) | the frames the world dropped (a statement said again), field patches (the pronoun of a mention, a modifier said at this mention, the number of this mention, quotation, the «next» links), the sentence boundaries that differ |

**The realizer** (frames → tree) says what the frames keep, and nothing it guesses. Every frame becomes a clause with the verb group of its `tam` (tense, aspect, voice, modality: the modal or «will» or «do», «have», «be» of the progressive, «be» of the passive, «to» of an infinitive; a passive says the patient as subject and the agent with its «by», an imperative no «you»); a frame without `tam` (a nominal event, `PRAG_OFF=tam`) gets past tense for events and speech, present for general statements, «do not» for a negation, «can» for `Fact(can)`. The agent is the subject unless the frame marks it unsaid (shared with the clause above, an imperative's «you», the noun of a relative clause), and a conjunct does not repeat the noun subject of the clause before; the patient is the object, the recipient and the place take the mention's own preposition (a recipient without one is an indirect object), the obliques theirs; then the role-less noun phrases, the clause's adverbs and discourse words, its markers and conjunction. A copula with an adjective, a noun or a place makes a copular clause. A frame without a predicate (`Bare`: a heading, a title, a noun phrase, an answer option) is headed by its own word when it has one (the word with the relation root: an adjective, an adverb, an interjection), else by its entity (the patient, the place or the first oblique, with the mention's preposition); its other obliques, words and noun phrases hang on that head, and the clauses that link to it too; its attributes and negation sum up its words for the readers and are not said again. The event a noun names (a nominal frame, `By::Nominal`: «the arrival of the Queen») is said by the noun's mention, not as a clause of its own (`PRAG_OFF=rt-nominal` says it again), unless the sentence has no other frame; a role-less noun phrase takes its mention's preposition. An entity is its pronoun if the frame kept one, the relative word that stands for it, or its head with the mention's determiner or possessor, its adjectives and other one-word modifiers, its prepositional modifiers, appositions and coordinated phrases with their conjunctions. A clause hangs where its link says, with its relation, under the noun of a relative clause; a frame without a link is a conjunct of the first. Punctuation comes from the clause structure (`en::lin::punctuate`: the full stop; a comma before a coordinated clause with its own subject and in lists of three or more, on a parataxis, a vocative, a discourse word, an apposition). Each step has a switch that turns it off: `PRAG_OFF=rt-shared-subject`, `rt-unsaid`, `rt-function` (fixed «a» then «the», «to», «in», «and», «because», «if»), `rt-links` (all conjuncts of the first, what was said a ccomp, a cause an advcl), `rt-punct` (the full stop only), `rt-prt-once`, `rt-when`. It reads only the frame's content (`frames::F`): never `Ent::text`, the phrase as written.

**The referring-expression generator** (world → frames, `world::refer`) says an entity as its pronoun (its class he, she, it or they from the registry, nominative as the subject and capitalized as the first subject of a sentence, accusative elsewhere) when it was mentioned in this sentence or the one before, it fills no other role of the frame, it has no modifier to say, and `prag::frames::Ctx` — the reader's own resolver, salience by role, halved at every sentence — would take the pronoun back to it; an unresolved «it» stays a pronoun. `ROUNDTRIP_OFF=refgen` switches it off (every entity by name).

**The world** is what `prag`'s discourse and `roles::world_docs` keep of a text, with every role and link. The worlds written by the large model in world's change language (`world/README.md`) are not used here.

### Classes

The class names used in the reports: `meta` (ids, lesson keys: not language), `layout` (line wraps, blank lines), `spacing`, `punct`, `case`, `form` (another form of the same lemma), `lemma→form` (a known word that is not what lemma and tag give: a tag or lemma error), `src-spelling` (no known word: archaic, dialect, misspelt), `typography` (’ “ ” for ' "), `order`, `function` (determiners, prepositions, conjunctions, markers, particles), `pron/name`, `tense/mod` (auxiliaries, modals, the copula, verb tense), `negation`, `quantity` (numbers, quantifiers, grammatical number), `dropped` (content words), `lexical` (another content word in the same place), `attachment` (same words, another head or relation), `sentence` (boundaries), `links` (next, cause, what was said), `quote` (direct or reported speech), `repetition`. The Hebrew route adds `accent` and `unparsed`; the site's pages add `markup`, `repeat`, `conllu` and `frames-table`.

The normal mode aligns the decoded words with the original in steps: the longest common subsequence of forms, then the same form elsewhere (order), typography, case, the same lemma (tense for verbs, form otherwise), a pronoun and a name in the same relation, content words of the same part of speech in the same relation (lexical). What is left is missing or added, each in the class of its word.

### Bits

The fixed bit rules are in [docs/english.md](../../docs/english.md#residual-classes-and-bits). A dictionary lemma costs 17 bits, a word outside the dictionary its string. A patch item is one field: a word whose lemma and tag both change is two items.

### Gates and controls

The exact gate, the damage, drop and shuffle controls and determinism are described in [docs/english.md, «Gates and controls»](../../docs/english.md#gates-and-controls). The crate's tests carry the negative controls of the decoders: a script without its inserts, a world residual without its patches or boundary toggles, a tree residual with a wrong form; `tam_is_read_and_realized` (no `tam` gives the tense items back), `en::morph` `pronoun_case_and_be_agreement_from_the_tree` (lemma and tag alone give they, was, this), `pronouns_by_salience` (the plain world decoder says no pronoun). In `run`, `--drop <class>` makes a decoder drop that class. Volumes run in parallel and merge in their order.

## How to run

```sh
cargo build --release -p roundtrip
B=target/release/roundtrip
$B probe "The dog didn't bark. 'Let there be light,' said God."      # every layer of a text, both ways: tokens, tree, frames, the world (entities, events, its residual), back
C=$MOVA_DATA/runs/corpus-0.4
$B run --threads 8 --out $MOVA_DATA/runs/roundtrip-<date>/run \
  lessons=lessons:$MOVA_DATA/school/courses \
  school-gold=conllu:$C/corpus-0.4/treebanks/mova-school-agreed-en.conllu \
  tales03=conllu:$C/corpus-0.3/treebanks/mova-tales-silver-en.conllu \
  torah=tsv:<torah.tsv> \
  alice=txt:$C/corpus-0.4/books/pg11.txt
$B run --normal --drop negation --out <dir> tales03=conllu:…          # a negative control
$B compare <before dir> <after dir>                                   # two runs side by side: what changed
cargo test --release -p roundtrip                                     # the model tests need $MOVA_DATA/en/models/en.bin
```

A set is `<name>=<kind>:<path>[@<stride>]`: `txt` (a book, paragraphs are blocks), `tsv` (`id<TAB>text` lines), `lessons` (a school directory; `title`, `explain`, `fact`, `exercise` values are text, every `stride`-th lesson), `conllu` (a treebank: its `# text`, its trees as gold, one volume per document, every `stride`-th document). In the normal mode a treebank's gold tree, with its frames and world, is a start too. The run writes `report.md` (every class at every layer and start, for every set), `examples.md` (up to four examples of every class at every layer and start), `residual.tsv` and `normal.tsv`. The model is `$MOVA_DATA/en/models/en.bin` unless `--model` says otherwise.

## Limits

- The frame realizer and the world decoder say only what the frames and the world keep, with fixed rules: much of what is left at the frames layer (punctuation that depends on word order, the words of phrases in no frame) is what these decoders do not try to say. The ranking says which parts of language the frames do not carry, not how good a realizer can be.
- The frames layer is compared on the tree: the residual is a tree edit script, so a word the frames keep in another form counts as a lemma or tag patch, and attachment patches count the realizer's clause structure against the parser's.
- Bits are estimates by fixed rules, not an entropy code; per-document regularities (one apostrophe style for a whole book, one line width) are counted per word.
- The `en.bin` model was trained on the school's parse database, so the school sets are in its domain; the tales and the Torah are not.
- The tales treebank's odd half is the test set of the tales parsing numbers; this diagnostic trains nothing on it.
- `prag` frames are read from `en`'s tree without world's matrix repairs (`world::frames::read` adds them).
- The open work the residual points to is listed in [docs/english.md, «Limits and open work»](../../docs/english.md#limits-and-open-work).

## The meta layer test: `roundtrip mixed`

Texts mix languages and styles: a commentary quotes Hebrew and Aramaic inside English, a lesson puts slang next to an exam question. `roundtrip mixed` feeds such mixtures to the skeleton and asks it to see that the pieces are different languages, read each with its own parser, keep one world for all of them, and give everything back byte for byte — comma for comma, error for error, each piece in its own language.

### How it works

**The sample** (seed 20261018, at most `--n` sentences a variety, 3–40 words):

| variety | source | route |
|---|---|---|
| en-plain, en-slang, en-formal | the school: explain/fact sentences, the slang tests' questions, the formal exams' questions | English |
| en-tale | the 0.3 tales treebank's text | English |
| en-jps | JPS 1917, Ruth and Jonah (with the verse of each sentence) | English |
| en-talmud | the Yerushalmi in Guggenheimer's English | English |
| en-shk | Shakespeare (`corpora/emode`) | English |
| hbo-accented, hbo-pointed, hbo-unpointed | WLC Ruth and Jonah (the Hebrew parser's dev books), each verse in one of three pointings: as written, accents off, letters only | Hebrew |
| hbo-x-mishnaic, jpa | the Yerushalmi's Hebrew and Palestinian Aramaic segments (`langseg talmud`'s labels) | Hebrew / text |
| arc-x-targum | Targum Onkelos | text |
| arc | WLC Biblical Aramaic (Daniel, Ezra, Jeremiah 10:11) | text |
| he-wiki, he-htb | UD IAHLTwiki dev (CC BY-SA) and UD HTB dev (CC BY-NC): read for measuring only, never shipped | Hebrew |

**The streams.** `mono-<variety>`: one variety, a sentence a line. `lines`: every sentence of every variety shuffled, a sentence a line (the same items as the mono streams together). `docs`: documents (`--docs`, 300 by default) of an English carrier sentence with one or two quotes of Hebrew or Aramaic — cut by the quote (`A «Q» B`), followed by it (`A «Q»`), or by two (`A «Q1» (Q2)`) — and the verse pairs of Ruth and Jonah (the JPS sentences of a verse, then `«` the Hebrew verse `»`). A stream is cut into volumes of 25 lines (`--volume`); a volume is one world. The streams, their gold spans and the items are written to `<out>/streams/`.

**The meta layer, text ↔ pieces.** `langseg` segments each line. A segment is cut where a document's quote marks or brackets (« » “ ” ( ) [ ]) stand between its words: langseg reads a Hebrew-script run whole, so two quotes side by side come out as one segment otherwise. Neighbouring segments of one route make one piece (English always; Hebrew and Aramaic unless quote marks part them). The gap between two pieces is cut at its last run of whitespace: what comes before it ends the piece before, the whitespace is the joint, opening marks start the next piece. Down, the pieces are joined with one space; another joint is residual.

**The routes.**
- English (en, translit): `en` tokens and tree, `prag` frames, exactly as the single-language round trip above. A sentence's residual goes to the item that holds most of its bytes.
- Hebrew (hbo, hbo-x-mishnaic, he): `hbo`'s segmenter and `gparse` (the pointed or unpointed model by the text; one `gparse annotate` call per model for all the pieces of all the runs, a failure waited out ten minutes, three times at most). Text ↔ tokens: the piece without its quote marks, in NFC (a word that is not NFC is kept as written), cut by `hbo::text::tokenize`; the decoder writes a space between tokens, none after a maqaf or before a mark. Tokens ↔ tree: each word's form is the commonest form of its lemma, tag and FEATS in the parser's training trees (a lookup generator), its accents residual when the lookup gives the form without them; a prefix (ו ה ב כ ל מ ש) joins the next word and a non-subject pronoun the word before in one written word; the swallowed article (a bare ה after a preposition) is not written. Tree ↔ frames: one frame per verb (lemma, subject, object, indirect object, obliques with their preposition, negation, adverbs; a verb of saying quotes its ccomp), a nominal clause is a fact, and `macula prag`'s rules read the sentence (form, act, speech, carried through the volume) into the first frame. Down: the simplest honest realizer and an edit script to the parsed tree, as for English.
- Text (the Aramaic varieties, other scripts): there is no Aramaic parser. The piece has the Hebrew route's token layer, and its tokens are the whole residual (class `unparsed`); nothing above them holds it.

**One world.** The frames of all English and Hebrew sentences of a volume, in order, make one world (`world::encode_by`, each residual item counted for its sentence's item). An entity named in Hebrew and one named in English are one entity only if linked: by the verse — a Hebrew piece found in an index of the WLC's verses and an English piece of the same volume found in an index of JPS 1917's sentences, joined when the English id is among the English words (and their lemmas) of the Hebrew lemma in `tanakh-en.tsv` (MACULA's commonest glosses and the OpenScriptures definition) — or by the name, a Hebrew and an English name anywhere in the volume, by the same dictionary. A Hebrew lemma joins at most one English id and the reverse. Down, the decoder gives a Hebrew sentence its Hebrew names back. Links are checked against MACULA's gloss of the word in that verse (the gold trees' `Gloss=`, read in context).

**Bits.** As in the single-language round trip; a character of the Hebrew script (a letter, a point, an accent) is 8 bits, as an English letter, not the 16 of its two UTF-8 bytes; a Hebrew lemma of the parser's training trees is log2 of their number.

**Runs.** Every mono stream with langseg and with the gold routing (`oracle`); `lines` and `docs` with langseg, the oracle, and two negative controls: everything to English (`all-en`), and langseg's languages shuffled over the segments of a volume (`shuffle`); the linking control `link-any` (the gloss alone, any Hebrew piece with any English piece, no verse); the ablation of the cut at quote marks. `--damage` is the gate's negative control: the exact decode drops the Hebrew tree residual and the joints, and the gate must show red.

### Run

```sh
cargo build --release -p roundtrip
roundtrip mixed --out $MOVA_DATA/runs/roundtrip-mixed-<date>/run --threads 8 \
  --school $MOVA_DATA/school/courses --tales <mova-tales-silver-en.conllu> --judaism $MOVA_DATA/corpora/judaism \
  --shakespeare $MOVA_DATA/corpora/emode/text/shakespeare.tsv --ud <ud-2.18> --hbo $MOVA_DATA/models/hbo \
  --langseg $MOVA_DATA/langseg/model.tsv --gparse <gparse binary>      # [--n 120] [--docs 300] [--seed N] [--volume 25] [--damage]
```

It writes `report.md` (routing, residual by variety and layer, the cost of mixing, links, controls, where the meta layer leaks), `residual.tsv` (run, mode, variety, layer, class, bits), `items.tsv` (every item's bits by layer in every run), `links.tsv` (every link of `docs`), the parses (`hbo-pointed.conllu`, `hbo-unpointed.conllu`) and `streams/`. The outputs hold Modern Hebrew under CC BY-SA and CC BY-NC: they stay in `$MOVA_DATA`. `hbo` and `macula` are libraries as well as commands for this (`hbo::{segment_line, annotate}`, `macula::prag::read_sentence`).

### Limits

- The Hebrew and Aramaic samples are small. Ruth and Jonah are the Hebrew parser's dev books (its choices were made on them), and langseg was trained on most of the Bible, the Targum and the two Yerushalmi tractates; the Yerushalmi's labels are langseg's own. None of this is a held-out measure of the parsers or of langseg.
- The Hebrew frames reader and realizer are the simplest honest ones, and the Hebrew generator is a lookup; their residual says what they do not try to say, not how good a Hebrew realizer can be. The English and the Hebrew estimators share the bit rules but not the trees, so bits compare within a variety (mono against mixed), less across languages.
- There is no Aramaic parser and no parser for Mishnaic or Modern Hebrew: the Biblical Hebrew route reads them. Modern Hebrew's open treebanks are CC BY-SA or NC (measure only).
- A quote's marks are stored as literals on its token layer, and the English parser reads a carrier cut by its quote as two fragments, not one sentence with a quoted object.
- A piece's residual goes to the item that holds most of its bytes; when langseg merges two items into one piece the attribution is coarse.
- Cross-language entity links go by dictionary words, not by the verse's word alignment; link correctness is judged against MACULA's gloss in context, where synonyms (LORD / God) count as wrong. Names and pronouns are joined only within a volume.
- The documents are built by template; real commentaries also transliterate (`translit`), abbreviate and cite by verse number, which this test does not sample.
- Modern Hebrew (UD IAHLT, CC BY-SA; HTB, CC BY-NC) is only read here; nothing built from it is kept.

## The block gate: `roundtrip blocks`

In training every lesson of the corpus mix must unfold back into itself bit for bit, as whole blocks, not sentence by sentence. A **block** is a whole lesson, a book chapter, or a run of whole chapters up to 2,500 words (`--max-words`), cut only at chapter or lesson boundaries; a single chapter longer than that stays whole. This is a hard training target, not a claim that the layers keep every word and meaning: the residual says what they lose. Why blocks: [docs/english.md, «Blocks, not sentences»](../../docs/english.md#blocks-not-sentences).

### How it works

**Block makers** (`roundtrip::blocks`, a library for `tools/translate` too). Each corpus is read into chapters in its own order; `blocks(corpus, &chapters, max_words)` groups them greedily into runs that stay inside a chapter's `group`.

| corpus | chapter | a run stays inside | layout of a chapter |
|---|---|---|---|
| school | a lesson (`$MOVA_DATA/school/courses`, `<course>/*.lesson`) | the lesson (a block each) | the lesson file: keys and other lines are metadata |
| tales | a chapter of the 0.3 tales books and *Just So Stories* | the book | the file itself |
| alice | a chapter of *Alice's Adventures in Wonderland* | the book | the file itself |
| tanakh | a JPS 1917 chapter (`dict/tanakh-jps1917.tsv`, a sentence goes to the chapter of its first verse) | the Bible book | heading `GEN 1` (metadata), the sentences as one paragraph |
| talmud | a halakhah of the Yerushalmi in Guggenheimer's English | the tractate's chapter | heading `Berakhot 1.1`, a paragraph a segment |
| shakespeare | a scene (`corpora/emode/text/shakespeare.tsv` with `.speakers`) | the play | heading `1H4 1.1`, a speech a paragraph under its speaker (metadata) |
| grammars | a manual file (`corpora/grammars-*/text/manual/**.md`), a chapter of an English grammar book (`text/books-en/*.txt`) | the manual, the book | the file itself |
| hbo | a WLC chapter (`dict/wlc-trees`, a verse a line, accents and points as written) | the JPS blocks (`like`) | the verses |

A book is cut at lines starting with CHAPTER, PART, BOOK, LECTURE, STORY or SECTION after a blank line, and in the tales and Alice also at a title line in capitals alone between blank lines; the text before the first heading is a chapter too. A "chapter" longer than 10,000 words means the maker sees no chapter structure (a dictionary, a handbook numbered by sections only, OCR without headings): it is cut at blank lines into runs of at most 2,500 words and the block says so (`para_cut`). The chapters of a file together are the file byte for byte (tests).

**Whole-block encoding.** A block is one volume: the reader's discourse (`prag::frames::Ctx`), the realizer and the world are one for the whole block; headings, speakers and ids are metadata pieces, paragraphs and blank lines layout pieces, and sentence boundaries a class of the world's residual — all of it must come back. Exact mode: every block is decoded from its world down with every residual and compared byte for byte. Normal mode: the whole block decoded from each start (tokens, tree, frames, world) against the block's sentences on one line, exact % and chrF, and sentence by sentence. Hebrew goes through the Hebrew route of `roundtrip mixed` (one volume a block).

**Units** (`pipeline::Unit`): `block` (the gate); `sentence` (the discourse, the realizer and the world reset at every sentence: the frames themselves change, a pronoun the reader cannot resolve stays a pronoun); `world` (the frames read with the block's discourse, the realizer and the world reset at every sentence: the same frames as `block`, so the bits compare). Hebrew has `block` and the verse alone.

**Controls**, on every 10th block of each corpus (`--controls`): `damage-<layer>` loses the first residual item of a layer in the exact decode (Hebrew: the whole tree residual) and the block must be red; `flat` (the block's sentences joined by a space) against `shuffle` (the same sentences shuffled inside the block). A block that fails to go up or come down (an error or a panic) is a red block with its reason, not the end of the run.

### Run

```sh
cargo build --release -p roundtrip -p jobs
jobs run --name rt-blocks --mem-gb 8 -- roundtrip blocks --out $MOVA_DATA/runs/roundtrip-blocks-<date>/run --threads 8 \
  --model <en.bin> --school $MOVA_DATA/school/courses --hbo $MOVA_DATA/models/hbo --langseg <langseg model> --gparse <gparse>
  # defaults: --judaism, --shakespeare, --corpora under $MOVA_DATA/corpora; tales and books under $MOVA_DATA/runs/corpus-0.4
  # [--only school,tales,alice,tanakh,talmud,shakespeare,grammars,hbo] [--stride N (lessons)] [--controls N] [--max-words 2500] [--hbo-books GEN,EXO,…]
  # [--school <dir>] [--tales <dir>] [--justso <txt>] [--alice <txt>]: the sources of those corpora (Just So Stories joins the tales)
  # [--site-punct <punct.tsv>]: the punctuation table of `site-learn` for the frames' realizer
  # MOVA_RT_LEMMA=0: a Hebrew stem's lemma is unknown (`_`) instead of guessed from its letters (an A/B switch)
```

It writes `report.md` (blocks per corpus, the gate and its controls, residual by layer and class, normal mode, the units compared, shuffle, the classes ranked by share of the total and by bits per sentence with the changes they ask for), the training signal `residual-blocks.tsv` (block unit: `corpus, block, layer, class, items, bits` — what the trainers and the agents read), `blocks.tsv` (a row per block and unit: chapters, words, sentences, tokens, exact, bits per layer, metadata bits, total), `total.tsv` (per run and corpus: blocks, exact, total bits per block, sentence and token), and per unit `block/`, `sentence/`, `world/` with `residual.tsv`, `normal.tsv`, `examples.md` in the format of `roundtrip run` (`roundtrip compare` reads them). The outputs stay in `$MOVA_DATA`.

API for other tools: `blocks::{school, book, books_dir, jps, wlc, talmud_en, shakespeare, grammars}` → `Vec<Chapter>` (`id`, `group`, `vol`, `para_cut`); `blocks::blocks(corpus, &chapters, max)` → `Vec<Block>` (`corpus`, `id`, `chapters`, `words`, `para_cut`, `vol`; `text()` is the block byte for byte); `blocks::like(corpus, &chapters, &blocks)` cuts another text of the same chapters as `blocks` (the WLC in the JPS blocks: the two sides of a translation are the same block); `blocks::run::flat(a, &volume, shuffle)`; `pipeline::Opts { unit, damage, .. }`.

### Limits

- Chapter headings are found by fixed rules: books without headings within 10,000 words (handbooks numbered by sections, dictionaries, primers without chapters) are cut at paragraphs, and single chapters over 2,500 words stay whole.
- The JPS, Talmud and Shakespeare blocks are rebuilt from sentence and segment records: their headings (`GEN 1`) are ours, their paragraphs the source's records, not a printed page; the school's lesson keys are metadata, counted aside.
- The `sentence` unit changes the frames themselves, so its bits measure a different, poorer representation; compare bits with the `world` unit.
- One world for a block plans sentences from one registry (every modifier at the first mention, sentences cut at unlinked events, links across boundaries), so its world layer pays for sentence planning and entity reintroduction that a world per sentence does not.
- The shuffle control is not red at the world layer: the reader re-resolves the shuffled pronouns by the same salience the decoder uses, and the world records what was read, not whether the story coheres. The damage controls are the gate's.
- Hebrew is the Torah in the JPS blocks; its normal mode is per sentence only; the Hebrew reader keeps no pronoun entities.

## The site's Markdown as blocks: `blocks::site_md`

The Markdown twin of mova.elamur.ai (`free/en/**.md`: the docs, the corpora pages with their ```` ```conllu ```` trees and frames tables, the dictionaries, the school runs) is a corpus of the block gate. A page is a chapter; a block is a page or a run of sibling pages (one folder, one Bible book) up to 2,500 words of prose, cut only at page boundaries.

### How it works

- **Prose up, markup aside** (`site_md::page_volume`). Paragraphs, list items, table cells with letters, headings and link texts are text blocks and go up the layers as English. The rest is layout of class `markup`: front matter, `<!-- … -->` markers, heading and list marks, table pipes and number cells, code fences; inside a paragraph, link targets, emphasis, backticks, escapes and inline HTML are marks at byte offsets of the block (`vol::Block::marks`), so the prose reads without them and the page comes back byte for byte. A paragraph a marker gives another language (`lang=hbo`, `dir=rtl`: the Hebrew verse, the Targum) is kept as written (`unparsed`).
- **Markup copied** (`vol::Copier`). Each markup literal is a copy of an earlier one of the block (its common prefix and suffix kept, the middle stored) or stored whole: a flag, the earlier literal (one bit when it is the one after the literal the last copy took — a page copied line after line from the page before — else its number), the edit (one bit when the same). With a dictionary learned on the train pages (`--site-dict`), each block starts with its section's frequent markup.
- **Repeated prose copied** (`site_md::repeat`, class `repeat`). A text block that is a copy of the prose before it in the block for at most a quarter of its bytes (a template's sentence with other names: «This is Deuteronomy 12:5 · JPS 1917 in the English of …») is coded as that copy and does not go up the layers. `ROUNDTRIP_OFF=site-repeat` keeps every block.
- **Trees and tables rebuilt from the layers** (`site_code`, classes `conllu` at the tree layer and `frames-table` at the frames layer). An English CoNLL-U is `en`'s annotation of the sentence its `# text` points to (a prose block or a given text), with a patch per field that differs (lemma 17 bits or its string, UPOS 5, tag 6, a FEATS pair 7, head log2(n+1), relation 6, MISC its string, log2(n+1) to name the word); its comment lines are copied. A Hebrew CoNLL-U (MACULA's, right to left) is written from its verse and a lexicon of the train pages (`site_heb`, `--site-heb`): the verse cut into MACULA's written tokens, each token into its words with its commonest analysis (form, lemma, UPOS, FEATS, relation, head inside the token, Gloss, Strong, Type), heads outside the token by the commonest offset of their UPOS and relation, Ref and MId from the verse and the token's number; a token cut into another number of words than the page is given whole, every other difference is a field patch (FEATS and MISC per pair). The English frames table is `prag::frames_md::table` of the frames `prag` reads from the rebuilt tree, one discourse state through the block's trees (a bit per table: start afresh or not; a bit: the note left out); the Hebrew verbs' table is `prag::frames_md::hebrew_table` of the rebuilt Hebrew tree (MACULA's `Frame=`). What a table still differs in is a line script. The page builder (an internal site tool) calls the same two functions, so a rebuilt table is the page's byte for byte.
- **The gate** decodes every derived piece from its code, the block's prose and the model alone, and compares it with the page; the damage controls lose a field patch of a tree (`damage-tree`) or a line of a table script (`damage-frames`) and must show red.
- **Sampling and split.** Pages are listed per section (`corpora/torah`, `corpora/tales`, `docs`, …) in the order of their paths, numbers as numbers; the runs are cut per section; `--site-target N` takes every k-th run so that about N are taken per section; a block goes to train, dev or test by the hash of its first page (test 1 in 10, dev 1 in 10: `site_md::split_of`, fixed).

### What the train pages teach: `roundtrip site-learn`

On the train split (the same sample, the Tanakh pages of `tools/translate`'s dev and test books left out whatever their split): the markup dictionary (`markup.tsv`: per section the literals repeated at least 3 times, at most 4,096, the most bytes saved first), the Hebrew trees' lexicon (`hebrew.tsv`), `en`'s order generator adapted (`en-lin.bin`: `lin.rs` trained again on its own trees and on `en`'s parses of the train pages' prose sentences, `Model::adapt`; the tagger and the parser are the same, so every parse is too), the punctuation table (`punct.tsv`), and every fourth of those sentences for `tools/translate`'s standard encoder (`english.tsv`). Negative controls: `--lin-shuffle` (each sentence's words shuffled before it is learned from) and `--shuffle-tables` (each section gets the next section's dictionary, each Hebrew token another token's analysis).

### Run

```sh
cargo build --release -p roundtrip -p jobs
S=<a frozen copy of the md twin>/free/en          # the live tree is rebuilt in place: measure on a copy
roundtrip site-learn --site-md $S --site-target 1000 --out $L --model <en.bin> --threads 8 --lin-every 2
jobs run --name site --mem-gb 8 -- roundtrip blocks --only site --site-md $S --site-target 1000 --site-split dev \
  --out <dir> --threads 8 --model $L/en-lin.bin --site-dict $L/markup.tsv --site-heb $L/hebrew.tsv
  # [--site-sections corpora/torah,docs] [--units block,sentence,world,controls] [--site-split train|dev|test|all]
  # site-learn: [--lin-weight W] repeats the train pages' sentences W times when the order generator is adapted (default 1);
  #   [--no-lin] stops after the tables (no en-lin.bin); [--punct-only] stops after punct.tsv; [--lin-shuffle] is the negative control
```

The report adds the site's tables: bits per block and per 1,000 words of prose with markup, CoNLL-U, frames tables and the prose layers apart; for each kind of derived piece how many there are, how many need nothing stored, and the bits of storing them whole against their code; what the codes store by kind. The test split is measured once, as the held-out check of a change.

### Limits

- The live md tree is rebuilt in place; runs measure a frozen copy.
- `repeat` takes templated prose out of the layers: its bits are those of the copy, and the layers' classes measure the rest; `ROUNDTRIP_OFF=site-repeat` gives them all.
- The copier's correctness is checked on every literal (a code that does not decode is stored whole); the exact gate copies markup literals, it does not decode them.
- Markup that follows from a page's id and section template (crumbs, the HTML link, the front matter) is copied from earlier pages, not derived.
- The Hebrew lexicon has no back-off: a token never seen is given whole, and heads outside a token follow the commonest offset only.
- A block cut inside a chapter starts the frames tables' discourse state afresh; the English tables of the tales' and Tanakh pages pay for it.
