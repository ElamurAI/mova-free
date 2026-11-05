# math — the SLM's basic mathematics module

`math` is exact mathematics for the small model: rationals, symbolic expressions, units with dimensions, steps with independent checks, and English questions read into expressions through UD trees.

The aim is the level of a university mathematics student. The crate has two parts:

- **`math eval`** (`rat.rs` on i128, units, the UD reader): level 1 — arithmetic, units, percentages, statistics,
  probability. Its application is a free weather chatbot: SLM only, deterministic, on CPU.
- **`mathsolve`** (`calc.rs` on `dashu`): a curriculum-based solver where the LLM writes a plan and the SLM executes it
  exactly and checks every step (section "`mathsolve`: a curriculum-based solver").

The place of mathematics among the SLM subsystems, and the slow models `arith` and `steps`, are described in
[docs/context.md](../docs/context.md#mathematics). School results are in [docs/school.md](../docs/school.md#results).

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
(`CheckFailed`). The checks of `math eval`:
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
3. Placeholders for the parser: "72 °F" → "72 degrees", "10 mi" → "10 units", so the `en` parser does not stumble
   on "°" and "F"; `en` parse (the `en.bin` model) and mapping of nodes to elements.
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

## `math eval` suites and tests

- `data/suite-v1.tsv` is the dev set; `data/holdout-v1.tsv` is held out and run once. Both were frozen by hash before
  the understanding code. The held-out set was written by the same author, so it is not an independent yardstick, only
  a sign that the frames are not tied to exact sentences.
- Phrases written after the code (parentheses, "x minus p %", the "degrees" scale from the target, printing) are
  pinned by the `probe_regressions` test.
- Negative controls: dimension errors, nonsense → "I don't understand", overflow and division by zero, absolute
  versus difference (the pair "change of 10 °F" / "10 °F" must give different answers).
- `cargo test`: unit tests of the core (conversions by exact definitions, affinity, associativity and
  distributivity, reduction, overflow, comparison without overflow) and end-to-end tests (brief samples,
  negative controls, the "no wrong answers" regression on suite-v1).

## `mathsolve`: a curriculum-based solver

`mathsolve` learns to solve problems from small to large, up to top-level world problems; for speed it calls a Rust
library for exact mathematics.

### Division of labour

- **The LLM reasons and writes a plan** in a typed step language (JSON): `compute`, `solve`, `factor`, `expand`,
  `derive`, `enumerate`, `check`, `claim`. It does not compute itself: expressions only.
- **The SLM (this crate) executes exactly and checks** each step by an independent path. If it fails, the plan goes back to the LLM
  with the reason, at most 2 retries, then an honest "not solved".
- **Learning is a memory of verified solutions**, not weights in a black box: problem features, method, plan,
  trace (`memory.jsonl`, table `solutions` in `$MOVA_DATA/db/mathsolve.duckdb`). The method is chosen by an averaged
  perceptron over problem features (it is visible which features outweighed and with what weight); the plan is a template from memory in
  which the numbers of the problem statement are replaced by slots. Where the perceptron is confident and the template fits, the SLM solves on its own, without the LLM.

### Modules

| module | what it does |
|---|---|
| `big.rs` | big integers and rationals — `dashu` (see "Exact arithmetic library" below); parsing and printing |
| `nt.rs` | GCD/LCM with a Bézout certificate (own extended Euclid), modpow by two paths, inverse, general CRT (moduli not necessarily coprime), primality: deterministic Miller–Rabin for u64 (12 bases), for big numbers MR with 30 bases and a Pocklington certificate or an honest "probably prime"; factorization: trial division up to 10^6 and Pollard–Brent with a step and time budget; φ, τ, σ_k, μ, divisors, π(n) by sieve, k-th prime, digits in base b |
| `comb.rs` | n! by a product tree, C(n,k) (second path — Legendre's prime factorization), P(n,k), Catalan, Stirling I/II, Bell, partitions p(n) (Euler versus dynamic programming), into k parts, into distinct parts (= into odd parts), derangements, Fibonacci (Cassini identity), multinomials |
| `poly.rs` | polynomials over Q: arithmetic, GCD, derivative, Yun's square-free factorization; rational roots; quadratics exactly `a ± b√d` (d < 0 — complex) with a check in Q(√d); degree ≥ 3 without rational roots — numerically (Aberth) marked "≈"; factorization over Q; inequalities N/D ⋈ 0 by signs on intervals; expression → rational function with a memory of denominators (a root of the denominator is discarded) |
| `linsys.rs` | linear systems of any size — Gauss–Jordan over the rationals: unique / infinitely many (free variables) / inconsistent (witness row) |
| `deriv.rs` | symbolic derivative with rule steps (sum, product, quotient, power, chain, exponential, u^v, trigonometric, inverse) and a central-difference check at 8 points |
| `calc.rs` | the plan expression language: exact evaluation, ~90 functions, enumeration with a bound variable (`sum`, `count`, `forall`, `first`…) with step and time limits; the independent path is the same formula in f64 or a fingerprint modulo 2^61−1 (for huge integers, where f64 lies in the low digits) |
| `plan.rs` | LLM plan → execution → trace with checks → gate; negative controls |
| `vmm.rs` | the teacher through the shared [`mova-teacher`](../teacher) Codex CLI transport (model from `MOVA_TEACHER_MODEL` or `MATH_MODEL`, default `gpt-6.1-terra`), a `calls.jsonl` log and a call cap before every call |
| `curric.rs` | levels, sampling with seed 20260926, comparison with the gold standard, prompts |
| `memory.rs` | problem features, perceptron, plan templates |
| `solve.rs` | configurations A, B, C and level 4 (Lean) |
| `lean.rs` | level 4: the statement is fixed, the LLM writes the proof, `lake env lean` checks it |
| `bin/mathsolve.rs` | CLI |

`rat.rs` on i128 serves units and weather; an i128 overflow in `math eval` falls through to the
exact `dashu` core: "What is 3 to the power of 100?" → 515377520732011331036461129765621272702107522001.

### Exact arithmetic library

We do not write exact arithmetic ourselves — we take a ready one. The criterion is the license (the product is closed),
then speed. **dashu** is used: an exact rational core does a GCD on every operation, and dashu's GCD and rationals are
the fastest among the permissive libraries. `bigbench/` (`cargo run --release`) compares dashu with num-bigint on the
same algorithms and cross-checks the results.

**Factoring.** Trial division and Pollard–Brent run within a step and time budget (20 s, 5·10^7 steps); a semiprime
with two large factors may not finish and is then honestly reported as "not factored". Such numbers need another
algorithm, ECM or the quadratic sieve: FLINT (`fmpz_factor`, LGPL: only dynamic linking of the system library, subject
to an owner decision) or our own ECM on dashu.

Not used for the closed product: PARI/GP (GPL), `malachite` (LGPL-3.0, static Rust linking violates
its terms), `symbolica` (commercial license). GMP/MPFR/FLINT (LGPL) — only dynamically and only by an owner decision; not
connected.

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
olympiads), OlympiadBench (Apache-2.0, competition problems; downloaded, not used). A candidate for level 4, not used:
Nemotron-IMO-Bench (CC BY 4.0, 200 olympiad proof problems written for the suite; "for evaluation, not for training").
Suite cards — `$MOVA_DATA/raw/mathsolve/cards/`. Level 3 takes mathematical subtopics only: TAL difficulty ≥ 2
without a topic filter also holds AP statistics, biology and physics.

### Configurations and memory

- **A** — the LLM alone (answer only).
- **B** — the LLM writes a plan, the SLM executes and checks, ≤ 2 retries.
- **C** — the SLM alone (perceptron + template from memory); where it cannot, B's answer.

The LLM is the teacher, called in batches of problems (`--batch`, `--par`); each call is logged with tokens.

**C's policy is precision over coverage:** the template must be complete (every number of the statement is a slot) and
unambiguous (there are no 0 and 1 among the statement's numbers), and the statement structure, a skeleton with "#"
instead of numbers, must match with LCS ≥ 0.9.

**Memory:** `memory.jsonl` in the run directory and table `solutions` in `$MOVA_DATA/db/mathsolve.duckdb` (also
`records` — each problem × configuration, `calls` — each LLM call with tokens).

**Negative controls** (`mathsolve controls`, all must be red): an intermediate result substituted in the trace (caught
by the fingerprint modulo 2^61−1 or by f64), the false conjecture "n² + n + 41 is prime", Fermat's false conjecture
"2^(2^n) + 1 is prime", a false LLM check, a chosen option that does not match the computed one, a non-integer result
where a count is expected. The core unit tests also have red controls (wrong GCD, wrong C(n,k), wrong derivative,
wrong substitution into a system).

### Trace example (`mathsolve show <id>`)

GSM8K, 1173:

```
[pods] compute 100 * (1 + 20/100) → 120            ✓ independent path: f64 gave 120
[per_dollar] compute pods / 12 → 10                 ✓ independent path: f64 gave 10
[_3] check per_dollar * 12 == 120                   ✓ LLM check: cycles per dollar × price = 120 pods
[answer] per_dollar → 10                            ✓ fingerprint modulo 2^61−1; ✓ type: integer
```

When the gate returns a plan, `show` prints each round with the failed check and its reason.

### `mathsolve` CLI

```
cargo build --release
../target/release/mathsolve prep                    # level suites from $MOVA_DATA/raw/mathsolve (seed 20260926)
../target/release/mathsolve run 0 B --split held    # A | B; --limit, --batch, --par, --reserve; cap MATH2_CAP
../target/release/mathsolve c 2                     # C: the SLM alone on the level's held-out sample
../target/release/mathsolve report                  # level × sample × configuration
../target/release/mathsolve show gsm8k-test-1173 B  # problem, plan, trace, gate history
../target/release/mathsolve memory                  # memory.jsonl → $MOVA_DATA/db/mathsolve.duckdb
../target/release/mathsolve controls                # negative controls of the gate (must be red)
../target/release/mathsolve calc "binom(1000,500) mod (10^9+7)"
../target/release/mathsolve lean-prep | lean-controls | lean-run
../target/release/mathsolve prep-train [N]          # training level: N GSM8K train problems (default 1000)
../target/release/mathsolve plan <file.json>        # execute a plan locally, show the trace
../target/release/mathsolve arith [--train N] [--epochs E] [--leaves L] [--beam B]   # subproblem trees on GSM8K, no LLM
../target/release/mathsolve steps --set svamp|gsm8k [--exact] [--epochs E] [--beam B] [--qread] …   # the transition reader; the SVAMP benchmark command is in ../tests/README.md
../target/release/mathsolve qread [svamp|svamp-train|asdiv|mawps-<file>] [show N]   # coverage and accuracy when the world answers
../target/release/mathsolve qread-text "<problem>"  # the world reader on one text (QREAD_TREE=1 prints the trees)
```

Run directory: `MATH2_RUN`, default `$MOVA_DATA/runs/math2-2026-09-26/` (`levels/`, `records.jsonl`, `records-c.jsonl`,
`calls.jsonl`, `memory.jsonl`, `lean/`). A repeated `run` skips problems already solved — we do not pay twice.

### `mathsolve` dependencies

| dependency | version | license | why |
|---|---|---|---|
| `dashu` (`-int`, `-ratio`, `-base`, `-float`, `-cmplx`, `-macros`) | 0.6.1 | MIT OR Apache-2.0 | big integers and rationals; the fastest of the permissive ones (`bigbench/`) |
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
- **Selection:** 10 statements by seed (without Putnam — competition problems), 5 dev + 5 held; candidates that do not
  elaborate in our Mathlib are skipped.
- **Configurations:** A — first attempt without feedback; B — with Lean errors fed back. A Lean proof costs far more
  than a numeric plan.

### Independent tests

- `tests/data/vmm/calc.tsv` — **100 expressions in the plan language** from an independent author (the teacher, which had not seen the code;
  prompts alongside): exact arithmetic and precedence, big integers, number theory (GCD, modpow, CRT, primes, φ, σ, μ,
  factorization), combinatorics (binomials, Catalan, Stirling, Bell, partitions, derangements, Fibonacci), aggregates with a
  variable, lists, logic; ERR — must be an error. Frozen by hash before they were run (commit `66711cecf`).
- The expected value is a literal that the same `calc` evaluates; we compare values, not notation.
- Test — `tests/vmm_calc.rs`; its negative control (a tampered expected value) must be red.

## Licenses

- Code — the Elamur Cosmic License (`LICENSE.md` at the repo root).
- `math eval` dependencies: `en` (own), `anyhow` (MIT/Apache-2.0). `mathsolve` dependencies — the table in its section.
- Unit definitions are international agreements (the 1959 yard and pound, conventional mmHg) — facts, not a work.
- The dictionary and suites are our own.

## Limits

`math eval`:

- Speed × time ("How far … in 3 hours at 60 mph") — an honest "I don't understand"; there is no "rate · time" frame.
- Yes/no questions ("Is 30 °C hotter than 80 °F?") give the difference, not "yes".
- Ordinal numerals ("10th power"), "percent off", "next 5 days" — not in the dictionary.
- Transcendental functions — only f64 with "≈"; no symbolic simplification (the tree is already symbolic).
- i128: 3^100 overflows (`math eval` falls through to the exact `dashu` core).
- Frames are rules, not a trained model; the dictionary is hand-made.

`mathsolve`:

- The SLM alone builds a plan only for problems of the same structure as one in memory; templates are whole plans,
  not steps.
- Level 3 is student/junior competition level; IMO-AnswerBench (CC BY 4.0, competition problems with a short answer)
  is in the grey zone and not used.
- Level 4 covers 10 ProofNet# statements, not the full 371; there is no memory of proofs and no statement
  formalization for olympiad problems.
- The core has no ECM or quadratic sieve (semiprimes with two large factors may not be factored), no Cardano/Ferrari radicals, nonlinear
  systems, integrals or linear algebra; irrational roots are approximate numbers in the plan language (the exact form
  is only in the trace).
- The two cores, `rat.rs` (i128, units) and `calc.rs` (dashu), are separate expression trees.

## Direction

Division of labour: **the LLM states the problem and writes the plan, the SLM computes exactly and checks.** The core is symbolic from
the start: numeric evaluation is only one of the operations on the tree; every result comes with an independent
check where possible (substitute a root back, differentiate an integral, compare A·A⁻¹ with I).

1. Arithmetic, units, percentages, statistics, probability — implemented in `math eval`.
2. Algebra: simplification, expansion and factorization, equations — linear, quadratic, systems by Gauss over the rationals
   (equations, systems, factorization and derivatives are implemented in `mathsolve`).
3. Analysis: derivatives, limits, Taylor series, integrals — table, substitution, by parts. Then Rubi rules as
   engine data (from memory — MIT; verify the license).
4. Linear algebra over the rationals: determinant, inverse, rank, eigenvalues via the characteristic polynomial.
5. Discrete mathematics and number theory: combinatorics on big integers, GCD, primes, modular arithmetic.
6. Probability and statistics: distributions, expectation, variance.
7. First-order ordinary differential equations.

Measurement — open suites: GSM8K and MMLU college mathematics (used by `mathsolve`), Rubi and SymPy tests. Verify each license
before use.
