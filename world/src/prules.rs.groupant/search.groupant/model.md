## forward

For fixed supplied lines L, options O and rules-file snapshot F, initialize current reads and banned rejected pairs. Each iteration sets w=round(500*microF1); trial each site/action; collect nonzero U(new)-U(old) across eligible conjunctions of one to three atom IDs. Keep positive-sum/min-line/precision candidates. Rank by deletion-adjusted sum descending, length ascending, Key ascending. Try first 60 entries, skipping banned; apply rule to all lines and accept only positive actual gain, min fixed, precision and sign-tail gates. Accept updates current reads/output; reject bans pair. Stop at cap or no accepted candidate. Data loading and CLI effects are separate external adapters.

## invariant

Output length<=cap. Every accepted iteration has positive gain for that iteration's fixed U=1000 sum(tp)-w sum(sys); changing w prevents treating all gains as one fixed global objective. Stable ordered-map ties require fixed dependency outputs. Rules remain proposed until external review. No optimality or generalization guarantee.

## cost

I<=cap; S trial sites/actions, A atoms/site, C candidate keys, L lines, E apply+score cost. Per iteration: O(S*(A+A^2+A^3)*log C) plus trial scoring, O(C log C) ranking and <=60*L*E actual rule trials. sign_p now O(n) time/O(1) memory; old sum of binomial-product loops was O(n^2). Current reads, candidate map and accepted rules retain memory. Parser/files/lexical services remain opaque; bounded timing is not full induction throughput.

## composition

Source-bound parent supplies exact shared state and dependency assumptions. Independently projected fragments do not prove their wiring.

## epoch-memory

Initial scope. Preserve unknown lexical, data and effect semantics. Source inspection and executed bounded helpers remain distinct.

## checks

Nine native tests in wave9_models (five prules, four school_tree), repeated twice per debug/release profile, plus compiled production mutants; see ../model.md#evidence in the file parent. Binomial word oracle n<=12, gate finite grid, supplied-head graph universe 6^4, feature boundaries and reverse fibers. No real learning/data-loading or parser-to-Reading run. Prospective math recorded before implementation; large-n old-code failure precedes repair.

## domain

Fix source revision, external dependency outputs, configuration and sufficient resources. Observe one terminating call or explicitly bounded trace. Arithmetic domains exclude overflow except named refusal tests. Specialized relations are source-derived partial models unless a finite oracle is explicitly named. No unrestricted reverse solver is implemented.

## behavior-complement

For safe counts, absent acceptance includes nonpositive gain, too few fixed lines, failed precision/sign thresholds, banned pairs and candidates after rank60. Native gate oracle checks numerical exclusions; candidate-generation exclusions remain source inspected.


## reverse-reconstruction

For gate true, preimage is every gain/fixed/broken/options tuple in acceptance-region below. For learned output retain all candidate traces matching accepted updates, rejected candidates and capped omissions. Empty output cannot distinguish cap0 from lack of eligible sites or failed thresholds; no unique training set reconstruction.


## acceptance-region

Accepted iff gain>0, fixed>=min, fixed/(fixed+broken)>=prec, P(fixed,fixed+broken)<=sig. On safe finite counts increasing positive gain alone does not change outcome. Zero changed count yields NaN and refuses even min=prec=0. NaN precision/significance refuses. Oracle uses integer cross-products for thresholds 0,1/2,1 and independent coin words for significance 0,1/4,1. Large witness accepts(1,550,550,1,0.5,1) succeeds after repair.

## History

### 2026-10-30 · epoch-memory

Initial 2026-10-30 scope. Preserve unknown lexical, data and effect semantics. Source inspection and executed bounded helpers remain distinct.
