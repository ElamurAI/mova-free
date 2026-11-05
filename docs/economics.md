# The economics of Mova

Mova is not mainly a way to spend fewer tokens. It changes **where intelligence is spent**.

A common AI workflow asks one large model to do everything in one expensive context: understand the task, split it up, remember the state, watch the workers, repair failures, judge the result and write it. Mova separates these jobs. Rust programs and Mova's own small language model (the SLM) keep the state, the queues, the routing and the routine reading. Large models stay in the system, but as **consultants**, called where broad judgment or a hard semantic step is worth their price.

## Who does what

```text
            Octopus — develops and reviews the system (an LLM, outside the runtime)
               │
             Crab — the persistent orchestrator (Rust on Mova)
       ┌───────┼────────────────┐
  Mova's animals         consultant Dolphins (LLMs)
  (Rust on Mova,         asked in plain English;
   each may hold         their advice is evidence,
   its own LLM terminal) never authority
```

This is not a ranking of intelligence. [Crab](animals.md) is central because it owns the *position*: durable state, configured capabilities, queues and the decision about what happens next. The consultants bring outside judgment. The Octopus changes the system itself. The most expensive intelligence does not have to sit in the most permanent seat.

Everyone talks in ordinary language; what must be exact — task ids, hashes, leases, permissions, journals — stays structured. A participant can be replaced while the conversation around it stays the same.

## The small model works at the margin

Today Crab's Rust control layer keeps worker state, queues and supervision policy, and Mova's native deputy maps supported English requests to configured capabilities. These components set the current boundary; a general SLM that chooses and performs arbitrary operational jobs is not established by them. The thesis is that each such job moved below the large model removes repeated reasoning, context rebuilding and bookkeeping from it, makes failures cheaper and lets work run in parallel.

The right comparison is not "SLM against a large model" but: *does the SLM take low-value load off the large model while the whole system produces as much or more?*

## Better questions, better consultants

When the routine is handled below, a consultant receives a narrowed problem, the relevant evidence, the known alternatives, the honest uncertainty and a concrete question. A raw call spends much of its capacity discovering the task; a prepared call spends it on the part that needs judgment. The execution layer is a **productivity multiplier** for the large model.

Prompts therefore become capital. A good prompt holds compressed operating knowledge — what matters, what may be left out, what must not be inferred, when to escalate — and it survives a change of model better than any model-specific trick. The lasting asset is the system's ability to ask useful questions.

## From saving money to making money

Token savings are the first, visible effect. They are not the goal. The goal is the ratio

```text
value of accepted work      throughput × chance of acceptance × value per task × reuse
──────────────────────  =  ────────────────────────────────────────────────────────────
     total cost             models + compute + storage + attention + failures + coordination
```

and the architecture aims to move both sides. The implementation does not establish token savings, profitability or a measured marginal return on more model spending; those need evidence for each workload. Once an extra unit of model spending produces more than a unit of value, spending less would lower profit. The threshold that matters is not *AI cost < human cost* but *marginal value of more AI work > its marginal cost* — and coordinated AI can make work possible that was never done before, because it was too fragmented, too uncertain or too long for one session.

A persistent orchestrator is what makes this pay. Without it, every new session rebuilds the organization: what runs, what was tried, what is trusted, what waits for review. Crab keeps those facts as state, so expensive intelligence is not consumed by organizational amnesia — like having a firm that knows when to call the brilliant consultant, instead of hiring the consultant for every action.

## Inputs that strengthen each other

Let *R* be deterministic Rust execution, *M* Mova's own cognition, *V* large-model consultation, *P* prompt and routing quality and *S* retained state. A naive view treats them as substitutes, `O = aR + bM + cV`. Mova bets on complementarity, `O = F(R, M, V, P, S)`: better state makes each consultation more useful, better prompts make the same model stronger, better routing stops expensive models being wasted, and the consultants' judgment improves the rules the SLM follows.

The frontier between them moves both ways. A task can travel from *the large model does everything* to *the SLM prepares and the model decides*, to *the SLM performs and the model reviews*, to *a native capability with periodic external audit*. Freed from routine, large models take on harder work, so their total use may stay the same or grow while the system produces far more. Falling token counts alone are not success.

A new model then changes parameters — cost, latency, context, reliability, strengths — rather than the architecture: closer to replacing a machine in a factory than rebuilding the factory.

## What to measure

Accepted tasks per hour and per unit of cost; consultant calls per accepted task; the share of tasks finished without escalation and of escalations that changed the result; human interventions; failure cost; the value of work that would otherwise not have been attempted. In the end one question: *does the system create more value than it consumes?*

## Limits

This is an architectural thesis, not a finished empirical proof.

- Coordination can become its own cost when many cheap operators duplicate work.
- Prompts can become hidden software; important ones need versions, measurements and a stated scope.
- A cheap operator can make expensive errors; high-stakes decisions may justify a consultant early.
- Agreement between consultants is not truth.
- Native execution must escalate when it lacks a capability; hiding gaps to save calls damages the real output.
- More spending is investment only when its marginal value is measured and exceeds its cost.

If the thesis holds, competition between model providers helps the system, prompt work becomes production engineering, and local software matters again. The bottleneck then moves from the price of inference to choosing tasks, evaluating them, earning trust and turning completed work into value — a healthier problem to have.

See also: [the hybrid system](hybrid-system.md), [the animals](animals.md), [architecture](architecture.md).
