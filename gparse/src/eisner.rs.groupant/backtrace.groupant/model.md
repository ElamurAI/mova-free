# gparse-eisner-backtrace

## descriptant

### Source-bound description

Stack expands complete to incomplete spans and writes one parent for each dependent then strips root slot.

## approximant

### Forward approximant

Stack expands complete to incomplete spans and writes one parent for each dependent then strips root slot.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

s==t stops expansion; stack uses backpointer arrays from DP.

#### Cost

O(n) backtrace events/output after charts, O(n) explicit stack.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

Backtrace never reads sc directly; its behavior is determined by chosen root and stored backpointers.

## reverse_reconstruction

Heads alone cannot reconstruct scores or unused split choices.

## complete_split

Complete-left pushes complete(s,r) and incomplete(r,t); complete-right pushes incomplete(s,r) and complete(r,t).

## edge_write

Incomplete-left writes heads[s]=t, right writes heads[t]=s.

## termination_domain

With valid chart splits produced by finite intermediate candidates, span sizes decrease until s==t. Nonfinite recurrences are outside this guarantee.

## root_projection

heads[0] omitted from returned heads[1..]; heads[root]=0.

## tests

### Checks and limits

Selected parent checks relevant to this fragment (not exhaustive): eisner::tests::best_tree_with_one_root; eisner::wave30::empty_singleton_and_first_root_tie; decoder_accepts_no_words

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.
