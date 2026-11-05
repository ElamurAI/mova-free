# mlab-builtins-io-format-parser

## descriptant

### mlab-builtins-io-format-parser

Scan Unicode chars; %% adds literal percent, flags/width/precision/length are collected before supported conversion membership; unsupported prefixes stay literal.

## approximant

### Forward relation

Scan Unicode chars; %% adds literal percent, flags/width/precision/length are collected before supported conversion membership; unsupported prefixes stay literal.

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

Literal spans are unescaped when flushed; supported conversions separate literal and spec segments.

#### Failure ordering

Numeric width overflow yields None; empty precision yields zero; no Result failure channel.

#### Cost

O(format characters) plus literal storage; later padding cost is outside this fragment.

## models

# Typed models — mlab-builtins-io-format-parser

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

%% excludes consuming an atom. Width parse failure excludes an explicit width rather than erroring.

## reverse_reconstruction

One literal % admits %% input or unsupported trailing-percent syntax; discarded length modifiers yield many equivalent spec antecedents.

## state_invariant

Literal spans are unescaped when flushed; supported conversions separate literal and spec segments.

## failure_precedence

Numeric width overflow yields None; empty precision yields zero; no Result failure channel.

## representation_transport

Offsets are character indices into a collected Vec<char>, not UTF-8 byte positions.

## resource_cost

O(format characters) plus literal storage; later padding cost is outside this fragment.

## tests

### Evidence and limitations

Parent source checks: builtins::io::tests::g_format, builtins::io::wave30_tests::cycling_unicode_and_empty. The fragment's actual statements lie in the recorded byte range; parent checks need not exercise every branch in this fragment. See ../model.md#tests for source-level control and limitations.

AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.
