# Anchors, gaps and question normalization

## forward

ask_of maps why to Cause, where to Place, when to Role(Attr) (the time the reader keeps as when: attributes; PRAG_OFF=wh-time restores Frame), who subject to Agent and other WH cases by tree role; Asked frame references predicate rank. units splits plain text and applies statement_order only for questions. plain gives a speaker label its verb of saying back before parsing (speaker_label, PRAG_OFF=speaker-label): a sentence-initial capitalized label of at most six words before its first comma and twelve in all, with no digit, quotation mark, LABEL_CLAUSE word, verb-only word or (after the first word) possible VBZ/VBD form, followed outside quotes by a colon and an opening quotation mark, gets " says" inserted at its first comma or end (Dad: "pass me the salt" -> Dad says: "pass me the salt"). repair functions can alter supplied word dependencies before frame extraction.

## behavior-complement

Universe: supported who-as-Nsubj question trees and candidate Asked observations. Exclude an Agent gap naming a different frame or changing it to Place. For empty text exclude any parser call in read_with. For speaker_label exclude any change to a text with no colon followed by an opening quote, any change inside a quotation, any byte change other than one inserted " says" per accepted label, and a second insertion into an already rewritten label (idempotence).

## reverse-reconstruction

An Agent gap does not recover the question's wording, subject agreement or normalized do-support. Different questions share Ask::Frame; when- and how-questions share Role(Attr). A bare time phrase keeps only its whole value (March 2022 -> when:march 2022), so the year alone is not recoverable from the option's attribute set as a separate value. speaker_label is injective on accepted labels: deleting each inserted " says" before its colon recovers the input, but a text that already read Dad says: "..." has the same image as Dad: "...", so the preimage of a rewritten text is the set of those spellings. Anchor role projections lose unstated/shared arguments.

## invariant

Question-specific rewrites are conditional; parser callback sees normalized unit text. Bare predicate sentinel is usize::MAX, not a valid token.

## cost

Sentence normalization linear-to-quadratic in text/token copying; anchor lookup scans predicates/roles. General repair correctness is not proven.

## question-coordinate

For two supplied units with Who as Nsubj in the second, asked.frame=1 and what=Role(Agent), while pred[1]=(1,1). Native fixture rejects conflating token position and global frame position.

## composition

Only supplied fixtures execute; natural language parser quality, all repairs, bare fragments and ambiguity are explicit gaps.

## epoch-memory

Time and name roles. Prediction before code: the Wikipedia school's when-questions are all guesses because the reader asks Frame and keeps only when:<lemma> (the year lost), and which-questions tie because the answer is a name inside the remembered entity; a whole time value as a when: attribute, when asking Role(Attr), and a name role (Ent::names) should lift when to ~60-75% and which to ~55-70% with the articles-shuffled control flat. Measured (CPU, peta build, train and cv courses, two identical runs): when 259 -> 881 of 1106 (shuffled 263), which 65 -> 200 of 214 (shuffled 64) on wikipedia; cv-a 143 -> 457, 27 -> 90; cv-b 155 -> 444, 24 -> 106; subjects -12 net on wikipedia. Ten school courses 3234 -> 3248 of 7283. Tests wh_role_tests: page 29 and their march have no time value, a cup of tea names no cup; the mutant that accepts any month word fails the time test (exact source restored).

## limitations

Partial syntax is not expanded semantic AST. No live parser, teacher, whole corpus, GPU or environment mutation. Acyclic input trees and fixed configuration/resource premises matter; malformed recursive ancestry, counter overflow, complete TAM/lexicon/quotation/repair semantics and all active-rule effects remain open. No executable approximant/error budget is claimed.

## History

### 2026-10-30 · epoch-memory

Wave15, 2026-10-30. Prospective repair: reject unresolved right heads in identity with a constant six-item guard, preserving coordination membership and its nontransitivity. Old production failed unresolved_identity_is_symmetric_even_in_coordination; repaired code passes. Compiled guard-removal control fails the same named assertion and exact bytes are restored. Eight focused tests pass twice debug and twice release. No speed, full semantic coverage or universal proof claim.

### 2026-11-03 · epoch-memory

Speaker labels, 2026-11-03. Prediction before code: labels are read with the speaker as subject of the quoted clause (Dad: "Leo, pass me the salt" -> dad pass salt to=i); restoring the verb of saying lets the existing say/quote reading assign me to the speaker and the vocative/you to the addressee; score effect expected small, reading effect large. Measured on 180 labelled test questions (commands, world, adult-life): speaker read as the one who says 14 -> 102, speaker read as doer of the quoted clause 20 -> 14. School courses (CPU, two identical runs on commands): commands 43 -> 40 of 115, nine primary courses 3053 -> 3065 (odd lessons +9, even 0). A guard-removal mutant (no possible-finite-verb check) fails what_is_no_speaker_label_stays_byte_for_byte; exact source restored.

### 2026-11-04 · epoch-memory

Time and name roles, 2026-11-04. Prediction before code: the Wikipedia school's when-questions are all guesses because the reader asks Frame and keeps only when:<lemma> (the year lost), and which-questions tie because the answer is a name inside the remembered entity; a whole time value as a when: attribute, when asking Role(Attr), and a name role (Ent::names) should lift when to ~60-75% and which to ~55-70% with the articles-shuffled control flat. Measured (CPU, peta build, train and cv courses, two identical runs): when 259 -> 881 of 1106 (shuffled 263), which 65 -> 200 of 214 (shuffled 64) on wikipedia; cv-a 143 -> 457, 27 -> 90; cv-b 155 -> 444, 24 -> 106; subjects -12 net on wikipedia. Ten school courses 3234 -> 3248 of 7283. Tests wh_role_tests: page 29 and their march have no time value, a cup of tea names no cup; the mutant that accepts any month word fails the time test (exact source restored).

### Undated · document

Subject: prag/src/frames.rs SHA-256 bfbc2f7905d878080a7e7dd30ae7f1bfe8f07313c3a4cf8f0254be29e72a4b88. Partial source model, finite native evidence. Fixed configuration and admitted finite resource domains; source-derived inner exclusions and ambiguous preimages, no inverse solver.
