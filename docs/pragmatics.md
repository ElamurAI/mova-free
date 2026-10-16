# Pragmatics: frames — who did what to whom, why, and what came next

Mova reads every text through one pragmatic structure, the way it reads every sentence through a UD tree. The structure is a list of **frames**, one per event or fact. It belongs to the skeleton (`prag::frames`), not to any single reader. There is no bag of words anywhere: a reader matches the roles of frames, never the raw words of whole lines.

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

Each frame:
- knows **who** did what: the agent, patient, recipient and place;
- keeps **links**: «because» gives a cause, «and» and sentence order give what happened next;
- resolves **pronouns** in reading order: «she» is the fox;
- says **where its roles came from**: here, a VerbNet class and frame.

The rest of this page is the reference: the fields of a frame, how roles and links are found, the yardstick that measures the reader, the open lexicons it is built from, and the rules it learns by itself.

## Calling it

```rust
// world (parser = the en annotator on cased text + the absurdity-matrix repairs):
let mut ctx = world::frames::Ctx::default();          // one discourse per text (a lesson, a story)
let r = world::frames::read(text, &mut ctx, false)?;   // r.frames, r.asked, r.aliases, r.roots
// any crate with its own parser:
let r = prag::frames::read_with(text, &mut ctx, question, &|sentence| words_of(sentence));
```

`world frames "<text>"` prints the frames (`FRAMES_TREE=1` adds the trees).

## A frame

One predicate of the tree (a verb, or a word with a copula):

| field | meaning |
|---|---|
| `kind` | `Event` (an action), `Fact(Link)` (a general statement: is a, is, has, can, does, lives in, made of, used for), `Say(Act)` (speech or thought, with its speech act from `prag::schema::Act`), `Bare` (a phrase without a predicate, e.g. an answer option «A frying-pan») |
| `agent`, `patient`, `recipient`, `place` | entities (`Ent`) |
| `verb` | the lemma with its particle (`put down`); `be` for a copula |
| `neg`, `cond` | negated; inside an «if» clause |
| `attrs` | a copula's adjective, «get tiny», manner adverbs, `when:day` for times |
| `obls` | other prepositional roles (`with=queen`) |
| `quote` | the frames a Say frame carries (what was said or thought) — its patient |
| `cause`, `next` | links to other frames (indices in the same `Read`) |

**Entities** are head lemmas (`the Cheshire Cat` = `the Cat` = `cat`); `Ent::same` compares them. Pronouns are
resolved by `Ctx` in word order (a pronoun sees only what came before it): agreement in number; he/she → a person
or an animate being (global's level-1 categories, as world's sense animacy) of the right gender (gendered nouns, an
earlier he/she), never a thing already called «it»; it → never a person; numbers and times are never antecedents;
salience by the best role of each sentence (subject > object > other, not stacking), halved per sentence, a bonus for
the pronoun's own role; a plain object pronoun is never its own clause's subject; a dummy «it» (it depends, it is
unkind to …, it means + a clause) stays unresolved and matches nothing. Inside quotes «I» is the speaker of the Say
frame that carries the quote (or the last speaker when no verb of saying does) and «you» its addressee.
(`PRAG_COREF`: 3 this resolver, the default; 1/2/4 the `coref` crate's sieve over the discourse alone or first; 0 the
old salience. On the frame bench the resolver beat the sieve: coref F1 .380 vs .345.)

**Links**: «because / since / as», a purpose «to …» (any to-infinitive that is not a control complement, also on a
noun) and «by V-ing» → `cause`; «so / therefore» → the clause right before is the cause; «so … that» → the result's
cause; «and / then / after / before / when / until» and consecutive sentences → `next`, kept only between actions
and speech (never from or to a state, a general fact or a condition); a ccomp of say/think/know/fear… and quoted
speech → `quote` of the Say frame — only a verb of saying or knowing carries a quote (a quotation of the book's
words in the narration is not speech).

**Roles, more**: an imperative's agent is `you`; object control (make X grow, tell X to go) gives the infinitive the
object as agent; gerund subjects are the activity («holding let»); phrasal verbs from the expression base (adverb
particles anywhere, prepositions only standing alone); the speaker after a quote («"…" cries the Badger»); the person
told or asked is the recipient; the act by the quote's ?/!; no `place` for persons, animals, times and measures; the
copula reading of a participle with two subjects («Pollution is harmful material put into air»).

**Questions** (`question = true`): the first wh-word gives `Asked { frame, what }` with `Ask::Cause` (why),
`Ask::Next` (what happened next), `Ask::Role(slot)` (who → agent, what → patient, what … do → verb, where → place,
how → attr, which N → N's role), `Ask::Frame` (yes/no, when, «put him right»). Do-support is put in statement
order before parsing («What did the cook throw?» → «the cook threw what?»), because the tagger misreads it; so are
modal questions («What should you do if …» → «you should do what if …»). «what … do» asks the verb; «what happens
when / would happen if X» → `Ask::Next` on X; «Put her right.» → `Ask::Frame` on the quoted claim.

Deterministic: word order, ordered maps; the same text and ctx give the same frames.

## The yardstick: the frame bench

`world frames bench [dev|held] [baseline] [worst N] [errors <role>] [prons] [agents]` — 240 lesson lines from books,
world, biology, everyday, history and reflection (dev 180, held-out 60), their frames written once by the teacher
(`world frames bench-make`, Opus; kept in the data directory under `runs/prag-bench/`: lines.tsv, gold.tsv, notes.tsv, calls.jsonl). The scorer
(`prag::fbench`) parses the gold and the reader's own `show` with one parser, aligns frames (verb head or two roles)
and gives P/R/F1 per role: agent, verb, patient, recipient, place, speaker (Say + act + speaker), says, cause, next,
coref (pronouns in roles), and a micro overall. Negative control: `baseline` (first noun agent, first verb, next noun
patient) scores ~.17. Every step of the reader can be switched off for measuring: `PRAG_OFF=name,name`.

10.10: dev .579 → .633, held-out .618 → .667 (run once at the end).

## The linguists' rules: VerbNet and WordNet (`prag::vn`, 10.10)

The roles come first from open lexicons written by linguists; the tree heuristics above are the fallback. **Only
permissive data builds the reader** (since 10.10): VerbNet (VerbNet 3.0 License), Princeton WordNet
(WordNet 3.0 License) and our own lessons. PropBank (CC BY-SA, share-alike) and NomBank / NOMLEX (research-only)
are **tests and measurement only**: converted outside the repo, never compiled in, never read by the reader or the
rule learner.

- **Tables** (compiled in; deterministic converters in `prag::vnconv`, each header records the source version and a
  checksum of what was read):
  - `prag/data/verbnet.tsv` — `prag lexicon convert <verbnet3.4> <propbank frames> <nombank.1.0> <out> [<test dir>]`:
    609 classes with subclasses, 6740 members, 1602 frames (a syntactic pattern → roles,
    `Agent V Theme {to} Recipient[+animate]`, selectional restrictions, semantic predicates). The same command writes
    PropBank's and NomBank's tables to the test dir (by default `raw-derived/prag-lexicon/` in the data directory, outside the repo).
  - `prag/data/nominals.tsv` — `prag lexicon nominals <wordnet 3.1 dict> <out>`: 4951 nouns that name a verb's event
    (3538 whose verb sense takes an object). A single-word noun whose first sense (WordNet's frequency order) lies in
    an event file (act, event, process, state, communication, cognition, feeling, phenomenon, possession — not
    artifact, person, object …) → the verb of the first such sense with a derivational link from this word («arrival»
    → arrive, «destruction» → destroy, «growth» → grow, «death» → die), unless the verb is the noun with a verb-making
    suffix («memory» → memorize, «germ» → germinate, «fright» → frighten: the verb comes from the noun); whether the
    linked verb sense takes an object comes from WordNet's verb frames («Somebody ----s something» …).
  - `prag/data/vnfreq.tsv` — `world frames vnfreq`: for each verb with several VerbNet classes, how many of its
    clauses in our four courses one class alone matched best (578 verbs, 999 rows, 12746 clauses; counted with this
    table switched off). Our own texts.
- **Matching** (`vn::best`): the clause as the tree has it — subject, objects (the indirect one first), prepositional
  phrases with their prepositions (also those the parser hung on the object), the complement's kind (that / to /
  -ing / wh / quote), passive with its by-phrase, an adjective complement — against every frame of the verb's
  classes (the phrasal verb first, then its head; a subclass inherits its parents' frames). Integer score: matched
  arguments, missing frame elements, unplaced objects, unmatched PPs (cheap: adjuncts), the role restrictions
  against the fillers' animacy (global level 1), and the lessons' preference (`vnfreq.tsv`: the class the verb's
  lesson clauses match best most often +2, another that ever won +1 — below one argument, +4: a tie-breaker). The
  core must match (every object and the complement placed), else no frame. (Until 10.10 PropBank's rolesets ordered
  the classes; that is gone.)
- **Roles applied over the heuristics**: an object VerbNet calls a Recipient and that is a person or an animal is the
  recipient (not when it controls a complement: «help her find»); the second object of a dative is the patient
  («gave the Mole a cake»); a PP that is a Destination / Location / Source / Trajectory is the place (never with
  by / with / of / for / about / like / as); a PP that is a Theme, Topic, Stimulus or Instrument is no place
  («look at the cat», «sting with tentacles»); a passive's plain «by» oblique that can act is the agent. The subject
  stays the agent whatever its role (the gold writes «the crop arrives» with the crop as the agent). A verb VerbNet
  lacks keeps the heuristics' roles (PropBank's prepositions for such verbs, ~90 frames, are gone).
- **Speech**: verbs all of whose VerbNet classes are communication (37.x: whisper, mumble, inquire, complain, gossip …,
  152 verbs) are speech verbs too.
- **Events of nouns** (WordNet): a noun of `nominals.tsv` with an argument becomes an Event frame of its verb — «the
  arrival of the Queen» → `Event queen arrive`, «the destruction of the city by the army» → `Event army destroy
  patient=city`, «Alice's answer» → `Event alice answer`. Positions by general rules: «by X» is the agent; «of X» is
  the patient when the verb sense takes an object, else the agent; «X's» is the agent, but the patient of a verb
  with an object when a «by» agent is there or X is a thing and no «of» patient is («the city's destruction»), and
  nothing when an «of» agent is there; other prepositions a place (spatial, placeable) or an oblique. They are
  appended after the sentence's own frames, without links. The bench's gold writes frames for predicates only, so its
  scorer leaves them out (`frames::scored`; `PRAG_VN=score-nouns` counts them); the student gains most from them (the
  formal-language exams).
- **Where the roles came from**: `Frame::how` (`By::VerbNet | Nominal | Heuristic`, the class or the noun, the frame
  pattern or the noun's WordNet file and transitivity, the role of each argument); `world frames` prints it under
  each frame, `world knowledge trace` after it (`⟨vn give-13.1 «NP V NP PP.recipient»: agent=Agent bicycle=Theme to
  boy=Recipient⟩`); the learned rules have it as atoms (vn-class, vn-frame, roles-by `vn` / `nom` / heuristic, vn-agent).
- **Switches** (measuring): `PRAG_OFF=vn` (all), `vn-verbnet`, `vn-obj`, `vn-dative`, `vn-by`, `vn-place`,
  `vn-recipient`, `vn-notplace`, `vn-say`, `vn-nom`; `PRAG_VN=recipient-any,to-recipient,noto-place,noq,score-nouns`
  (tried, worse or no better).
- `world frames lexicon [course …]`: the coverage of the lessons' verbs and nouns. On the four courses (47099 verb
  tokens, 2821 lemmas): VerbNet 96.1 % of tokens (78.6 % of lemmas); the rest are mostly parser errors and dialect
  words. WordNet names an event for 13143 of 89729 noun tokens (1147 lemmas); 1206 noun tokens (357 lemmas) have an
  argument and become an event in the reader (name 76, call 30, rule 26, word 24, amount 20, light 20, work 20 …).
- **Tests only** — `world frames nominal-check`: the WordNet nouns against NOMLEX on the lessons' nouns, NOMLEX read at
  run time from `raw-derived/prag-lexicon/nombank.tsv` in the data directory (never built in).

The licence fix (10.10, PropBank and NomBank out of the build; numbers: test right / understood / exam right over
books, world, biology, everyday; dev and held-out frame F1):

| reader | dev | held-out | test | understood | exams |
|---|---|---|---|---|---|
| main before (VerbNet + PropBank order + NomBank nouns) | .638 | .668 | 6301 | 9746 | 1815 |
| VerbNet only, no noun events (`PRAG_OFF=vn-nom`) | .637 | | 6312 | 9626 | 1672 |
| + WordNet nouns, every derivational link | .637 | | 6371 | 9703 | 1784 |
| + WordNet nouns, same-word nouns only in act/event/process/feeling senses | .637 | | 6339 | 9668 | 1774 |
| + the lessons' class preference (strict nouns) | .637 | | 6334 | 9665 | 1776 |
| + the preference, every link | .637 | | 6367 | 9700 | 1794 |
| **final**: preference, nouns without the verb-from-noun ones | .637 | .669 | 6365 | 9698 | 1790 |

NomBank's noun events gave +143 exams over no noun events; WordNet's give +118 of them back (83 %), and the tests
rise (+64 over main). PropBank's class order was worth one place on dev (.638 → .637) and nothing measurable in the
courses; the lessons' preference is neutral there (−5…−4 tests, +2…+10 exams). Course runs are deterministic (the
same binary twice: identical). School gates hold (ablate 2443, cv book's way 1116).

Licences and attribution: VerbNet 3.4 — VerbNet 3.0 License, © University of Colorado (Martha Palmer), free incl.
commercial with the notice; Princeton WordNet 3.1 — WordNet 3.0 License, © 2011 Princeton University, free incl.
commercial with the notice. Tests only, outside the repo: PropBank frames 3.1 — CC BY-SA 4.0; NomBank 1.0 /
NOMLEX-plus (NYU, A. Meyers) — distributed with permission, free for research, commercial terms not stated.

## Rules the reader learns itself (`prag::rules`, `world frames rules`)

Transformations applied after the reader (at the end of `read_with`), learned error-driven from the gold (after
Brill 1995, as `world induce` learns tree repairs): the bench's dev lines and the teacher's verdicts
(`runs/prag-teacher/log.tsv`: OK → the frame as it was, WRONG → the correction, MISSING → added; scored on the judged
frames only). The held-out lines are never read by the learner. Every rule is a sentence: «For a frame: if the
predicate word is tagged VBN, it has no object in the tree and comes after the word above it, then it has no agent.»

- **Sites**: a frame, a pronoun in a role, a participle on a noun that has no frame.
- **Atoms** (1–3 per rule): the UD relation and tag, own subject/object, marks and linking words, position, the kinds
  of the frame / the frame above / the one before, roles present, the agent's and patient's form and animacy (global
  level 1), verb classes (say / know / control / object control, `global::verb_class`), quoted / if / negated, links
  present; for a pronoun its slot, what it is resolved to and the candidate's form, animacy and gender. Lemmas only
  with `lex` (they overfit).
- **Actions**: role fixes (agent from the frame above / its patient / the noun, clear a role, patient↔recipient,
  oblique → place/patient, swap, not a frame, Say↔Event and the act), links (cause / next with the frame above or the
  one before, clear, says), a pronoun's candidate (previous agent / patient, the line's agent, the speaker, the agent
  above, the most salient before, the most salient person before, unresolved), a frame for a participle.
- **Learning**: each site × action alone; every subset of the site's atoms is a candidate summing the first-order
  change of the micro F1 (1000·right − 500·F1·made) over the sites it matches; the best with precision ≥ .9 and ≥ 10
  lines fixed is applied for real and kept if it gains. Thresholds by `world frames rules cv` (two folds of dev):
  the brief's 3 lines / .8 with lemmas gave in-sample +.073 and −.014 on unseen lines.
- **File** `prag/data/rules.tsv` (compiled in; `PRAG_RULES_FILE` reads another): id, status (proposed / accepted /
  rejected), action, atoms, the sentence, train fixed / broken, the checker's reason. The reader applies only
  `accepted` rules; `PRAG_RULES=proposed` adds the proposed ones (for measuring), `off` none. The queue for the checker:
  `runs/prag-rules/queue.tsv` in the data directory (dev F1 delta overall and per role, lines fixed and broken, before → after).
- Commands: `world frames rules learn | control | cv | show | apply-check`.
- 10.10 (VerbNet reader): atoms vn-class (the top class), vn-frame, roles-by, vn-agent; actions agent-from-by and
  to-from-obl; `delw` (default .5) halves a deletion's gain when candidates are ranked (the checker prefers rules that
  add or move); rejected rules are never proposed again and stay in the file with their reasons; new ids continue
  after the old ones. Learn on the VerbNet reader: 11 rules (r13–r23), all deletions again, dev .636 → .656 in-sample;
  only r22 uses a VerbNet atom. `delw 0 min 6` (runs/prag-verbnet/queue-adds.tsv, not the main queue) finds
  structural ones at weaker evidence: resolve-prev-agent, agent-from-patient, agent-from-head on vn-frame NP V NP,
  next between a noun event and its head (roles-by nb): dev .636 → .646.

10.10, first learning (12 rules proposed, none accepted yet): dev .633 → .655 in-sample, +.007 on unseen dev lines
(CV); held-out once .667 → .676 (next .125 → .000, speaker .333 → .357); the negative control (gold shuffled
between lines) learns only deletions and takes dev to .123. The courses lose with all 12 (books 597 → 587, world
720 → 707, biology 1942 → 1954, everyday 3039 → 2990): the rules cut `next` links and Say frames the student's
«what next» used — the queue says, per rule, what the student gains or loses.

## Clients

- `world knowledge` — the pragmatic student (`PragStudent`): remembers the frames of every lesson line, answers by
  matching the question frame's known roles and following the asked role (cause, next, the role itself, and — a weak
  way — the other frames of the anchor's own line); the generic «you» is no evidence; learns role-level weights only. `world knowledge trace <course> <lesson>` shows the frames behind every choice.
- `world knowledge teacher-check <course> <from> <to> [lines]` — the hidden teacher (Opus) checks the frames of
  sampled lines and logs every verdict to `runs/prag-teacher/log.tsv` in the data directory; it never answers tests.

## Why it lives in `prag` and the parser in `world`

`prag` depends on `en` only, so the types and the tree → frames reader live there. The parser world uses adds the
absurdity-matrix repairs (`world::rerank`, `world::induce`), which belong to world; world passes it in
(`world::frames::read`). Moving the repairs into `prag` would make `prag` depend on world's matrix — a cycle.

## Not yet

Weakest now (dev): `next` (F1 .23 — the gold's sense of «happened right after» is narrow), speaker and says (.32/.38:
dialogue without verbs of saying, quotes the parser splits), place (.39), coref (.42: no world knowledge of who
can do what, names without a gender), agent (.59: parser errors on long book sentences). Tense and time order beyond
`next`, quantities, modality beyond `cond` and Fact(Can), verb classes (global's classes as a weak match cost the
student 184 questions in biology — too broad).
