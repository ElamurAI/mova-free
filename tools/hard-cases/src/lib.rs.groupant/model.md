# mova-hard-cases-lib

## Descriptant

Crate root of mova-hard-cases: module map of the loop from reading failures to learning and shared helpers (SHA-256, atomic/exclusive writes, CoNLL-U block helpers, normalized-text hash, lane document ids hard:<16 hex>). Source: `tools/hard-cases/src/lib.rs`.

## Forward approximant

doc_id(source, document) = 'hard:' + first16(sha256(source NUL document)); norm_sha(t) = sha256(join(' ', whitespace-split t)); blocks(conllu) = trimmed nonempty blocks each ending in one LF; with_id replaces the sent_id line.

## Invariant

Pure helpers except write_atomic (temp + fsync + rename) and write_new (create_new, refuses to overwrite).

## Cost

Linear in input bytes.

## Composition

Parent of the hard-cases modules; group hard-cases.

## behavior_complement

Holds no policy and calls no process. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

doc_id is a one-way hash: the document id is recovered from the wave's selection, not from the id.

## Tests

cargo test --manifest-path tools/hard-cases/Cargo.toml --offline (9 unit tests, two identical runs) Finite fixtures and live samples; not a proof.

## History

### 2026-11-04 · Tests

cargo test --manifest-path tools/hard-cases/Cargo.toml --offline (2026-11-04: 9 unit tests, two identical runs) Finite fixtures and live samples; not a proof.
