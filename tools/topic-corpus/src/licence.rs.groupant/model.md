# mova-topic-corpus-licence

## Descriptant

Licence classes (public domain, CC BY, SA, NC, ND, restricted, unknown), document origins (library, Sea Spider, consultant draft, our notes) and the public-admission rule. Source: `tools/topic-corpus/src/licence.rs`.

## Forward approximant

classify(statement) by CC terms / PD / restricted words; public_admissible = origin Library and not local-only and class in {By, PublicDomain}.

## Invariant

Sea Spider pages, consultant drafts and notes are never public-admissible, whatever licence they state (operator rule).

## Cost

O(length of the statement).

## Composition

Parent: `topic-corpus`.

## behavior_complement

No SA, NC, ND, restricted or unknown text is admitted for public use; no web origin at all. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The class is a many-to-one image of the statement; the statement is not recoverable from the class. No unique inverse is claimed.

## History

### 2026-11-04 · Invariant

Sea Spider pages, consultant drafts and notes are never public-admissible, whatever licence they state (operator rule 2026-11-04).
