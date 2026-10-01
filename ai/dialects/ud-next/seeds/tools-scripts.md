# New tools: UPOS+FEATS → XPOS and treebank scripts

**Gist.** After 2.18 two scripts by Zeman appeared in `tools`, and two new pages on the site:
- `conllu_convert_uposf_to_xpos.pl` computes XPOS in a given tagset from UPOS and FEATS via Interset. It is the inverse of `conll_convert_tags_to_uposf.pl`;
- `survey_scripts.pl` determines which script (ISO 15924) each treebank is written in. The result is UD docs/survey-scripts.md`;
- UD docs/languages.md` — a list of languages with script codes.

This closed #1032 «How to document writing system used in a treebank?» (closed 07.06.2026, milestone v2.19).

**Level.** Universal, tools and metadata. Does not affect annotation.

**Evidence.**
- `diff -rq`: `tools/conllu_convert_uposf_to_xpos.pl` and `tools/survey_scripts.pl` exist only in the snapshot;
- `tools/README.md`, section «conllu_convert_uposf_to_xpos.pl»: «takes UPOS and FEATS, and based on them it computes XPOS in a specific tagset … The target tagset is a tagset identifier known to Interset; the default value is `cs::pdtc`»;
- UD docs/survey-scripts.md`: «automatically generated list of scripts that occur in the UD data».

Also in `tools`: `package_ud_release.sh` copies via `rsync` without `__pycache__`; `survey_language_families.pl` was updated; the snapshot has no `compat/`, `conllu-formconvert.py`, `conllu-w2t.py`, `create_iso_639_3_symlinks.py`.

**What it means for English annotation.** Directly — nothing. For Mova:
- Interset has an `en::penn` table. So `conllu_convert_uposf_to_xpos.pl -t en::penn` gives an independent check of UPOS+FEATS ↔ PTB pairs in our `en`, and also XPOS for treebanks without it. Using it requires Perl and `Lingua::Interset`. That is an external tool, so only on Mova's say-so;
- the script (`Latn`) will become a metadata field, and for the `treebanks/<language>/…` cards one more line.

**Registry.** Not applicable.

**Sources.** `tools/conllu_convert_uposf_to_xpos.pl`, `tools/survey_scripts.pl`, `tools/README.md`, UD docs/survey-scripts.md`, UD docs/languages.md`; #1032.
