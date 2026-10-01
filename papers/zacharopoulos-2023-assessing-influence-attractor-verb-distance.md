# Assessing the influence of attractor-verb distance on grammatical agreement in humans and language models

**Authors:** Christos Zacharopoulos, Théo Desbordes, Mathias Sablé-Meyer · **Year:** 2023 · **Venue:** Proceedings of the 2023 Conference on Empirical Methods in Natural Language Processing (EMNLP)
**Link:** https://aclanthology.org/2023.emnlp-main.998/ (ACL Anthology 2023.emnlp-main.998; DOI 10.18653/v1/2023.emnlp-main.998)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The study asks how the distance between an attractor noun and the verb affects subject–verb number agreement in people and in neural language models. An attractor is an intervening noun whose number may differ from the subject's. The authors construct English sentences of fixed length in which the attractor is either right before the verb or further away, plus a baseline with no intervening noun. Humans judged grammaticality of sentences presented word by word, and the same stimuli were given to two transformer models. Both humans and models err more when the attractor is close to the verb, but in the hardest conditions models fall to near chance while humans mostly overcome the interference. The authors attribute the proximity effect to local transition probabilities between neighbouring words, while acknowledging that cue-based memory retrieval could also explain it. Superficially similar behaviour thus hides different processing of syntax.

## How Mova uses it
- `ai/en/seeds/errors/attraction-of-phrase.md`: rule `en.errors.attraction-of` in the English grammar expert system (`ai/en/src/expert.rs`) — a singular head noun with a plural *of*-phrase followed by a plural verb ("A pattern of arrests indicate"). The paper motivates why an attractor adjacent to the verb is the typical error site.
- `ai/en/seeds/errors/plural-subject-singular-verb.md`: rule `en.errors.plural-subject-vbz`, same background.
- The rules check agreement structurally through the subject relation in the dependency tree, so they are immune to the linear-proximity effect the paper documents.

## Effectiveness in Mova
Not measured separately. Background reading for two rules. Against the reference treebanks: attraction through *of* in EWT 2.18 — verbs 7 cases / 1 violation (exactly "pattern … indicate"), copula or auxiliary 13 / 5; GUM 16 / 3 and 27 / 8.

---

👨‍🔬💥
