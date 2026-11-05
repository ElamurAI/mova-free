# Trigger content and prior-token license

## forward

Flatten earlier sentences in supplied order, retain the last 50 tokens. Visit current words in order. again/still/also/too use a nonzero, in-range, nonself direct head; too with ADJ/ADV head is excess, not additive. stop/quit/continue/keep verbs select the first Xcomp/Ccomp/Obj child. Factive verbs select the first Ccomp or Xcomp with own Nsubj/Mark child; nested ancestry projects its clause in token order without direct Mark children. A chosen lowercase predicate is licensed iff it occurs as a lowercase earlier lemma in the retained window.

## behavior-complement

Universe: (W,E, finite presupposition vector). Inner exclusions include direct-head again output from head 0, head beyond n or head i+1, licensing when the sole matching prior lemma precedes 50 trailing tokens, and clause content containing an unrelated token whose parent leaves the vector. These are source-derived absence relations, witnessed by invalid-head, history and clause assertions. They do not deny real-world truth or resolve arbitrary malformed graphs.

## reverse-reconstruction

For observed content and license bits, retain all trigger graphs generating that content and all histories with matching predicate membership exactly when licensed. Missing actors, irrelevant history and excluded markers are ambiguous. Prefixes older than the last 50 tokens can vary freely without changing output. Different selected complement coordinates can render identical content. This fiber is not a unique antecedent or causal explanation.

## history-window

Let H=flatten(E), r=min(50,len(H)), V={lower(H[j].lemma): len(H)-r <= j < len(H)}. License(key)=key in V. Sentence boundaries disappear, multiplicity disappears, order within the window disappears for membership. Earlier flattening still consumes B tokens. The independent oracle places RUN followed by 0,49,50,51 irrelevant tokens in another sentence and expects true,true,false,false.

## license-fiber

Fix W and its chosen predicate key and observe only licensed=b. Histories for b=true must have at least one key in their final 50 tokens; histories for false have none there, regardless of arbitrarily many matches earlier. Case variants map to one key. This relation specializes the output fiber to lexical support; it does not reconstruct the number, age beyond cutoff or truth of supporting events. The wrong cutoff production control falsifies the boundary.

## invariant

Current words are not appended internally. Head 0, out-of-range and self direct heads yield no predicate for adverbial triggers. First matching complement wins. Content and licensing share the chosen key, not every possible complement. The output remains ordered by triggering token.

## composition

The ancestry child receives selected complement index j and original words, filters each k and excludes Mark tokens only when their head is j+1. License checks use the chosen complement's lemma rather than the rendered whole clause. Independent content and license fibers must agree on that predicate identity.

## cost

History flattening O(B) time and storage; bounded membership scans cost O(50) per trigger. Up to n factive triggers each filter n tokens with n ancestor steps, giving O(n³) worst case, plus rendering. Bounds checks preserve the asymptotic cost.

## epoch-memory

Old-source fixtures exposed a self-headed again inference and dangling ancestor panic. Direct-head lookup now checks bounds and self identity; subtree climbing treats out-of-range links as no ancestry evidence. Cycles that do not reach the chosen top terminate under the existing hop bound. This is local resilience, not whole-tree rejection. Native tests use supplied Word rows only.
