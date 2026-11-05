# Lesson model

## forward

At most 2000 attempts: bind variables in order, derive lets in order, require all needs nonzero, evaluate nonnegative answer, fill selected template k modulo template count. Empty templates fail before RNG mutation. Exhausted unique names and invalid/reversed ranges fail. Inclusive numeric sampling uses i128 width and u128 residue, including full i64 domain. Wrapping seed addition defines large lesson numbers.

## behavior-complement

U is lesson/RNG/template/index inputs and output/final-RNG pairs with bounded expressions and helper domains. Absent success includes negative answer, unsatisfied conditions, exhausted names, reversed intervals and empty templates. Exhaustion returns None rather than guaranteeing a sample.

## reverse-reconstruction

B(d,s1) includes all initial seeds and rejected-attempt histories that produce d and final RNG s1. Modulo reductions and dropped variables lose information. Output alone cannot recover attempt count or template identity.

## coupling

Same seed and supplied lesson reproduce text/answer. Exercises/test/exam choose distinct containers and seed constants, but neither seed separation nor containers prove content disjointness. RNG(0) is an absorbing state; no uniformity promise. Failed attempts consume RNG state except early structural rejection.

## invariant

Successful observations obey the declared forward relation with the same supplied state and order. Failure is explicit; no hidden label or retained-state guarantee is introduced.

## cost

Bound 2000 times variable binding, expression and render costs. Name filtering bounded by 16-name table. Batch cost multiplies by requested count; accepted count can be smaller. No live learning or persistence modeled.

## composition

Evaluation and substitution share numeric bindings; field labels choose template containers. Child relations must agree on intermediate state, not merely final counts. Partial syntax binds the source; macro contents are opaque.

## epoch-memory

Initial review predicted arithmetic overflow and finite-pool boundary failures. Old native code failed six named witnesses: overflow, empty templates, name exhaustion, full-width range, reversed range and seed overflow. Narrow repairs retain public signatures; finite observations do not establish arbitrary-input totality. See file ../model.md#evidence for hashes and repeated results.

## checks

Eight wave12_models tests plus two existing lesson tests. Independent 19 by 19 by 7 arithmetic cases, precedence/eager failure literals, partition/substitution fixtures and exactly 2000 rejected RNG draws. Runtime guard mutations must fail named assertions. Evidence is finite; no whole-world or live course claim.

## History

### 2026-10-30 · document

Heading `# Lesson model, 2026-10-30` stays in document as `# Lesson model`.
