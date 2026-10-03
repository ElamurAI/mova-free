//! Three levels of SLM (small language model) logic (`train/README.md`):
//! 1. **Global** — principles and values, the exact core (mathematics; later physics, chemistry), world structure,
//!    shortcuts and links. Compiled, always on.
//! 2. **Domain modules** — the full power of one field; plugged in when the context calls for them.
//! 3. **Context** — document state and the conductor: global level → domain (coverage ≥ threshold) → LLM.
//!
//! Routing is transparent: every decision writes its scores and the threshold to the log.


use math::calc;

pub use global::{SelfReport, self_judge};

/// Operating mode. Design note: step-by-step explanations are a separate SLM mode for training, debugging, or when
/// the user asked for them in the prompt; normally just the answer, fast.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Answer,
    Explain(Why),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Why {
    Training,
    Debug,
    UserAsked,
}

/// A query to the SLM: text, mode and (later) parse.
#[derive(Clone, Debug)]
pub struct Query {
    pub text: String,
    pub mode: Mode,
}

impl Query {
    /// Mode from the user's intent: asks to explain the steps → explanation; otherwise just the answer.
    pub fn new(t: &str) -> Query {
        let l = t.to_lowercase();
        let asked = ["step by step", "explain", "show your work"].iter().any(|k| l.contains(k));
        Query { text: t.to_string(), mode: if asked { Mode::Explain(Why::UserAsked) } else { Mode::Answer } }
    }
    pub fn with_mode(mut self, m: Mode) -> Query {
        self.mode = m;
        self
    }
}

/// Who answered.
#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Global(&'static str),
    Domain(String),
    /// neither the global level nor the domains — to the LLM with the task broken down
    Escalate,
}

#[derive(Clone, Debug)]
pub struct Answer {
    pub value: String,
    pub by: Route,
    /// justification: steps, laws, principles
    pub why: Vec<String>,
}

/// Domain access level (`context.md`, "Thousands of domains").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    Public,
    Extended,
    Private,
}

/// A domain module.
pub trait Domain {
    fn id(&self) -> &str;
    fn version(&self) -> &str;
    fn access(&self) -> Access;
    /// How much the query is "mine", 0..1, and why.
    fn covers(&self, q: &Query, ctx: &Context) -> (f64, String);
    /// An answer with justification, or None — "not mine / can't do it".
    fn solve(&self, q: &Query, ctx: &Context) -> Option<Answer>;
}

/// Global level: for now exact mathematics and values; later world structure, physics, shortcuts.
pub struct Global;

impl Global {
    /// Exact core: expression → value with steps and independent verification.
    pub fn math(&self, q: &Query) -> Option<Answer> {
        let e = calc::parse(q.text.trim()).ok()?;
        let mut env = calc::Env::new(math::nt::Limits::with_secs(5.0));
        let v = calc::eval(&e, &mut env).ok()?;
        let mut why: Vec<String> = env.subs.iter().map(|s| format!("{} = {}", s.rule, s.result)).collect();
        if let Some(c) = calc::independent(&e, &env.vars, &v) {
            if !c.ok {
                return None; // independent verification disagreed — the global level does not answer
            }
            why.push(format!("check: {}", c.name));
        }
        Some(Answer { value: v.exact(), by: Route::Global("math"), why })
    }

    /// Values: which principles the situation touches (with explanation).
    pub fn principles(&self, text: &str) -> Vec<global::Judgment> {
        global::judge(text)
    }

    /// Shortcuts: consequences of the situation as chains of links (fall → impact → pain → harm).
    pub fn consequences(&self, text: &str) -> Vec<String> {
        global::consequences(text)
    }
}

/// Context: document state, domain registry, routing log.
pub struct Context {
    pub facts: Vec<String>,
    pub domains: Vec<Box<dyn Domain>>,
    pub threshold: f64,
    pub log: Vec<String>,
    pub global: Global,
}

impl Default for Context {
    fn default() -> Self {
        Context { facts: Vec::new(), domains: Vec::new(), threshold: 0.5, log: Vec::new(), global: Global }
    }
}

impl Context {
    pub fn register(&mut self, d: Box<dyn Domain>) {
        self.log.push(format!("domain {} v{} ({:?})", d.id(), d.version(), d.access()));
        self.domains.push(d);
    }

    /// Global level → best domain with coverage ≥ threshold → LLM.
    pub fn route(&mut self, q: &Query) -> Answer {
        let text = q.text.to_lowercase();
        let expr = ["step by step", "explain", "show your work"].iter().fold(text, |t, k| t.replace(k, ""));
        let qq = Query { text: expr.trim().trim_matches(|c: char| c == ':' || c == ',').trim().to_string(), mode: q.mode };
        if let Some(mut a) = self.global.math(&qq) {
            self.log.push(format!("\"{}\" → global level (math), mode {:?}", q.text, q.mode));
            if q.mode == Mode::Answer {
                a.why.clear();
            }
            return a;
        }
        let mut scored: Vec<(usize, f64, String)> = self.domains.iter().enumerate().map(|(i, d)| {
            let (s, why) = d.covers(q, self);
            (i, s, why)
        }).collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let line = scored.iter().map(|(i, s, why)| format!("{} {s:.2} ({why})", self.domains[*i].id())).collect::<Vec<_>>().join("; ");
        for (i, s, _) in &scored {
            if *s < self.threshold {
                break;
            }
            if let Some(a) = self.domains[*i].solve(q, self) {
                self.log.push(format!("\"{}\" → domain {} (coverage: {line}; threshold {:.2})", q.text, self.domains[*i].id(), self.threshold));
                return a;
            }
        }
        self.log.push(format!("\"{}\" → LLM (coverage: {}; threshold {:.2})", q.text, if line.is_empty() { "no domains".into() } else { line }, self.threshold));
        let mut why = vec!["neither the global level nor the domains took the query".to_string()];
        if !self.facts.is_empty() {
            why.push(format!("context for the LLM prompt: {}", self.facts.join("; ")));
        }
        Answer { value: String::new(), by: Route::Escalate, why }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Weather;
    impl Domain for Weather {
        fn id(&self) -> &str {
            "weather"
        }
        fn version(&self) -> &str {
            "0"
        }
        fn access(&self) -> Access {
            Access::Public
        }
        fn covers(&self, q: &Query, _: &Context) -> (f64, String) {
            let hits: Vec<&str> = ["weather", "rain", "forecast", "temperature"].into_iter().filter(|w| q.text.to_lowercase().contains(w)).collect();
            ((hits.len() as f64 / 2.0).min(1.0), format!("words: {hits:?}"))
        }
        fn solve(&self, _: &Query, _: &Context) -> Option<Answer> {
            Some(Answer { value: "rain".into(), by: Route::Domain("weather".into()), why: vec!["test domain".into()] })
        }
    }

    #[test]
    fn global_math_first() {
        let mut c = Context::default();
        c.register(Box::new(Weather));
        let a = c.route(&Query::new("12 + 6"));
        assert_eq!(a.by, Route::Global("math"));
        assert_eq!(a.value, "18");
    }

    #[test]
    fn domain_by_coverage_and_escalation() {
        let mut c = Context::default();
        c.register(Box::new(Weather));
        let a = c.route(&Query::new("What is the weather forecast for tomorrow?"));
        assert_eq!(a.by, Route::Domain("weather".into()));
        // negative control: coverage below the threshold — not a domain but the LLM; the log explains
        c.facts.push("Janet has 16 eggs".into());
        let b = c.route(&Query::new("Why did the fox praise the crow?"));
        assert_eq!(b.by, Route::Escalate);
        assert!(c.log.last().unwrap().contains("threshold"));
        assert!(b.why.iter().any(|w| w.contains("Janet")));
    }

    #[test]
    fn self_judgment_by_the_same_principles() {
        let r = SelfReport { branches: 16, verified: true, confidence: 0.9, secs: 0.4, usual_secs: 0.1, correct: Some(true), ..Default::default() };
        let j = self_judge(&r);
        assert!(j.iter().any(|x| x.principle == "perseverance" && !x.violated));
        assert!(j.iter().any(|x| x.principle == "diligence" && !x.violated));
        assert!(j.iter().any(|x| x.because.contains("I am slow")));
        let unsure = SelfReport { branches: 3, confidence: 0.1, escalated: true, ..Default::default() };
        assert!(self_judge(&unsure).iter().any(|x| x.principle == "humility" && !x.violated));
        // negative control: an overconfident mistake goes against humility
        let rash = SelfReport { branches: 1, confidence: 0.05, correct: Some(false), ..Default::default() };
        let j = self_judge(&rash);
        assert!(j.iter().any(|x| x.principle == "humility" && x.violated) && j.iter().any(|x| x.principle == "perseverance" && x.violated));
    }

    #[test]
    fn explain_mode_only_when_asked() {
        let mut c = Context::default();
        let a = c.route(&Query::new("12 + 6"));
        assert!(a.why.is_empty(), "normal mode — no explanations");
        let b = c.route(&Query::new("explain: 12 + 6"));
        assert_eq!(b.value, "18");
        assert!(!b.why.is_empty(), "the user asked — explanations present");
        let d = c.route(&Query::new("12 + 6").with_mode(Mode::Explain(Why::Debug)));
        assert!(!d.why.is_empty());
    }

    #[test]
    fn garbage_is_not_math() {
        let mut c = Context::default();
        assert_eq!(c.route(&Query::new("12 + + apples")).by, Route::Escalate);
    }
}
