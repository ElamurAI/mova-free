# Mathematical modeling of code

This page describes how Mova models its own Rust code. Every owned source
file carries a set of models in its adjacent [groupant](groupants.md): a
description, a forward approximant (the vidtvorant), a separate
reverse/complementary family (the typed obernants), an invariant, a cost
approximation, simple checks and a block of the groupant's `model.md` for the
mathematical combiner. Groups of files get models of their interactions at
progressively higher levels of abstraction. The page defines the terms, the
reverse-reconstruction laws, the combiner, executable Rust approximants, how
the models are used to diagnose real defects, and the limits of all of this.

## What is modeled

Every owned Rust file is registered and has a sidecar, but the models are
partial. Gaps stay visible; they are not filled with invented descriptions or
proofs. A registered file is a partially modeled file, not a verified one.

For one code unit the model set is:

| Role | Name | What it holds |
|---|---|---|
| D | descriptant | the `## descriptant` block of `model.md`: description, working memory of earlier modeling epochs, input to the combiner |
| A | forward approximant (vidtvorant) | how inputs and state lead to outputs, next state and cost |
| B | typed obernant family | behavior complement first, reverse reconstruction second, specialized variants |
| J | invariant | what is preserved under admissible transitions, or which consistency relation must hold |
| — | cost approximation | units, assumptions, prospective prediction, later corrected by Rust measurement |
| — | simple checks | positive examples, rejected inputs, preserved state, boundaries, negative controls |

The Rust source and its AST are inputs alongside these roles. The raw source
stays in its original file. A complex file may hold a recursive tree of
fragment groupants inside its `<filename>.groupant/` folder; each fragment has
its own source scope, models and checks, and its parent keeps the composition
contract. The inter-file registry still has one owner per source. Storage and
the groupant tree are described in [groupants](groupants.md).

## Vidtvorant

*From Ukrainian «відтворант».* Plural: vidtvorants.

A vidtvorant is a typed model that quickly reproduces a system's behavior, or
selected observable aspects of it, within a defined domain. It relates inputs,
states, results and observable effects. It may be a formula, logical rules,
executable code or a combination. Its subject may be a function, an algorithm,
a component, a learning process, another model or a whole system.

Controlled approximation is part of the design: a vidtvorant may deviate from
the original within declared bounds in exchange for speed, simplicity and
lower cost, so it can serve as a fast emulator for repeated exploratory runs.
An error allowance does not require errors; an ideal vidtvorant may reproduce
every specified observation exactly.

Each vidtvorant states its subject, admissible inputs and states, observable
results, assumptions, preserved properties, and error and performance
profile. Error is judged by explicit criteria; numbers, discrete decisions,
state changes and error outcomes may need different comparisons. A defect is
a violation of the declared conformance profile. Proven properties, measured
results and unverified assumptions are kept apart. Periodic comparison with
the original refines parameters, applicability limits and error estimates.

One system may have several vidtvorants with different domains, observations
and speed/accuracy/cost profiles. A metasystem may select and combine them,
accounting for compatible assumptions, preserved properties and error
propagation.

In a groupant manifest the vidtvorant is `components.approximant`; executable
profiles are listed in `components.rust_approximants`.

**Read the original's code.** When building a vidtvorant or an obernant, start from the program code of the original — its branches, guards, data structures and failure paths — not only from its observed inputs and outputs. The code shows which cases exist, where behaviour changes, what is refused and what can or cannot be recovered; observations then confirm and bound the model.

## Obernant

*From Ukrainian «обернант».* Plural: obernants.

An obernant is a typed complementary model. It supplements a forward
description by characterizing excluded behavior, compatible inputs or earlier
states, ambiguities, or constraints implied by observations. It asks:

- What behavior is excluded within a specified domain?
- Which inputs or earlier states could produce this outcome?
- What possibilities does an observation rule out?
- What information cannot be uniquely recovered?
- Under what conditions can a given error occur?

It may be a relation, a logical specification, an approximation or an
executable model. Its type decides how it is read and checked. Examples:

1. **Behavior complement.** A sort must preserve its input multiset, so
   outputs that add or drop values are excluded. This is one class of
   forbidden outcomes, not every possible sorting error.
2. **Reverse reconstruction.** For `f(x) = x²` over the reals, observing 9
   gives the compatible inputs `{-3, 3}`. Both are kept; no unique inverse is
   invented.
3. **Observation exclusion.** For `y = a ∨ b`, observing `y = false` excludes
   every input pair containing a true value.
4. **Ambiguity.** If a report records only that two nonnegative integers sum
   to ten, eleven ordered pairs remain compatible.
5. **Error condition.** An array lookup returning `OutOfBounds` corresponds to
   indices outside the valid range; the error alone does not identify the
   index or the array length.

Each obernant declares its subject, domain, observation scope, assumptions,
formulation and evidence status. Approximate obernants also state their
approximation semantics and, where applicable, an error metric or bound. An
inverse function is only one special case.

## A code-unit model

For a file, record its role, inputs, state, outputs, applicability
conditions, forward relation, inverse constraints, cost approximation, simple
checks, source version and known limitations. State which part of a large
file is actually modeled. A list of function names is not a behavioral model.

Write the forward behavior as a relation

    R(input, state, output, next_state).

A relation permits several outcomes, including explicit errors and measured
approximations. Timing or GPU arithmetic need not be pretended to be an exact
scalar function.

Negative-space testing is one partial view. Record forbidden transitions N
and investigate

    R ∩ N = empty.

Exclusions start as hypotheses or requirements until checked. A
counterexample can reveal a code bug, a missing assumption or a wrong model.
Absence from sampled runs does not prove that something cannot happen.
Negative tests are evidence for the obernants; they are not the obernants.

An inverse query asks which inputs and states can reach a target:

    Pre(target) = {(input, state) : there exists a transition to target}.

An empty preimage means the target is unreachable within the model and
domain. Many-to-one behavior has many preimages.

Leaf checks cover positive examples, rejected inputs, preserved state and
boundary conditions. Use bounded enumeration, property checks, negative
controls and Lean where they fit. Keep the correspondence between real code
and a model separate from a theorem about the abstract specification.

## Parent models

A group's state hides internal detail while keeping the properties its caller
needs. Compose child relations through their shared intermediate states, then
check interaction constraints separately: matching identities, source scope,
ordering, units, effects and failure propagation. Correct children do not make
a correct parent if the wiring is wrong.

The abstraction tree organizes ownership and levels of reasoning. Real imports
and calls form a graph across that tree; the registry is not a call graph.
Keep dependency and source fingerprints so a changed child invalidates the
parent assumptions that depend on it.

Cost models state units and assumptions. Sum only executed child paths,
account for shared caches, batches and parallelism, make a prospective
prediction, then correct the approximation with Rust measurements. A Lean
proof does not determine CUDA speed, numerical error or semantic accuracy.

### Worked example: assessment identity

A leaf projects the scored assessment into

    assessment_id = H(course, lesson_seed, tests, exams).

File names and titles do not affect scoring; question text, gold labels,
option order and lesson seed do. Hash collision resistance is an assumption,
not a proved injectivity theorem.

Another leaf binds a result to the expected learner and assessment, validates
every row, choice and correctness flag, recomputes the aggregate and checks
zero teaching reads and updates. The parent compares four bound grades:

    gain = new_after - new_before
    recommend = (gain > 0) and (old_after >= old_before).

Invariant: a baseline from a different question set is rejected before any
comparison receipt is produced, even when its question count and learner hash
match, so a foreign baseline cannot turn an unchanged score into a reported
gain. Zero-gain and regressed recommendations are excluded. The model does not
prove answers semantically correct or authenticate a fully forged report;
grader and filesystem trust remain explicit boundaries.

Lean proves properties of the binding/comparison model and emits a score
table (`formal/StudyComparison.lean`, checked table
`formal/StudyComparison.table`). Rust compares its real comparison policy
with that table and checks the combinations of binding guards; a real CLI
positive run and a foreign-baseline negative run check the wiring. New exams
carry `assessment_sha256` and comparison receipts use schema 3. Results
without assessment identity cannot serve as bound evidence; to re-examine a
historical run, regenerate it under a new root with the matching baseline
model instead of weakening the gate.

## Typed obernants

Every file, group, subsystem and root groupant must account for a family of
typed obernants. The two required baseline types, in order:

1. `behavior_complement`: an approximant of what the system does not do. For
   an explicit universe U of admissible transitions and behavior R, the exact
   target is U \ R.
2. `reverse_reconstruction`: from an output, recover compatible inputs or
   earlier states. The exact target is the converse/preimage of R (see
   [Reverse reconstruction](#reverse-reconstruction)).

Both are obligations. A missing model stays recorded as missing; an
"inapplicable" status needs a reason for review. Several models of one type
and any number of specialized variants are allowed. Counterfactual rule
changes, missing-state recovery or alternative objectives can be extra types
once their transformations are defined; counterfactual systems need explicit
changed rules and need not be inverses.

Each entry has an id, type, subject and revision, domain, observation scope,
operator, approximation semantics, assumptions, evidence and status. State
whether "behavior" means a transition, a trace or reachability over a
declared horizon. Invariants and identities are checked per type and domain;
no universal inverse or additive identity is assumed.

A behavior-complement model is labeled as an inner bound (proven absent
behavior), an outer bound (all absent behavior plus unknowns) or an empirical
hypothesis. The complement of an approximate forward model is not the
complement of the actual Rust behavior; unsampled behavior stays unknown. An
obernant derived only from an approximate forward model is not independent
evidence about the source.

**Specialization.** Dozens of entries can be useful for one file or group. A
specialization must name a different constraint, observation or
intervention; a new label alone is not another model. Examples: score fibers,
identity equivalence classes, first-failure partitions, reachability
preconditions, information-loss projections, resource-budget preimages,
recovery boundaries, counterfactual rule or argument changes. For each, give
the domain and target relation, what is held fixed, the approximation
direction or incompleteness, and a falsifiable oracle. A group model also
describes wiring, shared identity and stage ordering. Example catalogs:
[dispatch](../tools/enrich/src/main.rs.groupant/model.md#catalog),
[assessment](../tools/enrich/src/assessment.rs.groupant/model.md#catalog),
[retention](../world/src/knowledge/retention.rs.groupant/model.md#catalog),
[study](../tools/enrich/src/study.rs.groupant/model.md#catalog) and the
[self-study group](../models/self-study.groupant/model.md#catalog).

**Storage.** Entries live in `components.obernants` of the sidecar manifest,
with their texts as blocks of the sidecar's `model.md`. A `## complementer`
block may describe one partial reverse relation; it is not the whole family.
A `model.md#Section` reference or a section field names an exact plain H2
heading in the referenced artifact; this checks that a
reference resolves, not that the formula is true. `mova-modelcheck groupants`
checks source identity, artifact references and required type records. It
does not judge model semantics. Some registry `inverse` fields still hold
only exclusion checks; they are not completed reverse-reconstruction models
and must not be presented as such.

## Reverse reconstruction

The reverse-reconstruction type answers: given a result, which inputs or
earlier states are compatible? Undoing a reversible transformation is the
unique-recovery case.

### Exact relation and identity

For a forward model A: X → Y on its admissible domain X:

    B(y) = {x in X | A(x) = y}.

For a stateful or nondeterministic relation R(x, y), B(y, x) iff R(x, y).
Include relevant state, configuration, effects and observation scope in x and
y; missing traces can leave many compatible histories.

- Every actual input x belongs to B(A(x)).
- If A is injective on X, B(A(x)) = {x}, and a single-valued inverse b
  satisfies b(A(x)) = x on reachable outputs.
- A chosen b with A(b(y)) = y recreates the output; that alone does not show
  b recovered the original input.
- B(y) is empty for an unreachable result and may hold many antecedents.

The identity I is defined on the appropriate domain. For deterministic A, B∘A
relates x to every x' with A(x') = A(x); it is exactly I only for injective A.
A∘B is I on the image of A. An additive complement A + B = I is a different
construction with extra algebra.

For a composition A = A2∘A1, traverse in reverse order:

    B_A(z) = union of B_A1(y) over y in B_A2(z).

Intermediate-state constraints must agree. Independently projected child
models can add spurious candidates by losing shared correlations.

Examples: for A(x) = x + 3 over integers, b(y) = y − 3 cancels the
transformation. For A(u, v) = u over Boolean pairs, B(false) =
{(false,false), (false,true)}: the lost second component cannot be recovered
without a saved trace. Swapping both components is reversible without one.
[`formal/Complementer.lean`](../formal/Complementer.lean) checks candidate
inclusion, uniqueness under injectivity, reverse composition, the injectivity
requirement for a left inverse, ambiguity of projection, swap cancellation,
an unreachable result and addition of output-error budgets. These are
specification properties, not a general Rust inverse.

### Approximation and evidence

With an approximate forward model Â and an observed y, one candidate relation
is

    B_ε(y) = {x in X | d(Â(x), y) <= ε and constraints_J(x, y)}.

Record the metric, units, domain, model revision, tolerance and provenance of
each constraint. If the forward error at the true input is bounded by δ and
the observation error by η, δ + η is a sufficient residual budget to keep
that input, provided J holds. Finite measured errors are calibration, not
universal bounds.

- *Soundness*: returned candidates satisfy the declared relation.
  *Completeness*: all compatible candidates are kept. A sampled or pruned list
  need not be complete; an empty search proves unreachability only with a
  complete search or a separate proof.
- A small output residual does not imply a small input error; inversion can
  amplify errors or stay ambiguous.
- Ranking by simplicity, prior or penalty adds assumptions; record them in the
  descriptant. An invariant may filter candidates when justified; it cannot
  supply lost facts.
- Compatible antecedents are possible explanations, not identified causes. A
  score or hash alone usually cannot recover its questions, source or history.

Store the reverse relation, approximation assumptions, solver budget,
candidate traces and counterexamples in the `.groupant` folder.

## Mathematical combiner

The combiner is the mathematical subsystem that reads a groupant's roles and
proposes the next revision. At modeling epoch k its conceptual input is

    C_k = Combine(Rust_k, AST_k, A_k, B_k, J_k,
                  descriptions_k, own_MD_memory_0..k, measurements_k).

Rust is the source of implemented behavior. The AST gives structure,
interfaces and candidate rewrite locations; record parser/compiler
configuration and revision, since syntax alone does not establish runtime
semantics. Expanded syntax, control/data flow or compiler IR may become
inputs when a model needs them. A gives transitions and cost, B the reverse
and complementary questions, J the preserved properties. Keep invariants
apart from optimization goals: accuracy may rise in training while an
assessment-identity invariant stays fixed. Formulas may be machine-readable
or Lean; exact file extensions are flexible.

The combiner should produce:

- predicted behavior, resource cost, approximation error and inconsistencies;
- ranked mathematical, model and code transformation candidates;
- cheap experiments, invariance checks and counterexamples to seek next;
- results of selected real Rust probes and revised approximants;
- a new modeling-epoch Markdown record.

### Epoch memory

The descriptant keeps concise, useful information from earlier epochs:
hypotheses, rejected approaches, surprising observations, derivations,
prospective predictions, measured errors, open questions and reasons for
revising a model. Each item carries its epoch, source and model revisions,
scope, status and evidence reference. Statuses include hypothesis, modeled
prediction, measured, proved under assumptions, refuted and superseded.
Reading an old note is not new evidence; when a conclusion changes, keep the
old observation and add the revision. Large logs and tables stay in
measurement artifacts, referenced from the Markdown.

The conceptual update is

    (A_next, B_next, J_next, plan_next, MD_next) =
        C(current inputs, previous MD, new evidence).

This is a research interface, not a fitted universal formula. A modeling
epoch need not coincide with a training epoch.

### Through the tree

A parent combiner consumes its children's approximants, obernants,
invariants, interfaces and memories plus its own records, models the
interaction at a higher level and emits its own bundle. Explore many ideas on
cheap models, select bounded real Rust checks on the intended hardware, and
use discrepancies to update formulas, error and cost estimates and the next
epoch's notes. A stable invariant can reject a candidate; a failed presumed
invariant can expose a wrong model.

The [assessment descriptant](../tools/enrich/src/assessment.rs.groupant/model.md#descriptant)
is an instance grounded in the file and group models above. Algorithm
groupants can also be transferred into Mova code through explicit adapters;
see [correspondences](correspondences.md).

## Executable Rust approximants

Rust approximants are executable vidtvorants stored beside the mathematical
models. They support cheap experiments with Mova and its algorithms. The
exact implementation stays the reference. An approximant is never
automatically substituted in production.

### Error caps, metric and domain

The supported upper bounds are 1%, 2%, 5% and 10% (profiles P1, P2, P5,
P10). A cap is a maximum, never a target: a 5% profile may return an exact
answer, and lower actual error is welcome. Exactly 10% is inside P10; any
larger error fails. Configuration rejects other tiers without rounding or
clamping. A looser cap does not promise faster execution.

A percentage means something only with a metric, a domain and an observation
scope. Scalar relative error, objective loss, normalized rank displacement
and probability distance are different metrics; their percentages are not
interchangeable. Define the zero-reference case explicitly. Invalid input,
failed allocation, forbidden side effects and broken feasibility are not
small numerical errors.

### Classification

Each profile declares three independent axes, with stable labels shared by
the Rust enums and the manifest validator. A label describes a method; it
does not prove behavior.

| Axis | Labels in use |
|---|---|
| `metric_kind` | `scalar_relative`, `normalized_rank`, `objective_loss`, `probability_relative` |
| `method_kind` | `quantized_numeric`, `certified_sample`, `relaxation_certificate`, `interval_certificate`, `floating_normalization` |
| `assurance_kind` | `analytical`, `per_input_certificate`, `finite_observation` |

`analytical` needs a scoped mathematical argument. `per_input_certificate`
verifies the bound for the current input. `finite_observation` is only for
the worst error seen on named fixtures; it promises nothing on unseen inputs
and cannot be combined with an analytical declaration. Measurements can
falsify a claim; finite success cannot establish a universal guarantee.

### Binding

Each attachment records its source revision, executable artifact, cap,
metric, domain, assumptions, assurance kind and evidence, bound by exact
source, code and evidence hashes. A stale hash fails the structural check. A
declaration or structural check alone certifies neither accuracy nor speed.

### Kinds of approximant

| Kind | Example kernels | How the bound is obtained |
|---|---|---|
| Numeric | integer square root | quantized estimate with an analytical relative bound; zero exact |
| Order statistic | quickselect value | sampled rank certificate on the value; does not perform the exact API's in-place partition |
| Optimization | knapsack | feasible lower bound certified by fractional relaxation; exact DP fallback |
| Sequence | byte edit distance, strictly increasing subsequence | interval certificate (histogram lower bound vs aligned-edit upper bound; greedy witness length vs n); exact fallback |
| Range | numerical range minimum | global [m, M] bound, loss at most (M − m)/M; exact sparse table fallback |
| Pragmatic | RSA literal listener L0 and speaker S1 (`prag/src/rsa.rs`) | floating evaluation of the production formula with exact zero support and an analytical positive-cell relative bound, on a bounded domain (world/utterance counts, priors, costs, rationality, rectangular truth) |

Scope notes: byte distance is not grapheme distance; an approximate
subsequence witness need not match the exact indices; a compact range minimum
need not occur in the queried range; knapsack can answer some capacities
where the reference's allocation would fail, so it models the objective, not
resource-failure behavior. Unsupported pragmatic inputs return an explicit
error; L1, argmax decisions and display conversion are outside the model.
Other algorithm and Mova groupants have no executable approximant yet; that
gap is explicit. A whole-subsystem executable vidtvorant may exist without a cap:
the school student's (`student-vidtvorant.groupant` under `knowledge.rs`)
predicts the pupil's chosen option, understanding and answer time from word bags
with the code's decision structure, fitted to the pupil's traces. Its finite fit,
recorded in its groupant, meets no cap, so it is the groupant's executable
vidtvorant component, not a capped profile.

### Storage and running

For `mod.rs`, the companion lives at `mod.rs.groupant/rust_approximant.rs`
and has its own adjacent groupant. The original groupant keeps the
mathematical approximant and lists one `components.rust_approximants` entry
per cap. Companions are compiled by a shared runner, not copied into
production modules. The `mova-approx` crate supplies the cap values and
checked scalar/finite-evidence helpers; the runners compare companions with
the exact references and time them with compilation excluded, separating
classification and error records from timing.

```text
cargo test -p mova-approx --all-targets
cargo run --release -p mova-approx --example groupants
cargo run --release -p mova-approx --example groupants_next
cargo test -p mova-modelcheck
```

Write benchmark output to a fresh run directory under `$MOVA_DATA`. Record
fixtures, repetitions, output agreement, observed error, fallback count and
timing together. These are CPU kernel benchmarks, not training, GPU or
school measurements.

### Choosing and composing

First model the cheaper transformation, its error and expected work. Build
only after defining an observation that preserves the question: a rank
estimate serves quantiles but not an in-place partition interface; a
literal-listener bound says nothing about recursive pragmatic inference or
final answers. When a cheap certificate cannot establish the cap, use a
documented exact fallback or return an explicit unsupported-domain result,
and record fallback frequency: a safe surrogate can be slower than the
original. Do not add noise to occupy a looser cap.

There is no composition-wide bound. A parent must model error propagation
through its actual composition: for an L-Lipschitz downstream map, upstream
absolute error e contributes at most L·e, and a downstream error d gives at
most L·e + d on the stated domain. Relative errors, branch decisions and
stateful updates need their own analysis. Four children under 10% do not make
the parent under 10%. Parsers, effects, resource failures and categorical
decisions have no useful approximate semantics yet; keep that gap explicit
until a metric and invariant exist. A test harness or exact wrapper is not a
fast approximant.

## Diagnosis with vidtvorants and obernants

Let x be the admitted input and state and o the selected output observation.
A vidtvorant V relates x to possible o. An obernant describes an excluded set
E in a universe U, the compatible antecedents V⁻¹(o), resource limits,
failure partitions or observation loss. A native observation that conflicts
with a model is a lead, not a verdict: check subject version, domain,
observation and assumptions, then compare the code with an independently
stated requirement. The code may violate the requirement, or the model may
have omitted legitimate behavior; a model and its mechanically derived
complement cannot decide which.

The loop: state a forward prediction and the relevant typed boundary; build a
small discriminating input; observe the real Rust behavior; repair the
source or the model; keep the counterexample, the passing check and the
remaining limits in the descriptant. Exact toy witnesses can diagnose a
discrete defect without inventing a percentage metric.

Rules that repeatedly found real defects:

- **Union needs the full key.** Deduplicating by adjacency after sorting is a
  union only if the sort key includes every component of the identity. Sort
  by the whole key (for example priority, class, layer), not a projection of
  it.
- **A many-to-one decoder breaks a roundtrip.** If two stored forms decode to
  the same value (a key with and without a trailing CR), save-then-reopen is
  not an identity. Restrict the admitted domain (reject TAB, LF, CR where the
  codec needs it) and state the roundtrip only on that domain. Ambiguous
  historical data cannot be repaired uniquely without more evidence. Check
  the parent composition too: neighboring codecs may need different, not
  identical, policies.
- **Logical rows are not physical records.** Appending to a partial JSONL
  result must keep one object per line. Add LF exactly when the segment after
  the last LF is non-blank, after validation; a blank line passes tolerant
  readers but fails strict ones. A valid writer and a tolerant reader do not
  establish compatibility with a strict downstream reader.
- **Semantic equivalence is not budget closure.** A rewrite that preserves a
  relation can grow its syntax past the validator's limits. Use the full
  normalization only when its result fits; otherwise keep a bounded mode.
- **The model can be the thing that is wrong.** A resume step accepts any
  validated ordered subset of unique requests, not only a prefix; the correct
  forward observation is P followed by the remaining requests in order, and
  the reconstruction keeps many compatible histories. Fix the model, not the
  valid behavior.
- **Bind model, source and executed artifact.** A stale installed binary can
  contradict a contract the current source satisfies. Assign a discrepancy to
  the implementation only after the three bindings agree.
- **Error priority is a partition.** When validation runs before work (for
  example Bellman–Ford checks the source index, then every edge endpoint,
  before relaxing anything), the first invalid item in a fixed order decides
  the error, regardless of reachability or later defects. The error cannot
  recover the data after it. Model this as a first-failure partition,
  separate from the success fiber and the behavior complement.
- **Incremental carry needs agreement outside the changed set.** For old
  selection O, new selection F and changed IDs C, the merge
  (O \ C) ∪ (F ∩ C) equals F exactly when O and F agree outside C. Sampling
  after a filter can break this; refresh such sets fully. C must contain every
  ID whose stored semantic projection changed; then every pointwise observer
  agrees outside C. A coarser single predicate may need fewer IDs.
- **Prefer stable numerics with exact edge cases.** For a fair binomial tail,
  build mode-normalized weights outward (w_m = 1, w_(i−1) = w_i·i/(n−i+1),
  w_(i+1) = w_i·(n−i)/(i+1)) instead of multiplying huge coefficients by a
  tiny 2⁻ⁿ; handle exact cases (k = 0, k > n, the symmetric midpoint)
  explicitly. A real-arithmetic identity is not a floating-point error proof.
- **Repairs need red witnesses.** Every repair needs a witness that fails on
  the old code, or a regression test that failed before the fix; a fixture
  that a mutant survives must be strengthened.
- **Reject before mutation, bound every walk.** Reject invalid input
  (delimiter characters in stored labels, invalid indices, nonfinite values,
  malformed partial records) before any state changes; bound traversals of
  possibly cyclic structures; use checked or exact integer arithmetic where
  counts can overflow or exceed f64's exact range. Order-sensitive results use
  canonical order (`BTreeMap`, sorted keys, fixed floating-point reduction
  order).
- **Historical metrics keep their source.** When scoring code is fixed, earlier
  metrics keep the semantics of the source that produced them.
- **Exact algorithms have domains.** A finite prefix can admit several
  minimum-order recurrences; XOR and ordinary convolution agree only without
  carries; a minimum-cost assignment is a cycle-cover lower bound, not a tour.
  Check against independent oracles, not only round trips.

A descriptant keeps the question, source binding, observation, assumptions,
forward prediction, obernant, counterexample, repair and evidence status
together, and distinguishes an original-source failure, an intermediate
repair failure, a model-only correction and a still-unsupported domain.
Parent descriptants say which child observations survive composition and
what information is lost.

## Math-first reviews

Before an optimization, state the model, the invariants and the expected cost
change. Explore approximations and derivatives cheaply, then use Rust for the
implementation and real timing, with focused correctness checks and periodic
larger validation. Review existing algorithms for implementation bugs as well
as for new ideas. Save prospective predictions and correct them with
measurements; a wrong prediction is kept as a refuted note, not deleted. A
Lean proof alone does not establish runtime or task accuracy.

## Registry and checks

`models/contracts.json` is the public registry: each file or group node has a
description, state, forward and inverse clauses, cost model, named checks and
limits. Registry ownership, sidecar bindings and the check commands are in
[groupants](groupants.md#registry-and-checks) and the
[modelcheck README](../tools/modelcheck/README.md). Registration and structural
validity do not establish semantic correctness; run the relevant source checks
separately, and expand models first into active learning, inference and state
transitions.

## Limits

- Models are partial. Registration, structural validity and a passing gate
  are not verification.
- There is no general AST combiner, automatic reverse-model construction,
  candidate search, cross-groupant reverse execution or autonomous epoch
  update; the combiner is specified, and the Markdown describes and feeds it.
- Syntax projections are source-bound and partial; macro expansion and
  semantic AST analysis are absent.
- Executable approximants exist only for the kernels listed above. None is
  substituted into production, and no bound composes automatically. The
  student's vidtvorant (`world/src/knowledge.rs.groupant/student-vidtvorant.groupant`)
  is an executable vidtvorant of a whole subsystem whose finite fit meets no
  cap, so it has no `rust_approximants` entry.
- Lean results are specification properties; they say nothing about Rust
  correspondence, speed or numerical error unless separately checked.
- Unknown behavior stays unknown; sampled absence is not proof of absence.
