# Pragmatics: frames — who did what to whom, why, and what came next

This page describes how Mova reads the meaning of a text above the syntax tree. It covers frames (one per event or fact, with roles and links), how to call the reader, the frame bench that measures it, the VerbNet and WordNet rules it uses, the invariants it keeps, and the rules it learns. It also covers the algorithms from the literature that are built in, expressions read by their real meaning, event statistics of books, and the external pragmatics tools kept for tests. There is no bag of words anywhere: a reader matches the roles of frames, never the raw words of whole lines.

## At a glance

```text
$ world frames "The fox flattered the crow because she wanted the cheese. The crow opened her beak and the cheese fell."
#0 Event fox flatter patient=crow | cause→#1 next→#2
     roles by vn lure-59.3 «NP V NP»: agent=Agent crow=Patient
#1 Event fox(«she») want patient=cheese
     roles by vn want-32.1-1-1 «NP V NP ADV»: agent=Pivot cheese=Theme
#2 Event crow open patient=beak | next→#3
     roles by vn crane-40.3.2 «NP V NP»: agent=Agent beak=Patient
#3 Event cheese fall
     roles by vn escape-51.1-1 «NP V»: agent=Theme
```

Each frame knows **who** did what (agent, patient, recipient, place); keeps **links** («because» gives a cause, «and» and sentence order give what happened next); resolves **pronouns** in reading order («she» is the fox); and says **where its roles came from** (here a VerbNet class and frame). The structure belongs to the skeleton (`prag::frames`), not to any single reader.

## Calling it

```rust
// world (parser = the en annotator on cased text + the absurdity-matrix repairs):
let mut ctx = world::frames::Ctx::default();          // one discourse per text (a lesson, a story)
let r = world::frames::read(text, &mut ctx, false)?;   // r.frames, r.asked, r.aliases, r.roots
// any crate with its own parser:
let r = prag::frames::read_with(text, &mut ctx, question, &|sentence| words_of(sentence));
```

`world frames "<text>"` prints the frames (`FRAMES_TREE=1` adds the trees). Every step of the reader can be switched off for measuring with `PRAG_OFF=name,name`; the switch names are given below next to each step.

`prag` depends on `en` only, so the types and the tree → frames reader live there. The parser world uses adds the absurdity-matrix repairs (`world::rerank`, `world::induce`, see [knowledge.md](knowledge.md#the-absurdity-matrix)), which belong to world; world passes its parser in. Moving the repairs into `prag` would make `prag` depend on world — a cycle.

## A frame

One predicate of the tree (a verb, or a word with a copula):

| field | meaning |
|---|---|
| `kind` | `Event` (an action), `Fact(Link)` (a general statement: is a, is, has, can, does, lives in, made of, used for), `Say(Act)` (speech or thought, with its speech act from `prag::schema::Act`), `Bare` (a phrase without a predicate, e.g. an answer option «A frying-pan») |
| `agent`, `patient`, `recipient`, `place` | entities (`Ent`) |
| `verb` | the lemma with its particle (`put down`); `be` for a copula |
| `neg`, `cond` | negated; inside an «if» clause |
| `irrealis` | not said to have happened: the «to» complement of a wish, plan or attempt («wanted to eat»), and an «if» clause. The implication signatures (`prag/data/signatures.tsv`, after Karttunen and Nairn et al.) set it top-down with negation: «managed to eat» ate, «failed to eat» did not, «did not fail to eat» ate. `PRAG_OFF=factuality`, `cond-irrealis` |
| `tam` | tense, aspect, voice and modality of the verb group (`tam_of`): tense, form (Fin, Inf, ToInf, Ger, Part), modal, perfect, progressive, passive, do-support, imperative. `PRAG_OFF=tam`. The student does not use it yet; the round trip's realizer says the verb group from it |
| `attrs` | a copula's adjective, «get tiny», manner adverbs, `when:day` for times |
| `obls` | other prepositional roles (`with=queen`) |
| `quote` | the frames a Say frame carries (what was said or thought) |
| `cause`, `next` | links to other frames (indices in the same `Read`) |
| `link` | where the clause hangs: the frame above, the clause's relation (conj, advcl, xcomp, ccomp, parataxis, acl:relcl …) and its noun when that is not the predicate above. `PRAG_OFF=frame-link` |
| `unsaid` | roles filled from elsewhere: an agent shared with the clause above, an imperative's «you», the noun of a relative or infinitive clause. `PRAG_OFF=unsaid` |
| `manner` | the `attrs` that are manner adverbs in the text. `PRAG_OFF=frame-manner` |
| `words` | the clause's other one-word dependents with relation and tag (then, also, again, discourse words, an expletive «there»), and the words under them flattened onto the clause. `PRAG_OFF=frame-words`, `frame-rest` |
| `nps` | noun phrases on the clause in no role: time and measure without a preposition («last week»), vocatives, dislocated phrases, a second object, a free wh-word. `PRAG_OFF=frame-nps`, `frame-rest`, `wh-free` |
| `mark`, `cc` | the clause's markers («that», «if», «to») and its conjunction («but», «or»). `PRAG_OFF=frame-mark` |

**A sentence whose root is no predicate** (a heading, a title, a list item, «Yes.») gets one `Bare` frame, after the predicates' frames and before the nouns' events: the root's entity as the patient (or with its preposition the place or an oblique); a root that is no entity is its first `words` entry with relation `root`. `PRAG_OFF=frame-fragment`.

**An entity** (`Ent`) keeps its head lemma, number, name bit, adjectives and coordinated heads, and what this mention says: `poss` («his nose»), `mods` («a cup of tea»), `appos`, `words`, `conj`, `det`, `case`, `cc`, `rel_word` («who»). With `ent-rest` it keeps every other dependent too. The discourse (`Ctx`) keeps only the entity. `PRAG_OFF=ent-poss, ent-mods, ent-conj, ent-det, ent-case, rel-word, ent-rest`.

The fields from `link` down and the entity's fields from `poss` on keep what the tree says beside the roles, so that the frames can be said back (the round trip, `tools/roundtrip`, see [english.md](english.md)). The student, `show` and the frame bench do not read them.

**Anchors** (`Ctx::anchors`, one per frame of the last sentence): the predicate's token, the token of the frame it hangs on with the relation, and the head token of each role said in the clause. Nothing in the reader reads them; `tools/translate` reads its meta-world's tree through them. `PRAG_OFF=anchors`.

## How roles and links are found

**Entities** are head lemmas (`the Cheshire Cat` = `the Cat` = `cat`; `Ent::same`). **Pronouns** are resolved by `Ctx` in word order (a pronoun sees only what came before it): agreement in number; he/she → a person or an animate being of the right gender, never a thing already called «it»; it → never a person; numbers and times are never antecedents; salience by the best role of each sentence (subject > object > other), halved per sentence, with a bonus for the pronoun's own role; a plain object pronoun is never its own clause's subject; a dummy «it» (it depends, it is unkind to …) stays unresolved. With `PRAG_VN=gender-table` a common noun the closed lists say nothing of takes animacy and he/she gender from the coreference noun lexicon counted from Mova's own parses ([coref](../coref/README.md#the-noun-lexicon-srclexiconrs)), and «it» never takes a noun the lexicon calls animate; it is opt-in because the school did not gain (2026-11-04). Inside quotes «I» is the speaker of the Say frame and «you» its addressee. `PRAG_COREF` picks the resolver: 3 this one (default); 1/2/4 the `coref` crate's sieve alone or first; 0 salience alone.

**Links**: «because / since / as», a purpose «to …» and «by V-ing» → `cause`; «so / therefore» → the clause before is the cause; «so … that» → the result's cause; «and / then / after / before / when / until» and consecutive sentences → `next`, kept only between actions and speech (never from or to a state, a general fact or a condition); a ccomp of say/think/know/fear and quoted speech → `quote` of the Say frame. Only a verb of saying or knowing carries a quote.

**Roles, more**: an imperative's agent is `you`; object control (make X grow, tell X to go) gives the infinitive the object as agent; gerund subjects are the activity; phrasal verbs come from the expression base (adverb particles anywhere, prepositions only standing alone); the speaker after a quote («"…" cries the Badger»); the person told or asked is the recipient; the act by the quote's ?/!; no `place` for persons, animals, times and measures.

**Questions** (`question = true`): the first wh-word gives `Asked { frame, what }` with `Ask::Cause` (why), `Ask::Next` (what happened next), `Ask::Role(slot)` (who → agent, what → patient, what … do → verb, where → place, how → attr, which N → N's role), `Ask::Frame` (yes/no, when). Do-support and modal questions are put in statement order before parsing («What did the cook throw?» → «the cook threw what?»), because the tagger misreads them. «what happens when X» → `Ask::Next` on X.

Deterministic: word order and ordered maps; the same text and ctx give the same frames.

## The linguists' rules: VerbNet and WordNet

The roles come first from open lexicons written by linguists (`prag::vn`); the tree heuristics above are the fallback. **Only permissive data builds the reader**: VerbNet 3.4 (VerbNet 3.0 License, © University of Colorado), Princeton WordNet 3.1 (WordNet 3.0 License, © Princeton University) — both free including commercial use with the notice — and our own lessons. PropBank (CC BY-SA) and NomBank / NOMLEX (research-only) are **tests and measurement only**: converted outside the repo, never compiled in, never read by the reader or the rule learner.

Compiled tables (deterministic converters in `prag::vnconv`; each header records the source version and a checksum):

| table | built by | content |
|---|---|---|
| `prag/data/verbnet.tsv` | `prag lexicon convert <verbnet3.4> <propbank frames> <nombank.1.0> <out> [<test dir>]` | classes with subclasses, members, frames (a syntactic pattern → roles, selectional restrictions, semantic predicates). The same command writes PropBank's and NomBank's tables to the test dir (`$MOVA_DATA/raw-derived/prag-lexicon/`) |
| `prag/data/nominals.tsv` | `prag lexicon nominals <wordnet 3.1 dict> <out>` | nouns that name a verb's event: a noun whose first sense lies in an event file (act, event, process, state, communication, cognition, feeling, phenomenon, possession) → the verb of the first such sense with a derivational link («arrival» → arrive, «destruction» → destroy), unless the verb is made from the noun («memory» → memorize); transitivity from WordNet's verb frames |
| `prag/data/vnfreq.tsv` | `world frames vnfreq` | for verbs with several classes, how often each class alone matched best in our courses' clauses |

- **Matching** (`vn::best`): the clause as the tree has it — subject, objects, prepositional phrases with their prepositions, the complement's kind (that / to / -ing / wh / quote), a passive with its by-phrase — against every frame of the verb's classes (the phrasal verb first; a subclass inherits its parents' frames). An integer score counts matched arguments, missing elements, unplaced objects, unmatched PPs (cheap), role restrictions against the fillers' animacy, and the lessons' preference from `vnfreq.tsv` as a tie-breaker. The core (every object and the complement) must match, else no frame.
- **Roles applied over the heuristics**: an object VerbNet calls a Recipient that is a person or an animal is the recipient (not when it controls a complement: «help her find»); the second object of a dative is the patient; a PP that is a Destination / Location / Source / Trajectory is the place (never with by / with / of / for / about / like / as); a PP that is a Theme, Topic, Stimulus or Instrument is no place; a passive's «by» oblique that can act is the agent. The subject stays the agent whatever its role. A verb VerbNet lacks keeps the heuristics' roles.
- **Speech**: verbs all of whose classes are communication (37.x: whisper, mumble, inquire …) are speech verbs.
- **Events of nouns** (WordNet): a noun of `nominals.tsv` with an argument becomes an Event frame — «the arrival of the Queen» → `Event queen arrive`, «the destruction of the city by the army» → `Event army destroy patient=city`. «by X» is the agent; «of X» the patient when the verb takes an object, else the agent; «X's» the agent, or the patient in «the city's destruction». They are appended after the sentence's frames, without links; the bench's scorer leaves them out (`PRAG_VN=score-nouns` counts them).
- **Where the roles came from**: `Frame::how` (`By::VerbNet | Nominal | Heuristic`, the class or noun, the frame pattern, the role of each argument), printed by `world frames` and `world knowledge trace`.
- **Switches**: `PRAG_OFF=vn` (all), `vn-verbnet`, `vn-obj`, `vn-dative`, `vn-by`, `vn-place`, `vn-recipient`, `vn-notplace`, `vn-say`, `vn-nom`; `PRAG_VN=recipient-any,to-recipient,noto-place,noq,score-nouns` (variants kept for comparison, not default).
- `world frames lexicon [course …]` shows the coverage of the lessons' verbs and nouns; `world frames nominal-check` compares the WordNet nouns with NOMLEX read at run time from the data directory (tests only).

## The frame bench

`world frames bench [dev|held] [baseline] [worst N] [errors <role>] [prons] [agents]` is the yardstick of the reader. Its lines come from the lessons of six courses (books, world, biology, everyday, history, reflection), split into dev and held-out; their frames were written once by the teacher (`world frames bench-make`) and live in `$MOVA_DATA/runs/prag-bench/` (lines.tsv, gold.tsv, notes.tsv, calls.jsonl). The scorer (`prag::fbench`) parses the gold and the reader's own `show` with one parser, aligns frames (verb head or two roles) and gives P/R/F1 per role — agent, verb, patient, recipient, place, speaker (Say + act + speaker), says, cause, next, coref (pronouns in roles) — and a micro overall. The negative control is `baseline` (first noun agent, first verb, next noun patient). Decisions are made on dev; the held-out lines are run rarely and never read by any learner.

## Invariants over the corpus

`prag::invariants` lists rows the reader should never give. Each check has a unit test with a negative control (a correct frame is not flagged). A corpus-wide run writes one TSV per check (sentence id, text, row, detail) and `counts.tsv` for review; that run is not part of this repository.

| check | flags |
|---|---|
| `copula-no-who` | a copula frame with no subject |
| `time-in-where` | a word of time as the place (`frames::time_word`) |
| `vn-spread` | one verb in one syntactic frame given more than one VerbNet class |
| `unsaid-own` | a role marked «not said here» while the clause has its own dependent for it |
| `who-is-what` | the agent and the patient are the same entity (reflexives and bare pronouns aside) |

Reader steps that keep these rows out, each switchable with `PRAG_OFF`:

- `time-more`: days, months, «season», «weekend», «midnight», «dawn» are times, never places or objects («comes on Tuesdays» → `when:tuesday`);
- `acl-own`: a relative clause with a relative word («things that are untrue», «whose», «where») gets no role from its noun;
- `imp-do`: «Don't talk garbage», «Do come in», «Be quiet» are imperatives said to «you»;
- `vn-gap`: a relative clause with a subject and no object («things you throw away») shows the matcher its noun as the object;
- `vn-stable`: the VerbNet class is chosen on the clause's syntactic frame alone (animacy unknown); animacy then chooses the frame within the class, so one verb in one frame gets one class wherever it stands.

A place VerbNet calls a Goal outside the classes of putting, sending and motion, or a Result, is marked by `Frame::goal` («sorted the garbage into paper»); the site shows it under «other». The reader keeps it in `place`, because the student's weights lean on place matches (`PRAG_VN=goal-obl` makes it an oblique).

## Rules the reader learns itself

`prag::rules` (`world frames rules learn | control | cv | show | apply-check`) learns transformations applied after the reader, error-driven from the gold (after Brill 1995, as `world induce` learns tree repairs; see [training.md](training.md)). The sources are the bench's dev lines and the teacher's verdicts (`$MOVA_DATA/runs/prag-teacher/log.tsv`: OK, WRONG with the correction, MISSING). Every rule is a sentence: «For a frame: if the predicate word is tagged VBN, it has no object in the tree and comes after the word above it, then it has no agent.»

- **Sites**: a frame, a pronoun in a role, a participle on a noun that has no frame.
- **Atoms** (1–3 per rule): relation and tag, own subject/object, marks and linking words, position, the kinds of nearby frames, roles present, the agent's and patient's form and animacy, verb classes, quoted / if / negated, links present; for a pronoun its slot, its resolution and the candidate's form, animacy and gender; VerbNet atoms (vn-class, vn-frame, roles-by, vn-agent). Lemmas only with `lex` (they overfit).
- **Actions**: role fixes (agent from the frame above or its patient, clear a role, patient↔recipient, oblique → place/patient, swap, not a frame, Say↔Event), links (cause / next, clear, says), a pronoun's candidate, a frame for a participle, agent-from-by, to-from-obl.
- **Learning**: each site × action; every subset of the site's atoms is a candidate scored by the first-order change of micro F1 over the sites it matches; the best above a precision and lines-fixed threshold is applied for real and kept if it gains. `delw` (default .5) halves a deletion's gain in ranking, so rules that add or move are preferred. Thresholds are chosen by `cv` (two folds of dev). The negative control (gold shuffled between lines) learns only deletions.
- **File** `prag/data/rules.tsv` (compiled in; `PRAG_RULES_FILE` reads another): id, status (proposed / accepted / rejected), action, atoms, the sentence, train fixed / broken, the checker's reason. The reader applies only `accepted` rules; `PRAG_RULES=proposed` adds the proposed ones, `off` none. Rejected rules stay in the file with their reasons and are never proposed again. The checker's queue is `$MOVA_DATA/runs/prag-rules/queue.tsv`, with per-rule effects on dev and on the courses.

Most learned rules are deletions, and deleting `next` links and Say frames hurts the student's «what next» answers, so the checker accepts few. One rule is accepted: a motion verb with no object in the tree has no patient.

## Clients

- `world knowledge` — the pragmatic student (`PragStudent`): remembers the frames of every lesson line, answers by matching the question frame's known roles and following the asked role (cause, next, the role itself, and weakly the other frames of the anchor's line); the generic «you» is no evidence; it learns role-level weights only. `world knowledge trace <course> <lesson>` shows the frames behind every choice. See [school.md](school.md).
- `world knowledge teacher-check <course> <from> <to> [lines]` — the hidden teacher checks the frames of sampled lines and logs every verdict to `$MOVA_DATA/runs/prag-teacher/log.tsv`; it never answers tests.

## Algorithms from the literature

These algorithms are built into `prag` and `world` as deterministic, explained Rust: integer or exact-fraction arithmetic, fixed order, and a trace that says why. Card links point to the paper cards in `../papers/`.

| algorithm | source | where in code | status |
|---|---|---|---|
| Implication signatures (managed / failed to) | Karttunen; [Nairn et al. 2006](https://research.elamur.ai/free/en/papers/lingbulk-2006-nairn-computing-relative-polarity-textual-inference/) | `prag/data/signatures.tsv`, frame `irrealis` | built in |
| Social obligations (greeting, thanks, apology, goodbye, politeness question) | [Bunt et al. 2017](https://research.elamur.ai/free/en/papers/bunt-2017-revisiting-iso-standard-dialogue-act/), [2020](https://research.elamur.ai/free/en/papers/bunt-2020-iso-standard-dialogue-act-annotation/) (ISO 24617-2) | `world/src/talk.rs` | built in; the full dimension × function scheme is not |
| Politeness strategies: 20 detectors with integer weights | Danescu-Niculescu-Mizil et al. 2013 (strategy list as facts; data CC BY-NC-SA); [Brown & Levinson 1987](https://research.elamur.ai/free/en/papers/brown-1987-politeness-some-universals-language-usage/) | `world/src/talk.rs` | built in: a blunt request is done, and the fired strategies are noted |
| Indirect requests: «Can you X?» is a request only when the speaker wants it and the hearer is capable, willing and obligated | [Sarathy et al. 2020](https://research.elamur.ai/free/en/papers/sarathy-2020-reasoning-requirements-indirect-speech-act/), [Roque et al. 2020](https://research.elamur.ai/free/en/papers/roque-2020-developing-corpus-indirect-speech-act/), after Brown 1980 | `world/src/talk.rs` | built in; otherwise the direct reading (an answer about ability) |
| Scalar implicature on Horn scales (blocked by a stronger item, negation, cancellation) | Tatu & Moldovan 2012; [Krause 2024](https://research.elamur.ai/free/en/papers/krause-2024-gricean-maxims-nlp-survey/) | `prag/src/infer.rs`, `prag infer` | built in |
| Presupposition triggers (again, still, also, too, stop, continue, know, realize, regret, forget …), licensed by the same predicate in the previous 50 tokens or marked new | [Cianflone et al. 2018](https://research.elamur.ai/free/en/papers/cianflone-2018-let-s-do-it-again/) | `prag/src/infer.rs` | built in |
| Gricean maxims: Quality (a question → the speaker does not know; an exclamation → strong feeling), Manner (coordinated events are ordered, not with stative verbs) | Tatu & Moldovan 2012 | `prag/src/infer.rs` `maxims` | built in |
| Sarcasm by incongruity: polarity lexicon, explicit and implicit incongruity, pragmatic cues; the incongruent pair is the explanation | [Joshi 2017](https://research.elamur.ai/free/en/papers/joshi-2017-detecting-sarcasm-different-forms-incongruity/), [Riloff et al. 2013](https://research.elamur.ai/free/en/papers/prag-2013-riloff-sarcasm-as-contrast-between-positive/) | `prag/src/irony.rs`, `prag infer` | built in; meets the absurdity matrix's humor trigger in `world/src/domains.rs` |
| Rational Speech Acts: L0 ∝ [[u]]·P, S1 ∝ L0^α·2^(−αC), L1 ∝ S1·P in exact fractions | Monroe & Potts 2015; Frank & Goodman; [Anderson 2021](https://research.elamur.ai/free/en/papers/anderson-2021-tell-me-everything-you-know/) | `prag/src/rsa.rs`, `prag rsa scalar\|reference\|explain` | built in for small closed sets (scalar and reference games); negative control: an always-true utterance returns the prior |
| Questions under discussion: a stack; assertions attach to the deepest congruent open question; implicit questions accommodated from given lemmas only; answer leakage rejected | [Roberts 2012](https://research.elamur.ai/free/en/papers/roberts-2012-information-structure-discourse-towards-integrated/), [Riester et al. 2021](https://research.elamur.ai/free/en/papers/riester-2021-combined-discourse-representations-coherence-relations/), [QUDeval](https://research.elamur.ai/free/en/papers/wu-2023-qudeval-evaluation-questions-under-discussion/) | `world/src/qud.rs`, `world qud "<text>"` | built in |
| Error-driven transformation rules | Brill 1995 | `prag::rules` | built in (above) |

Each built-in piece has tests with negative controls. Known parser slips affect them: «The fox ate again» (*ate* tagged as a noun), «knows that the door is open» (the clause read as xcomp).

## Expressions by their real meaning

The pragmatics lens reads idioms, proverbs, phrasal verbs and verb phrases, slang and register-marked words by their real meaning, not literally. Its knowledge is the expression base `expressions.tsv`, built from the English Wiktionary dump by `tools/wikt-idioms`: kinded multiword entries (idioms, proverbs, phrases, verb phrases, phrasal verbs), other multiword entries, and single words with register or regional labels, each label with the gloss of the sense it marks. It is CC BY-SA and stays out of the free release.

- `global::expressions_in_sentence(forms, lemmas)` — non-overlapping matches, longest first, on lemmas («kicked the bucket» → kick the bucket) and on forms; «one's» matches a possessive, «someone» one word.
- A register label switches a domain on only through the main sense: slang (slang, informal, colloquial, vulgar, derogatory, offensive), formal (formal, literary, archaic), humor (humorous, jocular, ironic, sarcastic), hidden meaning (euphemistic, figurative). Rare labelled senses do not count.
- `prag::lens::lens(words)` gives readings (text, phrase, kind, meaning, register) and domains; `prag::lens::means` gives «what was meant». CLI: `prag lens <en model> "<text>"`.

Example: «Grandpa kicked the bucket, the poor old geezer.» → kicked the bucket = to die (euphemistic, humorous); geezer (informal, derogatory) → domains hidden-meaning, humor, slang.

The lens feeds «what was meant» and the domain switch. As features of the act classifier it gave no clear gain, so it is not used there.

**Domain modules.** `world domains "<text>" [--genre tale]` switches domains on with reasons; `world domains-eval <set.jsonl>` measures them on contrast sets with literal controls. The evidence: register labels on the main sense (regional varieties as `variety:<country>`), idioms and proverbs of two or more content words, the formal register measured on corpora (`global::register_z("legal")`: log-odds of public-domain legal and official books against tales, children's and other books, after Monroe et al. 2008; `world register-stats`; lexicons `global/data/register-{legal,tale,children}.tsv`), a register clash (formal language about everyday things → mock-formal humor), and absurdity that survives the genre's domain layer (a talking fox is humor in the real world, literal in a tale; see [knowledge.md](knowledge.md#the-absurdity-matrix)).

Limits: humor is the weakest domain — jokes rest on world knowledge and context, and word statistics catch only mock-formal clashes and absurdity. Modern slang («lowkey», «no cap») is mostly a secondary Wiktionary sense of common words; it needs time-sliced corpus statistics, not the dictionary.

## Event statistics of books

`world events <out-dir> <book.txt>... [--threads N]` (`world/src/events.rs`) parses every sentence of a book and counts events: agent, verb, patient, recipient (nouns only; pronouns are skipped). «The fox said to the crow» → `fox say →crow`; «the cheese fell» → `cheese fall`. The matrix's repairs are applied to the trees.

Outputs hold counts and ids only — no text, no trees:

| file | content |
|---|---|
| `books/<book>.json` | a summary: main actors, interactions (`crow ~ fox`), top events |
| `books/<book>.events.tsv` | the search index: sentence id `<book>:<n>` → event (ids stable within one version of the sentence split, `events::SPLIT`) |
| `events.tsv` | corpus-wide events with counts and number of books |
| `cells.tsv` | observed verb × noun × role (SUBJ/OBJ/TO) counts: the empirical side of the absurdity matrix |

Uses: what the corpus is about, a search index («where does a fox speak to a crow»), candidate cells for the matrix, and systematic errors (`world absurd-events`, see [knowledge.md](knowledge.md#error-search)); `world events-show <book.txt> <n>...` prints the sentences behind ids. For public-domain books the summaries and the index are public; for restricted books they stay internal, and answers are short summaries in our own words, never quotes. The two kinds are always run into separate output folders and never merged.

## External pragmatics tools

Open pragmatics software and models serve as task definitions, test sets and comparison points. None of them is wired into Mova, whose pragmatics stays rule-based and transparent.

| category | examples | use |
|---|---|---|
| RST and PDTB discourse parsing, discourse relations and markers | DMRST, DPLP, isanlp_rst, GumDrop, discopy, DISRPT shared tasks | discourse relations over our trees, as a comparison |
| coherence | coherence-rst, GCDC | tests |
| dialogue and speech acts | SILICONE models, MIDAS, DialogTag, SwDA | the speech act of Say frames, as a comparison |
| politeness | ConvoKit, `politeness`, Intel polite-guard | requests and imperatives against `talk.rs` |
| irony, sarcasm, metaphor | TweetEval irony model, sarcasm detectors, MelBERT | comparison with `irony.rs`; the tweet-trained models over-fire on plain sentences |
| implicature, presupposition, RSA | ImpPres, NOPE, CREPE, pragmeval, pypragmods | test sets for `infer.rs` and `rsa.rs` |
| certainty, hedging, stance | certainty-estimator, hedge detectors | tests |

**Licences**: GPL, NC and repositories without a licence file (all rights reserved) are for tests and measurement only. MIT, Apache and BSD are candidates for building, after a check of the data they were trained on. See `licence/DATA.md`.

## Limits

- The weakest roles are `next` (the gold's sense of «happened right after» is narrow), speaker and says (dialogue without verbs of saying, quotes the parser splits), place, coref (no world knowledge of who can do what, names without a gender) and agent (parser errors on long book sentences).
- No time order beyond `next`: the frame has its tense (`tam`) but no time axis. No quantities.
- Modality in the student goes no further than `cond`, `irrealis` and Fact(Can). Reading the modals of `tam` as irrealis, VerbNet's result states (who has the thing after a transfer, where it is after a motion) and global's broad verb classes as a weak match each cost the student questions, so they are off.
- Learned frame rules are mostly deletions; only accepted ones run.
- The external tools are not wired in.
