//! Batch prompt: rules (FairytaleQA types, explicit/implicit, anchors, format) and paragraphs numbered
//! P1..PN, paragraph sentences — [1]..[n]. LLM output is a compact 6-column TSV:
//! `p type ex anchors question answer`. Rules depend on the run rule: `RULES` — v1 (pilot, verbatim
//! as it was then), `RULES_V2` — an explicit answer is a verbatim span of a single anchor sentence.

use std::fmt::Write as _;

use crate::schema::Rule;
use crate::select::Para;

/// Rules v1 (the 26.09 pilot) — the same for all batches.
pub const RULES: &str = "You write reading-comprehension questions and answers about paragraphs of classic fairy tales and fables, in the style of the FairytaleQA dataset (questions written by education experts to check how well children aged 5-10 understand a story).

For EACH numbered paragraph below, write 3 to 6 question-answer pairs about that paragraph:
- mix question types: at least 3 different types when the paragraph allows, never all of one type;
- at least one implicit pair per paragraph;
- ask what matters for understanding the story (who does what and why, how characters feel, what leads to what), not trivia about wording;
- every answer must be supported by the sentences of this paragraph; the context line only tells who is who, do not ask about it;
- a question must make sense on its own: name the characters instead of using pronouns;
- answers are short: a phrase or one short sentence, at most 15 words;
- do not use knowledge of the rest of the tale.

type - exactly one of these seven names:
  character - who a character is or what a character is like (Who...? What kind of...?)
  setting - where or when the events happen (Where...? When...?)
  action - what a character does or did, or how (What did X do...? How did X...?)
  feeling - a character's emotion or reaction (How did X feel...?)
  causal relationship - why something happens: the earlier event or motive that leads to it (Why...? What made...?)
  outcome resolution - what happens as a result of an event (What happened after/when...?)
  prediction - what will probably happen next, predictable from cues in this paragraph (a plan, a promise, a warning, a threat)
ex - explicit or implicit:
  explicit - the answer is stated in the text; it may copy words from one sentence
  implicit - the answer is not stated in the text: it is inferred, or it combines or summarizes several sentences. An implicit answer must NOT appear word for word inside any single sentence of the paragraph; if it does, the pair is explicit.
anchors - the numbers of the paragraph's sentences that support the answer, comma-separated (for example 2 or 2,3); only sentences of this paragraph.

Output one line per pair, 6 TAB-separated columns:
p	type	ex	anchors	question	answer
p is the paragraph number (the number after P). Group the lines by paragraph, in order.

Examples (format only, not part of the task):
1	character	explicit	1	Who lived next to the old mill?	a poor miller and his three sons
1	feeling	implicit	2,3	How did the miller feel when the flour was stolen?	worried and angry
1	prediction	implicit	4	What will the youngest son probably do that night?	hide in the mill to catch the thief

Output ONLY these lines: no header, no explanations, no code fences.
";

/// Rules v2: an explicit answer is a verbatim span of a single anchor sentence (as in FairytaleQA), an implicit one is
/// in the model's own words and not a paraphrase of a single sentence.
pub const RULES_V2: &str = "You write reading-comprehension questions and answers about paragraphs of classic fairy tales and fables, in the style of the FairytaleQA dataset (questions written by education experts to check how well children aged 5-10 understand a story).

For EACH numbered paragraph below, write 3 to 6 question-answer pairs about that paragraph:
- mix question types: at least 3 different types when the paragraph allows, never all of one type;
- at least one implicit pair per paragraph;
- ask what matters for understanding the story (who does what and why, how characters feel, what leads to what), not trivia about wording;
- every answer must be supported by the sentences of this paragraph; the context line only tells who is who, do not ask about it;
- a question must make sense on its own: name the characters instead of using pronouns;
- answers are short: a phrase or one short sentence, at most 15 words;
- do not use knowledge of the rest of the tale.

type - exactly one of these seven names:
  character - who a character is or what a character is like (Who...? What kind of...?)
  setting - where or when the events happen (Where...? When...?)
  action - what a character does or did, or how (What did X do...? How did X...?)
  feeling - a character's emotion or reaction (How did X feel...?)
  causal relationship - why something happens: the earlier event or motive that leads to it (Why...? What made...?)
  outcome resolution - what happens as a result of an event (What happened after/when...?)
  prediction - what will probably happen next, predictable from cues in this paragraph (a plan, a promise, a warning, a threat)
ex - explicit or implicit:
  explicit - one sentence of the paragraph states the answer, and the answer is a VERBATIM SPAN copied from that sentence: the same words in the same order and form, nothing changed, added, reordered or skipped inside the span. You may start and end the span anywhere in the sentence, so leave out words at its edges; choose the shortest span that fully answers the question. Do not change word forms (not \"running\" for \"ran\"), do not replace pronouns with names inside the span, do not join parts of two sentences.
  implicit - no single sentence states the answer: it is inferred, or it combines or summarizes several sentences; write it in your own words. An implicit answer must NOT appear word for word inside any single sentence, and must not merely restate one sentence in other words: if one sentence states the answer, the pair is explicit and its answer is a span of that sentence.
anchors - the numbers of the paragraph's sentences that support the answer, comma-separated (for example 2 or 2,3); only sentences of this paragraph. For an explicit pair, the sentence the span is copied from must be among the anchors.

Output one line per pair, 6 TAB-separated columns:
p	type	ex	anchors	question	answer
p is the paragraph number (the number after P). Group the lines by paragraph, in order.

Example (format only, not part of the task):
=== P1 - \"The Miller's Sons\", paragraph 1
[1] Next to the old mill lived a poor miller and his three sons.
[2] One morning the miller found that half of his flour had been stolen.
[3] 'I will watch the mill tonight,' said the youngest son.
Output:
1	character	explicit	1	Who lived next to the old mill?	a poor miller and his three sons
1	action	explicit	2	What did the miller find one morning?	half of his flour had been stolen
1	feeling	implicit	2	How did the miller feel when the flour was stolen?	worried and angry
1	prediction	implicit	3	What will the youngest son probably do that night?	hide in the mill to catch the thief

Output ONLY these lines: no header, no explanations, no code fences.
";

pub fn rules(rule: Rule) -> &'static str {
    match rule {
        Rule::V1 => RULES,
        Rule::V2 => RULES_V2,
    }
}

/// Batch prompt: rules and paragraphs numbered 1..N in batch order.
pub fn prompt(paras: &[&Para], rule: Rule) -> String {
    let mut p = String::from(rules(rule));
    p.push_str("\nParagraphs:\n");
    for (i, a) in paras.iter().enumerate() {
        let by = if a.author.is_empty() { String::new() } else { format!(" ({})", a.author) };
        let _ = write!(p, "\n=== P{} - \"{}\"{by}, paragraph {}\n", i + 1, a.title, a.para);
        // context — only if the tale's previous paragraph does not stand right before in this same batch
        let adjacent = i > 0 && paras[i - 1].doc == a.doc && paras[i - 1].para + 1 == a.para;
        if !a.ctx.is_empty() && !adjacent {
            let _ = writeln!(p, "context (before this paragraph, do not ask about it): {}", a.ctx.join(" "));
        }
        for (k, s) in a.sents.iter().enumerate() {
            let _ = writeln!(p, "[{}] {}", k + 1, s.text);
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::select::Sent;

    #[test]
    fn prompt_numbers_paragraphs_and_sentences() {
        let a = Para {
            doc: "g:1:t".into(),
            book: "g:1".into(),
            title: "The Fox".into(),
            author: "Aesop".into(),
            para: 4,
            block: "p".into(),
            sents: vec![Sent { sent_id: "g:1:t:7".into(), text: "A fox ran.".into() }, Sent { sent_id: "g:1:t:8".into(), text: "He hid.".into() }],
            dialogue: false,
            words: 5,
            ctx: vec!["Night fell.".into()],
            batch: 0,
        };
        let mut b = a.clone();
        b.author.clear();
        let mut c = a.clone();
        c.para = 5;
        let p = prompt(&[&a, &b, &c], Rule::V1);
        assert!(p.starts_with(RULES));
        let p2 = prompt(&[&a, &b, &c], Rule::V2);
        assert!(p2.starts_with(RULES_V2) && p2.ends_with(&p[RULES.len()..]));
        assert!(p.contains("=== P1 - \"The Fox\" (Aesop), paragraph 4\ncontext (before this paragraph, do not ask about it): Night fell.\n[1] A fox ran.\n[2] He hid.\n"));
        assert!(p.contains("=== P2 - \"The Fox\", paragraph 4\ncontext (before"));
        // adjacent paragraph of the same tale right above — no context
        assert!(p.contains("=== P3 - \"The Fox\" (Aesop), paragraph 5\n[1] A fox ran."));
    }
}
