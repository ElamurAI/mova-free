# world-prules

## model

## forward

For fixed supplied lines L, options O and rules-file snapshot F, initialize current reads and banned rejected pairs. Each iteration sets w=round(500*microF1); trial each site/action; collect nonzero U(new)-U(old) across eligible conjunctions of one to three atom IDs. Keep positive-sum/min-line/precision candidates. Rank by deletion-adjusted sum descending, length ascending, Key ascending. Try first 60 entries, skipping banned; apply rule to all lines and accept only positive actual gain, min fixed, precision and sign-tail gates. Accept updates current reads/output; reject bans pair. Stop at cap or no accepted candidate. Data loading and CLI effects are separate external adapters.

## invariant

Output length<=cap. Every accepted iteration has positive gain for that iteration's fixed U=1000 sum(tp)-w sum(sys); changing w prevents treating all gains as one fixed global objective. Stable ordered-map ties require fixed dependency outputs. Rules remain proposed until external review. No optimality or generalization guarantee.

## cost

I<=cap; S trial sites/actions, A atoms/site, C candidate keys, L lines, E apply+score cost. Per iteration: O(S*(A+A^2+A^3)*log C) plus trial scoring, O(C log C) ranking and <=60*L*E actual rule trials. sign_p now O(n) time/O(1) memory; old sum of binomial-product loops was O(n^2). Current reads, candidate map and accepted rules retain memory. Parser/files/lexical services remain opaque; bounded timing is not full induction throughput.

## composition

frames::main routes rules to prules::main. load joins dev keys with teacher_gold, excludes held keys from teacher additions, and reads lesson prefixes. score bridges Read->serialized scored frames->parse_frame->alignment; judged_only filters unaligned frames and remaps links. Candidate ranking joins atoms/actions/apply_at with score, final acceptance joins apply_rule with accepts. Native tests exercise production pure helpers only; real data/caller wiring was source-inspected.

## epoch-memory

Initial epoch. Math first predicted utility and shuffle fibers and exact small sign-tail. Root review prompted old-code witness: sign_p(550,1100)=NaN, wrongly failing a permissive acceptance gate. Replaced overflow-prone coefficients times 2^-n with normalized relative masses around the mode; extracted existing final gate into accepts. read_lines override reset occurs only on success, learn reads rule file even at cap=0, and shuffle can be identity for split sizes 1,2,14; these inspected limits remain unchanged.

## checks

Nine native tests in wave9_models (five prules, four school_tree), repeated twice per debug/release profile, plus compiled production mutants; see model.md#evidence in the file parent. Binomial word oracle n<=12, gate finite grid, supplied-head graph universe 6^4, feature boundaries and reverse fibers. No real learning/data-loading or parser-to-Reading run. Prospective math recorded before implementation; large-n old-code failure precedes repair.

## domain

Fix source revision, external dependency outputs, configuration and sufficient resources. Observe one terminating call or explicitly bounded trace. Arithmetic domains exclude overflow except named refusal tests. Specialized relations are source-derived partial models unless a finite oracle is explicitly named. No unrestricted reverse solver is implemented.

## behavior-complement

Universe: fixed lines/options/file snapshot and bounded candidate traces. Inner absent relation excludes acceptance with nonpositive gain, insufficient fixed count, failed precision/sign threshold, or more than cap rules under safe arithmetic. Site gain alone cannot imply acceptance. final_gate_finite_oracle_and_refusals checks 3969 grid rows and NaN threshold refusals; this evidence does not complement an invented complete learner emulator.

## reverse-reconstruction

For learned output y retain all (L,O,F) and site/current-read traces that produce y. Empty output includes cap=0, no useful site delta, thresholds and top-60 failure; output does not distinguish those causes. A concrete finite inverse enumerates all 0<=tp<=sys<=4, tp<=gold<=4 vectors at w=500 and groups by U. utility_complete_small_fibers verifies inclusion, a 12-member zero fiber and unreachable U=250; no text/gold uniqueness.

## utility-projection

Fix w; U=1000T-wM ignores gold directly, and equal role-count redistribution preserves U. At w=500, U/500=2T-M; (0,0,0), (1,2,4), (2,4,2) lie in the same zero fiber. Duplicating a role doubles U. Gold still affects scoring and w outside this fixed-weight model. Overflowing integer casts/products excluded.

## sign-tail

P(k,n)=sum(i=k..n) C(n,i)/2^n. Let m=floor(n/2), a_m=1, a_(i-1)=a_i*i/(n-i+1), a_(i+1)=a_i*(n-i)/(i+1). Then a_i=C(n,i)/C(n,m), so P=sum(i>=k)a_i/sum(all)a_i. Outward ratios<=1, ideal mass<=1 and total>=1. k=0 returns1; k>n returns0. For odd n and k=n/2+1, complement-pair symmetry returns exactly0.5 before accumulation; n19,k10 native gate regression protects the threshold. Independent binary-word enumeration n<=12 and symmetry/finite checks n=1099,1100,4096 support bounded behavior. Rounding, far-tail underflow and threshold changes close to rounding error remain; no uniform relative error bound.

## acceptance-region

Accepted iff gain>0, fixed>=min, fixed/(fixed+broken)>=prec, P(fixed,fixed+broken)<=sig. On safe finite counts increasing positive gain alone does not change outcome. Zero changed count yields NaN and refuses even min=prec=0. NaN precision/significance refuses. Oracle uses integer cross-products for thresholds 0,1/2,1 and independent coin words for significance 0,1/4,1. Large witness accepts(1,550,550,1,0.5,1) succeeds after repair.

## candidate-budget

take(60) precedes banned skip, so banned entries consume ranked positions. Rank61 cannot be reached that iteration even with 60 banned predecessors. Bound is from source inspection; no complete candidate search or native learn run.

## atom-boundary

For A distinct eligible site atoms, candidates emitted are A plus choose(A,2) if atoms>=2 plus choose(A,3) if atoms>=3. atoms=0 still emits singleton rules; atoms>3 adds nothing. Lemma atoms filtered with nolex. Intern IDs follow first encounter, not lexical sorting. Rule features/actions and apply feasibility remain prag::rules assumptions.

## shuffle-inverse

For each dev/teacher split n>0, output[k].gold=input[(k+n/2+7) mod n]. Inverse is negative rotation; other row fields and other splits are identical. Empty split does no indexing. Original positions enable unique inverse permutation, duplicate values erase observable identity. Native sizes 0..16 check rotation, reconstruction and fields. No independence or useful statistical disruption guarantee.

## gold-filter

load selects dev with gold and teacher outside held/dev key sets, requiring teacher text trim-match. teacher_gold skips short/bad-index rows, rejects a whole line with unreadable shown/corrected frame, appends valid MISSING. judged_only aligns and removes unjudged system frames. This key-based filter was inspected, not exercised on external artifacts; schema/provenance beyond keys is assumed.

## effect-boundary

read_lines installs empty global override then performs fallible reads before clearing. Error may leave override active; no rollback or concurrency isolation claim. main learn writes queue then rules, without paired atomicity. control writes its report. These effects were not executed.

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

Initial 2026-10-30 epoch. Math first predicted utility and shuffle fibers and exact small sign-tail. Root review prompted old-code witness: sign_p(550,1100)=NaN, wrongly failing a permissive acceptance gate. Replaced overflow-prone coefficients times 2^-n with normalized relative masses around the mode; extracted existing final gate into accepts. read_lines override reset occurs only on success, learn reads rule file even at cap=0, and shuffle can be identity for split sizes 1,2,14; these inspected limits remain unchanged.

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

Final owned source SHA-256: `d3fe2fa0cbece21e34ed7b15054e8c2142db62e9e4ad771c8f8a40c5fa2c889d`.

- `registry-full.json` SHA-256 `95192018473c72a0c217edca58246e17a31efec401dd43b7567da9220949d0e6`.

- `groupants-final1.json` SHA-256 `00e3b888371d7db1de822a1b601662ad4f0f5e68cb978654f211c2d41e68eebf`.

- `changed-final.json` SHA-256 `7903f35571c3175a145710278de47c1bcefc3c0453f9831f49eb81cd9818907b`.

- `prules-final1.json` SHA-256 `e9ed00038dfef9b3e0de0b2ec5743a1939b693f7331c80b3482477c19b8dfb69`.

- `school-final1.json` SHA-256 `95be78cf6d3abd88a6d33e1bd4b67dd54b8c1174abf51f8cde54b39ea3af53ad`.
