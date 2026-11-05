# translate — Mova's translator through a meta-world

`translate` is Mova's own deterministic translator, and its hard test. A text is read down the standard stack of its language into a world, read from there into a **meta-world** that belongs to no language and no author, and written out of the meta-world by the **standard encoder** of the target language with the target author's **delta**. The standard stack of that language then takes it down to the text. The test is byte for byte. Whole blocks of verse-aligned Bibles must come back exactly: runs of chapters up to 2 500 words, with their layout (`blocks.rs`). What that takes, the residual, is measured in bits per block, per 1 000 words and per verse, by stage and by class. No large model translates or teaches here: every table is counted from the texts, and every choice says why.

## How it works

```text
source text ─► tokens ─► UD tree ─► frames ─► world            (the standard stack of the source: tools/roundtrip)
                                      │
                         reader into the meta-world             (the interlingua lexicon; for Hebrew, the Hebrew reader)
                                      ▼
                                 meta-world                     (concepts, coarse grammar, tree: no author, no language)
                                      │
               standard encoder of the target language + the author's delta
                                      ▼
target words ─► UD tree ─► tokens ─► text                       (the standard stack of the target, with the encoder's
                                                                 order and forms as hints)
```

### The meta-world

The meta-world of a verse is a graph of meta tokens. Each token has a concept, the coarse grammar the standard parse gives it (PTB tag), its head and its relation. With `--meta-rel coarse` the relation is a frame role (subject, object, oblique, a noun's modifier, a verb's modifier, a clause link, a function word, punctuation, root), and a punctuation mark hangs on the token before it. With `--clauses flat` a sentence's root and every coordinated or paratactic predicate hang on the top of the verse, so how an author joins clauses («…, and he said» or «…. He said») is not in the meta-world's tree.

**Function words between two authors** (`fwt.rs`, `--fwt on`) are the one part learned per author pair. For each trained pair, a small table holds the function tokens of the source the target drops in a context, and the runs of function tokens it inserts between two source tokens. A rule is kept with at least 3 cases and half of its context's. The table is applied to the source's meta-world before the target's encoder. Each token also keeps the source words it was read from, so an encoder can learn how an author writes it. What belongs to one author is left out:
- the spelling of a concept: *thou*, *ye* and *you* are one concept;
- the construction an author prefers: *it came to pass* and *it happened* are one concept, `come_to_pass`;
- the capital at the start of a sentence.

**The interlingua lexicon** (`meta.rs`) is learned once, for all authors, on the training verses of pairs of authors. The pairs are around two hubs: JPS–KJV, KJV–WEB and KJV–ASV, and with `--hub2 on` also JPS–ASV and ASV–WEB. JPS↔WEB is never paired. For each pair:
1. Each verse is read as a sequence of reading keys: the lemma in lower case; a punctuation mark as written; LORD and GOD (the divine name in capitals) in capitals.
2. The longest common subsequence of the two key sequences anchors the verse. The gaps between anchors are the places where the two authors differ.
3. Gap pairs of one to four keys on each side give word and phrase pairs. A word next to punctuation does not form a pair.
4. A pair is a candidate when it holds for at least 3 occurrences, 20% of the word's differing places and 2% of all its occurrences.
5. **Each candidate is judged by the residual it moves** (`--lexicon judged`, the default). What a merge saves the reader is measured (`--judge measured`): the reader's bits on up to 12 sampled training verse pairs where the two differ, with the merge and without it, times the merge's count (without the flag: an assumed 17 bits a word for every differing place it makes equal). With `--judge-pairs on`, a merge must also not cost any author pair more than it saves that pair's reader. A pair's cost counts its two authors' occurrences whose member the author and the tag no longer tell. It costs the encoder 17 bits a word for every occurrence, in all the judged corpora, whose member its context no longer tells: author, tag and relation for a word, author alone for a phrase. Candidates go in order of their counts, and a cluster's cost is recomputed as it grows. A merge is kept when it saves at least 51 bits more than it costs. `--lexicon mutual` is the alternative rule: each must be the other's most frequent partner, with 5 occurrences, 30% and 5%.

The concept is named after the most frequent member. The reader reads a verse with the longest known phrase first, then the word. A phrase's tree position is that of its word whose head lies outside it.

**The Hebrew reader** (`hebrew.rs`) is also learned, not written. It reads the Leningrad Codex as MACULA's UD trees.
1. It learns on two kinds of training verse pairs:
   - Hebrew with the meta-world read from JPS 1917, cut at the Hebrew verse numbers. Meta tokens align to Hebrew words by IBM Model 1. MACULA's English glosses act as a prior: 2 pseudo-counts per gloss word in each of 5 iterations. A place prior favours words at the same relative position.
   - Hebrew with the meta-world read from the KJV, where its verse numbers are the Hebrew ones (`--strong <eng-kjv>`). eBible's KJV writes each translated word with the Strong's number of the Hebrew word it renders, and MACULA writes the same numbers on the Hebrew words (`strong.rs`). A meta token with a number aligns to the Hebrew word with that number; the rest align by IBM Model 1.
2. The meta tokens aligned to a Hebrew word, and the unaligned tokens after them, form that word's chunk: וַיְהִי reads as `it come_to_pass`, and אֵת reads as nothing.
3. A Hebrew word reads as its most frequent chunk, in the most specific context seen at least twice: lemma, part of speech, verb type, person and number; then lemma, part of speech and verb type; then the lemma alone.
4. A lemma never seen reads as MACULA's gloss through the lexicon.
5. Chunk heads hang on the chunk of their Hebrew head.
6. With `--heb-grammar on` (`grammar_plan`), the reader also writes what Hebrew says by structure and English by words:
   - an *of* before a construct dependent;
   - a *the* before a construct head whose chain is definite;
   - each possessive suffix as a possessive before its noun;
   - a subject that follows its verb, moved before the verb.

The Hebrew world level uses MACULA's verbs and their frame arguments (`Frame=A0/A1/A2`), and the negation of `macula prag`.

### The standard encoder and the authors

The **standard encoder** (`profile.rs`) is learned on all the English we have:
- the training books of the four Bibles;
- a fifth of the school's training lessons, in three registers;
- the tales of the 0.3 treebank;
- the Talmud's English (Guggenheimer, CC BY).

Each text is read into the meta-world, and each meta token is seen next to the chunk of words written for it. The sentence capital is taken off, because it is the stack's rule. To encode a token, the standard takes the most specific context that was seen often enough and writes its most frequent chunk. The contexts, from most to least specific, are:
1. concept, tag and both neighbours;
2. concept, tag and relation;
3. concept and tag;
4. the concept alone.

A concept never seen gets its bare form: the concept inflected by `morph` from its tag.

An **author is a delta** over the standard: a sparse set of overrides, learned only from what the standard gets wrong on that author's training text. The overrides are learned from the coarsest context to the finest. An override «in this context write this chunk» is kept only when both of these hold:
- it fixes at least 3 more of the author's tokens than the prediction before it;
- it holds for at least half of that context's tokens.

A finer override wins over a coarser one. Where the delta says nothing, the standard decides. «Author» can be any style. Here there are eight: JPS 1917, KJV, WEB, ASV, the school's book lines (`school-plain`: `explain`, `fact`), its conversational test questions (`school-talk`: the voice that is half slang), its formal exam questions (`school-exam`), and the tales.

### Exact mode: the split code and the gate

For each target verse, the residual is split by stage (`measure.rs`):
- **reader** (world → meta-world): the edit script that turns the meta-world read from the source into the one read from the target. It is roundtrip's tree script on meta tokens: inserts, and patches of concept, tag, head or relation. To that come the meta tokens out of place: which ones (n choose k) and where each goes. The script first takes the pairs that a longest common subsequence and same-length gaps give. This stage is zero when source and target are the same text: it is the disagreement of the interlingua.
- **encoder** (meta-world → the author's words): the tree script from the parse of what the encoder writes for the target's own meta-world to the target's standard tree. It does not depend on the source: it is the encoder's loss as an autoencoder.
- **stack**: roundtrip's tree decoder takes the target tree to tokens. It is given the encoder's word order and forms as hints (`tree::Hints`), and the residual records what it still gets wrong: order, spelling, case and spacing. Then the tokens go to the verse text: the literal gaps and the tail.

The decoder runs the stages down: meta-world, encoder, parse, script, tree decoder, text. The gate compares the meta-world, the tree, the tokens and the text on the way. The **direct code** is gated too: it sends the target from the translation's own normal-mode output, with a tree script and the stack, the way the baselines are measured.

Classes are roundtrip's, in six groups:
- **lexical**: lexical, dropped, pronoun/name, quantity, negation;
- **construction**: attachment, tense/modality, sentence, links, quote, repetition;
- **order**;
- **function** words;
- **punct/case**: punct, case, spacing, typography, layout;
- **morphology**: form, lemma→form, source spelling.

Bits follow roundtrip's fixed rules: a dictionary lemma is 17 bits, a function word 8, a mark 5, a position log2(n+1).

### Normal mode

In normal mode the encoder with the target author's delta writes the source's meta-world, with no residual. The output is scored against the target verse by exact match, chrF, and roundtrip's divergence classes on the parsed output.

## Data

- **Bibles**: eBible's public-domain USFM, cut by `school-sents usfm` into verse pools: JPS 1917 (`engjps`), KJV (`eng-kjv`), WEB (`eng-web`), ASV (`eng-asv`), the 39 books of the Hebrew Bible. JPS is cut both at the English verse numbers and at the Hebrew ones (`--hebrew-verses`). The KJV is public domain everywhere but in the United Kingdom, where it stays under the Crown's perpetual letters patent. It is used here for measurement and for tables, and not shipped as text.
- **Hebrew**: `corpora/judaism/text/tanakh-wlc-macula` (MACULA Hebrew, CC BY 4.0) and `parsedb-hbo/prag.tsv`. Verses line up at the Hebrew numbers with JPS cut that way. For `wlc → kjv` and `wlc → web`, only the verses whose English and Hebrew numbers agree are used: those where the two JPS cuts give the same text.
- **Other English**: the school (`$MOVA_DATA/school/courses`, our own; courses of other languages' grammars left out), the 0.3 tales treebank's text (Project Gutenberg, public domain), the Talmud's English pool `talmud.tsv` written by the Talmud run of `cycle` (CC BY, for the standard only).
- **Split by book**, the same for every Bible. The test books are those at places 3 and 8 of every ten in the English order: NUM, 1SA, 2CH, PSA, JER, JOL, NAM, MAL. The dev books are at place 5: JOS, NEH, EZK, ZEP. The rest train. School lessons and tale books split the same way by their index. Of the school's held-out lessons, one in five is measured (`--sample N`, default 5).
- **Strong-tagged USFM** (KJV, WEB, ASV): `school-sents usfm` takes character marks off the verse as a whole, so the punctuation after `\w …\w*` is kept and `“\w Let` stays one word. The JPS pools do not depend on it.

## How to run

```sh
cargo build --release -p school-sents -p translate -p jobs
E=$RESEARCH_RAW/scripture/4-translations/ebible
P=$MOVA_DATA/runs/translate-<date>/pools
OT=GEN,EXO,LEV,NUM,DEU,JOS,JDG,RUT,1SA,2SA,1KI,2KI,1CH,2CH,EZR,NEH,EST,JOB,PSA,PRO,ECC,SNG,ISA,JER,LAM,EZK,DAN,HOS,JOL,AMO,OBA,JON,MIC,NAM,HAB,ZEP,HAG,ZEC,MAL
for a in jps:engjps kjv:eng-kjv web:eng-web asv:eng-asv; do
  target/release/school-sents usfm $E/${a#*:} $P/${a%%:*}.tsv ${a%%:*} $OT; done
target/release/school-sents usfm $E/engjps $P/jps-heb.tsv jps $OT --hebrew-verses
J=$MOVA_BIG/corpora/judaism
target/release/jobs run --name translate --mem-gb 14 -- target/release/translate eval \
  --pools $P --wlc $J/text/tanakh-wlc-macula --prag $J/parsedb-hbo/prag.tsv \
  --school $MOVA_DATA/school/courses --tales <corpus-0.3>/treebanks/mova-tales-silver-en.conllu \
  --talmud <talmud pool>/talmud.tsv \
  --strong $RESEARCH_RAW/scripture/4-translations/ebible/eng-kjv \
  --meta-rel coarse --judge measured --lb on --judge-pairs on --hub2 on --clauses flat --fwt on --heb-grammar on \
  --own-words 0.8 --typo on \
  --usfm $RESEARCH_RAW/scripture/4-translations/ebible \
  --out $MOVA_DATA/runs/translate-<date>/run1 --split dev --threads 10
# alternatives, off by default: --lexicon mutual, --feats on, --meta-tree frames, --fw on, --clauses on|joined, --lattice on, --std-vote corpus|context, --reorder on
# --sample N (default 5): one in N of the school's held-out lessons is measured
cargo test --release -p translate
```

The flags above are the current system. A run writes `report.md` (every table of the run), `examples.md` (verses with the reason of every choice), `numbers.tsv`, `lexicon.tsv` (each concept's members and the pair that linked them) and `deltas.tsv` (every override with its counts). Two runs of the same flags give byte-identical files. The test split runs with `--split test`, once per release; decisions are made on dev.

Each run reports normal mode (chrF and exact match per block and per verse), exact mode (bits per block, per 1 000 words and per verse, split and direct code), the residual by class, the reader stage alone, world-level agreement, and each author's delta on its held-out text. Baselines are measured beside every system: copying the source, a word-by-word table trained on the pair, and, from Hebrew, MACULA's glosses word by word. The dev books cut into blocks by `roundtrip blocks`' rule (`roundtrip::blocks::group`); this crate's own maker and `roundtrip::blocks::jps` must give the same runs, which every run checks.

### Options

- **The author's own words** (`Delta::with_own_words`, `--own-words 0.8`). For every concept and tag the author writes with one chunk in at least that share of its places, the delta keeps that chunk as an override, even where the standard already writes it in the author's own contexts, since another author's neighbours lead the standard elsewhere. The function-word tables also swap the concepts the target never writes for what it writes there (`FwTable::subs`): *unto* → *to*, *thee* → *you*, for trained pairs only.
- **Typography** (`profile::Typo`, `--typo on`). Each author's quotation marks and possessive, learned from its training text (WEB “ ” and *Moses’*, JPS ' ' and *Moses'*, KJV and ASV no quotes). The encoder writes them, and the normal-mode text joins them without the tokenizer's spaces («Moses’ minister», «“Take off …”»).
- **The Hebrew reader's grammar** (`hebrew::grammar_plan`, `--heb-grammar on`). MACULA's trees say by structure what English says by words. The reader adds:
  - an *of* before a construct dependent (`compound:smixut`);
  - a *the* before a construct head whose chain is definite (it ends in a name, an article or a possessive suffix);
  - a suffix pronoun (`nmod:poss`) as a possessive before its noun, by person, number and gender (*my, thy, his, her, our, your, their*);
  - a subject that follows its verb moved before the verb, after the verb's conjunction.

  An *of* or *the* the chunk already writes is not doubled, and the mark the first *and* brings at a verse's start goes. Hebrew idioms (`hebrew::idioms`): וַיְהִי with no subject of its own is *it came to pass*; עָשָׂה with כֵּן is *did so*; an article goes after a chunk's own leading *of*.
- **Modals on their own.** The interlingua lexicon takes no phrase that holds a modal (*may*, *shall*, *will* …), so a modal is never glued to a concept («may Yahweh»). A test in `meta.rs` checks the rule (`MODALS`, `side_ok`).
- **The 95% lower bound** (`--lb on`): the judge keeps a merge on the lower bound of its measured saving.
- **Alternatives, off by default:**
  - `--lexicon mutual`: the mutual-partner lexicon rule;
  - `--feats on`: pronoun number and person on meta tokens;
  - `--meta-tree frames`: the meta-world's tree through the frames' anchors;
  - `--fw on`: determiners and prepositions as features of meta tokens;
  - `--clauses on`: joiners on the clause's first word, clauses side by side; `--clauses joined`: joining as a choice conditioned on the source's joining and the clause relation (`fwt::JoinTable`);
  - `--lattice on`: the standard learns on the four renderings' shared backbone;
  - `--std-vote corpus|context`: the standard weighted one vote per corpus, or one vote per corpus in every context (`Standard::train_voted`);
  - `--reorder on` (`fwt::ReorderTable`): for each target author, a table of where a source author's clause part (subject, object, oblique, modifier) stands against its predicate and where the target puts it, learned on the trained pairs, with parts aligned by the concepts each verse has once.

### Controls

Every run measures its negative controls beside the systems: the gate control (a corrupted residual must fail), verses shuffled inside each block, the Hebrew reader learned on shuffled verse pairs, the wrong author's delta, a delta trained on shuffled units, the null delta, the lexicon learned on shuffled verse pairs, and the function-word, joining and reorder tables learned on shuffled verses.

### Limits of normal mode

1. **Quotes across verses.** The quote state is kept per verse, not per block, so a quote that opens in one verse and closes in another comes out the wrong way round.
2. **Swaps for pairs never trained.** The swap tables exist only for trained pairs; they are not composed through the hub (JPS→KJV→WEB), so JPS→WEB keeps *unto*, *thy* and *thou*.
3. **Own words do not yield to a finer context** the author has seen (KJV's *stand* where it writes *standest* after *thou*), and the share is flat.
4. **The Hebrew clause order inside quotes and objects**, and *the of*: an inserted article can still meet a chunk's *of*.
5. **The one-reader block code** has no position code that names the verse first.

## The parallel Tanakh as site pages, both ways (`--pages`)

The Tanakh corpus pages of the site (`corpora/<part>/<BOOK>.<c>.<v>/`: a verse in Hebrew with its MACULA tree, its pragmatics, its Targum and links to its English; `corpora/<part>/jps/<BOOK>.<n>/`: a JPS sentence with its tree and frames) are translated as page blocks, Hebrew pages → meta-world → English pages and English pages → meta-world → Hebrew pages (`pages.rs`). A chapter is its verse pages on one side and the sentence pages that begin in it on the other; the blocks are the JPS's chapter runs of `roundtrip blocks` (`blocks::jps`, `blocks::group`, 2,500 words), the other side cut the same (`blocks::like`).

A target page block costs **the translation** of its texts — each Hebrew verse, or each English unit (the JPS sentences that begin in a verse), with the split code (reader, encoder, stack) — and **the page** around them: the target pages through `roundtrip blocks`' site decoder (`roundtrip::blocks::site_md`) with the translated texts and the source's own texts given (`site_md::give`, class `given`), so the markup, the other prose, the trees and tables it writes from the layers, and the Targum are what is left.

**English → Hebrew** (`heb_enc.rs`). The **Hebrew encoder** is the Hebrew reader turned round, learned on the same training verse pairs: the reader's alignment gives each meta token to a Hebrew word (its chunk); the first token of a word's chunk emits the word (lemma, UPOS, XPOS, FEATS, relation), the others nothing; to encode, each meta token emits what it emitted most often in the most specific context seen twice (`profile::ctx_keys`), and a word hangs on the word its meta head's chain emitted first. The split code onto a Hebrew verse: *reader* — the script from the meta-world read from the English to the one the Hebrew reader reads from the target verse; *encoder* — the Hebrew tree script from what the encoder writes for the target's own meta-world to MACULA's words (`roundtrip::mixed::heb::frames_encode`), with the words out of place; *stack* — the words to their forms by the lookup generator of the training books' trees, a form looked up with the word before it in its written word (`Lookup::from_conllu_ctx`, `--heb-forms ctx`: the dagesh after וַ, the article's vowel), and the written words to the verse (`mixed::heb::tree_encode`, `tokens_up`); a verse whose written words are not its tree's (a paragraph mark glued to a word) stores its text. The gate decodes the meta-world, the words, the written words and the verse. A Hebrew encoder learned on shuffled pairs is the control.

### Run

```sh
roundtrip site-learn --site-md $S --site-target 1000 --out $L --model <en.bin> --lin-every 2   # tools/roundtrip README
jobs run --name translate --mem-gb 16 -- translate eval … (the current system's flags) \
  --pages $S --site-dict $L/markup.tsv --site-heb $L/hebrew.tsv --site-english $L/english.tsv --heb-forms ctx \
  --model $L/en-lin.bin --out <dir> --split dev
```

`--site-english` adds the site's train pages' English (`english.tsv`; the Tanakh's held-out books left out) to the standard encoder's training text. The adapted order (`en-lin.bin`) changes no translation stage (the encoder's order is the stack's hint); it applies to the pages' prose. The run appends «The parallel Tanakh as pages, both ways» to `report.md` and writes `pages.tsv` (a row per block and direction: verses, words, gate, the bits of each stage and of the page). The test split needs a higher `--mem-gb` than dev while it writes the pages.

**Limits.** English → Hebrew has no Hebrew generator of pointing from lemma, binyan and prefix, and no model of the accents as a layer of the verse's syntax. The Hebrew encoder emits no word for what no meta token stands for (אֵת, prefixes, suffix pronouns). The page's template is not derived from the page id.

## Many translations, each in its own world (`translate many`)

`translate many` (`many.rs`, `logic.rs`) takes every English translation eBible has, gives each its own world, and links the worlds through the meta-world. It is the same design as above, scaled from four authors to every English edition, and measured block by block.

### How it works

- **Pools.** One `school-sents usfm` cut per eBible folder, all books (`-`). The 66 books of the Protestant canon and the deuterocanon (`bible::DC`: Tobit, Judith, Greek Esther, Wisdom, Sirach, Baruch, the Letter of Jeremiah, the additions to Daniel, 1–4 Maccabees, 1–2 Esdras, the Prayer of Manasseh, Psalm 151, Greek Daniel) are read; front matter is not. The New Testament and the deuterocanon have book indices after the Hebrew Bible's, so `split_of` splits them by the same rule: dev 1CO, TIT, REV, BEL; test ACT, EPH, 1TI, JAS, 3JN, ESG, S3Y, 3MA, PS2.
- **Versification.** Verses line up by their English number. A chapter whose numbers are not the reference's (eng-kjv; for a book the KJV does not have, the first translation by name that has it) is left out for that translation: on average its verses must share more words of four letters or more with the reference's verse of the same number than with the neighbours. The Septuagint's and the Vulgate's Psalms, the LXX's Jeremiah and the Hebrew-numbered chapters of the Orthodox Jewish Bible fall out there.
- **Blocks.** The unit is a block: whole chapters up to 2 500 words, cut on the reference by `roundtrip blocks`' rule (`blocks::make_rt`), the same blocks for every translation. Each translation reads each block as one discourse: the frames' context goes through the block. Training uses one training block in four (`--train-every 4`); all dev blocks are measured. The test books are not read.
- **Worlds and the link.** A translation's own world `W_i` is its verses read with no lexicon: each word its own concept, the coarse relations, the clauses side by side. The lexicon reads it into the meta-world `M_i`. A translation goes `W_i → M_i → M_j → W_j → j`. Measured per block, the verses' figures summed (verse-anchored):
  - **up** and **down**: the tree script from `W_i` to `M_i` (what the lexicon changes of a world) and from `M_j` back to `W_j` (what the meta-world does not hold of a world, which the delta and the encoder pay);
  - **agreement**: the share of meta concepts and of (head concept, relation, concept) edges two translations share in a verse, punctuation left out;
  - **cross residual** `i → j`: the reader script `M_i → M_j` plus `j`'s encoder and stack. The encoder and stack stages do not depend on the source, so they are `j`'s autoencoder tail, computed once. Every translation into the reference and back, every third verse, runs whole through `Run::verse` with the gate, which checks the sum.
- **Logical closeness** (`logic.rs`). A verse's facts come from its frames: each event's verb, negation, modal and roles (agent, patient, recipient: the entity, and whether it is a name, a common noun or a pronoun), and each sentence's numbers by value (*threescore* = *sixty*). Two readings are compared through the lexicon. Events are matched along the longest common subsequence of their verbs (the order). A **contradiction** is a matched event whose negation differs, whose modals both stand and differ, or whose role is filled by two different names; or numbers that differ. A **reversal** is one entity (a name or a noun) that is the agent on one side and the patient on the other, in an event both sides read as transitive. An event one side has and the other lacks is a gap. The **logical distance** is the contradictions plus half the gaps. Both are per 1 000 words, apart from the bits. The order of events is reported, not counted in the distance: translations reorder clauses freely.
- **The sensitivity control** (`logic::Sens`). Into every target verse of every ordered pair, one change of meaning is put on an event the source verse matches, one kind at a time (negation flipped, agent and patient swapped, a number changed, two events reordered), and the comparison is made again: a distance of 0 between two editions means something only if the distance moves when the meaning does.
- **«No match».** The aligner never forces a match. It refuses a pair of verses of the same number whose script would cost more than writing the target verse from nothing; that verse is written from nothing (one bit for the mark) and counted. A verse one side lacks is marked by level: **canonical** if the book is not in that translation, or the verse is missing in a chapter it has; **structural** if the chapter is numbered another way, or the translation joins verses (`\v 1-2`). Inside a matched verse, a target meta token with nothing in the source behind it is a lexical mark (a function word) or a semantic one (a content word). Whole blocks one side lacks count as canonical marks and bits; they stay out of the cross residual.
- **Levels.** The reader script's bits are split by roundtrip's classes into four levels:
  - **lexical**: other words for one concept (lexical, function words, forms, spelling, case, pronoun or name);
  - **semantic**: another reading of an event or its roles (negation, quantity, tense and modality, attachment, words dropped, links, quotes);
  - **structural**: numbering, order and how the text is divided (order, sentences, punctuation, layout);
  - **canonical**: the verses and books one tradition has and the other lacks.
- **Training.** The baseline is a `translate eval` run's lexicon as it is (`--lexicon-from`, the four authors'). The many-pairs lexicon is learned by `meta::learn_gated` with the judged rule, the judge per pair and the 95% lower bound, on pairs of each translation with the two hubs (KJV, WEB) and with its nearest by the baseline's concept agreement on the training blocks. Every fifth measured translation by name that is not a hub or one of the four is held out of it: no pair, and not in the judge's corpora. The lexicon takes words only (`--max-len 1`) and is added to the baseline (`--extend on`): a learned cluster that touches none of the baseline's members joins it. Phrases come from the four authors' lexicon only. The standard is learned on all the translations' training blocks. Each translation has its delta (own words at 0.8) and its typography.
- **The logic gate.** A merge is refused when:
  - its two members differ in negation, modality or number (*can* ~ *be*, *should* ~ *to*);
  - or it raises the contradictions of its sampled verse pairs (`meta::logic_rise`).

  An override of a delta is taken out when its chunk has other negations, modals or numbers than its concept (`Delta::logic_gate`). A phrase pair whose sides share a word is no candidate (`LexParams::distinct`). *LORD say* ~ *lord seide* is *say* ~ *seide*, which the single words give.
- **Storage.** The training blocks are parsed once and kept compact: the trees and the facts. The dev blocks are streamed: parsed for every translation, measured, dropped. The lexicon, the standard and the deltas are the only state kept across blocks. A block's meta-world is stored once for all the translations: concepts by compact ids (`Interner`), each concept and edge with a bit mask of the translations that hold it.
- **Traditions** (`many::tradition`, by eBible's description of each edition):
  - Jewish: JPS 1917, Leeser;
  - Catholic: Douay-Rheims, the WEB's Catholic edition;
  - Orthodox: Brenton, the two LXX2012s, LXX Updated;
  - Protestant: the rest. The Messianic editions (WMB, OJB) keep the Protestant canon.

### Data

All `eng*` folders of `$RESEARCH_RAW/scripture/4-translations/ebible`. The library packs every USFM book of 64 KiB or more as seekable zstd; `school-sents usfm`, `blocks::read_layout` and `strong::read` read `.usfm.zst` as well as `.usfm`, and the pools are the same bytes either way.

- **Measured:** translations with at least 100 verses in the dev blocks.
- **Learn only:** engWycliffe (Pentateuch and Gospels), engoke (Pentateuch) and engourb (some OT and Gospel books) have no dev book.
- **Skipped:** enge2t has Jonah only.
- **Licences:** most are public domain; engojb and engtcent are CC BY 4.0; enge2t, engf35, engfbv, englsv, engourb, engPEV, eng-t4t and engULB are CC BY-SA 4.0: learned from and measured only. The lexicon and the tables of a run stay in `$MOVA_DATA/runs` and are not shipped.

### How to run

```sh
cargo build --release -p school-sents -p translate -p jobs
E=$RESEARCH_RAW/scripture/4-translations/ebible
P=$MOVA_DATA/runs/bible-many/pools
for d in $E/eng*; do target/release/school-sents usfm $d $P/$(basename $d).tsv $(basename $d) -; done
target/release/jobs run --name bible-many --mem-gb 8 -- target/release/translate many --pools $P \
  --lexicon-from <a translate eval run>/lexicon.tsv --ebible $E --out $MOVA_DATA/runs/bible-many/<run> \
  --threads 10 --train-every 4 --max-len 1 --extend on [--shuffled-lexicon on]
# the meta-world of the whole canon outside the test books (size and memory only):
target/release/translate many --pools $P --lexicon-from <run>/lexicon.tsv --out <dir> --threads 10 --canon on
```

Other options of `translate many` (defaults in brackets): `--near N` (3) pairs each learning translation with the hubs and its N nearest by the baseline's concept agreement on the training blocks; `--hubs a,b` (eng-kjv,eng-web) names the hubs; `--hold-every N` (5) holds every N-th measured translation (not a hub) out of the lexicon (0: none); `--reference NAME` (eng-kjv) is the versification every other translation is aligned to; `--max-words N` (2500) caps a block; `--learn on|off` (on) learns the lexicon from the pairs; `--distinct on|off` (on) refuses a learned pair whose two sides share a word; `--parallel-only on` (off) measures only on the Tanakh's dev blocks and only translations that have all of them, on the verses every measured translation has; `--rules on` writes the residual rules below; `--rules-limit N` (0: all) takes only the first N ordered pairs of `--rules on`, for timing; `--model <en.bin>` overrides the default model. `--shuffled-lexicon on` adds the lexicon learned on shuffled verse pairs as a control; the verses shuffled inside each block are the other control.

A run writes:
- `report.md`: every table of the run (between worlds, «no match», links and size, between traditions, the sensitivity control), the canons, per translation, per block, the reasons of contradictions;
- `pairs.tsv`: every ordered pair, every lexicon;
- `worlds.tsv`: each translation's links and autoencoder;
- `lexicon.tsv`, `refused-merges.txt`, `refused-overrides.txt`;
- the all-to-all square: `squares/<lexicon>-<measure>.tsv` (cross, distance, concepts, the four levels, «no match» by level), `square.md` (cross / distance), `square.html` (a heat map). Rows and columns go by tradition, then by average-linkage clustering (`many::upgma`).

`translate many` does not change `translate eval`: the same `eval` flags give the same files with and without the `many` code.

### Residual rules (`--rules on`)

For every ordered pair, `rules.rs` turns the reader's script from the source's meta-world to the target's into candidate rules:
- a concept for a concept;
- a tag for a tag;
- a concept dropped.

Each rule is learned on one half of the dev blocks (by the block's parity) and measured on the other. It is measured again with the verses paired by the wrong number inside the block (the control). The rules are then joined greedily, each next one measured on top of those before it. This is transformation-based learning (Brill 1995) on meta-worlds. `rules.tsv` has each pair's list; `rules.md` has the rules that gain most over all the pairs and, per pair, the share of the reader's bits they remove.

### Limits of `translate many`

- **The judge measures a merge only where its two members differ.** It does not see a phrase concept breaking the anchors elsewhere: a phrase read as one concept in one translation breaks the word-by-word anchors it has with the others. Phrases are therefore taken from the four authors' lexicon only.
- **The logical facts are as good as the frames.**
  - A capitalised *And* at a sentence's start parsed as a name gives contradictions such as «agent *joshua* against *and*».
  - Negations that move between clauses count.
  - Two different common nouns in one role are not a contradiction (another word for one thing is as likely); a common noun moving between agent and patient is.
  - The logic gate refuses on any rise in its samples, so a few spelling merges (*labor* ~ *labour*) are refused for one noisy contradiction.
  - Contradictions alone do not separate true pairs from shuffled ones: two different verses share few events, so few events can contradict; the distance counts the gaps.
- **Versification is decided per chapter, against the KJV's numbers, and dropped, not remapped.** The LXX's and the Vulgate's Psalms are left out, not renumbered. The STEPBible TVTMS tables (in the library, `5-greek-latin/versification`) would map them.
- **Blocks the source lacks are counted at the canonical level, not in the cross residual.** A verse missing within a chapter is canonical unless the translation joins verses there. Verses a textual tradition omits and verses an edition renumbers are told apart only that far.
- **The pairs** are each translation with two hubs and its nearest. The held-out translations have no pair; they still have their own delta and are in the standard.
- **One training block in four** is read, to keep memory bounded with every English edition.
- **The parse of old translations** is weaker than of modern ones (`parsecmp agree` against Stanza and UDPipe), so part of an old translation's distance is the reader's.

### Greek, Latin, Hebrew: sources and gaps

A non-English text plugs in the way the Hebrew one does (`hebrew.rs`): its own world, from its own parse, and a reader learned into the same meta-world on verse pairs, aligned by IBM Model 1 and, where a text has them, by Strong's numbers. In the library:
- **Ancient Hebrew:** the Westminster Leningrad Codex as MACULA's trees (`corpora/judaism/text/tanakh-wlc-macula`, already read by `--wlc`). Also OSHB, unfoldingWord's UHB and eBible's hboWLC.
- **Koine Greek, the Septuagint:** `5-greek-latin/` has:
  - eBible's grcbrent (pure LXX numbering) and grclxx (mixed numbering);
  - Swete (First1KGreek, CC BY-SA);
  - CrossWire's Rahlfs with morphology (NC: tests only).
- **Koine Greek, the New Testament:** SBLGNT (CC BY 4.0), Robinson–Pierpont, Tischendorf, Nestle 1904.
- **Latin:** the Clementine Vulgate (eBible latVUC, CrossWire VulgClementine, both PD).
- **Missing: a parser for Greek and Latin.**
  - `en` trains on UD treebanks with PTB tags (`xpos-fill` makes them from UPOS and FEATS). UD's Ancient Greek and Latin PROIEL treebanks hold the Greek NT and the Vulgate, but they are CC BY-NC-SA: tests only.
  - An open path is Opera Graeca Adnotata (CC BY-SA, so learning only), PTNK for the Septuagint (CC BY-SA), LLCT for Latin (CC BY-SA), or a parse projected from the English through the verse alignment.
  - Each needs the versification tables first: the LXX's and the Vulgate's chapters differ from the English in the Psalms, Jeremiah and Daniel.

## Limits

- The meta-world is a graph of concepts read from the parse, not a world of events. The generation path goes meta-world → words → parse. It does not go through roundtrip's frames and world, whose realizer is lossy (tools/roundtrip). The world level is measured as agreement only.
- The lexicon is learned on five pairs around KJV and ASV. «Never trained» (JPS↔WEB) means that no pair table and no lexicon pair joined those two authors directly; their words still meet through the hubs.
- Strong's numbers are used from the KJV only (WEB's and ASV's are projected and noisy). KJV text is used here for measurement and tables only; the Crown's rights in the United Kingdom are noted above.
- Bits are roundtrip's fixed-rule estimates, not an entropy code. The deltas' bits use the same rules.
- The school's held-out sample is one lesson in five of every tenth lesson index. The tales have 41 books, so their dev set is a few books.
- A block's layout is the marks between verses; a poetry line that goes on inside a verse (`\q2` with text) is not kept, the verse text being the pools' joined text. Blocks are cut by `roundtrip blocks`' rule over the pools' chapters; the block text with its USFM layout is this crate's (`blocks.rs`), since `roundtrip blocks`' JPS text has no verse lines.
- `wlc → kjv` and `wlc → web` use only the verses whose English and Hebrew numbers agree.
- The page blocks pair a Hebrew verse with the JPS sentences that begin in it; a sentence that runs into the next verse is counted with the verse it begins in.
- The Hebrew encoder writes words in the meta-world's (English) order and the order class pays for Hebrew's; it has no Hebrew word of its own for what English leaves unsaid.
