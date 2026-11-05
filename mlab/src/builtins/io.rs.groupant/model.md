# mlab-builtins-io

## descriptant

### mlab-builtins-io

Output registration and C-like format parser/cycler; scalar numeric formatting and num2str/mat2str conversions.

## approximant

### Forward relation

sprintf_impl parses format, flattens real numeric matrices column-major to atoms, then cycles segments until atoms consumed. Strings under numeric conversions expand to Unicode scalar codes. fprintf formatted-string branch writes the completed string through Interp.out and returns UTF-8 byte count only for nargout>0. A scalar nonchar first argument is ignored as fid only when followed by a char format. The numeric-first fallback formats all arguments with %g\n and returns no values even for nargout>0.

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

Formatting builds a String before calling out; string precision counts Unicode scalars, output count uses UTF-8 bytes. %g preserves negative zero via fmt_num, shortest canonicalizes it.

#### Failure ordering

Argument extraction and atom conversion precede output. Unknown format specs remain literal, excessive width/precision can exhaust resources rather than return a structured format error.

#### Cost

O(format bytes + expanded atoms + repeated segment/flag scans + emitted bytes); repeated conversion/flag scans can require O(format length * expanded atoms) work even with small output. String-to-code expansion and requested width/precision can dominate allocations. No resource bound or useful percent surrogate for exact text.

## composition

### Composition

sprintf_impl parses format, flattens real numeric matrices column-major to atoms, then cycles segments until atoms consumed. Strings under numeric conversions expand to Unicode scalar codes. fprintf formatted-string branch writes the completed string through Interp.out and returns UTF-8 byte count only for nargout>0. A scalar nonchar first argument is ignored as fid only when followed by a char format. The numeric-first fallback formats all arguments with %g\n and returns no values even for nargout>0.

Numeric atoms follow m.re storage, not printed row order; char matrices are one string atom. num2str default aligns columns while mat2str assembles row syntax.

#### format-parser

Scan Unicode chars; %% adds literal percent, flags/width/precision/length are collected before supported conversion membership; unsupported prefixes stay literal.

Boundary: Literal spans are unescaped when flushed; supported conversions separate literal and spec segments.

#### atom-cycle

Compute segments and atom queue; zero specs returns literals, otherwise cycle. Initially empty queue pads each spec once; exhausted noninitial queue returns immediately at the next spec.

Boundary: Every productive numeric/string conversion removes an atom or expands a finite string to code atoms; iteration ends when queue empties.

#### Interaction limit

Local fragment relations compose only with valid Value storage, caller error propagation and delegated providers. They do not establish whole-interpreter or host-filesystem guarantees.

## models

# Typed models — mlab-builtins-io

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

Unsupported atom types exclude any successful formatted output even for a literal-only format because atoms is called first. Empty argument queue excludes numeric substitution; empty specs can still contribute width padding.

## reverse_reconstruction

Rendered strings admit many inputs: integer/float atoms with same formatting, varying matrix shapes with identical real storage, ignored imaginary components and fid values in the qualifying scalar-fid/char-format branch. No inverse recovers those discarded components.

## state_invariant

Formatting builds a String before calling out; string precision counts Unicode scalars, output count uses UTF-8 bytes. %g preserves negative zero via fmt_num, shortest canonicalizes it.

## failure_precedence

Argument extraction and atom conversion precede output. Unknown format specs remain literal, excessive width/precision can exhaust resources rather than return a structured format error.

## representation_transport

Numeric atoms follow m.re storage, not printed row order; char matrices are one string atom. num2str default aligns columns while mat2str assembles row syntax.

## resource_cost

O(format bytes + expanded atoms + repeated segment/flag scans + emitted bytes); repeated conversion/flag scans can require O(format length * expanded atoms) work even with small output. String-to-code expansion and requested width/precision can dominate allocations. No resource bound or useful percent surrogate for exact text.

## tests

### Evidence and limitations

Scope: Formatting builds a String before calling out; string precision counts Unicode scalars, output count uses UTF-8 bytes. %g preserves negative zero via fmt_num, shortest canonicalizes it.


AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.

#### Cross-review refinement — 2026-10-31

Repeated flag/segment scans multiply conversion cost by expanded atom count; output length alone does not bound formatting work. The scalar-fid/char-format branch alone ignores a file identifier, and the numeric-first fallback returns no byte-count value. These source-derived qualifications change models only; retained runtime evidence and source hashes remain applicable.

### 2026-10-31 · tests

Selected named outcomes: builtins::io::tests::g_format, builtins::io::wave30_tests::cycling_unicode_and_empty. Each ran twice debug and twice release with sorted identical nonempty outcomes and numeric exit 0. Source-specific compiled control exercised "builtins::io::wave30_tests::cycling_unicode_and_empty": compile exit 0, assertion-red exit 101, exact-restored rerun exit 0. External receipts: octopus-wave30-io-2026-10-31 (native-commands.json, positive-receipts.json, controls.json).
