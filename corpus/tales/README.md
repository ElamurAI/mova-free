# Tales

42 books of fairy tales, fables and children's stories from
[Project Gutenberg](https://www.gutenberg.org/), one plain-text file per book
(`pgNNNN.txt`, where NNNN is the Gutenberg ebook number: the original is at
`https://www.gutenberg.org/ebooks/NNNN`).

**Changed:** the Project Gutenberg header and footer (license boilerplate and
trademark) are removed; the text itself is unchanged.

**License:** public domain in the USA, as marked by Project Gutenberg. Copyright
terms differ between countries: if you are outside the USA, check the status of
a book (author's and translator's dates) in your country.

**How Mova uses it:** the reading experiments in `ai/world` (`world read`,
`world read-train`) split each book into stories and paragraphs, summarize
paragraphs, ask short questions about them and train the sentence retriever and
answer extractor on those questions. Aesop's fables (`pg21`) also drive the
fable-logic experiments: judgments derived from predicates over the story world,
not from trigger words.
