# rule-derivation-rules

## descriptant

# rule-derivation-rules

## Forward

class retains lowercase closed-class lemmas. Induction counts each existing nearest POS target and gold-head hits. lists filters and orders by precision, applies and Rule order. Plain derivation uses first applicable non-cycling rule. apply_tr snapshots matching moves then applies in word order with current cycle checks.

## Invariant

Aligned vectors, valid heads, dir in {-1,1}, k in {1,2,3}; cond_sets needs a class atom. nth excludes its source. pending excludes a target whose head lies strictly inside the span except a head equal to the dependent. These are preconditions, not validated input checks.

## Cost

nth and cycle traversal O(n). For c atoms, cond_sets emits 1+(c-1)+choose(c-1,2) vectors of at most three atoms: 16 for ordinary six-atom words, 11 for five-atom roots. Induction and application conservatively O(n^2) per sentence.

## Composition

The syntax envelope includes load/run whose I/O semantics remain root dependency gaps. Classification is shared with packing, so lexical changes can change closed-class list lookup despite unchanged arcs.

## Epoch memory

Initial epoch: 90 assertions over twelve extracted pure source items with minimal enum dependencies. Closed-class erasure mutant rejected. No full learner, parser, GPU, corpus, filesystem-training or external state execution. Source-derived models and proposed oracles are distinct from executed bounded probes. See ../model.md#tests.

## catalog

# rule-derivation-rules typed models

## nonbehavior

Type: behavior_complement.

In admitted helper U, candidate inner absent behaviors: nth returning its source, returning a target with fewer than k matching POS words in that direction, cond_sets emitting empty or class-free sets. Loops establish the conditional model; literal directional probes and independent mask families provide finite evidence.

## antecedents

Type: reverse_reconstruction.

For nth returning j, all compatible POS sequences label j with u in direction d, with exactly k-1 u labels strictly between i,j; all other labels/lemmas remain free. None means fewer than k matches before the boundary. cond_sets preserves class and ordered subset members; duplicate atoms can prevent unique recovery. Probe checks directions and mask families.

## ranking

Type: decision_order.

lists admits applies>=min, hits>0 and not hidden; descending f64 precision precedes descending applies then ascending Rule. Stat::rel picks greatest count then smallest Rel order, or Dep. Ratios alone cannot recover list order. Proposed literal tie oracle is not executed.

## conditions

Type: finite_combinatorics.

For distinct ordered atoms c0..cm, cond_sets emits all subsets containing c0 with up to two tail atoms, preserving their input order. Count=1+m+m*(m-1)/2, not 2^m. Duplicates are positional and may repeat vectors. Probe checks prefix lengths 1..6 against independent bit-mask enumeration and literal counts.

## cycle-snapshot

Type: transition_order.

apply_tr computes all matching non-root moves from initial heads/labels, then mutates ascending word indexes and checks cycles on evolving heads. Matches are not recomputed. Same-head moves can overwrite relations. makes_cycle also returns true on an unrelated existing cycle after its traversal guard expires. Reverse retains overwritten fields and all prestates with compatible snapshotted moves. Cycle helpers executed; full apply_tr is not.

## History

### 2026-10-29 · document

Partial source-derived groupant, 2026-10-29. Full source inspected; this node specializes its explicit syntax envelope. Macro expansion and semantic AST are absent.

Subject: derive.rs SHA-256 2726211da9538f3f5c225292219dfda4d4fd6f15d6b68deaaaf53c834ba60842. Domain: finite admitted inputs with fixed dependency/schedule state. First type is a candidate inner behavior-complement bound in explicit input/output universe, second is an ambiguous compatible-preimage relation. Other models specialize observations or costs. See descriptant for assumptions and ../model.md#tests for evidence; structural acceptance is not semantic proof.
