# Value of the "atomic" component

Ablation (01.10: copies of the system without this component, measurements, a report on whether it is useful, and the result is erased). The copy without the component was built with `GLOBAL_EXCLUDE=atomic-`; measurements — `tools/bench --quick`; the copy and intermediate results were erased.

| metric | with component | without | delta |
|---|---|---|---|
| svamp_% | 57.3 | 57.3 | +0.000 |
| svamp_answered_correct | 162 | 162 | +0.000 |
| svamp_answered_wrong | 109 | 109 | +0.000 |
| world_correct_agreed | 87 | 87 | +0.000 |
| world_correct_judge_no | 16 | 16 | +0.000 |
| ftqa_rougeL | 0.215 | 0.212 | +0.003 |
| tales_EM | 24.5 | 24.5 | +0.000 |
| tales_F1 | 0.302 | 0.302 | +0.000 |
| tales_pairs_EM | 24.3 | 24.3 | +0.000 |
| fables_judgments | 12 | 12 | +0.000 |
| fables_gaps | 0 | 0 | +0.000 |

**Conclusion:** metrics where the component helps — 1; where it hurts — 0. (For gaps and wrong answers, lower is better: read the delta according to the metric's meaning.)
