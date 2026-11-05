# Factive clause ancestor predicate

## forward

For fixed words W of length n and selected in-range top j, under(k) starts x=k. For at most n iterations: return true when x=j; otherwise follow a parent h only if 1<=h<=n, setting x=h-1; return false on missing/root/out-of-range parent. Exhaustion returns false. For functional in-range parent graphs, any first reach of top occurs within n distinct vertices.

## behavior-complement

Universe: (W,k,j,b), with k,j supplied valid token coordinates and b Boolean. An inner bound of absent triples includes b=true when the path terminates at root or a dangling parent before top, and b=true for a cycle disjoint from top. Bounds are source-derived; malformed parent values are tolerated as absent evidence. The literal seven-token clause fixture includes a dangling branch and a two-node cycle and excludes both from content.

## reverse-reconstruction

Given true, compatible graphs include k=j as well as every graph with a parent chain from k reaching j under the bound. Given false, compatible graphs include roots, dangling edges and disjoint cycles. The Boolean erases hop count and all off-path parents. Neither answer recovers a unique dependency tree. A graph with top itself in a cycle may still give true: the relation is reachability to top, not global well-formedness.

## broken-parent

Hold a path prefix from k fixed. Outcomes partition into reaches-top, stops-at-root, stops-at-out-of-range, or exhausts-hop-bound. Root and broken-parent termination map to the same false observation, but have different repair hypotheses. A dangling unrelated branch should not abort the whole presupposition call; the old fixture did panic and the repaired fixture renders exactly she ran.

## invariant

Every indexed current coordinate is checked through get; only a bounded nonzero parent is followed. At most n steps, so non-reaching cycles terminate. Parent coordinates are one-based; selected top and current x are zero-based.

## composition

This predicate is private inside presuppositions and executes on its actual clause-filter path. Successful under alone does not emit a word: the caller also removes direct Mark children and preserves token order. Its true/false projection cannot determine complete rendered clause text without those caller constraints.

## cost

O(n) per membership call and O(1) extra state. n membership calls per selected clause give O(n²); do not add this to the parent's cubic bound again.

## epoch-memory

The compiled old source panicked while traversing a supplied usize::MAX parent on an unrelated token. Checked parent lookup now terminates false; valid ancestry and non-reaching cycle behavior are preserved. The source-bound fragment selects only the closure, while its executed witness calls the enclosing public API.
