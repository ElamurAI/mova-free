# Step-by-step Instructions and a Simple Tabular Output Format Improve the Dependency Parsing Accuracy of LLMs

**Authors:** Hiroshi Matsuda, Chunpeng Ma, Masayuki Asahara · **Year:** 2025 · **Venue:** Proceedings of the 18th International Conference on Parsing Technologies (IWPT, SyntaxFest 2025)
**Link:** https://aclanthology.org/2025.iwpt-1.2/ (ACL Anthology 2025.iwpt-1.2)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper asks how to get structurally valid, accurate Universal Dependencies parses out of large language models. Instead of bracketed trees, the model emits a reduced CoNLL-U-like table with only the ID, FORM, UPOS, HEAD and DEPREL columns, which makes non-projective trees easy to express and malformed output easy to repair. The instruction splits the task into ordered steps in a chain-of-thought style: part-of-speech tags first, then heads, then relation labels. Models are fine-tuned with LoRA and evaluated on 17 UD r2.15 treebanks, comparing several proprietary and open models. Splitting the task into steps gives a clear gain over a single-step prompt, and a mid-sized open model with a small number of trainable parameters beats UDPipe 2.0 and Hexatagger in LAS on all 17 treebanks. Structural errors such as cycles or multiple roots are almost absent, and a single multilingual model performs about as well as monolingual ones and transfers reasonably to unseen languages.

## How Mova uses it
- `ai/en/seeds/errors/llm-annotator-failure-modes.md` — cited as evidence in the catalogue of typical LLM annotation failures: formal errors (cycles are rare) and what helps (the UPOS → head → relation step order and tabular output).
- The same seed motivates which checks Mova's rule gates run on LLM-produced UD trees: heads and relations are checked first, because LLMs are better at tags than at trees.
- The general lesson (decompose the parse into ordered steps, constrain the output format) is background for the annotation workflow in `ai/annot`, where the deterministic Rust parser produces a draft and the LLM only corrects it.

## Effectiveness in Mova
Not measured separately. The paper serves as background evidence for one knowledge seed and for the design of the annotation gates; no ablation of the step order or the tabular format has been run in Mova.

---

👨‍🔬💥
