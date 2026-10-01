# GUMReddit — annotation without text

**Gist.** The Reddit part of the GUM corpus: 895 sentences and 16,364 words. Annotated with the same pipeline and the same customs as GUM (`../../gum/seeds/`). The Reddit text is not distributed: FORM, LEMMA and `# text` contain «_». They are restored by the `get_text.py` script from the GUM repository, which fetches the posts from a Reddit archive. Annotations — CC BY 4.0, text — under Reddit's terms.

**Conditions and exceptions.**
- Split: train 686 / dev 104 / test 105 sentences.
- Everything that does not depend on the words is present:
  - UPOS, XPOS, FEATS («_» in 4874 words, as in GUM);
  - heads and relations;
  - DEPS;
  - MISC with `Entity`, `Discourse` etc. («_» in no word);
  - MWT (406) — with «_» instead of the form.
- `# text` — underscores with spaces (*_ _ _ __*): no word lengths, only their number and the spaces.
- Document metadata contain `meta::author` — Reddit user names. `mova` does not need them. They should not be carried into derived data.

**Examples.** `# newdoc id = GUM_reddit_macroeconomics`, `# text = _ _ _ _ _ _ _ __ _ __ …`.

**In UD.** Our annotator does not measure GUMReddit: without text there is nothing to parse. Training tags and trees on it is possible only after `get_text.py`: the forms are needed both as features and for converter rules with `form=`. For counters without forms — `gum-family.md`.

**Sources.** README `UD_English-GUMReddit` («This repository only contains annotations, without the underlying textual data from Reddit»; `get_text.py`); LICENSE.txt (CC BY 4.0 for the annotations); Behzad & Zeldes 2020, «A Cross-Genre Ensemble Approach to Robust Reddit Part of Speech Tagging», WAC-XII; `data/raw/ud-docs/docs/treebanks/en_gumreddit/index.md`.
