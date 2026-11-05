# Sentence assembly and context snapshot

## forward

read_with snapshots up to eight positive-salience entities, builds units and repairs, appends frames at base b, offsets links/asked and joins consecutive roots if permitted. next-events removes links involving non-event or conditional frames; configured rules apply last.

## behavior-complement

Universe: fixed default configuration and supplied event-only sentence fixtures. Exclude missing offset next edge between two distinct unquoted roots with no earlier next, changing before snapshot to post-read entities, or adding output for an empty parse.

## reverse-reconstruction

before loses entities after the top eight and all zero-salience entries; text/output admits callback implementations returning identical supplied words. Sentence order can be visible through next but exact original typography is unrecoverable.

## invariant

Mutating ctx during reading must not retroactively alter before. Local links are offset once. Active rules can subsequently change output; fixed rule state is a premise.

## cost

Sum sentence extraction costs plus O(frames+links) offset/remap, context sorting O(E log E), and unbounded callback/rule work.

## snapshot-loss

Two contexts with identical first eight salient entity records and different lower-ranked entities produce equal before snapshots. That equality does not imply equal future pronoun resolution.

## composition

Tests execute tiny in-memory parse closures, no live parser or environment mutation. Resource defaults may be read by production helpers; effectful callbacks and nonempty active rules are not validated.

## epoch-memory

Wave15. Prospective repair: reject unresolved right heads in identity with a constant six-item guard, preserving coordination membership and its nontransitivity. Old production failed unresolved_identity_is_symmetric_even_in_coordination; repaired code passes. Compiled guard-removal control fails the same named assertion and exact bytes are restored. Eight focused tests pass twice debug and twice release. No speed, full semantic coverage or universal proof claim.

## limitations

Partial syntax is not expanded semantic AST. No live parser, teacher, whole corpus, GPU or environment mutation. Acyclic input trees and fixed configuration/resource premises matter; malformed recursive ancestry, counter overflow, complete TAM/lexicon/quotation/repair semantics and all active-rule effects remain open. No executable approximant/error budget is claimed.

## History

### 2026-10-30 · epoch-memory

Wave15, 2026-10-30. Prospective repair: reject unresolved right heads in identity with a constant six-item guard, preserving coordination membership and its nontransitivity. Old production failed unresolved_identity_is_symmetric_even_in_coordination; repaired code passes. Compiled guard-removal control fails the same named assertion and exact bytes are restored. Eight focused tests pass twice debug and twice release. No speed, full semantic coverage or universal proof claim.

### Undated · document

Subject: prag/src/frames.rs SHA-256 bfbc2f7905d878080a7e7dd30ae7f1bfe8f07313c3a4cf8f0254be29e72a4b88. Partial source model, finite native evidence. Fixed configuration and admitted finite resource domains; source-derived inner exclusions and ambiguous preimages, no inverse solver.
