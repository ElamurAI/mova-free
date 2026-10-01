# Speech Acts: An Essay in the Philosophy of Language

**Authors:** John R. Searle · **Year:** 1969 · **Venue:** Cambridge University Press (book)
**Link:** unknown
**License of the paper:** unknown (commercially published book, not openly licensed)

## Summary
The book develops a systematic theory of speech acts, building on Austin's work, with the claim that speaking a language is a rule-governed form of behaviour. It distinguishes the act of uttering words, the propositional act (referring and predicating) and the illocutionary act (promising, asserting, requesting, thanking, warning…). The core of the book analyses the conditions under which an illocutionary act succeeds, such as preparatory, sincerity and essential conditions, using promising as the main worked example. It also introduces the idea of illocutionary force indicating devices, linguistic signals such as mood, performative verbs or particular words that mark which act is being performed. The framework became a foundation of pragmatics and of later computational work on dialogue acts.

## How Mova uses it
- `ai/prag/data/cues.tsv` — the cue list's markers of illocutionary force (thanks, apologies, greetings, promises, warnings, suggestions, offers, yes/no) are compiled with this book (and Searle 1975 on indirect speech acts) as the theoretical basis; the list itself is Mova's own.
- `ai/prag/src/feats.rs` — the cues become the `cue` feature slot, alongside syntactic slots from the UD tree (mood, inversion, wh-word, modal, subject person…).
- `ai/prag/src/model.rs` — three classifiers (averaged perceptron, IGTree, k-NN) predict sentence-level fields including the speech act (`act`) and indirectness (`indirect`).

## Effectiveness in Mova
The cue list is not ablated, so the contribution of this source is not measured separately. The fields it informs, from the `prag` README, five-fold cross-validation on 2,986 sentences, accuracy / macro-F1 %: `act` — baseline 28.9 / 3.9, perceptron 70.3 / 38.0, IGTree 64.1 / 23.9, k-NN 68.1 / 26.0; `indirect` — baseline 92.7 / 48.1, perceptron 92.3 / 59.9.

---

👨‍🔬💥
