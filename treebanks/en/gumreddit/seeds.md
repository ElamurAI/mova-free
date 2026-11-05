# Seeds: treebanks/en/gumreddit

The seeds of this treebank, one section per seed. A section's anchor is the seed name: `seeds.md#errors`.

<a id="data"></a>

# GUMReddit — annotation without text

**Gist.** The Reddit part of the GUM corpus: 895 sentences and 16,364 words. Annotated with the same pipeline and the same customs as GUM (`../gum/seeds.md`). The Reddit text is not distributed: FORM, LEMMA and `# text` contain «_». They are restored by the `get_text.py` script from the GUM repository, which fetches the posts from a Reddit archive. Annotations — CC BY 4.0, text — under Reddit's terms.

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

**In UD.** Our annotator does not measure GUMReddit: without text there is nothing to parse (`en/README.md`, "text removed from the treebank"). Training tags and trees on it is possible only after `get_text.py`: the forms are needed both as features and for converter rules with `form=`. For counters without forms — `gum-family.md`.

**Sources.** README `UD_English-GUMReddit` («This repository only contains annotations, without the underlying textual data from Reddit»; `get_text.py`); LICENSE.txt (CC BY 4.0 for the annotations); Behzad & Zeldes 2020, «A Cross-Genre Ensemble Approach to Robust Reddit Part of Speech Tagging», WAC-XII; `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_gumreddit/index.md`.

<a id="gum-family"></a>

# GUMReddit — GUM customs measured without words

**Gist.** GUMReddit customs are the same as in GUM (`../gum/seeds.md`). But most counter rules rely on the form or lemma (*you*, *such*, *what about*), and GUMReddit does not have them. So here there is only what is visible from UPOS, XPOS, FEATS and the tree.

**Conditions and exceptions.**
- **The number of *you*:** a 2nd person personal pronoun (PRON, `Person=2`, `PronType=Prs`, neither possessive nor reflexive) has Number in 210 of 210 cases. In GUM — 2468 of 2468, in EWT — 4 of 2767. By features: `Number=Sing` 203, `Plur` 6.
- **`Degree` on RB:** 502 of 907 ADV/RB have `Degree`. GUM — 5359 of 10,228, EWT — 482 of 10,980.
- **`obl` on a noun root without a copula:** 2 of 6.
- **`dep`:** 12 — PRON 6, NUM 3, NOUN 2, SYM 1. PRON here is most likely *you* in *you guys*, as in GUM (`../gum/seeds.md#dep`).
- **XPOS** without `ADD`, `NFP`, `AFX`, as in GUM (`../gum/seeds.md#xpos-web`).

**Examples.** Without text there are no examples. The mechanism is as in `../gum/seeds.md`.

**In UD.** The rules in the text are feature-based variants of the GUM rules:
- `p[upos=PRON, feats.Person=2, feats.PronType=Prs, !feats.Poss, !feats.Reflex]` → `not p[feats.Number]`;
- `a[upos=ADV, xpos=RB]` → `not a[feats.Degree]`.

They are also counted on EWT, so they serve as a negative control: EWT 4 and 482, GUMReddit 210 and 502.

**Sources.** README `UD_English-GUMReddit`; `../gum/seeds.md`.
