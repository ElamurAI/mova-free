# The English layer

This page covers how Mova reads and writes English. It walks through the `en` crate step by step: the dictionary in code, the tokenizer, the tagger, morphology, the parser, the graph it stores and the generator that writes text back. It then describes the expert system of seeds and rules that checks the annotator, the deriver that builds trees by explicit rules, and the round trip (the autoencoder) that sends a text up through every layer and back down to show what each layer keeps and loses. The crate reference is [en/README.md](../en/README.md); the full round-trip reference is [tools/roundtrip/README.md](../tools/roundtrip/README.md).

## What it is

- **Text becomes a graph.** Words become numbered nodes; the relations between them ("who", "does what", "with what") become edges.
- **A generator writes text back from the graph.**
- **No neural networks in `en`.** It is built from tables trained on annotated trees and automata that walk those tables. Training penalizes a table when it makes a mistake, as with a neural net, but every weight sits in a table you can read, and the result is deterministic and fast on one CPU core. The GPU parser [`gparse`](../gparse/README.md) is a separate component.

The distributed model `en.bin` is trained on UD English treebanks plus the agreed school trees ([training.md](training.md)).

Every step below is real output of `en explain "<sentence>"`; run it on any sentence. The running example is **"Should I bring an umbrella tomorrow?"**

## How the compressor works, step by step

### Step 0. The dictionary in code

- **Lemma heap.** Every lemma has a frozen number: `the` = 0, `.` = 1, `be` = 2, `,` = 3 … `tomorrow` = 755. A number never changes; new words are appended at the end, so a graph written today stays readable later.
- **Forms.** Each inflected form (went, gone, going, goes) maps to lemma + class + frequency. `build.rs` compiles this table at build time; nothing is built at run time.
- **Constants.** Words that rules rely on are constants in code: `w::THE`, `w::QMARK`, `w::CELSIUS`. A rule such as "is the unit Celsius?" is `lemma == w::CELSIUS`, a comparison of two integers.

- **Lexicon extension.** Words the dictionary lacks (*grammatical, stochastic, vertices, GPU, Kerala*) come from a second compiled table made from Open English WordNet, the SPECIALIST Lexicon and public name lists, each row with its source. It answers only for forms the dictionary does not know, so lemma numbers and trained models are untouched; the taggers read it for unknown words by how the word is written (an acronym only in capitals). Details are in [en/README.md](../en/README.md).

Sources: AGID paradigms (permissive licence) and frequencies from UD; the extension from OEWN (CC BY 4.0), SPECIALIST (NLM terms), the 2010 U.S. Census surnames and the Names Corpus.

### Step 1. Tokenizer: a finite automaton

Every character has a class: Le (letter), Di (digit), Sp (space), Pe (period), Ap (apostrophe)… A table **state × class → (command, new state)** drives the automaton, one lookup per character.

```
"S" Le New St→Wo · "h" Le App Wo→Wo · … · " " Sp Sep Wo→St · "I" Le New St→Wo …
```

`New` starts a word, `App` appends a character, `Sep` closes the word; `St` is "between words", `Wo` "inside a word". Result: `Should │ I │ bring │ an │ umbrella │ tomorrow │ ?`

Dictionaries from the treebanks handle abbreviations ("Mr.", "U.S."), clitics ("did|n't", "John|'s") and hyphenated words.

### Step 2. Tagger: the class of each word

Tags are Penn Treebank classes: Should MD, I PRP, bring VB, an DT, umbrella NN, tomorrow NN, ? `.`

- **Averaged perceptron** (`ptag.rs`). Features of the word and its neighbours (±2), suffixes and prefixes, word shape, the dictionary in code and the two previous tags carry weights per class; the largest sum wins.
- **Beam of 8.** The tagger keeps the eight best chains of classes.
- **Unknown words.** A TnT-style prior (`tag.rs`: a trigram chain over classes with a suffix model, "-ness" → NN, "-ed" → VBD/VBN, plus a hint from the dictionary) feeds the perceptron.

### Step 3. Morphology: lemma and case

The lemma is the dictionary form: brought, bringing, brings → `bring`. The graph stores lemma and tag; the form is rebuilt from them (bring + VBD → brought).

- **Capitalizing the first word** is a property of the position, stored as a separate bit. After reordering, the generator writes "I should bring", not "I Should bring".
- **"an"** is stored as the lemma `a`; the form comes from a rule on the next word.
- **Escape.** If the form cannot be rebuilt from lemma and tag (a rare spelling), the form itself is stored. That is why lossless mode always returns the original.

### Step 4. Parser: a stack automaton

The parser has a **stack** (words waiting for a head) and a **buffer** (words not yet read). At each step a weight table picks SHIFT, LEFT(relation) or RIGHT(relation).

```
 1. stack [ROOT]                          · buffer [Should I bring an umbrella tomorrow ?] ⇒ SHIFT
 2. stack [ROOT Should]                   · buffer [I bring an umbrella tomorrow ?]        ⇒ SHIFT
 3. stack [ROOT Should I]                 · buffer [bring an umbrella tomorrow ?]          ⇒ SHIFT
 4. stack [ROOT Should I bring]           · buffer [an umbrella tomorrow ?]                ⇒ LEFT(nsubj): bring → I
 5. stack [ROOT Should bring]             · buffer [an umbrella tomorrow ?]                ⇒ LEFT(aux): bring → Should
 …
10. stack [ROOT bring umbrella tomorrow]  · buffer [?]                                     ⇒ RIGHT(nmod:unmarked): umbrella → tomorrow
11. stack [ROOT bring umbrella]           · buffer [?]                                     ⇒ RIGHT(obj): bring → umbrella
 …
14. stack [ROOT bring]                    · buffer []                                      ⇒ RIGHT(root): ROOT → bring
```

Here the parser makes a real error: "tomorrow" should attach to `bring` as a time adverbial, not to `umbrella`.

Features of the current state ("top of the stack is VB, the one below is PRP", "first word of the buffer is DT", and many more templates) are numbers: template number plus word and tag numbers. Each feature carries weights for the actions; the largest sum wins. The parser keeps a beam of 8 action sequences.

**Training** is a perceptron with early update (Zhang & Clark 2008): run through the treebank knowing the correct action sequence; when it falls out of the beam, stop, add +1 to the correct actions and −1 to the chosen wrong ones. Several passes, then the weights are averaged.

### Step 5. The graph: what is stored

```
 #  lemma (number)    tag  case     relation       head    space
 1  should (96)       MD   AsIs     aux              3     yes
 2  I (7)             PRP  Title    nsubj            3     yes
 3  bring (303)       VB   AsIs     root             0     yes
 4  a (6)             DT   AsIs     det              5     yes
 5  umbrella (16799)  NN   AsIs     obj              3     yes
 6  tomorrow (755)    NN   AsIs     nmod:unmarked    5     no
 7  ? (32)            .    AsIs     punct            3     no
sentence bits: interrogative — yes; first word capitalized — yes
```

A node is 12 bytes and holds no strings: lemma number (4 bytes), tag (1, enum), case (1), relation (1, enum), head (2), space after (1). A word missing from the dictionary goes into the graph's **local heap** and gets a number with the high bit set.

### Step 6. Generator: from the graph back to text

- **Lossless.** Nodes in stored order, lemma + tag → form, plus the a/an and capitalization rules: `Should I bring an umbrella tomorrow?`. Always exact.
- **Generative.** The order is recovered from the tree alone, as when a graph has no original order:
  1. the side of each dependent relative to its head, from a backoff table (from an exact key such as "closed-class lemma + head tag + interrogative + has aux" down to the relation alone): det goes left, obj goes right;
  2. the order on each side from a Markov chain over relations, solved by dynamic programming;
  3. word forms, position rules, spaces.

  Result: `I should bring an umbrella tomorrow?` The content is in place, but the generator does not yet perform question inversion.

### What it is for

A prompt comes in; it becomes a graph; the graph becomes typed commands for Rust code; the result is a graph; the generator writes text. No large model is called on this path: an input the parser cannot read keeps its failure. A domain builds on the same parser by self-training: the general parser annotates domain sentences, the generator is trained further on them, and a table over graph features maps a query graph to a typed command (for example, place, time, condition and unit of a weather query).

## The expert system: seeds and rules

The expert system checks the annotator's output against knowledge of grammar and word structure. It learns from grammars, papers and dictionaries, not from examples. Its knowledge is written as **seeds**: one piece of knowledge each, with a source. A seed is the kernel a rule grows from.

### How it works

1. **Seeds** live in `en/seeds/<section>/<name>.md`. Sections: `morph` (inflection and word structure), `nominal` (the noun phrase), `verbal` (the verb and the clause), `lexicon` (dictionaries and valency), `errors` (typical errors of parsers, large models and learners). Indexes: [seeds/](seeds/README.md).
2. **Rules** are ```` ```rule ```` blocks in a seed, executed by a deterministic Rust engine, `en::expert`.
3. **Self-check against gold.** Every rule runs over UD EWT. A rule that fires on gold is either wrong or has found a gold error; these cases are reviewed and the rule refined. Examples only check rules; they never generate them. Exceptions come from the seed text and its sources, not from examples.
4. **Uses:** checking annotation from the teacher, from Rust and from silver; hints for the teacher in edit mode ("word 5 violates rule X"); picking sentences worth promoting to silver; hard constraints for the parser beam.

When two seeds share a rule, it lives in one seed and the other carries the line "Rule: `<id>` in `<file>`".

### Seed format

````markdown
# <short title>

**Gist.** 1–3 sentences: what the grammar says.
**Conditions and exceptions.** …
**Examples.** 2–4 short English examples, original or quotes of up to ~10 words.
**In UD.** How the phenomenon looks in UD (EWT) annotation: UPOS, XPOS, FEATS, relations.
**Sources.** Jespersen MEG VI §3.2; Santorini 1990, VBZ; UD `docs/_en/…`; `2023.udw-1.7`.

```rule
…
```
````

### Rule language

```rule
rule: en.errors.singular-subject-vbp
what: singular noun subject before a VBP verb: missing -s or wrong subject
match: v[xpos=VBP]; s[rel=nsubj|nsubj:pass, head=v, upos=NOUN|PROPN, feats.Number=Sing, before=v]
require: exists c[rel=conj, head=s]
unless: s[lemma=lot|number|majority|rest|half|percent]; s[lemma=family|team|staff|government]
severity: warn
source: P17-1074 (VERB:SVA); Fowler MEU 1926, NUMBER
```

- **`match`**: nodes separated by `;`, each `name[condition, …]` with all conditions together. The engine enumerates every binding of distinct words that satisfies them.
- **`require`**: what must hold for each binding, else it is a violation. Clauses separated by `;` must all hold; ` or ` lists alternatives within a clause. Clause forms: `name[conditions]` on a bound node; `not name[conditions]`; `exists c[conditions]` (another, unbound word exists; it may refer to `match` nodes); `none c[conditions]`.
- **`unless`**: exceptions, same clause forms; if one clause holds, there is no violation. It is checked only where `require` failed.
- **`severity`**: `error` (the annotation is wrong) or `warn` (a suspicion).
- **`source`**: mandatory.

| condition | meaning |
| --- | --- |
| `upos=NOUN`, `upos=NOUN\|PROPN` | part of speech (one of several) |
| `xpos=VBZ` | PTB tag |
| `lemma=be`, `form=that` | lemma, form (case-insensitive) |
| `feats.Number=Sing`, `feats.Number!=Plur` | feature equals / differs |
| `feats.Number`, `!feats.Number` | feature present / absent |
| `rel=nsubj:pass`, `rel~nsubj` | exact relation / relation by base (nsubj, nsubj:pass, nsubj:outer) |
| `head=v`, `head=0` | head is node v / the root |
| `before=v`, `after=v`, `next=v`, `prev=v` | word order; `next`/`prev` are immediate neighbours by CoNLL-U word number, punctuation included |
| `suffix=ness`, `prefix=un`, `lemma.suffix=y`, `lemma.prefix=un` | end / start of the form or the lemma |
| `upos!=`, `lemma!=lot\|number`, `head!=0`, `rel!~nmod` … | negation of any condition ("none of the values") |
| `feats.Person=@s`, `feats.Number!=@s` | same feature as on node `s` / different |
| `lemma=@form` | lemma equals form, case-insensitive |

Examples:
- `a[upos=ADJ, rel=amod, after=n, lemma!=involved|due|elect, lemma.suffix!=able|ible]`: a postpositive adjective, except fixed ones;
- `require: r[feats.Person=@s]; r[feats.Number=@s] or not s[feats.Number]`: a reflexive pronoun agrees with the subject;
- `unless: none c[before=v, upos!=PUNCT|CCONJ|ADV|INTJ]`: only punctuation, conjunctions, adverbs and interjections precede the verb, so the subject is omitted at the start.

Engine semantics:
- There is no "and" across nodes inside one `unless` clause. Write a needed "and" in `require` in conjunctive normal form: `require: n[feats.Number=@d] or d[feats.Number=Plur]; n[feats.Number=@d] or n[feats.Number=Ptan]`.
- `feats.X=@s` is false when `s` lacks X; `feats.X!=@s` is then true. `@` occurs only in `feats.X` and `lemma=@form`.
- `;` and ` or ` split only outside `[…]`; a comma inside a value (`PronType=Int,Rel`) is not supported.
- A feature outside the `Feat` list is always absent in `feats.X` / `!feats.X` (the CoNLL-U reader drops unknown pairs); in `feats.X=@s` it is a parse error.
- Parse errors, never silence: an unknown or repeated field, a node given twice in `match`, a reference (`head=`, `before=`, `after=`, `next=`, `prev=`, `@`) to a node not in `match`.
- No access to MISC; "has a dependent with rel=X" is written `exists c[rel=X, head=v]`.

All seeds must parse with unique rule ids: the `cargo test` gate `seeds_all_parse`.

### Running it

```sh
en expert-check en/seeds <file.conllu>                 # up to three examples per rule
en expert-check en/seeds --gold <gold.conllu> <file>   # also marks each violation true or false
EN_EXPERT_LIST=1 en expert-check …                     # plus every violation: sent_id, rule, severity, nodes, text
```

### Limits

The rules are precise but see only the ungrammatical. They catch a small part of the errors in a tree; most errors produce grammatical trees with a wrong attachment. So they serve as hints and as a filter for silver, not as full control of annotation.

## Trees by logical derivation

The deriver builds dependency trees step by step, each arc from a named rule with a reason, the way the SLM solves arithmetic. It is far less accurate than the parser; its value is that every arc can be inspected, hidden rule by rule, and improved by self-play.

- `world derive <train.conllu>... --test …` induces decision lists of attachment rules from trees ("a DET attaches to the nearest NOUN to its right as det"); the root comes from a root rule; a rule that would close a cycle is skipped. An easy-first variant (most certain steps first) is kept for comparison.
- `world derive-learn <book-list> --test …` uses the SLM parser itself as teacher: book sentences are parsed by the working pipeline, and the deriver learns Brill-style transformation rules that move its decision-list tree towards the parser's tree, with no gold. Learned rules read like grammar: NOUN before NOUN → compound; ADV before ADJ → advmod; "be" before ADJ → cop of the adjective; NOUN after "and" → conj of the previous noun.
- `world derive-scan <book-list> [--rounds R]` parses the whole book corpus once, packs it, and in each round reports what the deriver already derives (agreement by part of speech, largest disagreements) and learns several non-interfering rules in two parallel passes.

Learning is incremental (Ngai & Florian 2001): fix counts are updated only for sentences a rule touched, and an index from condition sets to words gives a candidate's breaks without a pass over the data. `DERIVE_FULL_RESCORE=1` keeps full rescoring for comparison; equivalent rules can differ on ties. Long runs go through `tools/jobs`; quick checks use a `prefix=1%` sample of the tree store. Reference: [world/README.md](../world/README.md).

## The round trip: what each layer keeps and loses

Mova reads a text up through its layers (tokens, a dependency tree, frames, a world) and answers from the world. The autoencoder (`tools/roundtrip`) sends the text back down the same layers and asks at every boundary: **what must be stored beside this layer for the text to come back byte for byte?** That store is the layer's **residual**. Its size in bits says how much of the language the layer does not describe; its classes say which part.

### The layers, up and down

| boundary | up (reader) | down (decoder) | residual stores |
|---|---|---|---|
| text ↔ tokens | the tokenizer keeps the literal before every token | tokens joined by their spacing | layout (blank lines, wraps), unusual spaces |
| tokens ↔ tree | the parser: lemma, tag, head, relation | the generator: order from the tree, forms from lemma, tag and agreement | order where the generator is wrong, forms the rules do not give, spacing |
| tree ↔ frames | the frames reader: each event with verb, roles, tense, negation, links | a realizer that says only what the frames keep | an edit script from the realized tree to the parsed one |
| frames ↔ world | the story: entities by id, the line of events, who said what, why | one clause per event, one sentence per clause unless the residual joins them; an entity by name or by the pronoun the reader would resolve | frames the world dropped, field patches, sentence boundaries |

Every decoder is deterministic and adds no content the layer above lacks. Choosing "she" where the reader would resolve it to Mary is wording, not new content. A realizer that invented words to sound fluent would hide what the frames lose; this one shows it.

### Exact and normal modes

- **Exact.** Every layer keeps its residual, and a whole text must come back byte for byte, punctuation, line wraps and the source's spellings included. The residual's size is the measure.
- **Normal.** No residual. The text is decoded from each layer alone and compared with the original: divergence by class (words per 100 tokens that differ in order, punctuation, function words…), the share of sentences that come back exactly, and chrF (a 0–100 score of shared character sequences). From the world, the text reads like a summary that keeps who did what, in other words.

### Residual classes and bits

Each residual item has one class: layout, spacing, punctuation, case, typography; form, source spelling; order; function words, pronoun or name, tense and modality, negation, quantity; dropped content, another content word, attachment; sentence boundaries, links (next, because, what was said), quotation, repetition.

Bits are counted by fixed rules, not an entropy coder, so runs and versions compare:

| item | bits |
|---|---|
| a literal string | 8 per byte |
| a dictionary word | 17 |
| a function word, a negation, an auxiliary | 8 |
| a punctuation mark | 5 |
| a tag or a relation | 6 |
| a position among n words | log₂(n+1) |
| the order of a head with k dependents | log₂((k+1)!) |
| a flag | 1 |

Lengths and separators are not counted: the bits compare runs and rank classes; they are not a decodable code.

### Blocks, not sentences

A sentence alone cannot say who "he" is. The autoencoder works on **blocks**: a whole lesson, a book chapter, or a run of whole chapters up to 2,500 words, cut only at chapter boundaries. One block has one discourse and one world, and the whole block must come back.

### Gates and controls

- **Exact gate.** Every block is decoded from its world with every residual and compared with the original bytes; frames, tree and tokens are compared on the way. A failure names the layer and the sentence. The site's pages pass the same gate: the Markdown twin of each page, prose up the layers, markup copied, its trees and frames tables rebuilt from the layers.
- **Damage controls.** On every tenth block, for each layer in turn, the first residual item of that layer is lost on purpose; the block must turn red. Not yet tested: a decoder that ignores every later item, or falls back to a default for a wrong value. A seeded random item, dropped or changed, would cover both; it must be drawn from items whose loss changes the output.
- **Drop controls** (normal mode). `--drop negation` or `--drop punct` must make that class's divergence grow when decoding from the tree, which keeps every negation and punctuation token.
- **Shuffle.** The block's sentences shuffled, then sent up and back. The shuffled text is what must return, so this tests how much coherent order helps the generator (pronouns, links, sentence planning), not whether the world keeps the order of events.
- **Determinism.** Ordered maps only; two runs give byte-identical reports.

### When a change counts

A change counts when the residual shrinks on the development split with the gate exact and two runs identical, **and** a control learned on shuffled data gains clearly less, **and** the test split, measured once with the gate, falls too.

### Training on the residual

Each run writes items and bits per block, layer and class; training reads them. The markup dictionary, the Hebrew trees' lexicon and the adapted word order are learned from the residual of the site's pages. Parallel texts (the Tanakh in many English versions, with the Hebrew) are read into one meta-world and each translation written back out of it; the bits between two translations measure their distance, and the residual says what brings them closer.

### How to run it

`TALES_TREEBANK`, `EN_MODEL` and `HBO_MODEL` stand for your tales CoNLL-U file, `$MOVA_DATA/en/models/en.bin` and an Ancient
Hebrew (`hbo`) model.

```sh
# from the repository root; the model is $MOVA_DATA/en/models/en.bin unless --model names another
cargo build --release -p roundtrip
R=target/release/roundtrip; OUT=$MOVA_DATA/runs/roundtrip
$R probe "The dog didn't bark. 'Let there be light,' said God."   # every layer of a text, both ways
$R run --out "$OUT/before" lessons=lessons:$MOVA_DATA/school/courses     # exact gate and residual by class
$R run --normal --drop negation --out "$OUT/control" tales=conllu:"$TALES_TREEBANK"   # negative control
$R blocks --out "$OUT/blocks" --threads 8 --model "$EN_MODEL" --hbo "$HBO_MODEL"   # whole blocks of the training mix
$R run --out "$OUT/after" lessons=lessons:$MOVA_DATA/school/courses      # after a change
$R compare "$OUT/before" "$OUT/after"                             # side by side
```

### Limits and open work

- Tree ↔ frames is the weakest link: it loses the most per sentence. Its largest classes are dropped content words and punctuation; punctuation that depends on word order (a comma after a fronted clause) needs a step after ordering.
- The generator's word order is weak on inversion, fronted obliques, speech frames and multi-word names.
- Quantities per mention, modifiers the world says only at the first mention, and longer pronoun reference (possessives, pronouns beyond the previous sentence) still cost residual.
- Scanned grammars carry double spaces that a layout record or normalization would remove.
- Markup that follows from a page's address and template is not yet derived.
- There is no Hebrew generator; Hebrew lines cost far more than English ones.
- The bits rank layers and classes; a comparison with a general-purpose compressor and a trivial chrF baseline are not done. Unresolved mentions are counted; the accuracy of resolution is not.
