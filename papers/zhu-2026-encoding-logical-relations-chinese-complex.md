# Encoding Logical Relations of Chinese Complex Sentences within the Universal Dependencies Framework

**Authors:** Hongpu Zhu, Hongzhi Xu · **Year:** 2026 · **Venue:** Proceedings of the Fifteenth Language Resources and Evaluation Conference (LREC 2026)
**Link:** https://aclanthology.org/2026.lrec-1.910/ (ACL Anthology 2026.lrec-1.910; DOI 10.63317/2uasimdgfqin)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
Clauses inside Chinese complex sentences stand in logical relations such as cause, concession or conjunction, which are often left unmarked by connectives and can nest hierarchically. Standard Universal Dependencies flattens all of this into generic inter-clause labels like advcl, ccomp or parataxis. The authors replace those generic labels with 13 finer logical-relation types while leaving every other dependency untouched, so the resulting trees stay structurally compatible with UD and clause hierarchy plus relation type become a single parsing problem. They annotate about 1,769 newswire sentences for training and re-annotate the GSD-simp test split as an evaluation set. A BERT-based biaffine parser and a fine-tuned Qwen-3 model are compared with several general-purpose LLMs (GPT-4o, GPT-5, Claude 4, DeepSeek V3.2). The fine-tuned Qwen-3-8B scores best with UAS/LAS 0.840/0.757. The study shows that inter-clause logic fits into the familiar UD format, but implicit and deeply nested relations remain hard for automatic parsers, and accuracy degrades as sentences contain more clauses.

## How Mova uses it
- `ai/en/seeds/errors/llm-annotator-failure-modes.md` — a seed (a short declarative rule note) listing typical failure modes of LLM-based UD annotators, with a check for each. This paper is one of the sources for the failure mode "sentence complexity: accuracy falls as the number of clauses grows".
- The observation feeds the general design of the annotation pipeline: the deterministic Rust parser annotates first and a large model only edits the draft, and evaluation always uses an independent gold standard rather than corrected model output.
- The 13-relation scheme itself is not implemented; Mova's English pipeline keeps plain UD inter-clause labels.

## Effectiveness in Mova
Background reading: the paper supports one item of a checklist of LLM annotator errors. Not measured separately.

---

👨‍🔬💥
