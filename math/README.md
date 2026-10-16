# math — the SLM's basic mathematics module

`math` is exact mathematics for the small model: rationals, symbolic expressions, units with dimensions, steps with independent checks, and English questions read into expressions through UD trees.

**Goal** (26.09): compute at the level of a university mathematics student. v1 is level 1: arithmetic, units, percentages, statistics, probability. The first application is
a free weather chatbot: SLM only, deterministic, on CPU, in milliseconds.

## How it works

| module | what it does |
|---|---|
| `rat.rs` | rational numbers on i128: always reduced; overflow is an explicit `Overflow` error, no silent f64; comparison via continued fractions, without overflow |
| `units.rs` | dimensions (length, mass, time, temperature; fraction is dimensionless); units are an enum with exact factors to SI; the °C/°F/K scales are affine, Δ°C/Δ°F/ΔK are multiplicative |
| `expr.rs` | **symbolic core**: an expression is a tree (rational constants, symbols x, y…, constants π, e, operations, functions sin/cos/tan/exp/ln/sqrt/abs, aggregates, percentages, probability); quantities and absolute temperatures are nodes of the same tree |
| `eval.rs` | numeric evaluation is one of the operations on the tree: quantities with units, steps (rule, formula, substitution, result), gate checks |
| `fcheck.rs` | an independent path for the gate: the same tree, different arithmetic (f64 in SI, its own scale constants) |
| `lex.rs` + `build.rs` | dictionary `data/lexicon.tsv` → a constant `match` (form → enum); a duplicate is a build error |
| `understand.rs` | English question → `Expr` via the UD tree from `en` |
| `suite.rs`, `main.rs` | running suites; CLI |

### Exact core and units

- Exact definitions: 1 in = 2.54 cm, 1 ft = 12 in, 1 mi = 1609.344 m, 1 lb = 0.45359237 kg,
  1 inHg = 25.4 · 133.322387415 Pa = 3386.388640341 Pa (conventional mmHg), 1 hPa = 100 Pa.
- Units: m, km, cm, mm, in, ft, mi; kg, g, lb; s, min, h, day, week; °C, °F, K; %; km/h, mph, m/s; Pa, hPa, inHg.
  Compound units are products with exponents (`Mono`): km/h = km·h⁻¹; "miles per hour" → mph.
- **Temperature is affine.** A scale point (72 °F) and a difference (12 Δ°F) are different kinds:
  - point − point = difference; point ± difference = point;
  - point + point, number · point, sum of points — an error with an explanation;
  - mean, median, min/max of points are valid (affine combination, order);
  - converting a point shifts the zero, converting a difference only multiplies: "a change of 10 °F" = 50/9 Δ°C, while absolute 10 °F = −110/9 °C.
- "5 km + 3 kg" gives `dimension error (addition): length and mass`, not a number.
- Rounding only on output: a terminating decimal is exact, otherwise "≈ 22.2222 °C (exactly 200/9 °C)".

### Operations

- arithmetic: + − · / ^, parentheses, "squared/cubed", root (exact if possible: √144 = 12, otherwise ≈);
- percentages: "p % of x", percentage change, percentage share ("what percent of 80 is 12"),
  "x increased/decreased by p %", the linguistic convention "x minus p %" = x · (1 − p);
- statistics: mean, median, min, max, sum, range, product;
- probability "at least once" 1 − ∏(1 − pᵢ)^kᵢ, "never", "every time"; **the independence assumption is stated aloud**;
- unit and scale conversion.

### Steps and checks — the gate

Every operation is a step: `rule: formula | substitution | = result`. Example:

```
question: If the chance of rain is 30% each day, what is the chance of rain at least once in 3 days?
expr:    P(at least once)[p=30%, n=3]
  1. complement: 1 − p  |  1 − 30%  |  = 70 %
  2. never (independent events): ∏ (1 − pᵢ)  |  0.7^3  |  = 34.3 %
  3. at least once: 1 − ∏ (1 − pᵢ)  |  1 − 0.343  |  = 65.7 %
  ✓ enumeration of all outcomes — 2^3 = 8 outcomes, sum of probabilities 1, target 0.657
  ✓ independent f64 path (SI)
assumption: I treat the events (days) as independent — otherwise the formula does not hold
answer: 65.7 %
```

Checking is easier than generating, so the check is a gate: an answer that does not pass is not returned
(`CheckFailed`). v1 checks:
- **an independent path** for every answer: the same tree, f64 in SI; conversion does nothing there, so the exact
  result after conversion must have the same physical value;
- the inverse operation: sum − addends, quotient · divisor, result / x = p, old · (1 + change) = new;
- the reverse conversion gives back the original number exactly;
- probability — by enumerating all 2^N outcomes (N ≤ 16), exactly;
- mean: Σ(xᵢ − x̄) = 0 and min ≤ x̄ ≤ max; median: at least half ≤ and ≥; root squared = radicand.

Negative control of the gate (`gate_rejects_wrong_answers`): a deliberately wrong answer (16.09 km instead of
16.09344 km, 13 instead of 12) turns red.

### Understanding via the UD tree

1. The `en` tokenizer, small boundary fixes ("C." → "C" ".", "))" → ")" ")").
2. A dictionary chunker: numbers in digits and words (one…ninety, hundred, thousand, million, dozen, half,
   third, quarter, twice; "two dozen", "a third", "twenty-one", "1,000", "3/4", "-5"), quantities with units
   ("72 °F", "72 degrees Fahrenheit", "72 F", "10 miles per hour", "inches of mercury"), bare target units.
3. Placeholders for the parser: "72 °F" → "72 degrees", "10 mi" → "10 units" — the `en` parser no longer stumbles
   on "°" and "F"; `en` parse (model `ud-ewt-eslspok.bin`, 0.12 s) and mapping of nodes to elements.
4. Frames by lexical triggers, roles by the tree:
   - condition clause: `advcl` with `mark` if/when, a phrase with `case` with, preceding sentences;
   - case markers of quantities (of/from/to/in/between/by/than/for/on) — children with `case`;
   - function arguments — the subtree of its "of" phrase;
   - linear order is the fallback; how many markers were taken from the tree and how many linearly is visible in the trace.
   Arithmetic chains ("12 plus 30", "3 + 4 * 2", "(3 + 4) * 2") are read by an operator grammar in linear
   order: UD for "times/plus" is unreliable (in EWT "6 times 7" — times NOUN, 7 flat).
5. **If it does not come together — an honest "I don't understand" with a reason:** an unknown content word ("color", "banana"),
   no numbers, an operator without an operand, "of" between numbers without a percentage or fraction, degrees without a scale,
   a source without a unit ("Convert 72 to Celsius"), several candidates for the daily probability.

## CLI

```
cargo build --release
../target/release/math eval "What is 15% of 80?"      # expression, steps, checks, answer, trace
../target/release/math test data/suite-v1.tsv          # run a suite, summary by category
../target/release/math parse "Convert 72 °F to °C ."   # elements and tree (debugging)
```

The `en` model: `MATH_EN_MODEL` or `$MOVA_DATA/en/models/en.bin`.

## v1 measurements (26.09)

| suite | questions | correct | honest "I don't understand" | wrong |
|---|---|---|---|---|
| `suite-v1.tsv`, first run | 158 | 155 | 2 | 1 |
| `suite-v1.tsv`, after fixes | 158 | 157 | 1 | 0 |
| `holdout-v1.tsv`, single run | 30 | 30 | 0 | 0 |

- Both suites were frozen by hash before the understanding code (worklog 26.09, commits a67e6408 and f30e6ba8).
  suite-v1 was the dev set during development. The held-out set was written by the same author before the code —
  it is not an independent yardstick, only a sign that the frames are not tied to exact sentences.
- A probe after the code (25 new phrases) found 4 wrong answers: parentheses, "x minus p %", the "degrees" scale from the target
  (a guess), printing; fixed and pinned by the `probe_regressions` test.
- Negative controls are red: dimension 8/8, nonsense → "I don't understand" 12/12, overflow and division by
  zero 5/5, absolute versus difference 4/4 (+ the pair "change of 10 °F" / "10 °F" gives different answers).
- Speed: model 0.12 s; 158 questions in 0.04 s.
- `cargo test`: unit tests of the core (conversions by exact definitions, affinity, associativity and
  distributivity, reduction, overflow, comparison without overflow) and end-to-end tests (brief samples,
  negative controls, the "no wrong answers" regression on suite-v1).

## v2 (26.09): a curriculum-based solver

**Goal** (26.09): the mathematics module should learn — not a student, but a mathematical genius, taught to solve
top-level world problems, learning from small to large. Addendum: inside, for speed, the mathematics module may call
a Rust module (or one rewritten from C++) for exact mathematics.

### Division of labour

- **The LLM (Opus) reasons and writes a plan** in a typed step language (JSON): `compute`, `solve`, `factor`, `expand`,
  `derive`, `enumerate`, `check`, `claim`. It does not compute itself: expressions only.
- **The SLM (this crate) executes exactly and checks** each step by an independent path. If it fails, the plan goes back to the LLM
  with the reason, at most 2 retries, then an honest "not solved".
- **Learning is a memory of verified solutions**, not weights in a black box: problem features, method, plan,
  trace (`memory.jsonl`, table `solutions` in `$MOVA_DATA/db/mathsolve.duckdb`). The method is chosen by an averaged
  perceptron over problem features (it is visible which features outweighed and with what weight); the plan is a template from memory in
  which the numbers of the problem statement are replaced by slots. Where the perceptron is confident and the template fits, the SLM solves on its own, without the LLM.

### v2 modules

| module | what it does |
|---|---|
| `big.rs` | big integers and rationals — `dashu` (chosen by benchmark, see below); parsing and printing |
| `nt.rs` | GCD/LCM with a Bézout certificate (own extended Euclid), modpow by two paths, inverse, general CRT (moduli not necessarily coprime), primality: deterministic Miller–Rabin for u64 (12 bases), for big numbers MR with 30 bases and a Pocklington certificate or an honest "probably prime"; factorization: trial division up to 10^6 and Pollard–Brent with a step and time budget; φ, τ, σ_k, μ, divisors, π(n) by sieve, k-th prime, digits in base b |
| `comb.rs` | n! by a product tree, C(n,k) (second path — Legendre's prime factorization), P(n,k), Catalan, Stirling I/II, Bell, partitions p(n) (Euler versus dynamic programming), into k parts, into distinct parts (= into odd parts), derangements, Fibonacci (Cassini identity), multinomials |
| `poly.rs` | polynomials over Q: arithmetic, GCD, derivative, Yun's square-free factorization; rational roots; quadratics exactly `a ± b√d` (d < 0 — complex) with a check in Q(√d); degree ≥ 3 without rational roots — numerically (Aberth) marked "≈"; factorization over Q; inequalities N/D ⋈ 0 by signs on intervals; expression → rational function with a memory of denominators (a root of the denominator is discarded) |
| `linsys.rs` | linear systems of any size — Gauss–Jordan over the rationals: unique / infinitely many (free variables) / inconsistent (witness row) |
| `deriv.rs` | symbolic derivative with rule steps (sum, product, quotient, power, chain, exponential, u^v, trigonometric, inverse) and a central-difference check at 8 points |
| `calc.rs` | the plan expression language: exact evaluation, ~90 functions, enumeration with a bound variable (`sum`, `count`, `forall`, `first`…) with step and time limits; the independent path is the same formula in f64 or a fingerprint modulo 2^61−1 (for huge integers, where f64 lies in the low digits) |
| `plan.rs` | LLM plan → execution → trace with checks → gate; negative controls |
| `vmm.rs` | `claude -p` the same way as `annot/src/llm.rs` and `prag/src/opus.rs` (`api_env()`, effort via `--settings`), a `calls.jsonl` log and a spending cap before every call |
| `curric.rs` | levels, sampling with seed 20260926, comparison with the gold standard, prompts |
| `memory.rs` | problem features, perceptron, plan templates |
| `solve.rs` | configurations A, B, C and level 4 (Lean) |
| `lean.rs` | level 4: the statement is fixed, the LLM writes the proof, `lake env lean` checks it |
| `bin/mathsolve.rs` | CLI |

v1 (`rat.rs` on i128) remains for units and weather; an i128 overflow in `math eval` now falls through to the
exact v2 core: "What is 3 to the power of 100?" → 515377520732011331036461129765621272702107522001.

### Exact arithmetic: library benchmark

We do not write exact arithmetic ourselves — we take a ready one. The criterion is the license (the product is closed), then speed.
Measurement `bigbench/` (`cargo run --release`, min of 3, single thread; identical algorithms on all libraries,
results cross-checked):

| task | dashu 0.6.1 | num-bigint 0.4.8 | num-bigint 0.5.1 |
|---|---|---|---|
| 100 000! (product tree) | 0.036 s | 0.028 s | 0.028 s |
| 100 000! → decimal string (456 574 digits) | 0.064 s | 0.067 s | 0.067 s |
| 3^1 000 000 | 0.0125 s | 0.020 s | 0.020 s |
| 3^1 000 000 → decimal string | 0.075 s | 0.073 s | 0.073 s |
| GCD of two 10^5-digit numbers | **0.033 s** | 0.550 s | 0.550 s |
| modpow 2048 bits × 100 | 0.437 s | 0.385 s | 0.380 s |
| H(3000) exactly, rationals (num-rational 0.4.2) | **0.0022 s** | 0.256 s | — |

**dashu** was chosen: GCD is 16 times faster, rationals ~100 times faster. An exact rational core does a GCD on every
operation. The rest is even (±25%).

**Where pure Rust falls short.** Factoring semiprimes with two large factors: 25 digits (two primes ~10^12) —
0.1 s; 31 digits (~10^15) — 1.5 s; 37 digits (~10^18) — Pollard–Brent does not finish within the limits (20 s, 5·10^7
steps) and honestly says "not factored". What is needed here is not another language runtime but another algorithm: ECM or
the quadratic sieve. Ready-made options — FLINT (`fmpz_factor`, LGPL: only dynamic linking of the system library,
subject to an owner decision) or our own ECM on dashu. Such numbers did not come up in the plans of this wave.

Not used for the closed product: PARI/GP (GPL), `malachite` (LGPL-3.0, static Rust linking violates
its terms), `symbolica` (commercial license). GMP/MPFR/FLINT (LGPL) — only dynamically and only by an owner decision; not
connected in v2.

### Plan language

The LLM's answer is one JSON line per problem:

```json
{"id":"gsm8k-test-1173","method":"percent","steps":[
  {"id":"pods","op":"compute","expr":"100 * (1 + 20/100)"},
  {"id":"per_dollar","op":"compute","expr":"pods / 12"},
  {"op":"check","expr":"per_dollar * 12 == 120","why":"cycles per dollar times price gives the 120 pods"}],
 "answer":"per_dollar","type":"integer"}
```

| operation | what the SLM does | independent check |
|---|---|---|
| `compute` | exact evaluation of the expression | the same formula in f64 or a fingerprint modulo 2^61−1; a certificate for each function (GCD — Bézout, C(n,k) — Legendre, p(n) — another algorithm, modpow — other code, remainder — `0 ≤ r < m, m | a − r`…) |
| `solve` | linear systems (Gauss), a single polynomial or rational equation (roots with multiplicities, radicals), an inequality | substituting each solution into the original equations (radicals — exactly in Q(√d)), completeness of roots, product of factors; denominator roots are discarded |
| `factor` | integer — prime factors; polynomial — factorization over Q | product = original, each factor is prime |
| `expand` | expansion into coefficients | values match at three rational points |
| `derive` | derivative with rule steps | central difference at 8 points |
| `enumerate` | nested enumeration (bounds depend on outer variables), count/sum/min/max/list/first | step and time limits; completeness of enumeration |
| `check` | an LLM condition (substituting the answer into the problem statement) | must be true |
| `claim` | an LLM conjecture | enumeration of all small cases; a counterexample means rejection |

The answer gate: all checks green; type (`integer` — a count of items); for multiple-choice tests — the computed value
matches the chosen option. If it fails, the plan goes back to the LLM with the reasons (≤ 2 retries), then "not
solved". A panic inside the SLM is also a gate failure with a reason, not a crash of the run.

### Curriculum: from small to large

A higher level is entered only when, on the held-out sample of the current level, LLM + SLM checked and got ≥ 90% right.
Samples are fixed by seed 20260926 (`curric.rs`); each level is split in half: `dev` (memory and learning) and
`held` (held out).

| level | what | suites (license) | problems |
|---|---|---|---|
| 0 | arithmetic and word problems | GSM8K test (MIT) | 200 |
| 1 | algebra: equations, systems, polynomials | TAL-SCQ5K-EN, "equations" and "formulas", easy (MIT) — 40; MMLU high_school_mathematics, algebra (MIT) — 20 | 60 |
| 2 | number theory and combinatorics | TheoremQA, integer answers, topics GCD/primes/remainders/partitions/Stirling… (MIT) — 30; TAL-SCQ5K-EN, "number theory" and "counting" (MIT) — 30 | 60 |
| 3 | competition level | TAL-SCQ5K-EN difficulty ≥ 2, mathematical subtopics only (MIT) — 20; MMLU college_mathematics (MIT) — 20 | 40 |
| 4 | proofs | ProofNet# — Lean 4 statements from textbooks, without Putnam (MIT) | 10 |

SLM priors: for GSM8K the answer is an integer (all answers in the suite are counts), for TheoremQA — the answer type from
the suite's metadata; for multiple-choice tests — the options.

**Not taken:** hendrycks/competition_math and derivatives (MATH-500, PRM800K, MetaMathQA, NuminaMath — DMCA or
AoPS content), AIME/AMC suites (AoPS/MAA), deepmind/math_dataset (no license on the HF card, the original on
GitHub was not checked), MathOdyssey (no license on the card). **Grey zone — only by an owner decision:**
IMO-AnswerBench (CC BY 4.0, 400 problems with a short answer, but the problems come from the IMO Shortlist and national
olympiads), OlympiadBench (Apache-2.0, competition problems; downloaded, not used). For level 4 next — Nemotron-IMO-Bench
(CC BY 4.0, 200 olympiad proof problems written for the suite; "for evaluation, not for training").
Suite cards — `$MOVA_DATA/raw/mathsolve/cards/`.

The first selection for level 3 (TAL difficulty ≥ 2 without a topic filter) turned out to be noise — AP statistics, biology,
physics — so the level was re-selected before launch.

### Measurements (26.09)

LLM — `claude-opus-5-5`, effort high, batches of 10 problems, 2 calls in parallel. A — the LLM alone (answer
only); B — the LLM writes a plan, the SLM executes and checks, ≤ 2 retries; C — the SLM alone (perceptron + template from
memory), where it cannot — B's answer. Price and tokens are the per-problem share of a call.

| level | conf. | problems | correct | held correct | checked by SLM | $ | $/problem | output tokens/problem |
|---|---|---|---|---|---|---|---|---|
| 0 | A | 200 | 193 (96.5%) | 98/100 | — | 0.94 | 0.0047 | 51 |
| 0 | B | 200 | 195 (97.5%) | 99/100 | 200 | 1.69 | 0.0085 | 208 |
| 1 | A | 60 | 59 (98.3%) | 30/30 | — | 0.32 | 0.0053 | 79 |
| 1 | B | 60 | 59 (98.3%) | 30/30 | 60 | 0.93 | 0.0155 | 431 |
| 2 | A | 60 | 56 (93.3%) | 29/30 | — | 0.32 | 0.0054 | 89 |
| 2 | B | 60 | 56 (93.3%) | 29/30 | 60 | 0.76 | 0.0126 | 299 |
| 3 | A | 40 | 40 (100%) | 20/20 | — | 0.23 | 0.0058 | 99 |
| 3 | B | 40 | 40 (100%) | 20/20 | 40 | 0.58 | 0.0146 | 388 |

- **All levels 0–3 passed:** on the held-out samples B checked and got right 99%, 100%, 96.7%, 100%.
- **Retries:** the gate returned 10 plans out of 360 to the LLM (L1 — 5, L2 — 3, L3 — 2); after a retry all 10 passed and
  were correct. Reasons: a non-integer result where an integer was declared (`0/sqrt(2)` gave an approximate zero — now an exact
  zero stays exact), list concatenation with `+` (now supported), syntax.
- **The wrong answers of A and B are the same** — this is not arithmetic but interpretation: 5 of 200 in GSM8K are ambiguous statements
  ("70 times as many" — the increase or the total, "tripled his income" — the raise or the whole income); at level 2, of 4
  "wrong" answers 3 are **errors in the gold standard**, revealed by the SLM's verified traces: TheoremQA-356 (n² + 3n + 2 is divisible by
  6 for 17 values of n ≤ 25 — enumeration; gold 13), TheoremQA-112 (6 people at 2 identical round tables = c(6,2) = 274;
  gold 225 = c(6,3)), TAL "odd one out" (values [8, 6, 8, 8] → B; gold D); the fourth is an ambiguity
  ("digits 1, 3, 5" with or without repetition).
- **B costs ≈ 1.8–2.9× A**: the LLM writes a plan (~200–430 tokens per problem) instead of an answer (~50–100). At these
  levels Opus itself almost never errs in arithmetic, so B's gain is not in accuracy but in the fact that every
  answer has a verified trace, and errors in the gold standard become visible.
- **Operations in 360 plans:** compute 798, check 386 (the LLM itself writes a substitution check in ~every plan),
  solve 37, enumerate 17, expand 3, claim 3, derive 2.

**C — the SLM chooses the method on its own.**

| level | held | perceptron method = LLM method | solved by the SLM alone (correct) |
|---|---|---|---|
| 0 | 100 | 66 | 0 |
| 1 | 30 | 7 | 1 (1) |
| 2 | 30 | 23 | 1 (1); first policy — 3 (1) |
| 3 | 20 | 9 | 0 |

The first policy (perceptron margin ≥ 0.5, word Jaccard ≥ 0.35) gave two wrong answers at level 2: a CRT template was spoiled by
a "1" from the statement that could not be told apart from a plan constant, and "ordered subsets" and "subsets" are similar in words but not
in structure. The second policy — precision over coverage: the template must be complete (every number of the statement is a slot) and
unambiguous (there are no 0 and 1 among the statement's numbers), the statement structure is a skeleton with "#" instead of numbers, LCS ≥ 0.9. It
was devised after seeing the L2 held-out sample, so the clean check is only on L3 (0 attempts, 0
wrong). Both solutions by the SLM alone are near-verbatim duplicates within the suites. **Conclusion:** with 360 solutions
in memory the perceptron already guesses the LLM's method in 66% of GSM8K problems, but the SLM builds a plan on its own only for problems of the same
structure. To ask the LLM less often, memory needs to be orders of magnitude larger, with templates at the level of steps rather than whole
plans.

**Memory:** 360 verified solutions — `$MOVA_DATA/runs/math2-2026-09-26/memory.jsonl` and table
`solutions` in `$MOVA_DATA/db/mathsolve.duckdb` (also `records` — each problem × configuration, `calls` — each
LLM call with price and tokens).

**Negative controls** (`mathsolve controls`, 7 of 7 red): substituting an intermediate result in the trace
(3^40 + 17 → +1 — fingerprint modulo; 48/2 → 25 — f64), the false conjecture "n² + n + 41 is prime" (counterexample
n = 40), Fermat's false conjecture "2^(2^n) + 1 is prime" (n = 5), a false LLM check (roots [2, 4]), the chosen
option does not match the computed one, a non-integer result where a count is expected. The core unit tests also have red
controls (wrong GCD, wrong C(n,k), wrong derivative, wrong substitution into a system).

### Trace examples (`mathsolve show <id>`)

GSM8K, 1173 (checked in the first round):

```
[pods] compute 100 * (1 + 20/100) → 120            ✓ independent path: f64 gave 120
[per_dollar] compute pods / 12 → 10                 ✓ independent path: f64 gave 10
[_3] check per_dollar * 12 == 120                   ✓ LLM check: cycles per dollar × price = 120 pods
[answer] per_dollar → 10                            ✓ fingerprint modulo 2^61−1; ✓ type: integer
```

TheoremQA-356, "how many n ≤ 25 make n² + 3n + 2 divisible by 6" (gold 13 is wrong):

```
[cnt] compute count((n^2 + 3*n + 2) % 6 == 0, n, 1, 25) → 17     · enumeration of 25 cases = 17
[_2] check cnt == 25 - idiv(25, 3)     ✓ (n+1)(n+2) is always even and divisible by 3 if 3 ∤ n
[answer] cnt → 17                      ✓ fingerprint; ✓ type: integer
```

MMLU college_mathematics 18, work of a force along a curve (the gate returned the plan):

```
round 0: (-(1-0) + 0·(1²-0²) + (1³-0³)) / sqrt(2) → ≈ 0   ✗ type: integer declared, got an approximation
round 1: [dot_num] (-1)·(1-0) + 0·(1²-0²) + 1·(1³-0³) → 0   ✓ fingerprint
         [line_int] (-1 + 1³) - (-0 + 0³) → 0               ✓ (antiderivative −t + t³ of F·r′)
         check line_int == dot_num ✓; check work == 0 ✓; ✓ test: 0 — option C
```

### CLI v2

```
cargo build --release
../target/release/mathsolve prep                    # level suites from $MOVA_DATA/raw/mathsolve (seed 20260926)
../target/release/mathsolve run 0 B --split held    # A | B; --limit, --batch, --par, --reserve; cap MATH2_CAP ($15)
../target/release/mathsolve c 2                     # C: the SLM alone on the level's held-out sample
../target/release/mathsolve report                  # level × sample × configuration
../target/release/mathsolve show gsm8k-test-1173 B  # problem, plan, trace, gate history
../target/release/mathsolve memory                  # memory.jsonl → $MOVA_DATA/db/mathsolve.duckdb
../target/release/mathsolve controls                # negative controls of the gate (must be red)
../target/release/mathsolve calc "binom(1000,500) mod (10^9+7)"
../target/release/mathsolve lean-prep | lean-controls | lean-run
```

Run directory: `MATH2_RUN` or `$MOVA_DATA/runs/math2-2026-09-26/` (`levels/`, `records.jsonl`, `records-c.jsonl`,
`calls.jsonl`, `memory.jsonl`, `lean/`). A repeated `run` skips problems already solved — we do not pay twice.

### v2 licenses

| dependency | version | license | why |
|---|---|---|---|
| `dashu` (`-int`, `-ratio`, `-base`, `-float`, `-cmplx`, `-macros`) | 0.6.1 | MIT OR Apache-2.0 | big integers and rationals; the fastest of the permissive ones (benchmark above) |
| `num-modular`, `num-order` (via dashu) | 0.6.5, 1.2.0 | Apache-2.0 | modular arithmetic inside dashu |
| `serde`, `serde_json` | 1.0.229, 1.0.151 | MIT OR Apache-2.0 | LLM plans, traces, memory — JSON |
| `anyhow` | 1.0.104 | MIT OR Apache-2.0 | CLI errors |
| other transitive: `cfg-if`, `itoa`, `memchr`, `pastey`, `proc-macro2`, `quote`, `syn`, `static_assertions`, `unicode-ident`, `zmij` | — | MIT / Apache-2.0 / Unlicense; `unicode-ident` also Unicode-3.0 | utility, via serde and dashu |
| `num-bigint` 0.4.8 and 0.5.1, `num-rational` 0.4.2, `num-integer` 0.1.47, `num-traits` 0.2.19 | — | MIT OR Apache-2.0 | only `bigbench/` (measurement); not part of the `math` crate |

Lean 4 and Mathlib (Apache-2.0) are a separate verification tool in `$MOVA_DATA/lean/mathproofs`, not linked.
Problem suites are only for measurement and the solution memory; licenses are in the curriculum table above.

### Level 4: proofs in Lean (pilot)

- **Format:** the Lean 4 statement is fixed (ProofNet#, `lean4_src_header` + `lean4_formalization`), the informal
  statement is a hint. The LLM writes only the proof body after `:= by` (blocks `=== id … === end`); the SLM assembles
  the file itself, so the statement cannot be swapped.
- **SLM check:** `lake env lean <file>` in `$MOVA_DATA/lean/mathproofs` (Lean v4.35.0-rc3, Mathlib with cache,
  no network commands), limit 240 s. Rejection — a Lean error, the warning "declaration uses 'sorry'", or in the
  text `sorry`/`admit`/`axiom`/`native_decide`/`unsafe`/`implemented_by`/`extern`. Lean errors go back to the LLM,
  ≤ 2 retries.
- **Controls** (`mathsolve lean-controls`): a correct proof of a² + b² ≥ 2ab (`nlinarith`) is green; `sorry`,
  `admit` are red (SLM filter); a wrong tactic (`linarith`), a non-existent lemma are red (Lean error).
- **Selection:** 13 candidates by seed (without Putnam — competition problems), 3 do not elaborate in our Mathlib (renames
  since 2024), leaving 5 dev + 5 held.
- **Measurement:** A — first attempt without feedback; B — with Lean errors.

| | theorems | proved | $ | output tokens |
|---|---|---|---|---|
| A: LLM alone (first attempt) | 10 | 6 (dev 3/5, held 3/5) | 1.58 | 75 345 |
| B: LLM + Lean gate, ≤ 2 retries | 10 | 7 (dev 3/5, held 4/5) | 2.20 | 94 253 |

A retry rescued Herstein 4.6.2 (irreducibility of x³ + 3x + 2 over Q: `le_or_lt` disappeared in the new Mathlib — the LLM
rewrote it after the Lean error). Not proved: Axler 6.16 (`U.orthogonal = ⊥ ↔ U = ⊤` without finite dimensionality —
type classes are not synthesized; the statement seems false in general), Dummit–Foote 4.5.28 (a group
of order 105 with a normal Sylow 3-subgroup is abelian — 120 lines, "motive is not type correct"),
Dummit–Foote 7.2.2. The first call took 385 s and $0.93 for 5 theorems: Lean proofs are ~20 times more expensive than
numeric plans.

### Independent tests (tests3, 26.09)

- `tests/data/vmm/calc.tsv` — **100 expressions in the plan language** from an independent author (Opus, effort high, had not seen the code;
  prompts alongside): exact arithmetic and precedence, big integers, number theory (GCD, modpow, CRT, primes, φ, σ, μ,
  factorization), combinatorics (binomials, Catalan, Stirling, Bell, partitions, derangements, Fibonacci), aggregates with a
  variable, lists, logic; ERR — must be an error. Frozen by hash before the first run (commit `66711cecf`).
- The expected value is a literal that the same `calc` evaluates; we compare values, not notation.
- **First run — 100/100**, no defects found. The negative control (a tampered expected value) is red.
  Test — `tests/vmm_calc.rs`.

### What next

1. **Level 4 wider:** ProofNet# in full (371 statements) — when there is a budget; a memory of verified proofs as
   hints (similar statements → tactics). Olympiad proofs (Nemotron-IMO-Bench) — formalization first:
   the LLM writes a Lean statement, the SLM checks that it is not trivial (`simp`/`decide` do not close it, the negation
   is not proved quickly, small cases agree with a `claim` enumeration), and only then the proof.
2. **Grey zone — owner decision:** IMO-AnswerBench (CC BY 4.0, competition problems) is exactly the "top-level world
   problems" with a short answer; without it level 3 is only student/junior competition level.
3. **Ask the LLM less often:** templates at the level of steps (not whole plans), memory orders of magnitude larger (large measurements
   — when there is a budget), the perceptron also for choosing a subset of the plan language in the prompt (a shorter prompt).
4. **Core:** ECM for factoring large semiprimes (own on dashu or FLINT dynamically — owner decision), Cardano/Ferrari
   radicals, nonlinear systems (resultants), integrals (then Rubi rules as engine data), linear
   algebra (determinant, inverse, rank), symbolic root values in the plan language (currently irrational roots
   are approximate numbers, the exact form is only in the trace).
5. **Merging the cores:** v1 (`rat.rs`, i128, units) and v2 (`calc.rs`, dashu) — a single expression tree.

## Licenses

- Code — the Elamur Cosmic License (`LICENSE.md` at the repo root).
- v1 dependencies: `en` (own), `anyhow` (MIT/Apache-2.0). v2 dependencies — the table in the v2 section.
- Unit definitions are international agreements (the 1959 yard and pound, conventional mmHg) — facts, not a work.
- The dictionary and suites are our own.

## What v1 lacks

- Speed × time ("How far … in 3 hours at 60 mph") — an honest "I don't understand"; there is no "rate · time" frame.
- Yes/no questions ("Is 30 °C hotter than 80 °F?") give the difference, not "yes".
- Ordinal numerals ("10th power"), "percent off", "next 5 days" — not in the dictionary.
- Transcendental functions — only f64 with "≈"; no symbolic simplification yet (the tree is already symbolic).
- i128: 3^100 overflows (in v2 `math eval` falls through to the exact v2 core with dashu).
- Frames are rules, not a trained model; the dictionary is hand-made.

## Roadmap

Division of labour: **the LLM states the problem and writes the plan, the SLM computes exactly and checks.** The core is symbolic from
the start: numeric evaluation is only one of the operations on the tree; every result comes with an independent
check where possible (substitute a root back, differentiate an integral, compare A·A⁻¹ with I).

1. Arithmetic, units, percentages, statistics, probability — **v1 (26.09)**.
2. Algebra: simplification, expansion and factorization, equations — linear, quadratic, systems by Gauss over the rationals.
3. Analysis: derivatives, limits, Taylor series, integrals — table, substitution, by parts. Then Rubi rules as
   engine data (from memory — MIT; verify the license).
4. Linear algebra over the rationals: determinant, inverse, rank, eigenvalues via the characteristic polynomial.
5. Discrete mathematics and number theory: combinatorics on big integers, GCD, primes, modular arithmetic.
6. Probability and statistics: distributions, expectation, variance.
7. First-order ordinary differential equations.

Measurement — open suites: GSM8K, MATH, MMLU college mathematics, Rubi and SymPy tests. Verify each license
before use; none of them has been downloaded yet. GSM8K — reasoning problems, for the "LLM writes the plan,
SLM computes" pairing: the next step, a paid one.

Queue: continuation after world-building (priority set on 26.09). The first step of level 3, when its turn comes, is
a symbolic derivative with rule steps and a check by numeric comparison at several points.
