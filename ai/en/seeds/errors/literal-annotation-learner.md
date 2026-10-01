# Text with errors is annotated literally

**Gist.** UD annotation describes what is written, not what the author meant: an ungrammatical sentence gets a tree based on the actual words. This is the "literal reading" in the learner English treebank TLE, where every sentence also has a corrected parallel version. For spelling, UD has a separate mechanism:
- the form stays as it is;
- Typo=Yes is set;
- the lemma is taken from the normalized word;
- the correction goes into MISC as CorrectForm.

EWT 2.18 has 1439 tokens with Typo=Yes. The tag and UPOS there come from the intended word: *new* instead of *knew* → VBD, lemma *know*. Grammatical errors (agreement, verb form) do not get Typo: in *My wife know* the verb stays VBP.

An LLM annotator tends to silently "correct", i.e. tag what was intended instead of what was written. Then the agreement rules of this section stay silent, and the error in the text is lost.

**Conditions and exceptions.**
- Deliberately non-standard forms (*should of*, *ain't*, *them boys*) are not Typo but Style (see `vernacular-style.md`).
- A missing word is analysed as ellipsis, a superfluous one in speech as `reparandum`.
- The choice between ellipsis, coordination and listing depends on whether the annotator considers the sentence grammatical. This principle must be stated explicitly.

**Examples.**
- *He new it* → new: lemma know, VBD, Typo=Yes, CorrectForm=knew.
- *My wife know my secret* → know: VBP, no Typo.

**In UD.** Typo=Yes, CorrectForm (MISC), `goeswith` for a split word, `reparandum`. For L2 treebanks a separate subtype for deliberate violation of validation rules has also been proposed.

**Sources.** `P16-1070` (Berzak et al.: TLE, 5124 sentences, literal reading, errors hurt parsing less than was thought); `2025.udw-1.17` (Masciolini et al.: survey of seven L2 treebanks and UD rules for errors); `dickinson-2015-grammaticality-syntactic-annotation-learner-language` (grammaticality and annotation categories); `2022.bea-1.7` (Kyle et al.: SL2E — even a little L2 data raises LAS from 0.876 to 0.935).
