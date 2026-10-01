# ATIS — relative *that/which* is most often annotated as the conjunction `mark`

**Gist.** In a relative clause *a flight that arrives…* ATIS most often attaches *that* to the verb as `mark` with UPOS ADP. The relative-clause verb is then left without a subject. EWT and `mova` treat relative *that* as a pronoun: PRON with `PronType=Rel` in the role of subject, object or obl inside the clause. In EWT `mark` occurs here only once in 909 cases.

**Conditions and exceptions.**
- The remaining relative *that/which* in ATIS are `nsubj` with three different UPOS:
  - *which* PRON — 35;
  - *that* ADP — 42;
  - *that* DET — 29;
  - *that* PRON — 23.

  Here ATIS itself is inconsistent.
- *which* is also sometimes `mark` with ADP — 18 times.

**Examples.**
- `0083.dev` *i would like to find a flight that goes from tampa to montreal…* — *that*: ADP, `mark` → *goes*.
- `0002.dev` *…a flight from memphis to seattle that arrives no later than 3 pm* — *that* is not PRON.
- `0015.dev` *ground transportation that i could get in boston* — in EWT *that* would be `obj` of *get*.

**In UD.**

| rule | ATIS | EWT | CHILDES |
|---|---:|---:|---:|
| `tb.atis.relcl-mark`: relative *that/which* before an `acl:relcl` verb — `mark` | 293 / 463 | 1 / 909 | 61 / 271 |
| `tb.en.rel-that`: relative *that* as subject or object not PRON | 86 / 111 | 0 / 595 | 3 / 201 |
| `tb.en.rel-prontype`: relative pronoun without `PronType=Rel` | 67 / 67 | 1 / 1238 | 242 / 242 |

In ATIS `rel-prontype` is always violated: it has `PronType=Int,Rel` (`pos-feats-from-ptb.md`). CHILDES has no FEATS at all.

**Sources.** https://universaldependencies.org/en/dep/acl-relcl.html, https://universaldependencies.org/en/pos/PRON.html (relative pronouns); EWT README v2.11 (relatives), v2.14 (`Cxn=rc-…`: relative clause types); Cesur et al. 2024, `2024.bucc-1.11`, §3 (IN → ADP by rule).

```rule
rule: tb.atis.relcl-mark
what: relative that/which annotated as mark, not as a pronoun argument
match: v[rel=acl:relcl]; t[form=that|which, head=v, before=v]
require: not t[rel=mark]
severity: warn
source: UD en acl:relcl; EWT 2.18; counter
```
