# GUM — errors and residues after the rules

**Gist.** GUM was annotated manually with several passes of alignment with EWT, so the general rules find almost no systematic errors. What remains beyond the customs from other seeds occurs just as often in EWT. This is not a dialect difference but a limit of the rules themselves.

**Examples and numbers** (violations / matches; EWT in parentheses):
- **Infinitive root without a subject and without `Mood=Imp`** (`tb.en.imp-mood`): 64 / 2472 (EWT 38 / 3066). All 64 have `VerbForm=Inf`: these are items of instruction lists (`GUM_academic_librarians-18` *Identify possible subjects, based on experience…*, *Get feedback from librarians…*). EWT has 38 such roots of 3066, i.e. the "infinitive or imperative" boundary in list items is shaky in both treebanks.
- ***to* before a gerund — SCONJ `mark`** (`tb.en.to-part`): 48 / 3516 (EWT 58 / 4021): `GUM_letter_arendt-35` *I look forward to going over manuscript…*. A custom shared with EWT, not an error.
- ***whose* without `Case=Gen`** (`tb.en.poss-gen`): 26 / 3438 (EWT 16 / 3689), for example `GUM_fiction_lunre-13`. A residue shared by both treebanks.
- **Non-agentive `by`** (`tb.en.obl-agent`): 17 / 497 (EWT 16 / 411): *observed … by …* meaning "by (a time)", *dissolved by force*. Rule noise.
- ***excuse me* with `iobj`** — 6 (`iobj.md`). A candidate for a real error.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`): 3 / 1689 (EWT 0).

**In UD.** For merging, GUM is dangerous not because of errors but because of customs: `dep` (`dep.md`), `obl` in fragments (`obl-fragment.md`), the number of *you* (`you-number.md`), `Degree` (`adv-degree.md`), XPOS (`xpos-web.md`).

**Sources.** Rules in `../../seeds-overview.md`; `ai/en/src/expert.rs` ("Self-check on gold data": EWT errors caught by seed rules).
