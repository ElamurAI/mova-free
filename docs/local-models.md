# Local LLMs on the RTX 4000 (15.10.2026)

A fresh local model was wanted. Two were tried with llama.cpp built with CUDA (`$MOVA_DATA/llm`, sm_89; cmake from the parsers' venv, no sudo). Both are under Apache 2.0, 4-bit GGUF from unsloth:
- **Qwen3.8-27B** (dense, released August 2026), UD-IQ4_XS, 14.3 GB;
- **Gemma 4 26B-A4B** (MoE with 4B active), UD-IQ4_XS, 13.6 GB.

Task: the cycle's own requests from round 23 (8 sentences each: the draft tree, the WORLD and PRAG lines).

| | Qwen3.8-27B | Gemma 4 26B-A4B | Haiku 5.5 (Batches) |
|---|---|---|---|
| prompt reading | 756 tok/s | 2 285 tok/s | — |
| writing | 16 tok/s | 53 tok/s | — |
| one full request | 99 s | 55 s | ~2 s of a batch |
| full requests: sentences passing the gates | — | 53 of 120 | 119 of 120 |

The RTX 4000's memory (280 GB/s) makes a dense 27B slow, and both models patch far more than needed and break trees.

**Dispute mode** (`treeagree dispute-ask` / `dispute-apply`, design note: «deltas, less thinking, a couple of fixes»). Only the words whose head our parsers dispute are sent, as a choice: «17 helped: A 14:favorite acl | B 10:color acl | C 7:words acl». Round 23 has 5 156 such words in 1 526 of 2 200 sentences, sent as 191 prompts of ~1.1k tokens each, about 12× less input. Both local models answered by position, mostly «B», including words they were not asked about. Gemma took 816 s for the round, Qwen 13 s a prompt.

**Decision: stay with Haiku.** The models stay in `$MOVA_DATA/llm/models` for later tries. Dispute mode is kept for the teacher.

## Dispute mode on Haiku too (15.10)

The same 191 dispute prompts of round 23 went to Haiku 5.5 through Batches:
- with `effort low` and `max_tokens 2000`, it spent every token on thinking and wrote no answer (380 k output tokens, $0.114, 190 of 191 empty);
- with thinking disabled, it ignored the format and wrote explanations;
- with `max_tokens 12000`, it answered, but thought 2.5–4.5 k tokens per prompt, leaned to option B (Stanza/Trankit) like the local models, and saved little against the full mode.

**Decision: compressed questions do not suit LLMs — the full mode stays.** That is a cached prompt, deferred batches, a little context (WORLD), the sentence, and the MMM's tree and pragmatics. The Sonnet/Opus cascade stays for debugging.
