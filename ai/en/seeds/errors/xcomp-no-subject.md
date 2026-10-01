# xcomp has no subject of its own

**Gist.** `xcomp` is an open complement whose subject is taken from the main clause (control or raising): *I want to leave*, *She seems happy*. If the clause has its own subject, it is a closed complement — `ccomp`: *I know that he left*, *He said he would go*. xcomp/ccomp confusion is typical for parsers. In old SD conversions control was lost entirely.

**Conditions and exceptions.**
- The object of the main verb under control is the `obj` of the main verb, not the subject of the xcomp: *I want him to go* → obj(want, him), xcomp(want, go).
- A clause with *for* (*for him to go*) has `nsubj` in EWT and therefore is not `xcomp`.

**Examples.**
- *I want to go* → xcomp(want, go).
- *They made him leave* → obj(made, him), xcomp(made, leave).
- *He said he would go* → ccomp(said, go), nsubj(go, he).

**Check against gold.** EWT 2.18: `xcomp` with its own subject — 5 of 3831; GUM — 11 of 3640.

**Sources.** UD `_en/dep/xcomp.md`, `_en/dep/ccomp.md`; `2020.tacl-1.25` (BLiMP: control/raising is a separate category); english-banks §1.3 (W08-1301: control is lost in SD conversion).

```rule
rule: en.errors.xcomp-no-subject
what: xcomp has its own subject — a clause with a subject is ccomp
match: x[rel=xcomp]
require: none s[rel=nsubj|nsubj:pass, head=x]
severity: warn
source: UD _en/dep/xcomp.md, _en/dep/ccomp.md
```
