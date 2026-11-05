# Mova as a hybrid system

This page says what Mova is, introduces its mascot, and states the contract
between its three layers: what each layer does now and what remains direction.
For the pipeline, the learning loops and the crates, read
[the architecture](architecture.md).

## What Mova is

**Mova is a Hybrid Combinatorial–Operational System with a Combinatorial Small
Language Model (SLM) at its core and recursive use of its own capabilities.**
The Combinatorial SLM is the language core, written in Rust. Around it, the
system connects language understanding, mathematical models and explicit task
execution. [Groupants](groupants.md) describe code, models and their
interactions as one tree.

## The mascot

Mova's mascot is an octopus: curious, friendly and quick to write. The
[artwork](assets/mova-octopus.png) shows a teal octopus with coral undersides,
a pencil, a notebook and geometric blocks on a transparent background; the
[asset note](assets/README.md) has the image details.

In the project the 🐙 Octopus stands for Mova's partner, not for Mova itself:
a large language model (LLM) that develops and coordinates Mova and is not part
of its runtime. Mova's own animals (🦀 Crab, ⚡ Eel, 🐟 Fish, 🐋 Whales,
🕷️ Sea Spider, 🐚 Nautilus, 🐢 Sea Turtle) are Rust programs whose
understanding is Mova's. See
[the sea animals of Mova](animals.md).

## Three cooperating layers

| Layer | Responsibility | Current implementation and boundary |
|---|---|---|
| Linguistic | Turn text into structure and an interpretation. | `en` builds Universal Dependencies trees; `prag` and `world` read frames and worlds. A successful parse alone does not establish the intended task. |
| Combinatorial | Describe and compare models, constraints and checks; compose compatible observations. | Source-bound [groupants](groupants.md) hold vidtvorants, typed obernants, invariants and evidence ([mathematical modeling](mathematical-modeling.md)). The [correspondence scanner](correspondences.md) compares explicitly formalized finite relations. Automatic model selection and composition remain experimental. |
| Operational | Turn supported requests into concrete actions, coordinate workers and inspect results. | Configured capabilities, Crab protocols, persistent worker state and native adapters. An unsupported or ambiguous request keeps a failure instead of inventing an action. |

These are responsibilities within one architecture, not three independent
claims of general intelligence. Every transition between layers declares its
input, observation, assumptions and failure behavior.

## A request becomes a bounded plan

Crab, the Octopus's single Rust deputy, has an experimental native deputy
planner: the `mova-octopus` crate in `tools/octopus`. It is Rust, not the
Octopus. It accepts ordinary English text, parses it with the native parser,
makes a deterministic reader observation, and then a bounded rule layer
resolves configured capabilities and their arguments. Installed Rust handlers
execute the admitted actions.

The public worker is abstract. Its role, vocabulary, entity aliases, task
queues, service addresses and selected external models belong to deployment
configuration. A new handler extends the available operations without giving
the worker a permanent task identity.

For example, a configured deployment can read:

> Have a look at Dolphin's recent logs and ask Crab whether everything is
> running normally.

This becomes two configured actions: read the trusted log, and send Crab a
read-only status request. The report keeps the native UD observations, the
selected capabilities and the tool results. Empty logs do not establish
health, and a successful transport does not establish that every worker is
healthy. Without a referent, "Could you check its recent logs?" keeps
`missing_context` and performs no action.

This is configured task concretization, not arbitrary English task
understanding. The planner's native question-frame observation is separate
from trained pragmatic inference, which this path does not invoke. The source
and its checks are authoritative for the supported scope.

## Recursive self-use

A Mova workflow may call another Mova capability as one step: parse an
intermediate result, inspect a worker's output, compare a model observation, or
prepare a more concrete next task. The result returns to the caller through an
explicit interface. A nested call keeps its own source and configuration
identity, observation, budget and outcome.

A tool receipt, an annotation proposal, a mathematical proof and learned
knowledge are different kinds of evidence. Keeping a transcript does not change
the model. Revising a skeleton rule, importing reviewed graph knowledge and
training parameters are separate operations, each with its own admission and
checks. Concrete worker policies choose tasks and priorities outside the public
language core; [the animals page](animals.md) describes the worker interfaces.

## Models and checks in the loop

A vidtvorant reproduces selected observable behavior within its declared domain
and conformance profile; a controlled approximation makes it useful for fast
exploratory runs, and exact profiles are possible. A typed obernant describes
exclusions, compatible antecedents, ambiguities or another declared
complementary observation. Both are defined in
[mathematical modeling](mathematical-modeling.md).

Together they locate disagreement: the forward observation says what happens;
an appropriate obernant says what that result excludes or fails to identify.
Compatibility still matters. A finite coincidence is not a proof of source
equivalence, and an error bound needs a metric, a domain and evidence
([correspondences](correspondences.md), [groupants](groupants.md)).

The next operational step follows the admitted result, not merely a successful
exit code. An ambiguous parse can become a reviewed annotation task; reviewed,
versioned supervision can later enter a separate training path. Raw failures,
consultant answers and assessment labels never become training gold merely
because a worker produced them.

## Limits

- The deputy planner and the combinatorial tooling are experimental; a page
  describing them is not a release receipt.
- Task understanding covers configured capabilities only.
- Automatic selection and composition of models, and any composition-wide error
  bound, are direction, not current behavior.
