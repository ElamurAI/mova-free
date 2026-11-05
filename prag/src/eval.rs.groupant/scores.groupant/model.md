# prag-eval-scores

## forward
Require equal lengths and all classes in [0,C). Accuracy=sum_i[g_i=p_i]/max(N,1). Let gp_c and pp_c be marginal counts and tp_c diagonal counts. Macro-F1 averages 2tp_c/(gp_c+pp_c) over classes with nonzero denominator. Empty inputs yield (0,0), including C=0.

## invariant
Both metrics lie in [0,1]. Joint permutation and swapping gold/pred preserve results. Unused classes do not dilute macro-F1. Invalid lengths reject before class validation.

## cost
O(N*C) time from repeated per-class scans, O(C) f1 storage; new precondition checks O(N). No histogram optimization or performance claim.

## behavior-complement
Accepting mismatched lengths by zip truncation, counting invalid classes only in accuracy, diluting by absent classes, or returning NaN on empty input is excluded. Domain and observation are those of forward; this is a source-derived partial exclusion model, not a general proof.

## reverse-reconstruction
The metrics retain neither confusion matrix nor ordering. For N=2,C=2 both ([0,0],[0,0]) and ([1,1],[1,1]) map to (1,1); a common permutation preserves any score. Reconstruction must preserve N,C if supplied as side information. The relation is all compatible antecedents, with no uniqueness claim.

## specialization
Independent 2x2 confusion-matrix enumeration covers 341 paired binary sequences of lengths 0..4, using C=3 to check absent-class exclusion; five invalid-domain cases reject.

## composition
Children share source identity and dictionary/sample or record indices. Parent order and domains constrain joins; leaf observations alone do not establish external orchestration. Overlapping scopes must not double-count costs.

## checks
Seven wave14_models tests: four evaluation and three data; supplied inputs only. Debug twice and release twice; compiled production mutation per source must fail a named assertion. Syntax generated twice identically. Receipts summarize final evidence.

## epoch-memory

First coverage of these previously unmodeled files. Prospective review predicted no valid-domain metric change and O(N) extra validation. Old source accepted mismatched vectors; the rejection witness failed (six other tests passed). Narrow assertions preserve tuple API and reject invalid domains. Data production behavior remains unchanged. Tiny two-row fitting exercises three learners without any training job.

## limitations
Partial syntax projection, opaque macros, no semantic AST or universal proof. No live parser, DB subprocess, external corpus, teacher, GPU or training job. External IO, report formatting, selection corner cases, malformed Set/Train and empty tale windows remain outside executed scope. No performance claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first coverage of these previously unmodeled files. Prospective review predicted no valid-domain metric change and O(N) extra validation. Old source accepted mismatched vectors; the rejection witness failed (six other tests passed). Narrow assertions preserve tuple API and reject invalid domains. Data production behavior remains unchanged. Tiny two-row fitting exercises three learners without any training job.
