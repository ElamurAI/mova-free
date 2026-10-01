# ESLSpok — PTB tokenization without MWT and without `SpaceAfter`: the text cannot be reconstructed

**Gist.** ESLSpok words are the same as in EWT: contractions are split per PTB (*do n't*, *it 's*, *I 'm*). But:
- there are no MWT lines (`3-4 don't`);
- MISC is empty, so there is no `SpaceAfter=No`;
- `# text` is a tokenized string with spaces: *the man on the motorcycle 's fault .*

So the original text cannot be restored. EWT since 2.7 groups contractions into MWT, and `# text` there is the real string.

**Conditions and exceptions.**
- Separate words without MWT: *'s* — 209, *n't* — 179, *'m* — 110, *'ll* — 14, *'re* — 13, *'d* — 13, *'ve* — 4.
- `# text` with a space before *n't*: ESLSpok — 173 sentences, EWT — 0.
- *wanna* remained one word (VBP) — once. EWT splits it into *wan* + *na*: *wan* VBP 6, *na* TO 13.
- Segmentation is by the sample's sentences, without documents and paragraphs: there is no `# newdoc`.

**Examples.**
- `# text = And so this accident became the man on the motorcycle 's fault .`
- `# text = And then think about what we really wanna eat .`

**In UD.** For parsing and tagging there is no difference: `en` reads syntactic words and skips MWT lines (`en/src/conllu.rs`). The difference matters for the tokenizer (`tok.rs`) and for the full "text → graph → text" cycle: they cannot be trained on ESLSpok, because the spaces in `# text` are artificial. The converter into `mova` can restore MWT by the forms (*n't*, *'s*…), but can only guess `SpaceAfter` (see `../convert.md`).

**Sources.** README `UD_English-ESLSpok`; https://universaldependencies.org/en/tokenization.html ("new" PTB, MWT); EWT README v2.7 (MWT for contractions); Zeldes & Schneider 2023, `2023.udw-1.7`, sec. 3 (MWT in EWT 2.7, in GUM 2.8).
