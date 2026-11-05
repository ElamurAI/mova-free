# en-annotate-persistence

## descriptant

### annotate: persistence

Pack must succeed before filesystem changes; parent creation then .part write then rename. to_bytes/from_bytes delegate store; train_files accumulates tagged sentences before train. Failure is stage-specific; temporary file cleanup and crash durability are absent.

## approximant

### persistence forward scope

Pack must succeed before filesystem changes; parent creation then .part write then rename. to_bytes/from_bytes delegate store; train_files accumulates tagged sentences before train. Failure is stage-specific; temporary file cleanup and crash durability are absent.

Composition rejoins the parent at its explicit state/representation boundaries; guarantees apply only to this byte range, not unrelated parent branches.

## invariant

### State and cost

Annotator holds Model and Ud; Word holds form, lemma, UPOS, tag, FEATS, head and relation.

For aligned valid inputs, completion cannot change token forms or token count. It cannot guarantee repair equivalence with annotate: complete and kbest omit repair. A failed pack prevents save from creating directories; a failed rename may leave a .part file.

For n words, add tagging/parsing costs to O(n^2) child scans and bounded agreement walks; kbest repeats annotation for each retained tree. Save/load cost serialized byte size plus allocation/I/O. No length or allocation cap.

## models

## behavior_complement

For aligned valid inputs, completion cannot change token forms or token count. It cannot guarantee repair equivalence with annotate: complete and kbest omit repair. A failed pack prevents save from creating directories; a failed rename may leave a .part file.

## reverse_reconstruction

Given output Word sequence y and fixed model/ud, compatible antecedents include exactly form sequences equal to y.form plus tag/head/relation arrays whose completion yields y. Morphology/EModE/table backoff are many-to-one; outputs cannot recover training sentences. Successfully loaded bytes identify compatible serialized model states, not a unique byte encoding because map order may differ.

## layer_order

Repair uses pre-repair UPOS, then child flags and UPOS are recomputed; words_of performs EModE fixes last.

## array_domain

complete indexes tags/heads/rels/feature vectors by form index; unequal lengths can panic. Root uses head0 and words use 1-based heads.

## save_effects

pack first; mkdir parent if nonempty; write path.with_extension(part); rename. No fsync, concurrent writer isolation or rollback cleanup.

## empty_rows

Empty forms complete to empty; rows defaults missing space_after entries to true and emits ten columns with DEPS underscore.

## tests

### Evidence

Compiled source mutants cover construct rounded-floor boundary, parse root legality and store checksum gate. Other sources have no compiled source mutation claim. Preflight empty-model nonempty tagging failed in ptag; synthetic fixtures adjusted without production changes. Existing corpus/seeds tests not selected.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31. Exact byte range/hash in manifest. Partial lexical AST is shared with parent; no macro expansion, semantic proof or executable percent surrogate. Parent tests exercise selected reachable paths, not every statement in this fragment.

### 2026-10-31 · tests

Actual source test: annotate::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.

### 2026-11-04 · descriptant

2026-11-04 epoch (en-morph): source rebound to 4a126331234c; byte range 1226..4212 → 1226..4212. The scope text is unchanged (moved only).
