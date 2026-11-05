# mova-hard-cases-policy

## Descriptant

What a case may become: licence class -> training track (shipped, internal, measure; CC BY-SA internal by default), document -> split (train/dev/measure by SHA-256 of source and document), admitted judge kinds, markup filter. Source: `tools/hard-cases/src/policy.rs`.

## Forward approximant

class(licence) by tokens: nc/nd/research/nonexclusive -> restricted; cc0; public domain; ours; cc+by(+sa) -> cc-by(-sa); else unknown. track = tracks[class] or measure. split = (u32(first 8 hex of sha256(source NUL document)) mod 100) against train% and train%+dev%. markup(t) = wiki table/template/link/tag tokens or >=3 pipes.

## Invariant

Restricted and unknown licences never map to a training track unless configured explicitly; a document's split is a pure function of its id; validate refuses unknown track names and dev% = 0.

## Cost

O(len(licence)) per case.

## Composition

Used by wave (selection), judge (sample) and main (list).

## behavior_complement

Does not read licence texts or verify a source's licence statement; licence strings are as the sources state them. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Track and split do not determine the licence or document: many licences/documents share one class/split.

## Tests

policy::tests: licence classes and tracks with red controls (NC, ND, research, arXiv non-exclusive, empty, all rights reserved -> measure; invalid track name refused); split proportions on 2000 documents and stability; kinds; markup. Finite fixtures and live samples; not a proof.

## History

### 2026-11-04 · Tests

policy::tests (2026-11-04): licence classes and tracks with red controls (NC, ND, research, arXiv non-exclusive, empty, all rights reserved -> measure; invalid track name refused); split proportions on 2000 documents and stability; kinds; markup. Finite fixtures and live samples; not a proof.
