# Checks

batch::tests::batches_choose_cited_open_papers_and_close_on_the_deadline (2026-11-04). batch-plan against the 186,435-row library snapshot into a scratch run root: the configured priority papers first, then the first cited open papers by id. Finite fixtures, not a proof.

batch::tests::an_asked_out_batch_closes_early_and_retries_under_a_new_writer (2026-11-04): a usage-limit stop keeps the batch open (negative control); all asked out -> `exhausted` with both papers in `retry`; retried with their learners under a different writer; not retried again under the same writer; new papers follow. Finite fixtures, not a proof.

2026-11-04: tests of the batched/pod/stall/review/grammar/filter/selection changes listed in the crate README «Checks»; negative controls (grammar gate always passing, pod claim ignored, stall off, extraction check off) each turned their tests red, restored.
