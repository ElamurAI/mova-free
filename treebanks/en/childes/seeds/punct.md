# CHILDES — added punctuation with XPOS `?` and `!`

**Gist.** CHILDES transcripts have no punctuation. The harmonizers added the final sign from the sentence type (`# type`) and capitalized the first letter (Yang et al. 2025). The XPOS of these signs are the symbols `?` and `!` themselves, not `.` as PTB requires: in PTB `.` is the tag for sentence-final punctuation of any kind. EWT tags `?` and `!` as `.`.

**Conditions and exceptions.**
- Punctuation XPOS in CHILDES: `.` 38,560, `?` 8918, `!` 734. Another 5 PUNCT have XPOS `NNP`, `NN`, `HYPH` — errors.
- The UPOS of all these signs is PUNCT, the relation is `punct`, so this does not affect parsing.
- The punctuation is synthetic. Its amount and position say nothing about speech, and in the model the final sign correlates with `# type`.

**Examples.** `34740` (Sarah) *What else?* — *?*: PUNCT, XPOS `?`. In EWT the question mark has XPOS `.`.

**In UD.** The tag `?` or `!` in XPOS is not in the `en` PTB tag list (`en/src/gram.rs`, `Tag`). The CoNLL-U reader turns it into "no tag", so such sentences already do not go into tagger training (`Sentence::tagged()`). The `?`/`!` → `.` converter will bring them back into training.

**Sources.** Yang et al. 2025, `2025.udw-1.6`, sec. 3 "Harmonization" («we capitalize the first word of each utterance and infer sentence-final punctuation…»); Santorini 1990 (PTB), `.`; README `UD_English-CHILDES` («XPOS: automatic»).
