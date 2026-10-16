//! The pragmatics agreer: our SLM's labels stand unless written rules, on the evidence of open models
//! that agree with each other, say otherwise — like `tools/treeagree` for trees. Witnesses (`witness.py`, GPU): two
//! zero-shot NLI models (deberta tasksource, cross-encoder deberta) for act, polarity and uncertainty; a certainty
//! regressor; polite-guard and an irony model are reported only (one witness each never changes a label).
//!
//! Rules (first match per field; measured on cycle round 6, 2 088 school sentences):
//! - P0 tree-form: the sentence form comes from the tree, not the SLM (`tree_form`: «?» → q or wh, a root base-form
//!   verb with no subject opening the sentence → imp, «!» → excl, else decl), except for character voice: direct
//!   speech is labelled by the speech itself. The SLM called declaratives «imp».
//! - P1 imp-request: the tree says imperative and the act is narrate/assert/evaluate/other → request (warn if both
//!   NLI say warn).
//! - P2 lesson-assert: outside the books course, narrate → assert when both NLI say assert (p ≥ 0.2 each). The SLM
//!   learned on tales and calls lesson facts narration; in the books narration is real and stays.
//!
//! Switched off after a spot check (PRAG_AGREE_ALL=1 turns them on, for measuring): P3 evaluate, P4 polarity and
//! P5 hedge — the zero-shot witnesses agreed with each other there, but on the wrong answer (neutral facts as
//! «negative», plain statements as «uncertain»).

use std::collections::BTreeMap;
use std::fmt::Write as _;

/// A TSV with a header → rows as column maps, keyed by the first column.
pub fn table(text: &str) -> BTreeMap<String, BTreeMap<String, String>> {
    let mut lines = text.lines();
    let head: Vec<&str> = lines.next().unwrap_or("").split('\t').collect();
    lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            let m: BTreeMap<String, String> = head.iter().zip(c.iter()).map(|(h, v)| (h.to_string(), v.to_string())).collect();
            (c[0].to_string(), m)
        })
        .collect()
}

fn p(w: &BTreeMap<String, String>, k: &str) -> f64 {
    w.get(k).and_then(|x| x.parse().ok()).unwrap_or(0.0)
}

/// Agreed labels for one sentence: (form, act, polarity, hedge, rules applied).
pub fn agree_one(id: &str, l: &BTreeMap<String, String>, w: &BTreeMap<String, String>, tree: Option<&str>) -> (String, String, String, String, Vec<&'static str>) {
    let g = |k: &str| l.get(k).cloned().unwrap_or_default();
    let ws = |k: &str| w.get(k).map(String::as_str).unwrap_or("");
    let (mut form, mut act, mut pol, mut hedge) = (g("form"), g("act"), g("polarity"), g("hedge"));
    let mut rules = Vec::new();
    let all = std::env::var("PRAG_AGREE_ALL").as_deref() == Ok("1");
    // direct speech is labelled by the speech itself (the schema): the tree of «"Are you afraid?" whispers the Mole.»
    // is a declarative frame, the act is the question — for character voice the SLM's form stands
    if let Some(t) = tree.filter(|_| g("voice") != "character") {
        if t != form && matches!(form.as_str(), "decl" | "q" | "wh" | "imp" | "excl") {
            form = t.to_string();
            rules.push("P0-tree-form");
        }
    }
    let both = |a: &str, x: &str, th: f64| ws(&format!("nli1_{a}")) == x && ws(&format!("nli2_{a}")) == x && p(w, &format!("nli1_{a}_p")) >= th && p(w, &format!("nli2_{a}_p")) >= th;
    if form == "imp" && matches!(act.as_str(), "narrate" | "assert" | "evaluate" | "other") {
        act = if both("act", "warn", 0.0) { "warn".into() } else { "request".into() };
        rules.push("P1-imp-request");
    } else if act == "narrate" && !id.contains(":books:") && both("act", "assert", 0.2) {
        act = "assert".into();
        rules.push("P2-lesson-assert");
    } else if all && matches!(act.as_str(), "narrate" | "assert") && both("act", "evaluate", 0.3) {
        act = "evaluate".into();
        rules.push("P3-evaluate");
    }
    if all && pol == "none" && act == "evaluate" {
        for x in ["pos", "neg"] {
            if both("pol", x, 0.8) {
                pol = x.into();
                rules.push("P4-polarity");
            }
        }
    }
    if all && hedge == "0" && both("unc", "1", 0.6) && p(w, "certainty") < 4.3 && w.contains_key("certainty") {
        hedge = "1".into();
        rules.push("P5-hedge");
    }
    (form, act, pol, hedge, rules)
}

/// The sentence form from a CoNLL-U block: «?» → wh (a wh-word first) or q; a root base-form verb (VB) with no
/// subject or expletive and no auxiliary before it → imp; «!» → excl; else decl.
pub fn tree_form(block: &str) -> &'static str {
    let rows: Vec<Vec<&str>> = block.lines().filter(|l| !l.starts_with('#') && !l.is_empty()).map(|l| l.split('\t').collect()).filter(|c: &Vec<&str>| c.len() >= 8 && !c[0].contains('-') && !c[0].contains('.')).collect();
    let words: Vec<&Vec<&str>> = rows.iter().filter(|c| c[3] != "PUNCT").collect();
    let last = rows.iter().rev().find(|c| c[3] == "PUNCT").map(|c| c[1]).unwrap_or("");
    if last == "?" || block.trim_end().ends_with('?') {
        return if words.first().is_some_and(|c| matches!(c[4], "WDT" | "WP" | "WP$" | "WRB")) { "wh" } else { "q" };
    }
    if let Some(root) = rows.iter().find(|c| c[6] == "0") {
        let id = root[0];
        let has_subj = rows.iter().any(|c| c[6] == id && (c[7].starts_with("nsubj") || c[7].starts_with("csubj") || c[7] == "expl"));
        let aux_before = rows.iter().any(|c| c[6] == id && c[7].starts_with("aux") && c[0].parse::<usize>().unwrap_or(0) < id.parse().unwrap_or(0));
        // the verb opens the sentence (after at most a few softeners): «Look out.», «Please sit.», «Never touch…»
        let lead: Vec<&str> = words.iter().take_while(|c| c[0] != id).map(|c| c[2]).collect();
        let soft = lead.iter().all(|l| matches!(l.to_lowercase().as_str(), "please" | "just" | "never" | "always" | "do" | "don't" | "not" | "now" | "then" | "so" | "and" | "but" | "first" | "also"));
        if root[4] == "VB" && !has_subj && !aux_before && lead.len() <= 2 && soft {
            return "imp";
        }
    }
    if last == "!" {
        return "excl";
    }
    "decl"
}

/// `prag agree <labels.tsv> <witness.tsv> <trees.conllu> <out.tsv>` → agreed labels (+ the rules and the info-only
/// witnesses), and a report of what changed.
pub fn run(labels: &str, witness: &str, trees: &str) -> (String, String) {
    let l = table(labels);
    let w = table(witness);
    let forms: BTreeMap<String, &'static str> = trees
        .split("\n\n")
        .filter_map(|b| b.lines().find_map(|x| x.strip_prefix("# sent_id = ")).map(|id| (id.to_string(), tree_form(b))))
        .collect();
    let mut out = String::from("sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tagree\tpolite\tirony\n");
    let mut by_rule: BTreeMap<&str, usize> = BTreeMap::new();
    let mut moves: BTreeMap<String, usize> = BTreeMap::new();
    let mut n = 0;
    for (id, lr) in &l {
        let empty = BTreeMap::new();
        let wr = w.get(id).unwrap_or(&empty);
        let (form, act, pol, hedge, rules) = agree_one(id, lr, wr, forms.get(id).copied());
        for r in &rules {
            *by_rule.entry(r).or_default() += 1;
        }
        if act != lr["act"] {
            *moves.entry(format!("act {} → {act}", lr["act"])).or_default() += 1;
        }
        if form != lr["form"] {
            *moves.entry(format!("form {} → {form}", lr["form"])).or_default() += 1;
        }
        n += 1;
        let _ = writeln!(
            out,
            "{id}\t{form}\t{act}\t{}\t{hedge}\t{pol}\t{}\t{}\t{}\t{}",
            lr.get("indirect").map(String::as_str).unwrap_or("0"),
            lr.get("voice").map(String::as_str).unwrap_or("none"),
            if rules.is_empty() { "-".to_string() } else { rules.join(",") },
            wr.get("polite").map(String::as_str).unwrap_or("-"),
            wr.get("irony").map(String::as_str).unwrap_or("-"),
        );
    }
    let mut rep = format!("# Pragmatics agreement\n\n{n} sentences.\n\n| rule | sentences |\n|---|---|\n");
    for (r, c) in &by_rule {
        let _ = writeln!(rep, "| {r} | {c} |");
    }
    let _ = writeln!(rep, "\n| change | sentences |\n|---|---|");
    let mut mv: Vec<_> = moves.into_iter().collect();
    mv.sort_by(|a, b| b.1.cmp(&a.1));
    for (m, c) in mv {
        let _ = writeln!(rep, "| {m} | {c} |");
    }
    (out, rep)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
    }

    #[test]
    fn rules_need_agreeing_witnesses() {
        let l = row(&[("form", "decl"), ("act", "narrate"), ("polarity", "none"), ("hedge", "0")]);
        let w = row(&[("nli1_act", "assert"), ("nli1_act_p", "0.3"), ("nli2_act", "assert"), ("nli2_act_p", "0.25"), ("certainty", "5.0")]);
        assert_eq!(agree_one("school:biology:001-x.lesson:1", &l, &w, None).1, "assert");
        // negative controls: in a book narration stays; one witness alone changes nothing
        assert_eq!(agree_one("school:books:041-willows-ch5.lesson:1", &l, &w, None).1, "narrate");
        let w1 = row(&[("nli1_act", "assert"), ("nli1_act_p", "0.9"), ("nli2_act", "evaluate"), ("nli2_act_p", "0.9")]);
        assert_eq!(agree_one("school:biology:001-x.lesson:1", &l, &w1, None).1, "narrate");
        // the tree's imperative wins over the witnesses; the SLM's «imp» on a declarative tree does not
        let imp = row(&[("form", "imp"), ("act", "assert"), ("polarity", "none"), ("hedge", "0")]);
        assert_eq!(agree_one("school:everyday:001-x.lesson:1", &imp, &w, Some("imp")).1, "request");
        let d = agree_one("school:everyday:001-x.lesson:1", &imp, &w, Some("decl"));
        assert_eq!((d.0.as_str(), d.1.as_str()), ("decl", "assert"));
    }

    #[test]
    fn form_from_the_tree() {
        let imp = "# sent_id = a\n1\tLook\tlook\tVERB\tVB\t_\t0\troot\t_\t_\n2\tout\tout\tADP\tRP\t_\t1\tcompound:prt\t_\t_\n3\t.\t.\tPUNCT\t.\t_\t1\tpunct\t_\t_\n";
        assert_eq!(tree_form(imp), "imp");
        let decl = "# sent_id = b\n1\tSpiders\tspider\tNOUN\tNNS\t_\t3\tnsubj\t_\t_\n2\tare\tbe\tAUX\tVBP\t_\t3\tcop\t_\t_\n3\tarachnids\tarachnid\tNOUN\tNNS\t_\t0\troot\t_\t_\n";
        assert_eq!(tree_form(decl), "decl");
        let q = "# sent_id = c\n1\tWhy\twhy\tADV\tWRB\t_\t3\tadvmod\t_\t_\n2\tdo\tdo\tAUX\tVBP\t_\t3\taux\t_\t_\n3\tgo\tgo\tVERB\tVB\t_\t0\troot\t_\t_\n4\t?\t?\tPUNCT\t.\t_\t3\tpunct\t_\t_\n";
        assert_eq!(tree_form(q), "wh");
    }
}
