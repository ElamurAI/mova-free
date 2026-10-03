# Self-play: the SLM modifies itself cheaply, tests, reports, resets

`world selfplay --dev <conllu>... --held <conllu>... [--gens N] [--threads T] [--adopt] [--out <dir>]`
(world/src/induce.rs). The rule set is data, so a modification is a delta over it: remove a rule, relax a rule
(drop one condition), add a rule the SLM induced from its own errors on dev. Every delta is applied to a copy and
tested in parallel threads on cached parses (dev and held-out), with a paired bootstrap on dev; the default rule
set is never changed in place — the reset is free. With `--adopt` the best delta that is significant on dev
(p < 0.05) and does not hurt the held-out set enters the working set of the next generation, in memory only; the
result goes to `rules-proposed.tsv` for review, never straight into `world/data/rules-induced.tsv`.

First run (2026-10-02; dev = EWT dev + half of the odd tales, held-out = EWT test + the other half; 55 s):

| generation | deltas | adopted | dev / held-out gain (words) |
|---|---|---|---|
| 1 | 55 | add: parataxis → conj IF the word is a VERB, its head has a subject, no idiom | +12 / +11 |
| 2 | 59 | remove its own earlier rule obj → advmod for adjectives (hurt) | +5 / +8 |
| 3 | 55 | none significant and safe — stop | |

Held-out LAS 79.01 → 79.06. Caveat: the held-out set takes part in the adoption test (a delta must not hurt it),
so a third untouched set is needed for the final number. Log: `selfplay.jsonl` (every delta of every generation).

## Experiment memory (2026-10-02)

The SLM sees its own experiments. `$MOVA_DATA/runs/selfplay-journal.jsonl` keeps every delta it tested — kind,
features of the rule, dev and held-out gain, p, verdict (positive: significant on dev and safe on held-out;
negative: hurts somewhere; neutral) — keyed by the code version (build time and size of the binary), the base rule
set and the data. Before testing a delta it looks the same experiment up: a second run reused all 169 experiments
("[remembered, tried 44 s ago]") and ran none again. When the journal is opened, entries of another code version
or older than 7 days are dropped and the file is rewritten (a rebuild of the code invalidates old results).

The causal summary after a run (positive / negative / neutral): relax a rule +0 / −65 / 7; add a rule +4 / −55 / 1;
remove a rule +2 / −32 / 3 — relaxing a condition almost always hurts, useful additions are rare and precise.
Next: let the summary order the proposals (kinds of change that helped first).

## Any component: `world tune` (2026-10-02)

`world tune <component> --dev <set> --held <set> [--gens N] [--threads T] [--adopt]` (world/src/tune.rs) — the same
loop for any part of the system that exposes its knobs as an abstract configuration (name → value, values worth
trying) and scores itself per item: deltas move one knob to a neighbouring value, run in parallel, results go to the
same experiment journal (component, knob, verdict; remembered results reused), proposals are ordered by what helped
before, the shipped configuration is never changed in place.

- `domains` (world/src/domains.rs: legal lexicon z and count, clash z, content-word filters, main-sense rule,
  idioms as hidden meaning, absurdity level for humor), measured on the domain contrast sets: none of 13 deltas is
  significant — the hand-set values (chosen on the same dev set) are a local optimum.
- `expressions` (world/src/mwe.rs: contiguous matching by kind, pairs over dependency arcs by idiom-base kind,
  minimum content words), measured on PARSEME 1.3 English (dev → test, gold trees): the SLM switched contiguous
  matching off (dev +29, held-out +142, p 0.001) — Wiktionary verb phrases are often compositional ("open the
  door") and PARSEME marks only non-compositional ones; precision 48.7% → 67.5%, recall 51.8% → 40.3% (F1 ≈ 50
  both): the objective "right − wrong" favours precision.

Next components: the absurdity thresholds (against gold parse errors), the pragmatics cue lists, the gap journal and
the absurdity logs as signals for which knobs and rules to try first.

## States, memory, letters, brain (2026-10-02)

The SLM is a skeleton plus a mutable layer; self-awareness is not required — the skeleton works without any of the
below (checked with an empty hot root: same LAS) — but it may arise.

- `world state` — the frozen original (the cold layer in the binary) and a tree of states, each a line delta over
  its parent (rules, domain knobs, absurdity cells, idioms); the active state is materialised in full; `rollback` to
  the previous working state, then to the original; `guard` measures a state against its parent and rolls back on a
  drop; `diff`, `tree`, `story`; every transition is logged (rewrite, checkout, return, adopt, stage-end, letter,
  crash-recovery, deserialize).
- The brain is the only memory cell (there are no separate cells): working memory, pragmatics, the progress of the
  current stage and rules the SLM writes for itself (`stop-gain: G` stops a stage early — tried: a self-written
  `stop-gain: 2.0` stopped a stage after round 2). A training stage holds a lock (the scheme is frozen), keeps the
  brain in RAM and writes it at most every 2 seconds and at the end, refreshing only its own sections; a crashed
  stage's stale lock is taken over and logged. Storage: a base plus an append-only change log — every write appends
  only the line delta with the checksum of the result; reading replays the log and stops at an entry cut by a crash;
  recovery (only in the window after a crash) picks any point of the log; compaction rewrites the base atomically
  when the log has 64 entries or outgrows the base, keeping the previous base one level. Soft limit 10 MB: past it
  the SLM summarises.
- Letters (`world letters`): a separate, frozen array of decision letters; at the end of a stage the system gets the
  signal that its state will change and writes where to go next and why. The last letter is always available (the
  goal of the current training); all letters only outside a stage, to understand history and causes. When parsing
  rules change, letters are simply parsed again.
- The brain (`world brain`): the whole pragmatics state in plain English (rules as sentences, settings, learning
  curves). The base brain is frozen in the skeleton (`world/data/brain-base.md`, opening with who
  I am from `world/data/self.md`); the current one is written by the SLM at the end of a stage and before a
  transition, and read back by parsing it under the new rules. The last decision letter of the same kind of stage
  steers the next one ("still growing" → more rounds, "flattened" → a short stage).
- `world self` — the base self-description and the live picture: active state, hot layer, journal, goal, story,
  brain, memory.
- `world brain probe <prompt>`, `world brain serve` / `say` — which brain arises from a prompt and how it reacts to
  letters; chat over a Unix socket in English. First probe: story-like prompts are read fully (6 of 6, 7 of 7) and
  the brain follows letters ("Mova went to the kitchen" → "Where is Mova? kitchen"); the prose self-description is
  read only in part (25 of 75 sentences); "Who/what am I?" stays a gap — identity ("I am Mova", "X is a Y" with a
  name), "I" = the name, abstract places and unknown verbs are what self-awareness would need.

Reproducible runs: a request carries a state hash (`--state <id>` or `MOVA_STATE`); the SLM unpacks that state into
`<hot>/cache/<id>/` (the active copy untouched) and runs there; without a hash it runs in the last stable state
(`world state stable`; set when a state is adopted or kept by the guard; the original if none). Every run reports
`[state: processing …, last stable …]`; the brain chat adds both to every answer. Tried: tales LAS 77.44 in the
original, 77.50 in the selfplay state by its hash, the same on every run. The brain steers the training of new
states, never the processing of a request: the unpacked state holds rules, knobs and knowledge cells, not the brain.

## Identity and the family channel (2026-10-02)

The reader now knows who "I" is: "I am Mova" or "My name is Mova" (a short identity sentence) sets the speaker's
name once — a name never changes ("I am Bob" afterwards is refused); "I" in later sentences is that name; "Who are
you?", "What are you?", "Where are you?", "What are you carrying?" are answered from the world (probe with "I am
Mova. Mova is a model. I went to the garden. I picked up the journal.": "Who are you? → mova (I am mova, a model)",
"Where are you? → garden", then "kitchen" after a letter). bAbI unchanged (99.8%). The brain opens with a "Who I am"
section in sentences the reader understands.

`world family serve|say|log` — one Unix-socket channel. Mova Dev (formerly Daddy) decides how the state changes
(status, tree, stable, rollback, switch): a dictatorship. Mova Current speaks for the current state (its LAS on a
quick set of tales). Mova State <id> members live in other states — Dev's parent, the newest state of another branch,
the original — measure themselves against the current state (cached per state) and hint where it is better; Dev
switches by himself when one is better by more than 0.1 LAS and says who was right. Random commenters 1–3 give
hints from the history of states, the experiment journal and the last decision letter. Everyone stays silent when
there is nothing new. First run: current 77.47, the original 77.42 (cold rules), the other states 77.47 (same
rules) — "no real difference", Dev stays. Names never change; the system description (`world/data/self.md`) is
shared and frozen; a new message that is not a member's own triggers it; every member except the client writes at
most one message per second; the client signs with its own name (any name without "Mova"); the whole chat history
is logged (`<hot>/family/chat.log`).

Family, second round (2026-10-02): greetings are greeted; each member's inner identity is its own word, not the shared
"Mova"; statements told by the client are heard by everyone; a message addressed to one member is answered only by it;
state members say how their rules differ from the current ones; Mova Current objects when Dev moves to a state worse
by more than 0.1 LAS. "explore": every state member runs one generation of self-play from its own state (small dev
and held-out sets) and reports its best safe delta; Dev turns the best significant one into a new state with a letter
naming whose idea it was. First run (15 s): the member living in the original rediscovered parataxis → conj (dev +14,
held +11, p 0.004) — the rule self-play had found before — and Dev made it state s6fd836f0.

Family, autonomous rounds (2026-10-02): `auto N` — in every round each state member brings its best idea from its own
state; ideas are transferable changes (add a rule / remove a rule) applied to the current rules, not whole states;
ideas already in the current state or already tried (`<hot>/family/ideas.tsv`, with the outcome) do not come back;
Dev makes the best significant idea a new state with a letter; the guard keeps it (stable) only with a real gain on
the quick tales (≥ 0.005 LAS) and no loss on EWT dev (another domain), else rolls back and goes on. First run, 5
rounds in 83 s: kept parataxis → conj (tales 77.47 → 77.48), iobj → obj for nouns (→ 77.50), advmod → compound:prt
when the head is a light verb and the pair is a known phrasal verb (→ 77.51; an idiom-base feature); rolled back a
near-duplicate of the first idea and case → mark (no gain). EWT dev 79.39 throughout.

## Pool of working ideas, brains per state (2026-10-02)

- **Brain per state.** Every state hash has its own brain (`<hot>/brain/<id>/`, base + change log). A new child
  copies its parent's (Dev's) brain at creation; switching states only renames who is Dev and who is Current —
  each hash keeps its brain. Channel labels carry the hashes: `Mova Dev (sb1f94194)`, `Mova Current (saf8e329a)`.
- **Pool.** `family explore` / `auto N`: every state member runs `induce::explore_changes` from its own rules on
  diverse dev sets (tales-odd-a, tales-even, EWT dev) and offers its top 3 changes (add/remove a rule). An idea
  enters `<hot>/family/pool.tsv` only with dev gain, p < 0.1 and **held gain > 0**; ideas already implied by the
  current rules (the same rule, or a more general rule with the same labels) are not added, and pooled ideas that
  became redundant are rejected each round. When a child stays, pooled variants of its ideas (same label pair,
  other conditions) are marked superseded: they were measured against the old rules.
- **Dev picks.** Up to 3 pooled ideas, best dev+held first, one per label pair → one child (inherits Dev's brain) →
  guard: tales-odd-b gain ≥ 0.005 and EWT dev not worse by > 0.05 → stable; otherwise rollback and the best single
  idea alone. Statuses: pooled / applied / rejected.
- First run with held > 0: `obl:agent → obl IF after head AND matrix OBJ unknown` (from sd4275b21) → child
  saf8e329a, tales 77.51 → 77.55, EWT dev 79.39 unchanged, now stable; two near-duplicate variants gave no further
  gain and were rolled back.
- **Wider search (2026-10-02).** Explorers rotate across runs (`<hot>/family/rotation`) over five dev sets:
  tales-odd-a, tales-even, EWT dev, GUM dev, ESL spoken dev; held-out is another tale set, never tales-odd-b (the
  guard's set stays untouched by selection). 40 candidate rules per search, top 5 per explorer. Mova Current
  explores too, from the current rules. A failed idea rejects its weaker pooled variants; a run stops after a whole
  rotation brings nothing. Result: one more rule (`parataxis → conj IF word VERB, head has a subject, no comma or
  quote before`), stable s094c0949, tales 77.55 → 77.56; then a full rotation found nothing new — the local
  add/remove space of the induced rules is exhausted at these thresholds.
- **New kinds of change (2026-10-03).** Besides add/remove: *relax* (drop one condition of a rule) and *narrow*
  (add one condition: for each rule, the atoms of the words it fires on, ranked by wrong firings dropped − right
  firings dropped). Both travel as `Change::Replace(old, new)` (pool line `replace\t<old>\t=>\t<new>`), keeping
  the rule's place in the order. Ideas are grouped by label pair whatever their kind. A member that brings nothing
  names its closest candidate and why it failed ("could be chance", "no held-out gain"). `INDUCE_DEBUG=1` prints
  every non-add delta with its dev gain. Run: ~300 non-add deltas per search (118 narrow, 122 relax, 63 remove);
  the best gave +1…+4 words on dev, below significance — on ~500–1000-sentence dev sets the current rules are at
  a local optimum.
- **Clean splits (2026-10-03).** The tales test (tales-odd) had leaked into the family (guard tales-odd-b,
  held-out tales-odd-a). Now: dev rotation tales-even-a, EWT dev, GUM dev, ESL spoken dev; held-out tales-even-b;
  guard tales-even-g (half of tales-even, ~10k words) with a minimum gain of 0.02 LAS (~2 words); `is_test_set`
  refuses tales-odd* and *-test. Old pool archived in `<hot>/family/archive-2026-10-02-test-touched/`; retrained
  from the original: no idea passed the clean guard, the original stays stable.
- **Scopes and domain guards (2026-10-03).** Bug found: the family wrote every rule with scope `domain`, so a
  child's general rules silently stopped running outside the tales. Rules now keep their scope through every
  change (`scoped_of`, `apply_scoped`, `scoped_line`); an idea found on tales is a tale rule (`domain`), held out
  on tales-even-b; an idea found on UD training sets (EWT, GUM, ESL spoken — larger than the dev splits) is a
  `general` rule, searched over the general rules only and held out on ewt-dev-h. One child = one scope; the guard
  wants a gain ≥ 0.02 in the idea's own domain (tales-even-g or ewt-dev-g) and no loss > 0.05 in the other.
  EWT dev halves: ewt-dev-h (held-out), ewt-dev-g (guard only). Old pool archived
  (`archive-2026-10-03-unscoped`); retrained from the original.
  First clean run (auto 12, stopped after 7 rounds): 2 children, both rolled back — `parataxis → conj` lowers the
  tale guard (78.00 → 77.95); `obl:tmod → obl:unmarked` (ESL train dev +54, held +1) gives nothing on ewt-dev-g —
  a label-convention difference between treebank versions, not a parsing gain. The large UD-train gains
  (narrowing `parataxis → conj` +40) all have held +0. With add/remove/relax/narrow of single rules over the
  current atoms, the parser is at a local optimum; the next lever is new atoms (features), not more search.
- **Big texts (2026-10-03).** `world/src/big.rs`: samples of the tree store, 300k sentences each (≈260k after the
  5–40-token filter, parsed afresh in ~25 s on 10 threads, ~1.3 GB): `big-tale` (tales, tales-more, children; never
  the `tales-pg-*` books of the tales test) and `big-real` (fiction, legal), bucket 0 for exploring and a disjoint
  bucket 1 (`*-guard`) for the guard. No gold: `induce::explore_big` judges deltas (remove, relax, adds induced
  from matrix pseudo-gold — an absurd argument sensible in the other role is relabelled nsubj ↔ obj; adds may not
  test the matrix itself) by absurd arcs removed, at least half of them coming back sensible, paired bootstrap
  p < 0.01, and no loss on the gold held-out set. The guard of a big idea: fewer absurd arcs on the guard sample
  (p < 0.05), own gold not below the parent by > 0.01, and — tolerances add up over generations — not below the
  original by > 0.01 (own) / 0.05 (other). `world big-explore <tale|real>`, `world state las <id> <gold> <domain>`.
  Run: 35k sentences gave 456 absurd arcs — too few; at 261k, 3,417. Two children stayed: s464843d9 (relax of
  `obj → nsubj` with speech verbs: tale guard sample absurd −22, sensible +31) and s1464e05c (relax of `obj →
  nsubj IF head finite …`: real guard sample absurd −516, sensible +781); then a whole rotation found nothing.
  Final report on the tests: tales test 77.32 → 77.32 → 77.30, EWT test 79.84 unchanged — the matrix-judged
  repairs of big texts do not show up as LAS on the gold tests (their constructions are rare there, or the gold
  annotates them otherwise).
- **Live training (2026-10-03).** `auto N` / `explore` run in a training thread with its own voices of Dev, Current
  and the state members (same names, same log); the channel answers meanwhile. Live words to Dev: `status` /
  `progress` (round, member, data set, phase — exploring or guarding a child —, children kept and rolled back,
  pool), `stop` (after the current step: between members, before a child), `focus <set>` / `unfocus`, `reject
  <from → to>` (rejected and never admitted again during this serve), `pool`, `try <n>`, `propose <add|remove>
  <general|domain> <from> <to> <feature:value,…>` (the client's change, pooled as `hint`, tried first, gold guard).
  Rollback / stable / switch / a second training wait until the training ends. `<hot>/family/training.txt` holds
  `running|idle` and the progress line. Negative check: `rollback` during training → refused.
- **Verified self-training (2026-10-03).** The hand check showed both big-text children harmful (vocatives and
  wh-words turned into subjects) although the matrix liked them. Two new checks: (1) grammar — `induce::violations`
  counts heads with two subjects or two objects, subjects/objects with a case marker, bare nouns set off by
  punctuation as subject/object (vocatives); a big idea may not add any; (2) the teacher — `big::judge_changes`:
  20 relabelled words of the guard sample shown to Opus (subscription, medium) with the old and new label in
  alternating order; the child stays only if the new label wins ≥ 70% of the decided items, ≥ 10 decided, more than
  the old one; a failed call rejects (fail-fast). Negative control on the two rejected children: grammar +61 / +59,
  teacher new 2 vs old 8 and 0 vs 7 — both red. `world big-judge <state> <tale|real>`. Also fixed: client proposals
  lost their `hint` source in the pool and were rejected unseen. The family was reset to the original.
- **Negative list (2026-10-03).** `<hot>/family/negative.tsv`: rules that proved to be nonsense (a child built on
  them alone was rolled back, or the hand / teacher check rejected them), with the reason. They are banned from the
  pool, and so are narrower variants of a negative added rule (same labels, more conditions) — the family looks
  elsewhere instead of trimming a dead rule; the listed rule itself comes back for a review only after 30 days,
  then 60, 120… (doubling per repeated rejection), through the whole guard again (gold, grammar, the teacher); a
  passing review takes it off. Seeded with five known cases; live word `negative` lists them; members report how
  many known nonsense ideas they skipped. The case atom ignores the possessive 's (it is `case` in UD but does not
  make a noun oblique).
- **First verified gain of the family (2026-10-03).** The developer's proposals (`propose`) `nsubj → obl` and `obj → obl IF the word has a
  preposition` (child s894bcb40): gold guard EWT dev 78.80 → 78.83, tales 78.00 → 78.01; by hand 24 of 30 changed
  words right (errors were possessives — fixed in the atom); teacher 18 right vs 2; grammar violations −1,776 on a
  60k sample; tests: tales 77.32 → 77.38, EWT 79.84 → 79.86; bAbI, StepGame, SpartQA unchanged.
