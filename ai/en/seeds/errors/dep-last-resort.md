# dep — only as a last resort

**Gist.** `dep` means "there is a relation, but it could not be determined": an odd construction, a converter or parser failure. In the manual EWT gold it has almost disappeared — 5 per 254k words; versions 2.8–2.11 removed some `dep`s. An annotator that often uses `dep` "gives up" instead of analysing. LLMs do this on repetitions, ellipsis and abbreviations; in one pipeline the prompt even explicitly allowed `dep` for ellipsis.

**Conditions and exceptions.** GUM uses `dep` much more widely (184 in 2.18): for spoken fragments, and in old versions also for identifier numbers (*Page 3*). For the EWT style every `dep` is a reason to check the parse.

**Examples.**
- *Then, as if to show that he could, …* → dep(show, if) — an example from the guidelines.
- For ellipsis there is `orphan` and promotion of the remnant (see `gapping-orphan.md`), not `dep`.

**Check against gold.** EWT 2.18 — 5; GUM 2.18 — 184.

**Sources.** UD `_en/dep/dep.md`; `2023.udw-1.7` (v2.11: «removal of some uses of the dep relation»); `2025.findings-emnlp.863` (Kellert et al.: the prompt marks ellipsis via dep/orphan; LLMs are inconsistent on repetitions and abbreviations).

```rule
rule: en.errors.dep-last-resort
what: the dep relation — in the EWT style only for unparsable places; usually it is a refusal to analyse
match: d[rel=dep]
require: not d[rel=dep]
severity: warn
source: UD _en/dep/dep.md; 2023.udw-1.7; 2025.findings-emnlp.863
```
