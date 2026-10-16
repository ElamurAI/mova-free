# The English compressor, step by step, in plain terms

*A step-by-step walkthrough for readers who find the technical docs hard to follow. Every step below is
real output of the `en explain "sentence"` command; you can run it on any sentence.*

## The idea in three lines

1. **Input.** Text is parsed into a **graph**: words become numeric nodes, and the relations between them become edges ("who", "does what", "with what").
2. **Generation.** A **generator** builds text back from the graph.
3. **No neural nets.** There are no neural networks anywhere: there are tables trained on trees annotated by linguists (UD, 14,400 sentences), and automata that walk those tables.

Training works "like with neural nets": when a table makes a mistake, we penalize it. But everything is transparent, deterministic and fast: ~10k sentences/s on a single core.

Running example: **"Should I bring an umbrella tomorrow?"**

---

## Step 0. A dictionary in code: English as numbers

Before anything is parsed, a dictionary is already compiled into the binary:

- **The lemma heap.** 110,308 lemmas, each with a **frozen number**: `the` = 0, `.` = 1, `be` = 2, `,` = 3 … `tomorrow` = 755, `rain` = 3290. A number never changes; new words are appended at the end. So a graph written today will still be readable a year from now.
- **Full form coverage.** 242k forms (went, gone, going, goes…): each form → lemma + class + frequency. This is a "hash table in code": `build.rs` builds it at compile time, so nothing is built at run time.
- **Constants.** Words that rules rely on are constants in code: `w::THE`, `w::QMARK`, `w::CELSIUS`. The rule "are units named?" is `lemma == w::CELSIUS`, i.e. a comparison of two integers, not strings.

Sources: AGID (paradigms for 112k English lemmas, permissive license) + frequencies from UD.

## Step 1. Tokenizer: a finite automaton (the idea from `html_parser.c`)

Every character has a **class**: Le (letter), Di (digit), Sp (space), Pe (period), Ap (apostrophe)… There is a table **state × class → (command, new state)**. Transitions go "class to class", not "word to word", as originally proposed.

```
"S" Le New St→Wo · "h" Le App Wo→Wo · … · "d" Le App Wo→Wo · " " Sp Sep Wo→St · "I" Le New St→Wo · " " Sp Sep Wo→St …
```

- `New`: start a word;
- `App`: append a character;
- `Sep`: a space closes the word;
- `St`: "between words", `Wo`: "inside a word".

One table lookup per character.

Result: `Should │ I │ bring │ an │ umbrella │ tomorrow │ ?`

On top of the automaton sit dictionaries from the treebank: abbreviations ("Mr.", "U.S."), clitics ("did|n't", "John|'s"), hyphenated words. Exact tokenization match is 90% on the EWT treebank and 98% on PUD.

## Step 2. Tagger: the class of each word

A tag is a word's grammatical class (the Penn Treebank system, 53 classes):

| word | tag | what it is |
|---|---|---|
| Should | MD | modal verb |
| I | PRP | pronoun |
| bring | VB | verb, base form |
| an | DT | article |
| umbrella | NN | noun |
| tomorrow | NN | noun ("tomorrow" acts as a noun here) |
| ? | . | final punctuation |

How the choice is made:
- **A transition table "class, class → class".** After MD and PRP, VB is the most likely. This is a Markov chain, a dense array of 54³ numbers.
- **A table "word → classes".** For known words, we take the classes from the treebank.
- **Unknown words.** A suffix model handles them ("-ness" → NN, "-ed" → VBD/VBN), plus a hint from the dictionary in code.
- **Choosing the chain.** Exact Viterbi finds the most likely chain of classes for the whole sentence at once.

Accuracy is 93.1%. The dictionary hint added +2.2–2.8 points on unknown words.

## Step 3. Morphology: lemma + case

The **lemma** is the dictionary form: "brought", "bringing", "brings" → **bring**. The graph stores the lemma and the tag, and the form is reconstructed: bring + VBD → brought (from the dictionary in code).

```
Should → should · I → I [capitalized] · bring → bring · an → a · umbrella → umbrella · tomorrow → tomorrow
```

Position rules:
- **Capitalizing the first word** is a property of the position, not of the word: in the graph it is a separate bit. So after reordering, the generator writes "I should bring", not "I Should bring".
- **"an"** is stored as the lemma "a". The form comes from a rule based on the next word: before a vowel, "an".

If the form cannot be reconstructed from the lemma (a rare spelling, "an hour"), an **escape** kicks in: the form is stored as is. That is why lossless mode gives **100%**.

## Step 4. Parser: a stack automaton (the sentence tree)

This is an automaton too. It has a **stack** (words waiting for a head) and a **buffer** (words not yet read). At each step a **weight table** picks one of three actions:
- **SHIFT**: move a word from the buffer onto the stack;
- **LEFT(relation)**: the top word of the stack becomes the head of the neighbor below it;
- **RIGHT(relation)**: the reverse.

```
 1. stack [ROOT]                          · buffer [Should I bring an umbrella tomorrow ?] ⇒ SHIFT
 2. stack [ROOT Should]                   · buffer [I bring an umbrella tomorrow ?]        ⇒ SHIFT
 3. stack [ROOT Should I]                 · buffer [bring an umbrella tomorrow ?]          ⇒ SHIFT
 4. stack [ROOT Should I bring]           · buffer [an umbrella tomorrow ?]                ⇒ LEFT(nsubj): bring → I
 5. stack [ROOT Should bring]             · buffer [an umbrella tomorrow ?]                ⇒ LEFT(aux): bring → Should
 6. stack [ROOT bring]                    · buffer [an umbrella tomorrow ?]                ⇒ SHIFT
 7. stack [ROOT bring an]                 · buffer [umbrella tomorrow ?]                   ⇒ SHIFT
 8. stack [ROOT bring an umbrella]        · buffer [tomorrow ?]                            ⇒ LEFT(det): umbrella → an
 9. stack [ROOT bring umbrella]           · buffer [tomorrow ?]                            ⇒ SHIFT
10. stack [ROOT bring umbrella tomorrow]  · buffer [?]                                     ⇒ RIGHT(nmod:unmarked): umbrella → tomorrow
11. stack [ROOT bring umbrella]           · buffer [?]                                     ⇒ RIGHT(obj): bring → umbrella
12. stack [ROOT bring]                    · buffer [?]                                     ⇒ SHIFT
13. stack [ROOT bring ?]                  · buffer []                                      ⇒ RIGHT(punct): bring → ?
14. stack [ROOT bring]                    · buffer []                                      ⇒ RIGHT(root): ROOT → bring
```

Tree:
```
bring (root)                 — main verb
   Should (aux)              — auxiliary
   I (nsubj)                 — subject
   umbrella (obj)            — object
      an (det)               — article of umbrella
      tomorrow (nmod:unmarked) — parser error: "tomorrow" should attach to bring as a time adverbial
   ? (punct)
```

How the table picks an action: it looks at **features** of the current state. For example:
- "the top word of the stack is VB, the one below it is PRP";
- "the first word in the buffer is DT";
- …and 33 more templates.

Each feature has weights for the three actions; the weights are summed and the largest sum wins. Features are numbers (template number + word and tag numbers), no strings.

**Training (perceptron).** We run through the 14,400 treebank sentences, knowing the correct action sequence for each. If the table picked the wrong action, we add +1 to the weights of the correct action for all features of that state and −1 to the wrongly chosen one. This is repeated 10 times over the whole corpus, then the weights are averaged. That is the "training like with neural nets", except the weights sit in a table where you can see them.

Quality: 86.3% of heads correct with gold tags and 82.3% with our tags. Speed: half a million words per second.

## Step 5. The graph: what is stored

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

- **A node is 12 bytes, no strings:**
  - lemma number: 4 bytes;
  - tag: 1 byte (enum);
  - case: 1 byte;
  - relation: 1 byte (enum);
  - head number: 2 bytes;
  - space after: 1 byte.
- **New words.** If a word is not in the dictionary (a rare name), it goes into the graph's **local heap** and gets a number with the high bit set, as originally proposed: global and local heaps.

## Step 6. Generator: from the graph back to text

**Lossless.** We take the nodes in their stored order, lemma + tag → form, plus the a/an and capitalization rules: `Should I bring an umbrella tomorrow?`. Match is 100% across all sentences.

**Generative** (the order has to be recovered from the tree; this is how generation from a graph will work when the original order is absent):
1. **The side of each dependent** relative to its head. A backoff table: from an exact key ("closed-class lemma + head tag + interrogative sentence + whether there is an aux") down to a coarse one (just the relation). Example: det goes left, obj goes right.
2. **The order on each side** is set by a Markov chain over classes ("nsubj follows aux"…). Dynamic programming finds the best sequence.
3. **Word forms**, position rules, spaces.

Result: `I should bring an umbrella tomorrow?`. The answer is in place, but the generator did not perform **question inversion** ("Should I"): it put the subject before the auxiliary. This is a known weak spot, queued for improvement.

On weather forecasts, the order is usually recovered fully:

```
Mostly sunny, with a high near 72.
graph → generative → Mostly sunny, with a high near 72.
```

## Step 7. Domain: further training on our own parses

- **Method.** Nobody has annotated trees for weather. We take thousands of weather sentences, the general parser builds trees for them itself, and the generator is further trained on those trees (self-training). This way the domain picks up its own style without any manual annotation.
- **Two weather variants** (24.09):
  - **US:** °F, mph, NWS style ("Mostly sunny, with a high near 72");
  - **international English:** °C, km/h, Environment Canada style ("Mainly sunny. High 22.").

  Both collectors are in Rust with serde structs; the data is open (public domain and the ECCC Open Licence).
- **Input query parser.** Query graph → a typed `WeatherQuery`, a ready "command" for the weather code:
  ```
  "what is the temperature in celsius in Kyiv this weekend"
    → { place: "Kyiv", time: "this weekend", condition: "temperature", unit: C }
  ```
  - Slots are found by a table over graph features (F1 88%).
  - Units are found by a rule on constants: `lemma == w::CELSIUS`.

## What it is all for

1. A prompt comes in.
2. Graph.
3. Commands for Rust code (`WeatherQuery`).
4. Result graph.
5. Text from the generator.
6. If the parser fails, a fallback path kicks in: the AI rewrites the query more simply, and the parser tries again.

The AI here only builds deterministic inputs; it does no processing of its own. The same mechanics fit RAG document expansion and import/export: the expensive AI stays only where it cannot be avoided.

## Glossary

| term | what it is |
|---|---|
| lemma | the dictionary form of a word (went → go) |
| tag (class) | a word's grammatical class: NN noun, VB verb, JJ adjective… |
| dependency tree | each word hangs on one "head" word with a relation (subject, object…) |
| UD relation | edge type: nsubj subject, obj object, aux auxiliary, det article, case preposition, punct punctuation |
| Markov chain | the probability of the next class depends on the previous ones (here, two) |
| Viterbi | an algorithm that finds the most likely chain of classes for the whole sentence at once |
| perceptron | a weight table "feature → action" that is penalized for mistakes |
| escape | the form could not be reconstructed from the lemma, so the form itself is stored |
| chrF | similarity of two texts by matching character sequences (0–100) |
