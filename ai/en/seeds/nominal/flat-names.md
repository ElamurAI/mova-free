# Personal names and "type + name" — flat, head first

**Gist.** A multiword personal name (*Hillary Rodham Clinton*) has no grammatical head inside, so UD builds it "flat": all words depend on the first. Likewise names where the type comes first (*Lake Mead, Mount Everest, Hotel California*), numeric identifiers (*page 394, Route 66, World War II*) and foreign names with function words (*Ludwig van Beethoven*). Poutsma notes that compound names made of "insignificant" parts behave as one whole (e.g. without an article).
**Conditions and exceptions.** A title before a name (*Mr., Dr., President*) is not `flat` but `nmod:desc`; likewise *Jr., Inc.* after a name. Names with transparent structure (*Natural Resources Conservation Service, the King of Sweden*) are ordinary syntax.
**Examples.** *Barack Obama* → `flat(Barack, Obama)`; *Mr. Smith* → `nmod:desc(Smith, Mr.)`; *Route 66* → `flat(Route, 66)`.
**In UD.** `flat` is always to the right of the head (EWT 2.18: 2 501 of 2 501). *Mr./Mrs./Ms.* — PROPN with `nmod:desc` to the left of the name.
**Sources.** https://universaldependencies.org/u/dep/flat.html, https://universaldependencies.org/en/dep/flat.html, `nmod-desc.md`; Poutsma GLME vol. 3, Ch. XXXI §29–30 (pp. 597–598 = book pp. 577–578).

```rule
rule: en.nominal.flat-head-first
what: flat — the head is first, the other words of the name to the right
match: h[]; f[rel~flat, head=h]
require: f[after=h]
severity: error
source: UD u flat; UD en nmod:desc (Personal Names, Numbered Entities); UD validator (flat left to right)
```

The rule is `en.errors.title-desc` in `errors/titles-and-company-suffixes.md`.
