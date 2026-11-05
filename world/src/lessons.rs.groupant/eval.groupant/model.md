# Lesson model

## forward

Tokenize ASCII identifiers and integer strings, then precedence-climb with eager operands. Comparisons/Booleans return 0 or 1. Division requires zero remainder. Checked i64 add/subtract/multiply/negation/division/remainder propagate None on overflow. Entire token stream must be consumed.

## behavior-complement

U is finite expressions/environments with sufficient stack, paired with Option<i64>. Inner complement excludes Some for unknown symbols, invalid suffix, inexact or zero division and overflow. Eager evaluation excludes Some(0) for 0 && (1/0).

## reverse-reconstruction

B(v) is every syntactically accepted expression and environment evaluating to v. For v=2, 1+1 and 4/2 are compatible; unused bindings remain arbitrary. B(None) mixes syntax, missing-name, inexactness and overflow failures.

## coupling

Failure partition retains reasons only in the model: runtime None merges them. Literal positive 9223372036854775808 cannot be parsed, so MIN is accessible through an environment or representable computation, not that positive token followed by unary minus.

## invariant

Successful observations obey the declared forward relation with the same supplied state and order. Failure is explicit; no hidden label or retained-state guarantee is introduced.

## cost

Tokenization O(B); map probes O(log V); parser visits O(T) tokens, with recursive stack O(T) worst-case. Arbitrary nesting can exhaust the stack; no universal totality claim.

## composition

Evaluation and substitution share numeric bindings; field labels choose template containers. Child relations must agree on intermediate state, not merely final counts. Partial syntax binds the source; macro contents are opaque.

## epoch-memory

Initial review predicted arithmetic overflow and finite-pool boundary failures. Old native code failed six named witnesses: overflow, empty templates, name exhaustion, full-width range, reversed range and seed overflow. Narrow repairs retain public signatures; finite observations do not establish arbitrary-input totality. See file ../model.md#evidence for hashes and repeated results.

## checks

Eight wave12_models tests plus two existing lesson tests. Independent 19 by 19 by 7 arithmetic cases, precedence/eager failure literals, partition/substitution fixtures and exactly 2000 rejected RNG draws. Runtime guard mutations must fail named assertions. Evidence is finite; no whole-world or live course claim.

## History

### 2026-10-30 · document

Heading `# Lesson model, 2026-10-30` stays in document as `# Lesson model`.
