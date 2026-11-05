# mlab-builtins-io-atom-cycle

## descriptant

### mlab-builtins-io-atom-cycle

Compute segments and atom queue; zero specs returns literals, otherwise cycle. Initially empty queue pads each spec once; exhausted noninitial queue returns immediately at the next spec.

## approximant

### Forward relation

Compute segments and atom queue; zero specs returns literals, otherwise cycle. Initially empty queue pads each spec once; exhausted noninitial queue returns immediately at the next spec.

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

Every productive numeric/string conversion removes an atom or expands a finite string to code atoms; iteration ends when queue empties.

#### Failure ordering

atoms rejects tables/string arrays before any output is returned; format allocation panic remains outside MError model.

#### Cost

O(format scan + repeated segment traversal and per-atom flag scans + atom expansion + output bytes); format/flag work can multiply by expanded atom count, and long padding can dominate.

## models

# Typed models — mlab-builtins-io-atom-cycle

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

A queue exhausted mid-cycle excludes later literal segments after the next unfilled spec. String precision truncates %s only, not %c.

## reverse_reconstruction

Output prefix corresponds to any atom queue whose consumption ends at the same spec boundary; literal-only output permits any convertible atoms because none are consumed.

## state_invariant

Every productive numeric/string conversion removes an atom or expands a finite string to code atoms; iteration ends when queue empties.

## failure_precedence

atoms rejects tables/string arrays before any output is returned; format allocation panic remains outside MError model.

## representation_transport

%s/%c numeric atoms produce valid Unicode scalar or shortest fallback; numeric spec strings expand by Unicode characters.

## resource_cost

O(format scan + repeated segment traversal and per-atom flag scans + atom expansion + output bytes); format/flag work can multiply by expanded atom count, and long padding can dominate.

## tests

### Evidence and limitations

Parent source checks: builtins::io::tests::g_format, builtins::io::wave30_tests::cycling_unicode_and_empty. The fragment's actual statements lie in the recorded byte range; parent checks need not exercise every branch in this fragment. See ../model.md#tests for source-level control and limitations.

AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.
