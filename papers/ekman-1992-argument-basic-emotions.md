# An Argument for Basic Emotions

**Authors:** Paul Ekman · **Year:** 1992 · **Venue:** Cognition and Emotion 6(3–4):169–200
**Link:** https://doi.org/10.1080/02699939208411068 (DOI 10.1080/02699939208411068)
**License of the paper:** unknown (publisher copyright; no open legal copy known)

## Summary
Ekman argues that emotions are best understood as a small number of discrete "basic" families rather than as points in a few continuous dimensions. He proposes a set of characteristics that separate basic emotions from each other and from other affective states. These include distinctive universal signals (notably facial expressions), distinctive physiology, automatic appraisal, typical antecedent events, quick onset, brief duration and unbidden occurrence. The article reviews evidence on these characteristics and responds to critics of the basic-emotions view. The emotions most often associated with this line of work are happiness, sadness, fear, anger, surprise and disgust. The paper is a theoretical position piece in psychology, not a computational work.

## How Mova uses it
- `ai/global/seeds/shortcuts/feelings.md`: the seed that maps event types to feelings (loss, death, separation → sadness; gain, rescue, reunion → joy; threat, darkness, the unknown → fear; deceit, theft, insult → anger; the unexpected → surprise) cites Ekman's basic emotions as the backbone of its category list. Mova adds gratitude and shame as social feelings, which do not come from Ekman.
- The seed is compiled into the global (first) layer, and the story reader (`world read-feel`, `ai/world/src/main.rs`) uses it to infer a character's feeling from events when no explicit feeling word is present.

## Effectiveness in Mova
Ekman only supplies the emotion inventory; the inference rules are Mova's own. The feelings component built on this seed was measured as a whole: on FairytaleQA "feel" questions, 0.037 → 0.141, and together with the event graph the reader scores 0.212 overall. The paper's own share of this gain was not measured separately.

---

👨‍🔬💥
