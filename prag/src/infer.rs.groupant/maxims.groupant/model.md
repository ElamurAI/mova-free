# Punctuation and event precedence

## forward

Filter PUNCT words from body. Question detection reads trimmed sentence suffix ?, ?! or !?; WH first form selects wh wording, AUX_INV first form selects yes/no wording without the auxiliary, otherwise declarative wording. Emit question before exclamation, where ! or !? suffix emits feeling. For each Conj VERB with distinct in-range nonzero head, require VERB head and and Cc child on the second verb, no stative lemma on either, no REORDER lemma anywhere and then only when attached to the second verb. Emit the head event before current event.

## behavior-complement

Universe: (W,S, finite maxim vector). Inner exclusions include reflexive event precedence, emission from root/out-of-range event heads, a manner pair with no and Cc on the second verb, and exclamation before question when both are emitted. Tests witness distinct valid pair versus invalid/reflexive heads and both mixed punctuation forms; old fixtures additionally witness stative and reorder blockers. This says nothing about actual chronological truth.

## reverse-reconstruction

For a rendered event pair retain every graph whose selected distinct verb forms, and attachment and blockers reproduce it. Identical surface forms can denote distinct coordinates; content does not recover positions. Quality output loses original punctuation tokens because punctuation comes from S, and PUNCT words are dropped from body. A question alone cannot distinguish all texts with the same terminal marker and body.

## event-order

Hold W fixed with source coordinates h and i. The emitted ordered pair is (h,i), using the dependency head as first event; the rule does not require h<i in text. Distinctness h!=i is now required. Shared event forms can still render equal strings, so a no-self *string* restriction would be wrong. Finite fixtures check h=0,i=2 for valid open/walk and reject actual identical coordinate. This is a linguistic heuristic, not a chronology proof.

## invariant

Question then exclamation then Conj token order. Manner head lookup is within bounds; its coordinate differs from the triggering event. All maxims are licensed irrespective of earlier text; the API has no history input. Word forms drive first-question classification while lemmas drive event gates.

## composition

The same words feed punctuation-body rendering and dependency gates but sentence punctuation is a separately supplied string. Caller agreement between string and words is a premise for linguistic interpretation, not enforced. The chain test deliberately supplies only suffixes to isolate this API observation.

## cost

Body/lemma construction O(n); up to n conjunction candidates each scan n conjunction/reorder tokens, O(n²) total with bounded strings. Distinct-head/bounds guards add constant work per candidate.

## epoch-memory

Old-source self-headed Conj emitted ran before ran. The repaired guard excludes self and dangling heads; valid legacy manner cases pass. Mixed punctuation remains legacy behavior and is tested without a tokenizer or annotator. No production punctuation or order heuristic was broadened.
