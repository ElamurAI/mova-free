# Titles (Mr., Dr.) and Inc. — nmod:desc

**Gist.** A title before a name (*Mr., Dr., Professor, actor*) and a corporate suffix (*Inc., Corp., Ltd., LLC*) are optional "descriptors": they can be removed and the name stays grammatical. Since UD 2.15 they are `nmod:desc` of the name's core. Previously titles went into `flat` together with the name or into `compound`. Schneider and Zeldes showed that `flat` is wrong here: the first name and surname then do not form a separate constituent. Against `compound` there is its own argument: the title agrees in number (*Presidents Obama and Biden*), while a compound dependent is almost never plural.

**Conditions and exceptions.**
- *Co. 1691* ("company No. 1691") and *Peters and Co.* (conjunct) are not descriptors. So *Co.* is not in the rule.
- A position with an article is `appos`: *the president, Joe Biden*.
- An ordinal in a name (*Elizabeth II*) is `flat`.

**Examples.**
- *Mr. Magoo* → nmod:desc(Magoo, Mr.).
- *Apple Inc.* → nmod:desc(Apple, Inc.).
- *JFK Jr.* → nmod:desc(JFK, Jr.).

**Check against gold.** EWT 2.18: titles before PROPN — 128, violations 0; company suffixes — 48/0. GUM — 84/1 and 3/0.

**Sources.** UD `_en/dep/nmod-desc.md` (Personal Names, Business Names); `2021.udw-1.14` (Schneider, Zeldes: 373 such constructions in GUM; company suffixes — ~2.5 per 10k tokens in EWT); `2023.udw-1.7` (Marvel Consultants, Inc.); english-banks §1.4 (2.15: `nmod:desc`).

```rule
rule: en.errors.title-desc
what: a title right before a name (Mr., Dr.) — nmod:desc of that name, not flat and not compound
match: t[form=mr.|mrs.|ms.|dr.|mr|mrs|ms|dr, !feats.Typo]; n[upos=PROPN, next=t]
require: t[rel=nmod:desc, head=n]
severity: error
source: UD _en/dep/nmod-desc.md (Personal Names); 2021.udw-1.14; UD en nmod:desc (Personal Names)
```

```rule
rule: en.errors.company-suffix-desc
what: a corporate suffix after a name (Inc., Corp., Ltd.) — nmod:desc
match: n[upos=PROPN]; t[form=inc.|inc|corp.|corp|ltd.|ltd|llc|plc, head=n, after=n]
require: t[rel=nmod:desc]
severity: warn
source: UD _en/dep/nmod-desc.md (Business Names); 2021.udw-1.14; 2023.udw-1.7
```
