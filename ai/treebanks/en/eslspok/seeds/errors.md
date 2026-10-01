# ESLSpok — known annotation errors

**Gist.** ESLSpok annotates learner errors literally, and that is not a fault but a custom (`data.md`). Faults are where the annotation contradicts its own guideline. The converter does not fix them. Counters — the rules of `../../seeds-overview.md`, violations / matches.

**Examples and numbers.**
- ***to* as `obl` or `xcomp`** — 8 times: `obl` 5, `xcomp` 3. A PART cannot be an argument.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`): 1 of 35 passives with `aux:pass`.
- **`obl` on a noun without a copula** (`tb.en.obl-fragment`): 7 of 20:
  - `file00084.txt_27` *Hokkaido city , for six years .* — a fragment; here EWT could also give `obl`;
  - `file00646.txt_48` *please the ticket to me .* — *me*: `obl` → *ticket*. For EWT this is `nmod`.
- **A demonstrative as a noun with UPOS DET** (`tb.en.dem-pron`): 1 of 121.
- **A sole `iobj` without `obj`** (`tb.en.sole-iobj`): 6 of 26, for example `file00054.txt_71` *I can not tell you about what you can do*. Under UD 2.0 this should have been `obj`. A sole `iobj` has been allowed only since UD 2.12 (the "Sole iobj" amendment). So these 6 are either errors by their own guideline, or an early switch to the new one.
- **`dep` on Japanese words** (26) — not an error but a custom (`old-ud.md`). But `dep` does not say what role the word has in the sentence.

**In UD.** There are few errors, because the annotation was done by two people with adjudication. ESLSpok's main problem for merging is not errors but old customs and missing layers.

**Sources.** https://universaldependencies.org/en/dep/iobj.html; UD docs/changes.md` No. 7 (Sole iobj, 2.12); Kyle et al. 2022, `2022.bea-1.7` (agreement 95.1 / 86.5 before adjudication).
