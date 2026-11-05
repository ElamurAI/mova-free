# gparse-cli-score

## descriptant

### Source-bound description

score zips inputs and predictions, counts head, joint head/relation and independent tags then returns percentages.

## approximant

### Forward approximant

score zips inputs and predictions, counts head, joint head/relation and independent tags then returns percentages.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

n=0 returns four zeros; wrong head always gives LAS zero for that token.

#### Cost

O(zipped token pairs), O(1) counters.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

Correct relation with wrong head cannot increment LAS.

## reverse_reconstruction

A tuple of aggregate scores allows permutations and unmatched suffixes; n counts only paired tokens.

## zip_boundary

Unmatched sentence/token suffixes never increment n.

## empty_metric

Denominator max(n,1) gives all-zero metrics at n=0.

## joint_label

LAS increments only inside correct-head branch.

## tag_metric

XPOS and UPOS comparisons independent of head/relation.

## tests

### Checks and limits

Selected parent checks relevant to this fragment (not exhaustive): wave30::score_zip_and_plan_failure_precedence; train/eval/annotate external effects remain unexecuted

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.

### 2026-11-04 · descriptant

#### Epoch 2026-11-04

breakdown counts (words, right heads, right heads and relations) per length bucket, per seen/unseen word and per GUM genre, ordered length buckets first, then seen/unseen, then genres by name; eval prints them as rates under the unchanged overall line.
