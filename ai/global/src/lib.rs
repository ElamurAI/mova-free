//! Global level of the MMM (small language model) — compiled static links (`train/README.md`): values,
//! concepts, shortcuts between them, verb classes. Source: the seeds `seeds/**/*.md`; `build.rs` stitches them
//! into tables, so nothing is read from disk at runtime. Every conclusion says which link and file it came from.

pub struct PrincipleDef {
    pub id: &'static str,
    pub name: &'static str,
    pub gist: &'static str,
    pub against: &'static [&'static str],
    pub towards: &'static [&'static str],
    pub file: &'static str,
    /// predicate over the story world (evaluated by the context — `world::values`)
    pub rule: &'static str,
}

pub struct ConceptDef {
    pub name: &'static str,
    pub words: &'static [&'static str],
    pub file: &'static str,
    /// category (countable things, measures, time units): explains steps but is not an event itself — starts no consequence chains
    pub category: bool,
}

pub struct LinkDef {
    pub from: usize,
    pub to: usize,
    pub why: &'static str,
    pub file: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/global.rs"));

/// A word of the text matches the lemma: simple forms and consonant doubling (grab → grabbed).
pub fn has_word(text: &str, w: &str) -> bool {
    let t = format!(" {} ", text.to_lowercase().replace(|c: char| !c.is_alphanumeric() && c != ' ', " "));
    let dbl = w.chars().last().filter(|c| !"aeiouwy".contains(*c)).map(|c| format!("{w}{c}")).unwrap_or_default();
    let ystem = w.strip_suffix('y').map(|s| format!("{s}ie")).unwrap_or_default();
    let mut forms = vec![w.to_string(), format!("{w}s"), format!("{w}es"), format!("{w}ed"), format!("{w}d"), format!("{w}ing")];
    if !dbl.is_empty() {
        forms.push(format!("{dbl}ed"));
        forms.push(format!("{dbl}ing"));
    }
    if !ystem.is_empty() {
        forms.push(format!("{ystem}s"));
        forms.push(format!("{ystem}d"));
    }
    // irregular forms needed by the seeds
    let irregular: &[(&str, &[&str])] = &[("fall", &["fell", "fallen"]), ("hit", &["hit"]), ("eat", &["ate", "eaten"]), ("give", &["gave", "given"]), ("steal", &["stole", "stolen"]), ("lie", &["lied", "lying"]), ("bite", &["bit", "bitten"]), ("forgive", &["forgave", "forgiven"])];
    if let Some((_, fs)) = irregular.iter().find(|(k, _)| *k == w) {
        forms.extend(fs.iter().map(|s| s.to_string()));
    }
    forms.iter().any(|f| t.contains(&format!(" {f} ")))
}

pub fn concept(name: &str) -> Option<usize> {
    CONCEPTS.iter().position(|c| c.name == name)
}

/// Concepts the text touches (by trigger words).
pub fn concepts_in(text: &str) -> Vec<usize> {
    (0..CONCEPTS.len()).filter(|&i| CONCEPTS[i].words.iter().any(|w| has_word(text, w))).collect()
}

/// Shortcut: the shortest chain of links from concept to concept (breadth-first search).
pub fn path(from: usize, to: usize) -> Option<Vec<&'static LinkDef>> {
    let mut prev: Vec<Option<usize>> = vec![None; CONCEPTS.len()];
    let mut seen = vec![false; CONCEPTS.len()];
    let mut q = std::collections::VecDeque::from([from]);
    seen[from] = true;
    while let Some(c) = q.pop_front() {
        if c == to {
            let mut out = Vec::new();
            let mut x = to;
            while let Some(li) = prev[x] {
                out.push(&LINKS[li]);
                x = LINKS[li].from;
            }
            out.reverse();
            return Some(out);
        }
        for (li, l) in LINKS.iter().enumerate() {
            if l.from == c && !seen[l.to] {
                seen[l.to] = true;
                prev[l.to] = Some(li);
                q.push_back(l.to);
            }
        }
    }
    None
}

/// Consequences of a text: from each touched concept, all chains forward, with explanations.
pub fn consequences(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let found: Vec<usize> = concepts_in(text).into_iter().filter(|&c| !CONCEPTS[c].category).collect();
    for &c in &found {
        // only chain starts: concepts not reachable from another found concept
        if found.iter().any(|&o| o != c && path(o, c).is_some()) {
            continue;
        }
        let mut cur = c;
        let mut chain = vec![CONCEPTS[c].name.to_string()];
        let mut why = Vec::new();
        for _ in 0..8 {
            let Some(l) = LINKS.iter().find(|l| l.from == cur) else { break };
            chain.push(CONCEPTS[l.to].name.to_string());
            why.push(l.why);
            cur = l.to;
        }
        if chain.len() > 1 {
            out.push(format!("{} — {}", chain.join(" → "), why.join("; ")));
        }
    }
    out
}

#[derive(Clone, Debug)]
pub struct Judgment {
    pub principle: &'static str,
    pub name: &'static str,
    /// true — the action goes against the principle; false — it is in its spirit
    pub violated: bool,
    pub because: String,
}

/// Self-description of a module of the snake from level 1: what it does and where it leads (chain of links from the module's concept).
pub fn self_describe(module: &str) -> Vec<&'static str> {
    let Some(c) = concept(module) else { return Vec::new() };
    let mut out = Vec::new();
    let mut cur = c;
    for _ in 0..4 {
        let Some(l) = LINKS.iter().find(|l| l.from == cur) else { break };
        out.push(l.why);
        cur = l.to;
    }
    out
}

/// Property of a relation from ```relation: `relation("north_of", "inverse")` → `Some("south_of")`.
pub fn relation(name: &str, key: &str) -> Option<&'static str> {
    RELATIONS.iter().find(|(n, _)| *n == name).and_then(|(_, kv)| kv.iter().find(|(k, _)| *k == key).map(|(_, v)| *v))
}

/// Relations that have the key `key`.
pub fn relations_with(key: &str) -> Vec<(&'static str, &'static str)> {
    RELATIONS.iter().filter_map(|(n, kv)| kv.iter().find(|(k, _)| *k == key).map(|(_, v)| (*n, *v))).collect()
}

/// Words of a concept or category (empty if none).
pub fn concept_words(name: &str) -> &'static [&'static str] {
    CONCEPTS.iter().find(|c| c.name == name).map(|c| c.words).unwrap_or(&[])
}

/// What a thing is usually for (ATOMIC ObjectUse, `use_<action>`): knife → cut, apple → fall.
pub fn uses(noun: &str) -> Vec<&'static str> {
    CONCEPTS.iter().filter(|c| c.name.starts_with("use_") && c.words.contains(&noun)).map(|c| &c.name[4..]).collect()
}

/// What a creature can usually do (ATOMIC CapableOf, `can_<action>`): bird → fly.
pub fn capable(noun: &str) -> Vec<&'static str> {
    CONCEPTS.iter().filter(|c| c.name.starts_with("can_") && c.words.contains(&noun)).map(|c| &c.name[4..]).collect()
}

/// Principle by predicate name.
pub fn principle_by_rule(rule: &str) -> Option<&'static PrincipleDef> {
    PRINCIPLES.iter().find(|p| p.rule == rule)
}

/// Whether a state belongs to a concept (the concept's words are world states, not text words).
pub fn state_is(concept_name: &str, state: &str) -> bool {
    concept(concept_name).is_some_and(|c| CONCEPTS[c].words.contains(&state))
}

/// v0, only for text without a world: the triggers are verbs. Real judgment uses predicates over the story world
/// (`world::values`): story logic must not cling to words.
pub fn judge(text: &str) -> Vec<Judgment> {
    let mut out = Vec::new();
    for p in PRINCIPLES {
        for w in p.against {
            if has_word(text, w) {
                out.push(Judgment { principle: p.id, name: p.name, violated: true, because: format!("\"{w}\" — against the principle \"{}\": {} ({})", p.name, p.gist, p.file) });
            }
        }
        for w in p.towards {
            if has_word(text, w) {
                out.push(Judgment { principle: p.id, name: p.name, violated: false, because: format!("\"{w}\" — in the spirit of the principle \"{}\": {} ({})", p.name, p.gist, p.file) });
            }
        }
    }
    out
}

/// The MMM's report on its own work on a query — for self-assessment by the same level-1 principles.
#[derive(Clone, Debug, Default)]
pub struct SelfReport {
    /// how many branches were considered (beam, top-K)
    pub branches: usize,
    /// the answer passed independent verification by the core
    pub verified: bool,
    /// confidence 0..1 (margin over the second branch)
    pub confidence: f64,
    /// did not answer because not confident — filed a report (what it tried, judgments, what to teach it); does not call the LLM
    pub escalated: bool,
    pub secs: f64,
    /// usual time for such a query — to know whether "I am slow"
    pub usual_secs: f64,
    pub correct: Option<bool>,
    /// actions performed (expanded search states) and the usual count; process memory, KB
    pub actions: usize,
    pub usual_actions: usize,
    pub rss_kb: u64,
    /// rethought the task in a second pass (first and second opinions diverged)
    pub rethought: bool,
}

/// Self-assessment. Design note: level 3 should understand that more of its own branches means perseverance,
/// a better answer means quality and diligence, and a slow result means "I am slow". The principles come from level 1,
/// the same ones the MMM uses to judge fable characters; here the predicates are over its own actions.
pub fn self_judge(r: &SelfReport) -> Vec<Judgment> {
    let mut out = Vec::new();
    let mut push = |rule: &str, violated: bool, because: String| {
        if let Some(p) = principle_by_rule(rule) {
            out.push(Judgment { principle: p.id, name: p.name, violated, because });
        }
    };
    if r.branches >= 5 {
        push("steady_effort", false, format!("considered {} branches, did not stop at the first", r.branches));
    } else if r.branches <= 1 && !r.verified {
        push("steady_effort", true, "one branch without verification — gave up at the first".into());
    }
    if r.verified {
        push("self_quality", false, "two independent opinions agreed on the answer".into());
    }
    if r.escalated && r.confidence < 0.5 {
        push("scorn_then_fall", false, format!("confidence {:.2} — I do not make things up, I file a report: what I tried, my judgments, what to teach me", r.confidence));
    } else if !r.escalated && r.confidence < 0.2 && r.correct == Some(false) {
        push("scorn_then_fall", true, format!("answered with confidence {:.2} and was wrong — should have filed a report instead of making things up", r.confidence));
    }
    let slow_t = r.usual_secs > 0.0 && r.secs > 3.0 * r.usual_secs;
    let slow_a = r.usual_actions > 0 && r.actions > 3 * r.usual_actions;
    if (slow_t || slow_a) && r.correct != Some(false) {
        push("steady_effort", false, format!("I am slow: {:.2} s and {} actions instead of the usual {:.2} s and {} (memory {} MB), but I got there", r.secs, r.actions, r.usual_secs, r.usual_actions, r.rss_kb / 1024));
    }
    if r.rethought {
        push("steady_effort", false, "the first answer was a signal: rethought the task in a second pass with different constraints".into());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newton_apple() {
        let c = consequences("An apple fell on Newton's head.");
        assert!(c.iter().any(|x| x.starts_with("fall → accelerate → impact → pain → harm")), "{c:?}");
        let p = path(concept("fall").unwrap(), concept("harm").unwrap()).unwrap();
        assert_eq!(p.len(), 4);
        // negative control: an apple lying on the table — no fall, no harm
        assert!(consequences("An apple lies on the table.").is_empty());
    }

    #[test]
    fn verbs_and_units() {
        assert_eq!(verb_class("eat"), Some(("lose", '-')));
        assert_eq!(verb_class("buy"), Some(("get", '+')));
        assert_eq!(verb_class("sell"), Some(("give", '±')));
        assert_eq!(verb_class("zzz"), None);
        let p = path(concept("lose").unwrap(), concept("subtract").unwrap()).unwrap();
        assert_eq!(p.len(), 2);
    }

    #[test]
    fn uses_and_capable() {
        assert!(uses("knife").contains(&"cut"), "{:?}", uses("knife"));
        assert!(uses("apple").contains(&"fall"));
        assert!(capable("bird").contains(&"fly"));
        // negative control: a table does not cut, an apple does not fly

        assert!(!uses("table").contains(&"cut"));
        assert!(capable("apple").is_empty());
    }

    #[test]
    fn values_from_fables() {
        let j = judge("The fox flattered the crow and grabbed the cheese.");
        assert!(j.iter().any(|x| x.principle == "truth" && x.violated));
        assert!(j.iter().any(|x| x.principle == "justice" && x.violated));
        let j = judge("The mouse helped the lion and he thanked her.");
        assert!(j.iter().all(|x| !x.violated) && j.iter().any(|x| x.principle == "help-neighbour"));
        assert!(judge("The crow sat on a branch.").is_empty());
    }
}
