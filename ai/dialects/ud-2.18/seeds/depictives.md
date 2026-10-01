# Depictives: three answers in the guidelines, advcl in the data

**Gist.** An optional depictive is an adjective describing a participant during the action (*She entered the room sad*). `specific-syntax.md` gives two outdated answers for it: `acl` and `advmod`. The 2.10 amendment and `_en/dep/advcl.md` say `advcl`. EWT follows the amendment.

**Guideline.**
- `2.18:https://universaldependencies.org/en/specific-syntax.html:612`: «_Depictives_ are also represented with the `acl` relation»;
- `2.18:https://universaldependencies.org/en/specific-syntax.html:876–878`: «[<!> May be subject to change] Depictives … should be analyzed using the `advmod` relation». Examples: `advmod(stuttering, unable)`, `advmod(rest, assured)`;
- `2.18:https://universaldependencies.org/en/dep/advcl.html:49–60` and `2.18:UD docs/changes.md` No. 4 (05.2022, 2.10, AMENDMENT) «Optional Depictives»: `advcl(entered, sad)`;
- `2.18:https://universaldependencies.org/en/dep/acl.html:54`: `acl` «no longer used for optional depictives».

**EWT 2.18 data** (engine):
- ADJ as `advcl` on a VERB — 208. Of these, without `mark`, `cop` and a subject, i.e. depictives — 22: *came back **dead***, *came into office **obsessed** with Iraq*, *died in its sleep **aged** 21*, *feed them … **uncut***, *put it **alone** in a breeding tank*;
- ADJ as `acl` on a VERB — 0;
- ADJ as `advmod` on a VERB — 11, and none is a depictive: *strong-arm*, *effective immediately*, *Worse for Israel*, *Right "mouse" click*;
- the guideline's own examples are annotated differently in EWT: *unable* — `parataxis`, and *assured* (*rest assured*) — a VERB with `advcl`.

**Where the discrepancy comes from.** The `specific-syntax.md` sections were written before the 2.10 amendment and not updated. Line 612 even contradicts line 878 of the same file.

**Sources.** `2.18:https://universaldependencies.org/en/specific-syntax.html, `2.18:https://universaldependencies.org/en/dep/advcl.html, `2.18:https://universaldependencies.org/en/dep/acl.html, `2.18:UD docs/changes.md#optional-depictives`, `2.18:https://universaldependencies.org/u/dep/xcomp.html:202`; report `data/runs/bones-2026-09-25/verbal.md`, discrepancy 4.

```rule
rule: ud218.depictive-advcl
what: ADJ depictive as advcl on a verb without mark, cop and subject (2.10 amendment)
match: v[upos=VERB]; a[upos=ADJ, rel=advcl, head=v]
require: exists c[rel=mark|cop|nsubj|nsubj:pass|csubj, head=a]
severity: warn
source: UD 2.18 UD docs/changes.md#optional-depictives; https://universaldependencies.org/en/dep/advcl.html:49
```

```rule
rule: ud218.depictive-advmod
what: ADJ as advmod on a verb — the analysis of specific-syntax.md:878
match: v[upos=VERB]; a[upos=ADJ, rel=advmod, head=v]
require: not a[rel=advmod]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/specific-syntax.html:878
```

```rule
rule: ud218.depictive-acl
what: ADJ as acl on a verb — the analysis of specific-syntax.md:612
match: v[upos=VERB]; a[upos=ADJ, rel=acl, head=v]
require: not a[rel=acl]
severity: warn
source: UD 2.18 https://universaldependencies.org/en/specific-syntax.html:612
```
