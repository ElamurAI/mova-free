# ATIS — verb features derived from the PTB tag: imperative as infinitive, agreement only in 3sg

**Gist.** Verb FEATS in ATIS depend only on the PTB tag (VB, VBP, VBZ, MD…), not on the sentence. Therefore:
- the imperative *show me…* has the tag VB and gets `VerbForm=Inf`, like an infinitive;
- VBP and VBD have no Number and Person: the tag does not carry them;
- MD has no features at all;
- there is no `Voice=Pass` anywhere.

EWT and `mova` write the imperative as `Mood=Imp|VerbForm=Fin`, and set Number and Person on every finite verb from the subject.

**Conditions and exceptions.**
- VBZ gets `Number=Sing|Person=3` — this is visible from the tag.
- `Tense=Pres|VerbForm=Part` on all *-ing* forms, 884 times. There is no `VerbForm=Ger`, although since 2.14 EWT distinguishes gerund and participle.
- *you* has `Number=Sing` (214 subjects), while in EWT *you* has no Number. EWT gives a verb with subject *you* number `Sing`, 759 of 765.

**Examples.**
- `0003.dev` *show me round trip flights…*: *show* — `VerbForm=Inf`, a root without a subject.
- `0002.dev` *i want a flight…*: *want* — `Mood=Ind|Tense=Pres|VerbForm=Fin`, without `Number=Sing|Person=1`.
- `0007.dev` *could i have listings…*: *could* — FEATS «_».

**In UD.** Counters — the general rules from `../../seeds-overview.md`. Entries: violations / matches, all parts together.

| rule | ATIS | EWT |
|---|---:|---:|
| `tb.en.imp-mood`: VERB root without subject, aux and mark lacks `Mood=Imp` | 1709 / 2599 | 38 / 3066 |
| `tb.en.fin-agree`: finite verb with Tense without Number/Person | 1654 / 2903 | 0 / 19 126 |
| — of them present tense (VBP) | 1632 | 0 |
| — past (VBD) | 22 | 0 |
| `tb.en.modal-fin`: modal does not have exactly `VerbForm=Fin` | 783 / 783 | 0 / 4048 |
| `tb.en.part-voice`: participle with `aux:pass` without `Voice=Pass` | 9 / 9 | 0 / 1643 |

Modals without features: *will* 335, *would* 235 (including *'d* in *i'd*), *can* 183, *should* 14, *may* 14, *must* 1.

The remaining 890 of the 2599 `imp-mood` matches are roots with a subject, aux or mark, such as *i would like…* with an infinitive under aux.

**Sources.** Cesur et al. 2024, `2024.bucc-1.11`, §3 (PTB tags → UD by rules); README `UD_English-Atis` («Features: converted from manual»); https://universaldependencies.org/en/feat/Mood.html, `feat/VerbForm.md`; EWT README, v2.13 («Ensure verb features are complete and consistent with tags»), v2.14 (`VBG`: Ger versus Part); UD docs/changes.md` No. 18 (2.19): a feature underspecified by the form is taken from context, so Number/Person on VBP come from the subject ("Agreement", ATIS −2 UFeats).

```rule
rule: tb.atis.vbp-bare
what: ATIS — present finite without Number (VBP form without agreement)
match: v[feats.VerbForm=Fin, feats.Tense=Pres, !feats.Number]
require: v[feats.Number]
severity: warn
source: README UD_English-Atis (FEATS from the PTB tag); counter
```

```rule
rule: tb.atis.you-number
what: ATIS — you has Number=Sing (in EWT you has no Number)
match: p[form=you, upos=PRON]
require: not p[feats.Number]
severity: warn
source: EWT 2.18 (you: Case|Person=2|PronType=Prs); counter
```
