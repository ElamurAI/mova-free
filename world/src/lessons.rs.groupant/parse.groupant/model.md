# Lesson model

## forward

Parse supplied line-oriented text by first colon; repeated scalar fields replace, list fields append. Required nonzero lesson number, nonempty answer, at least one exercise and test. Range syntax and expressions are deferred until draw. No implicit train/test partition is inferred from text.

## behavior-complement

U is finite supplied texts paired with Result observations. Inner absent transitions include successful unknown fields or missing required fields. Whitespace normalization and default fields are part of the relation; no claim of complete grammar validation.

## reverse-reconstruction

B(l) contains all texts whose recognized fields build l, including arbitrary blank/comment lines and overwritten scalar prefixes. File string is an input retained in l. Lost comments make this noninjective.

## coupling

Partition membership is the field label; identical exercise and exam strings are accepted. Separation of containers does not establish unseen semantic content.

## invariant

Successful observations obey the declared forward relation with the same supplied state and order. Failure is explicit; no hidden label or retained-state guarantee is introduced.

## cost

One pass over bytes plus owned field copies: O(B) time/storage, ignoring allocator failures.

## composition

Evaluation and substitution share numeric bindings; field labels choose template containers. Child relations must agree on intermediate state, not merely final counts. Partial syntax binds the source; macro contents are opaque.

## epoch-memory

Initial review predicted arithmetic overflow and finite-pool boundary failures. Old native code failed six named witnesses: overflow, empty templates, name exhaustion, full-width range, reversed range and seed overflow. Narrow repairs retain public signatures; finite observations do not establish arbitrary-input totality. See file ../model.md#evidence for hashes and repeated results.

## checks

Eight wave12_models tests plus two existing lesson tests. Independent 19 by 19 by 7 arithmetic cases, precedence/eager failure literals, partition/substitution fixtures and exactly 2000 rejected RNG draws. Runtime guard mutations must fail named assertions. Evidence is finite; no whole-world or live course claim.

## History

### 2026-10-30 · document

Heading `# Lesson model, 2026-10-30` stays in document as `# Lesson model`.
