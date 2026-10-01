# EWT — known annotation errors

**Gist.** EWT is the base of `mova`, so the converter does not fix its errors: otherwise the round trip EWT → `mova` → EWT would stop being identity. Errors are caught by seed rules (`en/seeds`, error level) and tagger suspicions. Fixes are a separate layer of edits, not a dialect.

**Conditions and exceptions.** Numbers — `en expert-check en/seeds` on all parts of EWT 2.18: error-level violations out of matches.

**Examples and numbers.**
- **`compound` to the right of the head** (`en.nominal.compound-head-final`) — 23 of 8690: `email-enronsent21_02-0030` *EY4096.1 PERFORMANCE*.
- **Infinitive VB under *Do*, while *Do* is not aux** (`en.verbal.vb-aux-infinitive`) — 7 of 3888: `email-enronsent08_02-0045` *Do … take*.
- **aux on a non-finite head** (`en.verbal.aux-head-nonfinite`) — 6: *could … missed*.
- ***what* as `det:predet`** (`en.nominal.predet-xpos`, `det-is-det`) — 5: `reviews-287360-0002` *What …*.
- **Imperative with inconsistent features** (`en.verbal.imperative-form`, `imperative-aux-do-only`) — 2 + 2: `reviews-326439-0002` *TRY*.
- **Free relatives annotated as interrogatives** — a known problem from the README (Known Issues), not caught by a rule.
- **General counters** (`../../seeds-overview.md`) give small residues on EWT:
  - `tb.en.imp-mood` — 38 of 3066;
  - `tb.en.propn-number` — 475 of 16,562 PROPN without Number;
  - `tb.en.obl-fragment` — 12 of 196;
  - `tb.en.num-feats` — 15 of 5051;
  - `tb.en.pron-lemma` — 9 of 5039.
  
  Some of them are edge cases rather than errors.
- **`Typo=Yes`** — 1431, of which 1429 have `CorrectForm` in MISC. These are not annotation errors but marked text errors; about 200 more were added in 2.18.

**In UD.** Of the 213 error-level rules in `en/seeds`, only 16 have violations on EWT train, 1–17 cases per rule. Almost all of them are errors of EWT itself (`ai/en/src/expert.rs`, "Self-check on gold data"). Under "examples are not truth" these are tagger and parser suspicions, not a reason to change `mova`.

**Sources.** README `UD_English-EWT`, Known Issues («Many free relatives are incorrectly analyzed as interrogative»), v2.18 («Mark about 200 tokens as typos»); `ai/en/src/expert.rs`.
