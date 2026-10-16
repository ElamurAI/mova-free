# Nerves: the graph that ties words, code, worlds and the self

The design (2026-10-03), as the plan for the small students.

## Triggers — words hooked to code

An English word is hooked to a trigger: a compiled Rust function of the skeleton (`world/src/triggers.rs`). «minute»
fires the duration trigger, which knows a minute is sixty seconds and counts with it. The world of the trigger's topic
is its explanation: the world's sentences that use the trigger's words, with their sources. The concept lives in the
code; the world explains it.

## Both ways — events of the self activate words

Events of the skeleton activate words, and the words call the events up (`<hot>/activations.tsv`, with time and
duration):

| event | words it activates |
|---|---|
| a message in the chat | message, chat, conversation |
| a task done | task, work, command |
| a training round | training, learning, work, round |
| a search for ideas and their check | research, science, hypothesis, experiment |
| a world gathered | world, knowledge, reading, sources |

So the words of the world of time — «a minute», «long», «just now» — are tied to what the small ones lived through:
how long their own thoughts take, when the last message came. This is how a person's sense of time is tied to seeing,
moving and the speed of their own thoughts: the hardware of perception bound to the software of meaning.

## The nerve graph

Every activated word, and every word met in the chat, in a parse or in a text, activates nodes of a graph (nerves).
Activation spreads along the edges and reaches the matching places: messages in the chat, words of the parse, sentences
of the worlds, triggers in the code. And the other way: an active place in a world or the code activates its words.

- **Nodes**: lemmas and concepts — the words of the triggers, the events, the parts and sentences of the worlds.
- **Edges**: a trigger's hooks, an event's words, words that share a world sentence, a sentence and its source.
- **Spreading**: deterministic, a fixed number of steps with fixed weights, ordered maps only — the same input gives
  the same activation, every time.

## Alive in training, a fixed determinant at work

- **The graph of a state is fixed**: fast, deterministic, unchanging. It is compiled with the state and does the work.
- **The graph lives in training**: the search for optimisations changes it — new nodes from gathered worlds, edges
  strengthened or cut by what held on the guard sets.
- Searching for optimisations, answering questions, rest and self-awareness are the AI's states of life; the work is
  done by its unchanging determinant.

## Status

- Done: triggers of time (duration, cycles, clock, calendar), the self events and their recall, the perception report
  (`world worldbook-perceive`).
- Next: the nerve graph built per state from the triggers, the events and the worlds; spreading on a message or a
  parse; its use to pick the world sentences that explain a trigger (now picked by word overlap).

## The determinant does the same work everywhere

The fixed graph of a state is a function, not a process: the same input gives the same output whatever the hour, the
load of the processor, the version of the operating system or the system itself.

- **Time is an input.** «Now» is passed in; the skeleton's clock stays outside the determinant and its reading is logged.
- **Order is fixed.** Ordered maps and sets only (`BTreeMap`, `BTreeSet`), never `HashMap`; directory listings are sorted.
- **Threads cannot change the result.** Work split across threads is combined in a fixed order.
- **Integer arithmetic.** Weights and activations are fixed-point integers; no `exp`, `ln`, `pow` from a platform's
  maths library, whose last bits differ between systems.
- **Normalised text.** Line endings, Unicode NFC, `/` in paths; nothing depends on the locale.
- **A gate that can show red.** The same input is run twice, also under load from other threads, and the output hashes
  must match; a planted `HashMap` order must turn the gate red. Across systems the same test runs in the public
  repository's CI on Linux, macOS and Windows.

## Later: caches and trodden paths

Because the determinant is the same everywhere, its results can be cached for good: the hash of the graph and the hash
of the input name the answer. Paths of spreading that are walked again and again can be compiled into shortcuts, like a
gradient descent wearing a track; shortcuts that keep meeting form a matrix of convergence, so that a question is
answered by looking up where it already converges — the system holding the answers it has already worked out.

## Thinking ahead and wit (2026-10-04)

- A thought is never bad. A rejected idea is a dormant hypothesis, set aside with the circumstances it failed in; it
  wakes when they change (a new state, new code, new data or worlds) — `family.rs`, the negative list.
- Reacting to a stimulus from nothing is slow. Fear of the future makes a mind prepare: between rounds the small ones
  answer the likely questions about the things of their worlds before anyone asks (`know::anticipate`,
  `<hot>/anticipated.tsv`); the chat looks there first. The cache is a determinant of the rules of knowledge.
- Wit is applying a cached chain of decisions to an unexpected situation; genius is re-applying principles there.
  Next: the nerve graph finds a principle of the gathered worlds that matches a new situation by its structure, and
  applies it.
