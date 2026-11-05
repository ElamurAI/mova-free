# math-qworld-learned

## descriptant

### math-qworld-learned

script_labels visits numeric script occurrences and binds each to the first unused equal exact nonconstant quantity. WorldModel predicts argmax over ['=','+','-','r']; learned cells join the highest-scoring prior pair only if score>0.

## approximant

### Scoped forward relation

script_labels visits numeric script occurrences and binds each to the first unused equal exact nonconstant quantity. WorldModel predicts argmax over ['=','+','-','r']; learned cells join the highest-scoring prior pair only if score>0.

This source-derived relation is partial; no executable percentage surrogate or replacement is supplied.

## invariant

### Scoped domain and invariants

Labels must align with qs; floating weights/scores must permit partial_cmp. Training uses10 passes and does not guarantee separation of conflicting labels.

#### Cost

Training scans item quantities and all i<j pairs for10 epochs; learned write compares each item with all earlier ones. Feature string/hash and exact cell work add to O(N²) pair enumeration.

## models

## behavior_complement

No source quantity occurrence can be labelled twice by script_labels. A pair with score0 cannot join a prior cell. At all-zero operation weights, a non-rate prediction is excluded because max_by keeps the last tied r.

## reverse_reconstruction

Two script lines assigning the same number to different slots can match duplicate-valued quantities in order; swapping otherwise equivalent lines changes occurrence labels. Many scripts differing in ignored lines produce the same labels. Distinct feature-weight maps can share all selected op/pair scores.

## zero_weight_boundary

Default WorldModel chooses r for every operation; write_learned then emits commands but leaves state empty. Pair score0 creates a new cluster instead of joining.

## greedy_cluster_history

For item i, choose the last maximal positive prior-pair score and reuse that prior item's assigned slot; cluster names come from the first item that created a slot, not the newest item's description.

## tests

### Scoped evidence

Greedy script duplicate binding and default learned rate/no-cell dispatch; no training.

Named actual source fixtures: qworld::wave28::{exact_world_overwrites_and_labels_duplicates}.

Parent model.md#tests records commands and paired profiles. No additional coverage inferred from parent fixtures outside this byte range; source-scoped numeric/parse properties remain partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced duplicate catalog sentences with concrete exclusions, compatible-antecedent examples and two distinct scoped behavioral views. Source and prior execution evidence are unchanged; this revision ran no tests.
