//! Global level of the SLM (small language model) — compiled static links (`train/README.md`): values,
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

/// A selectional-preference list from ```selection (`seeds/shortcuts/selection.md`); empty if none.
pub fn selection(name: &str) -> &'static [&'static str] {
    SELECTION.iter().find(|(n, _)| *n == name).map(|(_, ws)| *ws).unwrap_or(&[])
}

/// Categories (```category) whose word list contains the word, in table order (deterministic).
pub fn categories_of(word: &str) -> Vec<&'static str> {
    CONCEPTS.iter().filter(|c| c.category && c.words.contains(&word)).map(|c| c.name).collect()
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

/// The SLM's report on its own work on a query — for self-assessment by the same level-1 principles.
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
/// the same ones the SLM uses to judge fable characters; here the predicates are over its own actions.
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
    fn selection_lists() {
        assert!(selection("agent_animate").contains(&"eat"));
        assert!(selection("animate_cats").contains(&"wd_animal"));
        assert!(categories_of("cheese").contains(&"wd_food"));
        // negative control: unknown list and unknown word
        assert!(selection("zzz").is_empty());
        assert!(categories_of("zzzz").is_empty());
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

/// The hot layer of level-1 knowledge: files in the folder named by `GLOBAL_HOT`, read once at start-up, override
/// or extend the cold tables compiled into the binary — new knowledge tried without recompiling; it moves into the
/// cold layer (`data/`) only after the gates. Files (all optional, same formats as `data/`):
/// `absurdity-roles.tsv`, `absurdity-roles-tale.tsv` (verb, noun, SUBJ, OBJ) and `idioms.tsv` (verb, object, kind,
/// …; kind `none` removes a cold entry).
struct Hot {
    real: std::collections::HashMap<(String, String), [u8; 2]>,
    tale: std::collections::HashMap<(String, String), [u8; 2]>,
    idioms: std::collections::HashMap<(String, String), &'static str>,
}

fn hot() -> &'static Hot {
    static H: std::sync::OnceLock<Hot> = std::sync::OnceLock::new();
    H.get_or_init(|| {
        let dir = std::env::var("GLOBAL_HOT").ok().map(std::path::PathBuf::from);
        let read = |name: &str| dir.as_ref().and_then(|d| std::fs::read_to_string(d.join(name)).ok()).unwrap_or_default();
        let cells = |t: String| {
            t.lines()
                .filter(|l| !l.starts_with('#'))
                .filter_map(|l| {
                    let c: Vec<&str> = l.split('\t').collect();
                    if c.len() < 4 {
                        return None;
                    }
                    let (s, o): (u8, u8) = (c[2].parse().ok()?, c[3].parse().ok()?);
                    Some(((c[0].to_string(), c[1].to_string()), [s.min(4), o.min(4)]))
                })
                .collect()
        };
        let idioms = read("idioms.tsv")
            .lines()
            .filter(|l| !l.starts_with('#'))
            .filter_map(|l| {
                let c: Vec<&str> = l.split('\t').collect();
                let kind: &'static str = match *c.get(2)? {
                    "idiom" => "idiom",
                    "light-verb" => "light-verb",
                    "collocation" => "collocation",
                    "phrasal-verb" => "phrasal-verb",
                    "candidate" => "candidate",
                    _ => "none",
                };
                Some(((c[0].to_string(), c[1].to_string()), kind))
            })
            .collect();
        Hot { real: cells(read("absurdity-roles.tsv")), tale: cells(read("absurdity-roles-tale.tsv")), idioms }
    })
}

/// Absurdity matrix (real-world scale, `data/absurdity-roles.tsv`, `docs/absurdity.md`): `[SUBJ, OBJ]` scores 0–4
/// of `noun` as the doer / the direct object of `verb`; `None` if the cell is unknown. The hot layer comes first.
pub fn absurdity(verb: &str, noun: &str) -> Option<[u8; 2]> {
    if let Some(v) = hot().real.get(&(verb.to_string(), noun.to_string())) {
        return Some(*v);
    }
    let v = ABSURD_VERBS.binary_search(&verb).ok()? as u16;
    let n = ABSURD_NOUNS.binary_search(&noun).ok()? as u16;
    ABSURD_CELLS.binary_search_by(|c| (c.0, c.1).cmp(&(v, n))).ok().map(|i| [ABSURD_CELLS[i].2, ABSURD_CELLS[i].3])
}

/// Absurdity in a domain: `"tale"` uses the fairy-tale layer where it has the cell, otherwise the real-world
/// table; any other domain (`"real"`) is the real-world table.
pub fn absurdity_in(domain: &str, verb: &str, noun: &str) -> Option<[u8; 2]> {
    if domain == "tale" {
        if let Some(v) = hot().tale.get(&(verb.to_string(), noun.to_string())) {
            return Some(*v);
        }
        let v = ABSURD_VERBS.binary_search(&verb).ok()? as u16;
        let n = ABSURD_NOUNS.binary_search(&noun).ok()? as u16;
        if let Ok(i) = ABSURD_TALE_CELLS.binary_search_by(|c| (c.0, c.1).cmp(&(v, n))) {
            return Some([ABSURD_TALE_CELLS[i].2, ABSURD_TALE_CELLS[i].3]);
        }
    }
    absurdity(verb, noun)
}

/// Is the verb a row of the absurdity matrix?
pub fn absurd_has_verb(verb: &str) -> bool {
    ABSURD_VERBS.binary_search(&verb).is_ok() || hot().real.keys().any(|(v, _)| v == verb)
}

/// Is the noun a column of the absurdity matrix?
pub fn absurd_has_noun(noun: &str) -> bool {
    ABSURD_NOUNS.binary_search(&noun).is_ok() || hot().real.keys().any(|(_, n)| n == noun)
}

#[cfg(test)]
mod absurd_tests {
    #[test]
    fn matrix_cells() {
        // the fox eats, the cheese is eaten; the cheese does not eat
        assert_eq!(super::absurdity("eat", "fox").map(|s| s[0]), Some(0));
        assert_eq!(super::absurdity("eat", "cheese").map(|s| s[0]), Some(4));
        assert_eq!(super::absurdity("eat", "cheese").map(|s| s[1]), Some(0));
        // negative control: unknown words give no cell
        assert_eq!(super::absurdity("eat", "zorb"), None);
        assert!(!super::absurd_has_verb("gnaw"));
        assert!(super::absurd_has_noun("crow"));
        // the fairy-tale layer: a fox may talk, cheese still does not eat
        assert!(super::absurdity_in("tale", "say", "fox").unwrap()[0] < super::absurdity_in("real", "say", "fox").unwrap()[0]);
        assert_eq!(super::absurdity_in("tale", "eat", "cheese").map(|s| s[0]), Some(4));
    }
}

/// Is the verb intransitive by the absurdity matrix: the OBJ score is 4 for at least 80% of the nouns in its row ("live" 84%: "live a life"; "eat" 57%)?
pub fn absurd_intransitive(verb: &str) -> bool {
    let Ok(v) = ABSURD_VERBS.binary_search(&verb) else { return false };
    let v = v as u16;
    let lo = ABSURD_CELLS.partition_point(|c| c.0 < v);
    let hi = ABSURD_CELLS.partition_point(|c| c.0 <= v);
    let row = &ABSURD_CELLS[lo..hi];
    !row.is_empty() && row.iter().filter(|c| c.3 == 4).count() * 10 >= row.len() * 8
}

#[cfg(test)]
mod intransitive_tests {
    #[test]
    fn go_is_intransitive_eat_is_not() {
        assert!(super::absurd_intransitive("go"));
        assert!(super::absurd_intransitive("live"));
        // negative control
        assert!(!super::absurd_intransitive("eat"));
        assert!(!super::absurd_intransitive("zorb"));
    }
}

/// The kind of a verbal multiword expression ("idiom", "light-verb", "collocation", "phrasal-verb") for a verb and
/// its object or particle (`data/idioms.tsv`); `None` if the pair is not a known expression.
pub fn idiom(verb: &str, object: &str) -> Option<&'static str> {
    if let Some(k) = hot().idioms.get(&(verb.to_string(), object.to_string())) {
        return (*k != "none").then_some(*k);
    }
    IDIOMS.binary_search_by(|x| (x.0, x.1).cmp(&(verb, object))).ok().map(|i| IDIOMS[i].2)
}

#[cfg(test)]
mod idiom_tests {
    #[test]
    fn known_expressions() {
        assert_eq!(super::idiom("take", "place"), Some("idiom"));
        assert_eq!(super::idiom("open", "fire"), Some("idiom"));
        assert_eq!(super::idiom("pay", "attention"), Some("idiom"));
        assert_eq!(super::idiom("give", "up"), Some("phrasal-verb"));
        // negative control
        assert_eq!(super::idiom("eat", "table"), None);
    }
}

/// One entry of the expression base (`data/expressions.tsv`, CC BY-SA, Wiktionary): what an expression really means.
#[derive(Clone, Debug)]
pub struct Expression {
    pub phrase: &'static str,
    /// idiom | proverb | phrase | phrasal-verb | verb-phrase | multiword | word
    pub kind: &'static str,
    pub pos: &'static str,
    pub meaning: &'static str,
    /// register labels with the meaning of the labelled sense ("slang: …; humorous: …")
    pub register: &'static str,
    /// regional varieties with the meaning of the labelled sense ("UK: …; Australia: …")
    pub variety: &'static str,
}

fn expression_base() -> &'static std::collections::BTreeMap<&'static str, Vec<Expression>> {
    static BASE: std::sync::OnceLock<std::collections::BTreeMap<&'static str, Vec<Expression>>> = std::sync::OnceLock::new();
    BASE.get_or_init(|| {
        let mut m: std::collections::BTreeMap<&'static str, Vec<Expression>> = std::collections::BTreeMap::new();
        for l in include_str!("../data/expressions.tsv").lines().filter(|l| !l.starts_with('#')) {
            let c: Vec<&'static str> = l.split('\t').collect();
            if c.len() >= 4 {
                m.entry(c[0]).or_default().push(Expression { phrase: c[0], kind: c[1], pos: c[2], meaning: c[3], register: c.get(4).copied().unwrap_or(""), variety: c.get(5).copied().unwrap_or("") });
            }
        }
        m
    })
}

/// Entries for an exact phrase (lowercase, words separated by single spaces).
pub fn expression(phrase: &str) -> &'static [Expression] {
    expression_base().get(phrase).map(Vec::as_slice).unwrap_or(&[])
}

/// The domain a register label switches on: slang, formal, humour, hidden meaning.
pub fn register_domain(label: &str) -> Option<&'static str> {
    match label {
        "slang" | "internet slang" | "informal" | "colloquial" | "vulgar" | "derogatory" | "offensive" | "nonstandard" => Some("slang"),
        "formal" | "literary" | "archaic" | "legal" | "law" | "officialese" | "bureaucratese" | "poetic" => Some("formal"),
        "humorous" | "jocular" | "ironic" | "sarcastic" => Some("humor"),
        "euphemistic" | "euphemism" | "figurative" | "figuratively" | "idiomatic" => Some("hidden-meaning"),
        _ => None,
    }
}

/// A found expression: word span `[start, end)`, the entry, and the domains its labels switch on.
#[derive(Clone, Debug)]
pub struct ExprMatch {
    pub start: usize,
    pub end: usize,
    pub entry: Expression,
    pub domains: Vec<&'static str>,
}

/// Expressions in a sentence given its lowercase forms and lemmas: multiword matches on forms ("spill the beans")
/// and on lemmas ("kicked the bucket" → kick the bucket), non-overlapping, longest first.
pub fn expressions_in_sentence(forms: &[&str], lemmas: &[&str]) -> Vec<ExprMatch> {
    let mut out = expressions_in(lemmas);
    for m in expressions_in(forms) {
        if m.end - m.start > 1 && !out.iter().any(|x| x.end - x.start > 1 && x.start < m.end && m.start < x.end) {
            out.push(m);
        }
    }
    out.sort_by_key(|m| (m.start, std::cmp::Reverse(m.end)));
    out
}

/// Expressions in a lemma sequence, longest first, non-overlapping for multiword ones: "one's" matches a possessive
/// (my, your, his, her, its, our, their), "someone"/"something" match any one word. Single words count only with
/// a register label (they switch on a domain).
pub fn expressions_in(lemmas: &[&str]) -> Vec<ExprMatch> {
    const POSS: [&str; 7] = ["my", "your", "his", "her", "its", "our", "their"];
    let lower: Vec<String> = lemmas.iter().map(|w| w.to_lowercase()).collect();
    let mut out = Vec::new();
    let mut used = vec![false; lower.len()];
    for len in (1..=6).rev() {
        for start in 0..lower.len().saturating_sub(len - 1) {
            if used[start..start + len].iter().any(|u| *u) {
                continue;
            }
            let words = &lower[start..start + len];
            // candidate phrases: as is, and with a possessive or one placeholder generalised
            let mut cands = vec![words.join(" ")];
            for (i, w) in words.iter().enumerate() {
                let mut g = words.to_vec();
                if POSS.contains(&w.as_str()) {
                    g[i] = "one's".into();
                    cands.push(g.join(" "));
                } else if len >= 3 && i > 0 && i + 1 < len {
                    // a placeholder only inside a phrase ("give someone the slip"); at an edge it matches
                    // anything ("said good" ~ "something good")
                    for ph in ["someone", "something", "somebody"] {
                        g[i] = ph.into();
                        cands.push(g.join(" "));
                    }
                }
            }
            for c in cands {
                for e in expression(&c) {
                    if len == 1 && e.register.is_empty() && e.variety.is_empty() {
                        continue;
                    }
                    // an entry without a real gloss ("do n't" → ".") is no reading
                    if e.meaning.chars().filter(|c| c.is_alphabetic()).count() < 3 || e.meaning.contains("[[") {
                        continue;
                    }
                    // a domain is switched on only through the main sense ("kick the bucket" = to die: humorous,
                    // euphemistic); a rare labelled sense does not ("old" has a slang sense, "give up" too)
                    let labels = e.register.split("; ").filter_map(|x| x.split_once(": ")).filter(|(_, g)| *g == e.meaning).map(|(l, _)| l.trim());
                    let mut domains: Vec<&'static str> = labels.filter_map(register_domain).collect();
                    // a regional variety on the main sense switches the variety on ("variety:UK")
                    for v in e.variety.split("; ").filter_map(|x| x.split_once(": ")).filter(|(_, g)| *g == e.meaning).map(|(v, _)| v.trim()) {
                        domains.push(match v {
                            "US" => "variety:US", "UK" => "variety:UK", "Australia" => "variety:Australia", "Scotland" => "variety:Scotland",
                            "Ireland" => "variety:Ireland", "India" => "variety:India", "Canada" => "variety:Canada", "New Zealand" => "variety:New Zealand",
                            "South Africa" => "variety:South Africa", "Singapore" => "variety:Singapore", "Philippines" => "variety:Philippines",
                            "North America" => "variety:North America", "Nigeria" => "variety:Nigeria", _ => "variety:Wales",
                        });
                    }
                    if len == 1 && domains.is_empty() {
                        continue;
                    }
                    domains.sort();
                    domains.dedup();
                    out.push(ExprMatch { start, end: start + len, entry: e.clone(), domains });
                    if len > 1 {
                        used[start..start + len].iter_mut().for_each(|u| *u = true);
                    }
                    break;
                }
                if used[start] && len > 1 {
                    break;
                }
            }
        }
    }
    out.sort_by_key(|m| (m.start, std::cmp::Reverse(m.end)));
    out
}

#[cfg(test)]
mod expression_tests {
    #[test]
    fn real_meanings_and_domains() {
        let m = super::expressions_in(&["he", "kick", "the", "bucket", "yesterday"]);
        let k = m.iter().find(|x| x.entry.phrase == "kick the bucket").expect("kick the bucket");
        assert_eq!(k.entry.meaning, "To die.");
        assert!(k.domains.contains(&"humor") && k.domains.contains(&"hidden-meaning"));
        // a possessive generalises to one's
        assert!(super::expressions_in(&["she", "lose", "her", "temper"]).iter().any(|x| x.entry.phrase == "lose one's temper"));
        // negative control: a literal sentence has no multiword expression and switches on no domain
        let lit = super::expressions_in(&["the", "old", "dog", "eat", "the", "meat"]);
        assert!(lit.iter().all(|x| x.end - x.start == 1 && x.domains.is_empty()), "{lit:?}");
        assert!(lit.is_empty(), "{lit:?}");
        // forms find what lemmas lose
        assert!(super::expressions_in_sentence(&["spill", "the", "beans"], &["spill", "the", "bean"]).iter().any(|x| x.entry.phrase == "spill the beans"));
    }
}

/// Register lexicons measured on corpora (`data/register-<name>.tsv`: word, z — log-odds against the pooled other
/// corpora, Monroe et al. 2008; `world register-stats`): "legal", "tale", "children". The z of a word, if listed.
pub fn register_z(name: &str, word: &str) -> Option<f32> {
    type Lex = std::collections::BTreeMap<&'static str, f32>;
    static LEX: std::sync::OnceLock<Vec<(&'static str, Lex)>> = std::sync::OnceLock::new();
    let all = LEX.get_or_init(|| {
        let parse = |t: &'static str| -> Lex { t.lines().filter(|l| !l.starts_with('#')).filter_map(|l| l.split_once('\t')).filter_map(|(w, z)| Some((w, z.parse().ok()?))).collect() };
        vec![("legal", parse(include_str!("../data/register-legal.tsv"))), ("tale", parse(include_str!("../data/register-tale.tsv"))), ("children", parse(include_str!("../data/register-children.tsv")))]
    });
    all.iter().find(|(n, _)| *n == name).and_then(|(_, m)| m.get(word).copied())
}
