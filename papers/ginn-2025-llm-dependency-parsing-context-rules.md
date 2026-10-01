# LLM Dependency Parsing with In-Context Rules

**Authors:** Michael Ginn, Alexis Palmer · **Year:** 2025 · **Venue:** Proceedings of the 1st Joint Workshop on Large Language Models and Structure Modeling (XLLM 2025)
**Link:** https://aclanthology.org/2025.xllm-1.17/ (ACL Anthology 2025.xllm-1.17; DOI 10.18653/v1/2025.xllm-1.17)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper tests whether explicit symbolic rules in the prompt help large language models do dependency parsing for low-resource languages, where there is too little annotated data to train a classic neural parser. It uses UD treebanks of eight typologically diverse languages (including Bambara, Cantonese, Erzya, K'iche' and Yoruba), with non-projective sentences removed. Four kinds of prompt additions are compared: only the list of allowed relation labels; rules that the LLM first writes itself from five annotated examples; word contexts drawn from training data (head plus relation pairs); and the official UD annotation guidelines. Each is tried with 0, 3 and 5 in-context examples chosen by chrF++ similarity. Symbolic knowledge clearly helps when there are no examples, but the advantage almost vanishes at five examples. Word contexts work best and are also cheapest, since they need no extra LLM call, while the human-written guidelines help least. On test data no LLM beat mBERT/XLM-R-based parsers or UDPipe, which makes this a useful negative result. It also suggests that compact rules can replace long example lists and shorten prompts.

## How Mova uses it
- `ai/en/seeds/errors/llm-annotator-failure-modes.md`: one of the sources for the catalogue of typical errors LLM annotators make in UD. The paper backs item 6: LLM-generated rules tend to be too narrow (for example treating a verb as AUX) and ignore word order, and UD guidelines in the prompt help only when there are no examples.
- Each failure mode in that catalogue is paired with a deterministic check in the English error-rule set. Mova's draft mode is designed around this picture: the Rust model annotates first and the large model only edits, rather than the large model parsing from scratch with guidelines in the prompt.

## Effectiveness in Mova
Background reading. It supports one point in a design note and does not drive any rule or module directly. Not measured separately.

---

👨‍🔬💥
