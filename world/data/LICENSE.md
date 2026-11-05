---
module: world-data
licence: CC-BY-SA-4.0
requires: [attribution, share-alike, licence-notice, indicate-changes]
allows: [commercial-use, modification, redistribution]
sources: [Mova (CC-BY-SA-4.0), Project Gutenberg books (public domain)]
notes: [the school (courses and school dictionaries) is a separate archive with its own LICENSE.md files; meanings written with Anthropic Claude models as teacher]
---

# Licence and sources of `world/data`

Data, not code. The school's courses, lessons and school dictionaries are not in this folder: they are the release
archive `school-0.5` (`mova fetch 0.5 school`, installed into `$MOVA_DATA/school`), each folder with its own
`LICENSE.md`. This file covers this folder.

**Licence:** Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0,
<https://creativecommons.org/licenses/by-sa/4.0/>). Attribution: "Mova, the Elamur team". Mova's code is under the
Elamur Cosmic License ([../../LICENSE.md](../../LICENSE.md)); that licence does not cover this folder.

| files | what | sources |
|---|---|---|
| `dictionary-improved.tsv`, `dictionary-sources/**` | the pupils' explanatory dictionary | written for Mova by Anthropic's Claude models as teacher (their terms restrict using outputs to develop competing models); examples from the lessons and from public-domain tales of Project Gutenberg |
| `questions.tsv` | questions on Aesop's fables | text: *Aesop's Fables*, Project Gutenberg #21, public domain; questions Mova's own |
| `cmds-*.txt`, `v3-merchant.tsv` | world command scripts over public-domain tales and that book's sentences | Aesop's fables, a Japanese tale, Andrew Lang's *Arabian Nights* (Project Gutenberg #128), public domain; scripts Mova's own |
| `kinds.txt`, `know-qa*.tsv`, `skeleton-ops.tsv`, `stepgame-contrast*.tsv`, `brain-base.md`, `self.md` | body plans, check questions, operation words, StepGame contrast phrasings (after Zhengxiang Shi, Qiang Zhang and Aldo Lipani. 2022. *StepGame: A New Benchmark for Robust Multi-Hop Spatial Reasoning in Texts.* AAAI 2022; no StepGame text), the base self-description | Mova's own |
| `rules-induced.tsv` | tree-repair rules induced by Mova (`world induce`) | Mova's own |
