# prag-frames

## model

# Supplied trees into discourse frames

## forward

On finite acyclic supplied UD trees and fixed configuration/lexicons/rules, compose lexical entities (with a name role, Ent::names: noun compounds, proper-noun compound/flat words, appositions, named «of» modifiers), predicate frames (an obj/obl/nmod with a time value adds when:<value> and when:<year>, an event under during/after/before/since/until/throughout adds when:<noun>; PRAG_OFF=wh-time), clause links, question gaps and sentence offsets. read_with snapshots salient before-context, calls the supplied parser per unit, skips empty parses, concatenates frames and shifts local indices by preceding frame count. scored optionally removes nominal frames and remaps retained edges.

## behavior-complement

Universe U pairs admitted supplied inputs/configuration with candidate observations. Exclude a retained local link shifted by any amount other than the preceding frame count, a right unresolved head matching coordination, or a nominal target surviving scored when score-nouns is off. These are inner exclusions for selected projections, not all absent reader behavior.

## reverse-reconstruction

For observation y retain every supplied tree/context/configuration yielding y under the selected projections. Different surface forms and modifier/provenance fields can share head identity; dropped nominal frames and empty parses leave ambiguous antecedents. No inverse parser or complete antecedent solver is supplied.

## invariant

Token and frame coordinates must be kept distinct. Sentence-local anchors correspond to last sentence only; Read.pred carries sentence/token coordinates and frame links carry global frame coordinates.

## cost

Tree assembly O(W); predicate reading includes repeated scans, recursive phrase copying and lexicon lookup. Conservative cost O(W squared plus recursion/lexicon/rule work plus output); no finite global bound for arbitrary dependency cycles, effectful parser callbacks or configured resources.

## coordinate-join

For sentence s with base b, a local frame index j maps to b+j, while predicate token k maps to (s,k). Root, asked, quote, cause, next and link must use the appropriate map. Native question/read and scored tests check distinct instances.

## composition

Pipeline composition requires shared context/configuration and preserved predicate order, not independent child marginals. The seven fragments are partial projections; recursive malformed trees, full lexicon role ranking, all TAM/repair/quotation cases and external active-rule effects remain gaps.

## epoch-memory

Hyphenated adjectives and coordinated phrases. Model: (1) `hyphen_adj` — a VERB token with base relation
amod directly after a «-» punct token (and a first part before it) is the second part of a compound adjective
(«short-eared», «fast-moving»), so `is_pred` is false for it: no predicate frame («Event _ ear»), the noun keeps the
lemma among its attributes, the tree is unchanged (PRAG_OFF=hyphen-adj). (2) `pp_conj` — a conj noun with its own
case marker whose head has none («8 eyes, usually in two or three rows»: rows/conj>eyes with «in») is not a
coordinated entity (left out of `and`, `conj` and the phrase's kept words) but a phrase of the verb, read like the
parser's other misplaced object modifiers (place=row; PRAG_OFF=conj-pp). Complement: a plain participle modifier
(«a sleeping owl») keeps its frame; spaced hyphens and other tokens are untouched; «to London and to Paris» (both
conjuncts with case) stays coordinated. Reverse: the absent frame is recoverable only from the tree. Measured with the
student (world decision groupant): no changed medium-tier answer contains a hyphenated -ed/-ing word; hyphen-adj alone
6 811 → 6 796, exam +5; conj-pp alone −9, exam +9 (learning-path changes); all three changes full tier 19 813 → 19 817,
exam +18. Tests complete_tests::a_hyphenated_compound_adjective_is_no_predicate and
a_coordinated_prepositional_phrase_is_the_verbs_phrase (each red with its switch off).

## limitations

Partial syntax is not expanded semantic AST. No live parser, teacher, whole corpus, GPU or environment mutation. Acyclic input trees and fixed configuration/resource premises matter; malformed recursive ancestry, counter overflow, complete TAM/lexicon/quotation/repair semantics and all active-rule effects remain open. No executable approximant/error budget is claimed.

## evidence

### Evidence

External registry scaffold validation passes twice identically; only the owned source and sidecar are committed. Structural identity checks do not establish semantic truth.

## History

### 2026-10-30 · epoch-memory

Wave15, 2026-10-30. Prospective repair: reject unresolved right heads in identity with a constant six-item guard, preserving coordination membership and its nontransitivity. Old production failed unresolved_identity_is_symmetric_even_in_coordination; repaired code passes. Compiled guard-removal control fails the same named assertion and exact bytes are restored. Eight focused tests pass twice debug and twice release. No speed, full semantic coverage or universal proof claim.

### 2026-10-30 · evidence

Source SHA-256 a7914e172b612868c5455d7bda07790e093d74043ca72196b708fcec5da64913. External receipt set octopus-wave15-frames-2026-10-30. Eight native tests repeated twice debug/twice release with identical named outcomes. Old code and compiled production guard-removal mutant fail unresolved_identity_is_symmetric_even_in_coordination with unresolved right operand it matched coordination. Exact source restored before positives. Syntax staged and generated twice identically. Existing unused-mut compiler warning remains. No full suite or full semantics claim.

- old.log: 84c3880fe924e735efbf11f983b72a63cb21d3a51fe04c6c152583211f5e3523

- mutant.log: eff4a0a6948665224ff7425814e32c568b82288cc82f4526086d461f895726d2

- debug-1.log: ca48306081a7124ab77c825b7d4aa281d68411197a0ad00381b2628e66b8ed3d

- debug-2.log: 868fcbed76f1ad0517ff0568e5b166b83a2b0e57632f557eb19415029311946c

- release-1.log: b607d8aacfe8d2c5ad860fc6c7c9d1dc5f19e76e88bddc11a02e9f5d322ee58c

- release-2.log: 43f5096105d08031326d36641cb19aaeff3e9b789525517cd8b1d2b6133e9ff1

- debug-1-named.txt: 3bbae3dcb9d0dd077314f1da2e1df0ce1a81af4f1627ffc229cb3efad280342f

- debug-2-named.txt: 3bbae3dcb9d0dd077314f1da2e1df0ce1a81af4f1627ffc229cb3efad280342f

- release-1-named.txt: 3bbae3dcb9d0dd077314f1da2e1df0ce1a81af4f1627ffc229cb3efad280342f

- release-2-named.txt: 3bbae3dcb9d0dd077314f1da2e1df0ce1a81af4f1627ffc229cb3efad280342f

- syntax-1.json: 5066d15e14928110987916a55b928299985e262d1e968040c5f6ddb5a5db4e86

- syntax-2.json: 5066d15e14928110987916a55b928299985e262d1e968040c5f6ddb5a5db4e86

- groupants-1.json and groupants-2.json: e96799fe3a0b42789b431e3ddbcc3d60244a0c386f9b2f0c9da1effa7e34fa4b

- changed.json: 27f534c6668aebeb3a4d5b045d9c7a1ad13bf75ed28fdda2adac2d84dcbb11f0

### 2026-11-03 · evidence

Speaker-label receipt, 2026-11-03, source SHA-256 57539e706753c6525e72b0cf977d6542ac73139c3194af99971ced9618caca2e. Three native tests (cargo test -p prag --lib speaker_label) pass in release; the finite-verb guard-removal mutant fails what_is_no_speaker_label_stays_byte_for_byte on "Your mate fell off the wall"; exact source restored before positives. PRAG_OFF=speaker-label reproduces the previous commands trace byte for byte. Syntax regenerated. No full suite or full semantics claim.

### 2026-11-04 · epoch-memory

Rule sentences, 2026-11-04 (student-rules). `implications(fs)` maps the frames of one reading to rules condition ⇒ conclusion using only the reader's clause links and markers: an advcl clause marked if/when/whenever/once ⇒ the chain of its main clause and that clause's xcomp/ccomp complements (a speech condition contributes its quoted frames); the quoted csubj/nsubj of mean/ask/tell (or, with no agent, what the verb quotes) ⇒ its complement chain; a cause link C of K ⇒ K. Frames coordinated with a condition join it; «do» with it/that/this/so in the conclusion is marked as the condition's action. Behavior complement: no rule from «unless», from a negated «mean», from markers outside the four, or where either side is empty; no reading is changed (frames, links and traces stay byte-identical). Reverse reconstruction: a rule names its frames but not the sentence; many link layouts give the same rule. `interrogative(r, f)`: an auxiliary before the subject of the frame's predicate. Prediction and measurement are in the student (world/src/knowledge.rs, decision groupant). Tests implication_tests: the if-request and the mean-pattern compile; a «that» clause and a subjectless «mean» state no rule; «you can pass» is not asked. Mutant accepting any marker as a condition fails the «that» control (exact source restored).

Time and name roles, 2026-11-04. Prediction before code: the Wikipedia school's when-questions are all guesses because the reader asks Frame and keeps only when:<lemma> (the year lost), and which-questions tie because the answer is a name inside the remembered entity; a whole time value as a when: attribute, when asking Role(Attr), and a name role (Ent::names) should lift when to ~60-75% and which to ~55-70% with the articles-shuffled control flat. Measured (CPU, peta build, train and cv courses, two identical runs): when 259 -> 881 of 1106 (shuffled 263), which 65 -> 200 of 214 (shuffled 64) on wikipedia; cv-a 143 -> 457, 27 -> 90; cv-b 155 -> 444, 24 -> 106; subjects -12 net on wikipedia. Ten school courses 3234 -> 3248 of 7283. Tests wh_role_tests: page 29 and their march have no time value, a cup of tea names no cup; the mutant that accepts any month word fails the time test (exact source restored).

### 2026-11-04 · limitations

Event questions, 2026-11-04 (student-verb). `statement_order` never takes a preposition for the main verb after do-support (PRAG_OFF=q-do-prep: «the population ofed the valley grow»). `repair_wh_passive` (questions only, PRAG_OFF=wh-passive): a when / where root with a copula «be» whose subject carries a past-participle acl, or a subject noun carrying the copula and the wh-word under a participle, becomes the participle's passive with the wh-word its adverb («When was the college founded by Y?» asks `Y found patient=college` when:?). In `frames_of_tree_with` (PRAG_OFF=wh-event-noun) a time or place asked of «happen», «occur» or «take place» (no determiner) moves to the nominal frame of its subject noun. `gerund_nom` (PRAG_OFF=vn-gerund): a noun -ing form of a VerbNet verb with an «of / by» argument names that verb's event (transitive when a class has «NP V NP»). Complement: «take a place», a gerund without an argument, a participle without a wh root, and agent nouns («Who was the founder of X?») are not read as events. Reverse: the moved ask names the event frame; the «take place» frame keeps its subject. Tests event_question_tests (the unrepaired tree asks «be»; «take a place» keeps «take»; a bare gerund names no event; each switch off makes its assertion fail).

### 2026-11-04 · evidence

Time and name roles receipt, 2026-11-04, source SHA-256 0dc8bae8fbaecb483282ae3cb0da28501fe49edbd55481eeeb18868ff09871fc. Two native tests (cargo test -p prag --lib wh_role_tests) pass in release; the mutant accepting any month word (not only a name or a season) fails a_time_value_is_a_date_or_a_year on «their march»; exact source restored. prag 146 lib tests and world 451 lib tests pass in release (world with its harness root); qagen, yomhadin, translate, roundtrip pass. PRAG_OFF=wh-time,wh-which reproduces the previous traces of the ten courses byte for byte (the step's off-run equals the earlier default).

Rule sentences receipt, 2026-11-04, source SHA-256 34ba2906a706e748847ae1e553ab877c5579c9e9a59dc2f0295a4fa09e205e36. Two native tests (cargo test -p prag --lib implication_tests) pass in release; the mutant that takes any clause marker as a condition fails rule_sentences_give_condition_and_conclusion on the «that» control; exact source restored. prag 148 lib tests, world 455 lib tests (harness root wave28-dialogue-*) and tools/fish tests (20 + 1) pass in release. The reader's frames are untouched: PRAG_OFF=student-rules reproduces the ten courses' traces byte for byte.

Event questions receipt, 2026-11-04, source SHA-256 bfbc2f7905d878080a7e7dd30ae7f1bfe8f07313c3a4cf8f0254be29e72a4b88. Three native tests (cargo test -p prag --lib event_question_tests) pass in release; with PRAG_OFF=wh-event-noun, vn-gerund or q-do-prep the matching test fails; the repair test holds its own unrepaired control. prag 151 lib tests, world 453 lib tests (harness root wave28-dialogue-*), tools/fish 20+1 pass. Measurements: docs/training.md section 31.

### 2026-11-05 · epoch-memory

Hyphenated adjectives and coordinated phrases, 2026-11-05. Model: (1) `hyphen_adj` — a VERB token with base relation
amod directly after a «-» punct token (and a first part before it) is the second part of a compound adjective
(«short-eared», «fast-moving»), so `is_pred` is false for it: no predicate frame («Event _ ear»), the noun keeps the
lemma among its attributes, the tree is unchanged (PRAG_OFF=hyphen-adj). (2) `pp_conj` — a conj noun with its own
case marker whose head has none («8 eyes, usually in two or three rows»: rows/conj>eyes with «in») is not a
coordinated entity (left out of `and`, `conj` and the phrase's kept words) but a phrase of the verb, read like the
parser's other misplaced object modifiers (place=row; PRAG_OFF=conj-pp). Complement: a plain participle modifier
(«a sleeping owl») keeps its frame; spaced hyphens and other tokens are untouched; «to London and to Paris» (both
conjuncts with case) stays coordinated. Reverse: the absent frame is recoverable only from the tree. Measured with the
student (world decision groupant): no changed medium-tier answer contains a hyphenated -ed/-ing word; hyphen-adj alone
6 811 → 6 796, exam +5; conj-pp alone −9, exam +9 (learning-path changes); all three changes full tier 19 813 → 19 817,
exam +18. Tests complete_tests::a_hyphenated_compound_adjective_is_no_predicate and
a_coordinated_prepositional_phrase_is_the_verbs_phrase (each red with its switch off).

### Undated · document

Subject: prag/src/frames.rs SHA-256 bfbc2f7905d878080a7e7dd30ae7f1bfe8f07313c3a4cf8f0254be29e72a4b88. Partial source model, finite native evidence. Fixed configuration and admitted finite resource domains; source-derived inner exclusions and ambiguous preimages, no inverse solver.
