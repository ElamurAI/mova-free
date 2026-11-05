# world-school-tree

## model

## forward

For supplied aligned words/lowercase forms, predicates are roots or Conj/Advcl/Parataxis words with verb POS or own Nsubj/Cop/Expl. clauses assigns nonpunctuation tokens to first ancestor predicate within len steps, drops subordinate cc, sorts groups by first member. subject inspects at most four predicates via Conj parents. counted follows exact Nummod to allowed noun/proper, optionally redirects measure to first of-marked child. Full read_tree is an ordered normalization/parser/question/sentence/clause fold followed by relation closure, numeric settlement, thing carry and price conversion; only a partial source model covers that fold.

## invariant

Aligned arrays and valid queried positions required. Clause member indices ascend, never duplicate across groups, and omit punctuation. Cycles without predicates drop their tokens after bounded traversal. head returns None for zero/out-of-range. subject returns first discovered subject/expletive within four inspected predicates. Exact relation subtype matters for passive and counted. No general answer-correctness or arithmetic-safety invariant.

## cost

n words: kids O(n), is_pred O(n). clauses O(n^2) predicate discovery plus n walks of <=n steps with linear predicate membership, conservative O(n^3); groups O(n) storage. subject<=4 child scans; counted may nest case_of in kids, O(n^2). Full fold repeats scans and lexical/parser calls. Relation closure clones relations and scans holdings; every successful step appends a previously missing actor, bounded by finite endpoint set under safe arithmetic. No measured full-reader cost.

## composition

school::read_tree calls this module. TSent::new invokes frames::parse_words; on/resolve_pronouns/school_norm/number_word/MEASURES and MOVA_TREE_NOUNS affect behavior. Native tests inject Word arrays then call actual clauses/subject/passive/counted/case_of. No parser normalization or Reading fold is executed. Shared last subject/event, per-unit state and numeric provenance prevent inferring whole-fold correctness from independent features.

## epoch-memory

Initial epoch. Subject depth cutoff and dropped orphan/cyclic clauses are documented current semantics. Counted redirects measures without checking redirected child POS. Affine relation arithmetic uses truncating i64 division and unchecked products; reverse calculation is not a full inverse. No production code changed in this file.

## checks

Nine native tests in wave9_models (five prules, four school_tree), repeated twice per debug/release profile, plus compiled production mutants; see model.md#evidence in the file parent. Binomial word oracle n<=12, gate finite grid, supplied-head graph universe 6^4, feature boundaries and reverse fibers. No real learning/data-loading or parser-to-Reading run. Prospective math recorded before implementation; large-n old-code failure precedes repair.

## domain

Fix source revision, external dependency outputs, configuration and sufficient resources. Observe one terminating call or explicitly bounded trace. Arithmetic domains exclude overflow except named refusal tests. Specialized relations are source-derived partial models unless a finite oracle is explicitly named. No unrestricted reverse solver is implemented.

## behavior-complement

Universe: aligned supplied Word arrays with valid query indices and finite feature results. Inner complement excludes punctuation clause members, duplicate membership, Some head on invalid head number, passive=true absent exact passive child, and counted Some on non-Nummod query. Finite graph and feature tests falsify violations in supplied domains; arbitrary parsed language remains outside.

## reverse-reconstruction

Retain all supplied trees with observed feature projection y. Same heads/rels/POS and different ordinary forms often have same clauses/subject; native sam/holds and lee/gives witness ambiguity. Outputs cannot reconstruct lemmas, punctuation, dropped cycles or source text. Graph comparison is complete only over fixed four labels and 6^4 head arrays; no unrestricted inverse search.

## ancestor-partition

Follow head edges to first predicate; without predicate before invalid edge or repeated vertex, omit. Omit nonpredicate Cc beneath predicate. Group and sort by minimal member. Independent visited-set oracle agrees on all 1296 four-token head graphs, including head5 out-of-range, self cycles and punctuation roots; fixed relation/POS labels restrict the conclusion.

## subject-horizon

Check own Nsubj/Expl first, then only Conj parent, up to four checks total. Depth3 inherits root subject; depth4 does not. Children tie by token order. Native depths0..5 verify cutoff. Cycles are bounded by iteration count, not guaranteed subject discovery.

## passive-projection

Passive(p) iff a direct child has exact AuxPass or NsubjPass. Base Aux/Nsubj alone does not suffice. Four-relation fixture checks distinctions. Many child words/tags/subtrees share existence bit, so passive observation cannot reconstruct voice semantics.

## measure-target

Require exact Nummod, valid NOUN/PROPN head and absence from excluded forms. Ordinary target returns head. A measure returns first child whose first Case child has form of; redirected child's POS is unrestricted. Oracle crosses three head POS, two query relations, three head words plus invalid heads. This is attachment selection, not numerical/unit correctness.

## question-priority

A parsed nonempty sentence ending '?' takes question branch before event clauses. ask_whose prioritizes named subject, first pre-than name, comparison place, eligible noun subject and surface fallbacks. Conditional fields can retain earlier question values when later evidence is absent; no fresh-question reset guarantee. Source inspection only.

## relation-fiber

Fix safe integers p,q,d with q!=0: a=trunc(b*p/q)+d. An observed a may have multiple b. Reverse code computes trunc((a-d)*q/p) when p!=0, choosing one candidate rather than full converse. For p=1,q=2,d=0,a=1, b=2 and3 are compatible; code picks2. Domain excludes overflow and zero denominators. This is inspected arithmetic, not a tested complete Reading inverse.

## state-priority

First-matching early-continue branches make clause order observable: comparisons, parts/rest/double, unknown amount, measure, groups/prices/rates/packaging/distribution/places, then holding/event. Relation closure appends only missing actor with known partner and uses first actor holding; unseeded components stay unresolved. No commuting independent-clause model.

## lexical-effects

Fix on registry, name lists, normalization/number_word/parser snapshots and environment. Final thing carry uses latest actor thing then global last; price conversion replaces matching events with pay and resets other event fields. Supplied feature tests avoid these lexical/hot dependencies. No retained-learning or full-school result inferred.

## evidence

### Bounded evidence

The binomial reference enumerates all bit strings n<=12 for each threshold, separate from the production recurrence. Gate oracle has 3969 finite grid combinations plus two NaN refusals. School graph oracle enumerates 1296 head arrays for fixed four POS/relation labels with an independent visited-set traversal. Subject depths 0..5 and 18 counted-target combinations plus 3 invalid heads exercise boundaries. Utility zero fiber has 12 members; shuffle sizes 0..16 reconstruct gold and preserve row fields. These are finite observations.

Compiled controls:

Production mutant `gain >= 0` is rejected by `prules::wave9_models::final_gate_finite_oracle_and_refusals`; production subject horizon 5 is rejected by `school::school_tree::wave9_models::subject_depth_boundary_and_passive_features`. Both compile and fail named assertions. Exact source bytes restored; all 9 positives rechecked. Old production sign_p(550,1100) failed the new central-tail assertion with NaN before repair. This is a witnessed numerical bug, distinct from intentionally wrong compiled controls.

Independent review then exposed n=19, k=10 rounding to 0.5000000000000002. A native equality assertion failed before the exact odd-central symmetry branch was added; the existing test also requires accepts(1,10,9,1,0.5,0.5). Complement-pair symmetry proves that exact central probability is 0.5. Other near-threshold rounding remains outside this narrow guarantee. The repair uses mode-normalized binomial masses: outward ratios <=1 avoid overflowing coefficients times underflowing 2^-n; ratio of sums cancels common scale. Small-word oracle agrees within 1e-12; large n=1099,1100,4096 symmetry/range checks and a permissive gate witness pass. Floating rounding and far-tail underflow remain, and threshold decisions extremely close to numerical rounding can differ. There is no universal relative-error or exact statistical guarantee.

Release microtiming (extracted exact old/current helpers, 200 calls per row; n=100,500; two repetitions):

```text
n=100 repetition=0 method=old calls=200 ns=13562586 checksum=107.958923738718170
n=100 repetition=0 method=mode calls=200 ns=98295 checksum=107.958923738718056
n=100 repetition=1 method=old calls=200 ns=6029109 checksum=107.958923738718170
n=100 repetition=1 method=mode calls=200 ns=49966 checksum=107.958923738718056
n=500 repetition=0 method=old calls=200 ns=75240173 checksum=103.566464555334647
n=500 repetition=0 method=mode calls=200 ns=131433 checksum=103.566464555334647
n=500 repetition=1 method=old calls=200 ns=74515642 checksum=103.566464555334647
n=500 repetition=1 method=mode calls=200 ns=131924 checksum=103.566464555334647
```

Timing verifies outputs agree within 1e-12 only at these points; machine load and warm-up vary. O(n^2) to O(n) is a source-derived helper operation-count change; these timings do not measure complete induction or school throughput.

Structural validation used an external registry preserving all prior nodes and adding the two leaves under a temporary root. No central registry or ancestor manifests edited. Final groupants report is valid with zero diagnostics; repeated report bytes match. changed lists exactly the two owned sources, with no missing entries. Both source syntax projections were regenerated twice with identical bytes. These checks establish bindings/declarations, not semantic proofs.

## History

### 2026-10-30 · epoch-memory

Initial 2026-10-30 epoch. Subject depth cutoff and dropped orphan/cyclic clauses are documented current semantics. Counted redirects measures without checking redirected child POS. Affine relation arithmetic uses truncating i64 division and unchecked products; reverse calculation is not a full inverse. No production code changed in this file.

### 2026-10-30 · evidence

Heading `### Bounded evidence, 2026-10-30` stays in evidence as `### Bounded evidence`.

Raw logs: $MOVA_DATA/runs/octopus-wave9-learning-2026-10-30/. No real training, parser, teacher, held-out, full school or persistence run. Nine new native tests pass twice in each profile; no ignored tests in these focused runs. Root/ancestor registry integration is separate.

- `debug1.log`: 9 passed, normalized identity 84854453923e99800cbb832ae5448608970b50486f1471b0acd3053eabffb8da. Raw SHA-256 `f8f485efd632bfbf2c7ba91ed14f789e6a8f36263de64d9d09c18b6e4ab825ae`.

- `debug2.log`: 9 passed, normalized identity 84854453923e99800cbb832ae5448608970b50486f1471b0acd3053eabffb8da. Raw SHA-256 `e7b3e37597dc3aaa57eae737245237e7f48e44aec5d97023a89946e1c4dec9ad`.

- `release1.log`: 9 passed, normalized identity 84854453923e99800cbb832ae5448608970b50486f1471b0acd3053eabffb8da. Raw SHA-256 `1a09dc359a67f3349e41c0979a4b487c73dd6da1f928d403095df1bcac087e90`.

- `release2.log`: 9 passed, normalized identity 84854453923e99800cbb832ae5448608970b50486f1471b0acd3053eabffb8da. Raw SHA-256 `e41181c78e51f4761b518f4ada1a79fa9020e67e6b00681a78890849f920bcaf`.

- `restored.log`: 9 passed, normalized identity 84854453923e99800cbb832ae5448608970b50486f1471b0acd3053eabffb8da. Raw SHA-256 `6b7f43b807bcaa48e72ac525b8f5d0f89f62026493fa75e444f20eda07eb3eba`.

- `prules-mutant.log` SHA-256 `2ddd21b587809230205970926af07ab54edddb6c9b428bb5dfd3258a7b98b2ee`.

- `school-mutant.log` SHA-256 `31c18943ea3faf8b76d5fc3f7a97ebc36db22fbdaa76d9af432ea52db50dbc52`.

- `old-large-failure.log` SHA-256 `f548ebbfdc60b57aec1d2a4e3eae920c0f7a0143943bdb29372756d1ba7bc754`.

- `central-rounding-failure.log` SHA-256 `d5c3dfec5585989f603279f2aeb8afb98ca2df4ec6c75974b3fdcb205bc6091f`.

- `prospective.md` SHA-256 `b2dfc6836ba1c0ddf19e9a01eaff89cbb0122ab1d41c9b233f4415e8f901b697`.

- `timing.rs` SHA-256 `8f9abe0b748e799878ac313fad3b56b94c71d96633e52ccba20df40df4104fbc`.

- `timing.log` SHA-256 `14c82805d8b83c5f8b8b3f5ca8c0f40ceef8b49bc74836493703fc7f4d4f2d65`.

Final owned source SHA-256: `ca5ada75804e1d2eb3a8a5e0b05331b00c4198682962fcbbb221a1020927711b`.

- `registry-full.json` SHA-256 `95192018473c72a0c217edca58246e17a31efec401dd43b7567da9220949d0e6`.

- `groupants-final1.json` SHA-256 `00e3b888371d7db1de822a1b601662ad4f0f5e68cb978654f211c2d41e68eebf`.

- `changed-final.json` SHA-256 `7903f35571c3175a145710278de47c1bcefc3c0453f9831f49eb81cd9818907b`.

- `prules-final1.json` SHA-256 `e9ed00038dfef9b3e0de0b2ec5743a1939b693f7331c80b3482477c19b8dfb69`.

- `school-final1.json` SHA-256 `95be78cf6d3abd88a6d33e1bd4b67dd54b8c1174abf51f8cde54b39ea3af53ad`.
