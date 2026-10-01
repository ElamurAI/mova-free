# XPOS cannot be an empty string (validator)

**Gist.** The validator checks something new at level 2: the XPOS column is never an empty string. An unspecified XPOS is written as `_`.

**Level.** Universal; validator (`udtools` 0.2.7 → 0.2.8). This change is not in `changes.md`: it is format, not a guideline.

**Before (2.18).** `2.18:tools/udtools/src/udtools/level2.py` — there is no check for empty XPOS. `udtools/pyproject.toml`: `version = "0.2.7"`.

**Now (snapshot 24.09.2026).**
- `tools/udtools/src/udtools/level2.py:247–273`, `check_xpos_format`: `if cols[XPOS] == None or cols[XPOS] == ''` → Error, level 2, testclass MORPHO, testid `empty-string-in-xpos`, «Unspecified XPOS must be encoded as an underscore ('_'), the field must not be empty.»;
- `tools/udtools/src/udtools/validator.py:286` — the check is called for every token line;
- `tools/udtools/pyproject.toml:7` and `__init__.py`: `0.2.8`.

Other validator changes are only logging stubs under `###!!!`, docstrings and a corrected link in a `data.py` message. `level3.py`–`level6.py` and `validate.py` did not change in substance. The tests were rewritten for the new package.

**Evidence.** `diff -r ud-tools-v2.18/udtools tools/udtools`.

**What it means for English annotation.** There is no empty XPOS in the EWT data, in the other 13 English 2.18 treebanks, or in `corpus/mova-*-en.conllu`: checked with grep, 0 lines. `en` writes PTB tags. The change matters for exporting `mova` into other languages where XPOS is a string and may be unknown: write `_`, not empty.

**Registry.** Not applicable: this is a column format. The gate must be in the CoNLL-U writer. It cannot be expressed in the seed rule language: that language does not see raw columns.

**Sources.** `tools/udtools/src/udtools/level2.py`, `validator.py`, `pyproject.toml`; `2.18:tools/udtools/…`.
