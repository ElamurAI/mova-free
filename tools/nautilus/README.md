# Nautilus — `mova-nautilus`

Nautilus is one of Mova's animals (Rust on Mova): the dictionary maker. It grows a dictionary the way the nautilus
grows its shell, chamber by chamber: one layer of entries from one course or corpus at a time. See [the sea animals of
Mova](../../docs/animals.md).

It works natively first, on two kinds of layer: a **draft** made from a course or corpus, and an **existing layer**
of the dictionary builder that it improves through review-only proposals (it never writes the layer file). For a
draft: A layer's entries are the commonest content words of its source as Mova annotates them (its
own lemmas and UPOS), with what Mova counted (uses, documents, the UPOS share, the classes of the documents, the words
it goes with), up to three examples with their source ids, and the part of speech and meaning for a pupil from the
pupils' dictionary (`world/data/dictionary-improved.tsv`). Which of the pupils' numbered senses a word has in its
source is Mova's own pick where its overlap is confident ([Senses](#senses)); a teacher is asked only for the rest,
which sense number the examples show, and a new sense only when none fits — and only through the animal's own
`mova-terminal` and the operator's word.

## Commands

    nautilus make CONFIG LAYER                       draft a configured layer natively (no teacher); prints JSON
    nautilus stats LAYER_TSV                         entries, examples and the draft mark of a layer file
    nautilus teacher-task LAYER_TSV SOURCE [N]       print the teacher question for the first N rows (nothing is sent)
    nautilus work CONFIG TEACHERS PROFILE STATE [STEPS|- [SECS]]   the standing loop over the configured layers
    nautilus pool-work CONFIG TEACHERS PROFILE MEMBER STATE ASSIGNMENT REPORT LEDGERS [STEPS|- [SECS]]
                                                     a work-pool member: the job Crab leased in ASSIGNMENT
    nautilus dry CONFIG LAYER [ITEMS]                the first ITEMS (10) items of a layer's job natively: words asked,
                                                     settled by Mova and never asked, question bytes (nothing is sent or written)
    nautilus backlog CONFIG LEDGERS [LAYER]          open teacher questions per layer (native); writes each backlog.json
    nautilus job-task CONFIG LAYER                   print the next teacher question of a layer's job (nothing is sent)
    nautilus pool-seed STATE LEDGERS LAYER MEMBER    seed a job's ledger once from an instance's own work journal
    nautilus replay CONFIG OUT TERMINAL_ROOT...      measure the loop on copies with earlier real answers (nothing is sent)
    nautilus work-summary STATE                      events, throughput, rejection and hold reasons
    nautilus work-resume STATE                       continue after a limit has passed
    nautilus plan DEPUTY_CONFIG CONFIG ID < text     read English, run it with Nautilus's hands, print the answer
    nautilus serve DEPUTY_CONFIG CONFIG SOCKET LOCK  answer English on a private socket (one per LOCK; STOP ends it)
    nautilus ask SOCKET ID [urgent] < text           send English to a serving Nautilus

Build and test (standalone crate with its own workspace):

    cargo build --release --manifest-path tools/nautilus/Cargo.toml --offline --locked
    MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin cargo test --manifest-path tools/nautilus/Cargo.toml --offline --locked

## Layers

`nautilus.example.json` names each layer: `source` (`commands`, `wikipedia`, `wikipedia-examples`, `course` or
`layer`), `out` (drafts),
`base` (the pupils' dictionary), `course` and `model` (commands), `courses` and `model` (course), `corpus`,
`articles` (an optional limit), `file` and `format` (layer), `top` (drafts), `licence` (the line written into a
draft), `licence_class`, `shipped`, `native_picks` (drafts; default true, false asks every word that has examples)
and the `name` used in answers; `wikicorpus`, `per_word`, `seed`, `deep`, `jobs`, `min_count`, `skip` and `examples`
for `wikipedia-examples` (below). Next to `layers`: `senses` (the shared sense store,
an absolute path; absent: none), `batch` (words asked per question, 1–40, default 25) and `pacing_secs` (seconds
between two questions of one instance, at least 30, default 600).

**Licences.** `licence_class` is one of `ours`, `public-domain`, `cc0`, `cc-by`, `cc-by-sa`, `nc`, `research-only`
(`commands` defaults to `ours`; `wikipedia` and `wikipedia-examples` are always `cc-by-sa`; `course` and `layer` must
name it). A `cc-by-sa` layer (its `out` and examples file) is refused inside a Git work tree, so it never lands in
the repository. A layer with
`shipped: true` (part of the repository or a release) is refused with `cc-by-sa`, `nc` or `research-only` inputs
when the configuration loads, and again when a lesson it reads or the layer file it improves declares CC BY-SA in its
`#` header. `stats` and `backlog` show the licence line and class.

- **commands**: the command course (`$MOVA_DATA/school/courses/commands/*.lesson`; only what is taught — every `explain`,
  `fact` and `exercise` line and an exercise's options is a unit, id `commands:<lesson>:<field>:<n>`; the `test` and
  `exam` lines and their options are never read, nor a lesson file named `held-out` or `heldout`) and the command corpus (`$MOVA_DATA/corpus/commands/*.jsonl`, the
  prompts Crab read, id `commands-corpus:<file>:<line>`), annotated by `en.bin`. The repository carries a draft of
  this layer, `$MOVA_DATA/school/dictionaries/commands.tsv`.
- **wikipedia**: the Wikipedia school's corpus (`$MOVA_DATA/corpus/wikipedia/articles/*/ast.conllu`, Mova's trees
  already; id `wiki:<Title>:<n>`; the article's class from `meta.json`). CC BY-SA 4.0 like its source: its examples
  quote Wikipedia and its counts derive from it, so the layer stays outside the repository and any release
  (`licence/DATA.md`); showing it anywhere needs CC BY-SA attribution.
- **wikipedia-examples**: the large lazy English Wikipedia corpus (`corpus` is its root; the internal `wikicorpus`
  tool, not in the public tree, whose binary `wikicorpus` names; without it this layer is not available). Words are sampled, not counted in title order (`src/sampling.rs`): the
  content lemmas of tier 1's word frequencies (`<corpus>/tier1/words.tsv`, from `wikicorpus words`) with at least
  `min_count` (20) occurrences, minus the words of the `skip` layers, ordered by group — two numbered senses or more
  in the pupils' dictionary first, then one, then none — and within a group by `ln(1 + f) * max(s, 1)` (f the
  tier-1 occurrences, s the numbered senses), ties by the word. For each of the first `top` (up to 50 000) words
  the corpus's sampler (`wikicorpus examples-batch`) gives up to `per_word` (100) varied examples, kept whole in the
  examples file (`examples`, default `<out>.examples.jsonl`; appended, a rerun fetches only missing words; a failed
  call stops, no retry). Of those, the row keeps three chosen for the sense question (the question shows the first
  two, cut around the word): the clearest first, then the most different by sense, UPOS, article, topic and head
  word, near-duplicates never. A word with fewer than two usable examples is left out and the next one taken. CC BY-SA
  like `wikipedia`.

- **course**: school course folders (`$MOVA_DATA/school/courses/<course>`), annotated by `en.bin`. Only what is taught is
  read: a lesson's `explain`, `fact` and `exercise` lines (with an exercise's options), id
  `<course>:<lesson>:<field>:<n>`. The `test` and `exam` lines are never read, nor a course folder whose name marks
  it held out (a word `test`, `exam`, `control` or `heldout`, or `held-out`), nor a lesson file named `held-out` or
  `heldout`.
- **layer**: an existing layer file (`file`) in one of the builder's formats (`format`): `records` (the pupils'
  records and the layers built from them: word, part of speech, meaning, sense, examples, note, any extra columns
  kept), `judaism-senses`, `emode-senses` and `mystic-senses` (the builder's senses files). Nautilus finds the weak
  rows natively and asks its teacher only for those; see [Improving a layer](#improving-a-layer).

The draft output has the columns of the school layers (`# word	part of speech	meaning for a pupil	sense in the lessons
examples	note`) and a second line `# draft: …` that marks it as a native draft. The same input gives the same bytes.

## In English

The deputy capabilities (`deputy.example.json`, handler `nautilus`, operations `draft` and `stats` with the layer in
the trusted parameters, no slots): "Make the commands dictionary draft.", "Please build the commands dictionary.",
"How many entries does the commands dictionary have?", "How many entries does the Wikipedia dictionary have?",
"Build the Wikipedia dictionary draft.", and both in one prompt. Refused with a review candidate: negation, a
condition, "Delete …", a dictionary without its name ("Make the dictionary draft."), and a question about a
draft ("Is the Wikipedia dictionary drafted?"): only `stats` (declared `observes`) answers questions. "Draft the
Wikipedia dictionary." is read (a noun reading of the opening "Draft" is repaired to the imperative). Answers are realized from the result, in the form "I
have made the commands dictionary as a draft: N entries with M examples, from L lessons and P prompts. It is not
reviewed yet, and no teacher wrote any of it."

`serve` holds a host-wide singleton lock (`LOCK`): a second start prints "already running as pid N" and exits with
status 5. As a Crab worker it is off by default (`crab-workers.example.json`: `autostart: false`).

## The teacher: its own terminal

`teacher.rs` is the only way to a teacher. The interface is narrow:

```rust
pub enum TerminalError { Limit(String), Failed(String) }
pub trait PrivateTerminal {
    fn id(&self) -> &str;                                            // this animal instance's own terminal
    fn ask(&self, question: &str) -> Result<String, TerminalError>; // the whole answer as text
}
```

`MovaTerminal` is the thin adapter over [`mova-terminal`](../terminal/README.md): `id()` is the `animal` of its
terminal profile, `ask` is `terminal::ask_wait` (the terminal's own lock, its idle and unchanged-screen checks, no
replay while a question is pending, the answer file); an error naming a usage or rate limit becomes `Limit`. One
terminal per animal instance, held by `crab-tmux-worker` in its own session; never shared, never a Dolphin.
`Unplugged` stands in when the terminal profile cannot be read, and the tests use fakes; the tests start no terminal
and call no model.

A teacher profile (`teachers.example.json`) names the terminal id, its `mova-terminal` profile
(`terminal.example.json`), the provider and model labels and the purposes (`dictionary`, `questions`). The public
default is Opus through Claude Code; Sol (`gpt-6.1-sol`) through the Codex CLI is the recommended alternative.
`submit` refuses without `operator_word` in the configuration, for a profile that does not serve the purpose, for
another profile's terminal and for a question over 99 000 bytes (under the 100 000 `mova-terminal` takes); it journals `reserved` before the question and
`answered`, `limit` or `failed` after it; a question already in the journal (including an uncertain reservation) is
never asked again.

`crab-workers.example.json` lists four Crab workers, all `autostart: false`: the English service, the standing loop,
the terminal holder and a work-pool member (`pool-work`).

## Improving a layer

A row is **weak** when, per format: `records` — its meaning is under 40 characters, its sense under 80, or it has
fewer than three examples; `judaism-senses` — the word has no primary record and no part of speech or meaning of its
own (the builder leaves it out of the layer), or it has two or more corpus examples and neither a sense nor an
interpretation; `emode-senses` — its modern equivalent, meaning or difference is empty; `mystic-senses` — its sense
is under 80 characters, or the word has no primary record and no meaning of its own. A weak row is asked only with
evidence (an example that uses the word, or a sense to start from) and never when it is junk ([Senses](#senses)).
Rows with fewer than two corpus examples that use the word (`judaism-senses`, `emode-senses`; Mova's lemmas, scan
boilerplate left out) are **thin** and not asked: their examples are quotations of the corpora, which a teacher does
not write.

`batch` asked rows make one question (`nautilus job-task CONFIG LAYER` prints the next one); rows Mova settles
natively ride along and are recorded as native picks. Each entry shows why it is weak, the current value of each asked
field that has one, the pupils' numbered senses, the store's fixed sense, two examples cut around the word, and its
`answer:` line. Only the weak fields are asked — `records`: meaning, sense, new example sentences (part of speech
only when empty); `judaism-senses`: sense and interpretation, part of speech and meaning only without a primary record;
`emode-senses`: the empty ones of modern equivalent, meaning and difference; `mystic-senses`: sense, part of speech
and meaning only without a primary record. The teacher answers one line per word: the word, the sense pick (a number,
`2,5`, `new`, `same` for the fixed sense, `-` when no senses are listed) and the asked fields, `-` keeping a field. A
line counts with exactly those fields, each `-` or 2–1200 characters with no date, none saying the word is absent
from the examples. At least half of the asked words need a well-formed line; the
proposal holds the lines that change their entry, in the builder's own columns, and a `# pick` line per picked sense
(a sense field that restates a numbered sense is left out of the builder line and marked `restates sense N`): the
examples quoted from the corpora,
the kind, forms, tests and note come from the row, never from the teacher, and part of speech and meaning of a
`judaism-senses` or `mystic-senses` line stay empty when the pupils' dictionary has the word, as the builder writes
them. A proposal is review-only; admission is the builder's separate merge. Every row of the repository's layer files
reads back to its own bytes (`improve::tests::rows_read_back_byte_for_byte`).

## In a work pool

Crab hands out the work ([tools/crab](../crab/README.md#work-pools)): a pool member is `nautilus pool-work`, one
instance per teacher terminal. Between its steps — never in the middle of a question — it reads the lease Crab wrote
into its assignment file (`job`, `part` of `parts`, `lease_id`, `expires_unix`, `params.layer`); an expired, released
or missing lease is no work. Each step works one item of the assigned layer (a draft not made yet is first made
natively), then writes the member's report (`lease_id`, `state`: `working`, `finished`, `stopped` after a limit,
`idle`, `refused`; `open_items`) and the job's `backlog.json` (`open_items`: teacher questions left at most), which
Crab's backlog probe reads.

A job keeps its own ledger, `LEDGERS/<layer>/ledger.jsonl`: every item closed and every question asked for the job,
by whichever member. A question is reserved there before it is asked, and a member never asks a question the ledger
already holds, so a job moves between members — each with its own terminal, teacher journal and pacing — without
asking anything twice. A limit holds the item in the ledger at once. `nautilus backlog CONFIG LEDGERS` writes every
job's `backlog.json` natively; `nautilus pool-seed` seeds a job's ledger once from a loop's earlier journal.

## The standing loop

`work` runs bounded steps (`work.rs`) over the configured drafts (an improved layer is worked by `pool-work`, with the
same loop and its own job): pick the next words of a layer until `batch` of them need a teacher → Mova's own reading
(junk and words fewer than two of whose examples use them are not asked; a confident native pick is recorded) → ask
the teacher which numbered sense the examples show for the rest → keep the answer as a review-only proposal
(`STATE/proposals/<id>.txt`, `# pick` lines) when at least half of the asked words get a usable pick (a `new:` sense
of 20–600 characters that does not say the word is absent; a restatement becomes its sense number), else reject it;
Mova's own picks go to `<id>.native.txt` → journal (`STATE/work.jsonl`) → the sense store → next. One question at a
time, a new one at least `pacing_secs` after the last; a usage or rate limit stops the loop (journaled) until
`work-resume`; a refused or failed question holds its item for a person, except a question that never reached a
terminal (no tmux session or server): it is held once and asked once more when the terminal is ready. A state
folder inside a course or inside an improved layer's folder is refused; nothing is admitted by the loop. Item keys
are `<layer>:<word>+<word>…`; the numeric keys of earlier tens (`<layer>:<k>`) still close their ten words.
`STOP` in the state folder ends a run; one loop per state folder (`work.lock`).

The tests (build commands above) have negative controls for the operator-word, terminal-id, ledger, shipped-licence
and held-out checks, and for lemma matching (a prefix match), the junk length, the absent-answer gate, the
restatement gate and the unsent retry: removing any of them turns its test red.

## Senses

`senses.rs`. The pupils' dictionary gives most words numbered senses (`1) … 2) …`); a question shows each sense's
head (the text before its first example, at most 70 characters) and asks which number the examples show.

- **Words in examples** are matched on Mova's own lemmas (the built-in dictionary of `en`: a token's form and every
  lemma its analyses give, comparatives and superlatives only from UD; the Early Modern English verb endings), never
  on prefixes: «aged» is no example of «ag», «bier» none of «bi».
- **Junk**, never asked: a word under three letters, a scan abbreviation (`ibid`, `isbn`, `http`, …); examples
  holding scan or licence boilerplate (URLs, `license:`, `versionSource`, "scanned at", "notice of attribution", …)
  do not count as uses.
- **Native picks**: the content lemmas the examples share with each sense (a quoted phrase of a sense found in an
  example counts three). Confident when the word has one sense and they share a lemma, or when the best sense shares
  at least two lemmas and two more than the next; then nothing is asked and the pick is recorded with its evidence.
  For a layer row only when every asked field is one a pick settles (a sense or interpretation).
- **Leaner questions**: two examples per word cut to about 160 characters around it, without source ids; no counted
  line; only the weak fields.
- **Gates**: a line saying the word is not in the examples, or that they cannot establish a sense, is no answer; a
  written sense whose content lemmas mostly repeat a numbered sense (Dice ≥ 0.5, at least three shared) is marked
  `restates sense N`, not a new sense.
- **The sense store** (`senses` in the configuration, one JSONL file shared by every layer and instance): each pick
  as `{word, pick, sense, layer, by, evidence, admitted: false}`. A word's first numbered or new pick fixes its
  general sense; a later layer shows it (`fixed: N (by layer)`) and asks only whether this layer uses the word
  differently (`same`); where Mova's own pick agrees, nothing is asked.

`nautilus replay CONFIG OUT TERMINAL_ROOT…` measures all of this on copies: it reads the earlier questions and answers
of the terminal roots, copies each layer's asked rows to `OUT/layers/`, runs the loop over the copies with a terminal
that answers from the earlier real answers (a draft's sense as `new:`, a layer's asked fields from the old columns),
and reports questions, bytes, words per question, native picks, junk, restatements and rejections. Nothing is sent.

## Scope

Nautilus makes native drafts and review-only proposals for the layer formats listed above. It does not merge
proposals into canonical dictionary layers and does not publish a dictionary site; admission (`world dict admit`,
[building the dictionary](../../docs/training.md#building-the-dictionary)) and a teacher step authorized by the
operator stay separate.
