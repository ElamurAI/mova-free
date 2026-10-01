# Tale questions

12,251 short questions about the tales, one JSON object per line: `story`,
`para` (paragraph the answer comes from), `question`, `answer` (at most three
words, copied verbatim from the text), `multi` (the answer needs more than one
paragraph). Files are grouped by book set: `grimm`, `aesop`, `andersen`, `blue`
(Blue Fairy Book), `english`, `indian`.

**How made:** a large language model wrote simple questions for each paragraph
with an answer that occurs literally in the text; Mova keeps a question only if
its answer is found in the paragraph.

**License:** ours, Apache-2.0 OR MIT.

**How Mova uses it:** training and evaluation of short-answer reading in
`ai/world` (`world read-train`, `world read-eval`): for each question Mova
finds the sentence and extracts the answer span, with an averaged perceptron
over features of the sentence and the candidate.
