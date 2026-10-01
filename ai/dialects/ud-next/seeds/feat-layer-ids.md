# Feature layer identifiers: from advice to rule (universal)

**Gist.** Layered features such as `Number[psor]` have a layer identifier. Lowercase Latin letters in it used to be only recommended; now they are required. The trigger seems to have been Old Georgian: after 2.18 its layers `sauf2` and `stack2` were renamed to `dsauf` and `stackb`. The validator still lets digits through.

**Level.** Universal, `_u-overview/feat-layers.md`. A guideline; the validator was not changed.

**Before (2.18).** `2.18:https://universaldependencies.org/u/overview/feat-layers.html:80`: «We recommend that the layer identifiers consist of lowercase English letters».

**Now (snapshot 24.09.2026).**
- https://universaldependencies.org/u/overview/feat-layers.html:80`: «The layer identifiers consist of lowercase English letters». The other change in the file is a removed trailing space on line 261;
- `docs/_oge/feat/`: new pages `Case-dsauf.md`, `Number-dsauf.md`, `Case-stackb.md`, `Case-sauf.md`, `Number-sauf.md`. The text still has the old names: `Case-stackb.md:14` — `Case[stack2]`, `Number-dsauf.md:15` — `Number[sauf2]`. About the pages themselves — PRs #1258–#1266 (June 2026);
- validator: `tools/udtools/src/udtools/utils.py:60`, layer — `\[[a-z0-9]+\]`, i.e. digits are allowed. `tools/data/feats.json` has no layers with digits.

**Evidence.** `diff` of `feat-layers.md`; the listing of `_oge/feat/`; `utils.py`. In parallel the site renamed pages: `Number[io].md` → `Number-io.md` (`_ka`, `_oge`). This is infrastructure, not annotation.

**What it means for English annotation.** Nothing: EWT and the en registry have no layered features. For `mova` — a constraint on future own layers: the identifier is only `[a-z]+`, no digits, even though the validator would accept digits.

**Registry.** Not applicable.

**Sources.** https://universaldependencies.org/u/overview/feat-layers.html, `2.18:https://universaldependencies.org/u/overview/feat-layers.html, `docs/_oge/feat/*`, `tools/udtools/src/udtools/utils.py`.
