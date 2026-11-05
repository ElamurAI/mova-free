# algorithm-gcd

## descriptant

### gcd descriptant

The local mathematical combiner reads the authoritative Rust source, source-bound partial syntax, forward relation, five typed models, invariants and finite test observations. It updates conditional claims while retaining earlier predictions and failure observations. This is a documented review operation, not an autonomous generator.

Two tests passed twice. Divisor enumeration covers every pair in 0..=128: 16,641 cases, including (0,0). Literals check u64 maximum, powers of two and a long consecutive-Fibonacci chain. Mutating terminal return a to b compiled and failed (exit 101). An earlier remainder-to-division control did not terminate within 10 seconds; it was bounded externally and replaced by the terminating return mutation. Production source never changed.

Scope is successful calls on lawful generic implementations where applicable. Partial syntax omits macro expansion, cfg resolution and semantic typing. Finite oracles do not establish all inputs or full Mova integration.

Parent workspace wiring and higher-level registry composition are handled separately; these leaf relations establish no application integration by themselves.

Test-extraction epoch: the test harness is now adjacent in ../tests.rs. model.md#test-extraction retains the new oracle scope and mutation observations alongside this earlier epoch.

## model

# gcd model

## Forward

gcd(a,0)=a. For b>0, gcd(a,b)=gcd(b,a mod b). Thus gcd(0,0)=0; otherwise return the largest positive common divisor.

## Invariant

The positive common-divisor set is unchanged by (a,b)->(b,a mod b). For b>0 the new second argument is strictly smaller; termination reaches (g,0). Remainder divides only by a positive value and multiplication is absent.

## Cost

O(1+log(min(a,b)+1)) remainder steps for nonzero values, at most one for a zero input; O(1) space. Fixed-width u64 operation cost assumed; no timing measured.

## nonbehavior

Type: behavior_complement. U=u64^2 x u64. R(a,b,g) is the forward gcd relation. U minus R contains g!=0 for (0,0), and otherwise g=0, a value not dividing both inputs, or a common divisor smaller than another common divisor. This is a mathematical exact target, source-derived and only bounded-tested.

## antecedents

Type: reverse_reconstruction. For fixed g>0, compatible inputs are (g*x,g*y) with products fitting u64 and gcd(x,y)=1, including (0,g),(g,0). For g=0 only (0,0). Multiple coprime pairs preserve ambiguity; no unique reconstruction.

## symmetry

Type: argument_permutation. Swap inputs: gcd(a,b)=gcd(b,a) for all u64 pairs. Control flow length may change, but output does not. Full bounded grid compares both ordered pairs with the divisor oracle.

## zero_boundary

Type: boundary_specialization. Restrict input to a=0 or b=0. Output is max(a,b), including zero at the intersection. This specialization explains absent division by zero and is checked with zero/max literals and bounded pairs.

## divisor_lattice

Type: predicate_projection. For (a,b)!=(0,0) and positive d, d divides output iff d divides both inputs. Output is the greatest element under divisibility of common divisors. Oracle explicitly enumerates bounded common divisors; unbounded predicate equivalence remains a source-derived claim.

## tests

### Bounded evidence

Two tests passed twice. Divisor enumeration covers every pair in 0..=128: 16,641 cases, including (0,0). Literals check u64 maximum, powers of two and a long consecutive-Fibonacci chain. Mutating terminal return a to b compiled and failed (exit 101). An earlier remainder-to-division control did not terminate within 10 seconds; it was bounded externally and replaced by the terminating return mutation. Production source never changed.

Commands: rustfmt --edition 2021 SOURCE; rustc --edition 2021 --test SOURCE -o EXTERNAL_BINARY; EXTERNAL_BINARY --test-threads=1. The two runs' status logs match after elapsed-time removal. Mutants compile from external copies and must exit 101; repository sources remain unchanged. Run artifacts are outside the repository under the task run directory in MOVA_DATA.

Scope is successful calls on lawful generic implementations where applicable. Partial syntax omits macro expansion, cfg resolution and semantic typing. Finite oracles do not establish all inputs or full Mova integration.

Current extraction epoch: tests now live in ../tests.rs; see model.md#test-extraction for four focused tests, new coverage and refreshed evidence. Earlier two-test results above describe the prior epoch.

## prospective

### Prospective

Saved before implementation. Reuse repository-owned Euclidean recurrence in math/src/nt.rs::gcd_u64 and math/src/calc.rs::gcd_small, inspected before writing. Predict divisibility/maximality against bounded divisor enumeration, symmetric zero handling, gcd(0,0)=0, O(log(min(a,b)+1)) divisions plus a constant for zero.

Plan: independent bounded exhaustive oracle, literal boundaries, two deterministic test-status runs and one source mutation that must fail. No timing or full-system claim. No external source or solutions downloaded. Parent owns workspace/registry integration. Syntax projection follows rustfmt and stable source.

## test-extraction

### Test extraction

The implementation prefix is unchanged. Its cfg(test) module now loads ../tests.rs, whose adjacent groupant records the expanded finite domain. Two original tests remain and two new tests pass twice. Truncating terminal gcd through u32 is rejected by full_width_binary_oracle, including the full-width boundary matrix (compiled external copy, exit 101).

Binary gcd uses trailing-zero extraction, shifts, comparisons and subtraction, never Euclidean remainder. Consecutive integers supply a separate algebraic coprimality oracle.

Swap input arguments without changing result; scale consecutive positive integers x,x+1 by a factor only when both checked products fit, expecting exactly that factor.

Partial syntax is refreshed because inline test items moved out. The production source hash changes for module placement, not algorithm behavior. Earlier prospective and executed evidence is retained; current detailed evidence is in ../tests.rs.groupant/model.md#evidence. Full crate check at this isolated base: 36 tests and one doctest pass; owned-source formatting and strict crate Clippy pass. Parent binding/registry checks remain integration work.

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29. Prospective assumptions are preserved in model.md#prospective. Composed from the repository-owned Euclidean implementations in math/src/nt.rs and math/src/calc.rs, found before coding.

### 2026-10-29 · tests

Heading `### Bounded evidence 2026-10-29` stays in tests as `### Bounded evidence`.

### 2026-10-29 · prospective

Heading `### Prospective epoch 2026-10-29` stays in prospective as `### Prospective`.

### 2026-10-29 · test-extraction

Heading `### Test extraction 2026-10-29` stays in test-extraction as `### Test extraction`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
