# gparse-data-batching

## descriptant

### Source-bound description

batch_prag allocates BT input/mask and BT² roles, adds root and gathers real rows; planner sorts and flushes before oversized additions.

## approximant

### Forward approximant

batch_prag allocates BT input/mask and BT² roles, adds root and gathers real rows; planner sorts and flushes before oversized additions.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

At budget zero each selected sentence remains as a singleton; root key=1 and mask=0.

#### Cost

O(BT²+word characters+S log S), O(BT²) arrays; u32 index casts unchecked.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

Root never enters rows or mask as real word; zero budget never drops a singleton.

## reverse_reconstruction

Given masks/rows, lengths are recoverable for valid canonical batches; forms are not because hashing/dropout collide.

## row_address

inp uses4*(k*T+i), roles use(k*T+d)*T+p; root i=0.

## drop_state

Only positive drop draws RNG for words; prag draws only for nonempty roles and positive prag_drop.

## plan_order

Sort by(length,index); flush before adding if nonempty and prospective padded token total exceeds budget.

## role_precedence

First nonzero filler role survives, predicate flag adds8 once; invalid manual role indices may panic.

## tests

### Checks and limits

Selected parent checks relevant to this fragment (not exhaustive): data::tests::features_line_up_with_words; data::cost_tests::cost_counts_real_batch_shapes_and_filters_skipped_sentences; data::wave30::projection_roles_and_zero_budget

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.

### 2026-11-04 · descriptant

#### Epoch 2026-11-04

batch_full replaces the body of batch_prag (now a wrapper with ngram=0). It precomputes n-grams per word when ngram>0, sizes g by the longest list, and pushes arc_conf, rel_conf and alts per row in row order. No new RNG draws.
