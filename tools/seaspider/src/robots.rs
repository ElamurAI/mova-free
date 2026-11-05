//! robots.txt per RFC 9309: parsing, group selection for the configured product token and the
//! allow/disallow decision for a path.
//!
//! Adapted from the family Spider (`spider/src/robots.rs`): longest match wins, Allow wins a tie,
//! `*` and `$` patterns, percent-encoding canonicalised to one form, at most 500 KiB parsed. The
//! Spider's peer-crawler consent policy is not carried over; Sea Spider applies the RFC alone.
//! New here: product tokens are matched with a version suffix stripped (`MovaBot/1.0`), the
//! decision names the rule that decided it, and `/robots.txt` itself is always allowed.

use regex::Regex;

/// RFC 9309 §2.5: how many bytes of robots.txt are parsed.
pub const MAX_BYTES: usize = 500 * 1024;

struct Group {
    agents: Vec<String>,
    lines: Vec<(String, String)>,
}

/// Which group of robots.txt applies to the robot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupChoice {
    /// One or more groups name the robot's own product token.
    Own,
    /// No own group; the `*` groups apply.
    Star,
    /// Neither; nothing restricts the robot.
    None,
}

impl GroupChoice {
    pub fn as_str(self) -> &'static str {
        match self {
            GroupChoice::Own => "own",
            GroupChoice::Star => "*",
            GroupChoice::None => "none",
        }
    }
}

/// One Allow/Disallow line of the chosen groups.
#[derive(Clone, Debug)]
pub struct Rule {
    pub allow: bool,
    /// The canonical pattern as written (after percent-encoding normalisation).
    pub pattern: String,
    re: Regex,
}

/// The rules that apply to one robot.
#[derive(Clone, Debug)]
pub struct Rules {
    pub rules: Vec<Rule>,
    pub crawl_delay: Option<f64>,
    pub group: GroupChoice,
}

/// The decision for one path and the rule that made it (None: no rule matched, allowed).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Verdict {
    pub allowed: bool,
    pub rule: Option<(bool, String)>,
}

/// A parsed robots.txt: every group is kept.
pub struct Robots {
    groups: Vec<Group>,
}

/// The product token a User-agent value names: lowercased, version suffix (`/1.0`) dropped.
fn agent_token(v: &str) -> String {
    v.split('/').next().unwrap_or("").trim().to_ascii_lowercase()
}

impl Robots {
    pub fn parse(txt: &str) -> Robots {
        let txt = txt.strip_prefix('\u{feff}').unwrap_or(txt);
        let mut cut = txt.len().min(MAX_BYTES);
        while !txt.is_char_boundary(cut) {
            cut -= 1;
        }
        let mut groups: Vec<Group> = Vec::new();
        let mut in_agents = false;
        for raw in txt[..cut].lines() {
            let line = raw.split('#').next().unwrap_or("").trim();
            let Some((k, v)) = line.split_once(':') else { continue };
            let (k, v) = (k.trim().to_ascii_lowercase(), v.trim().to_string());
            if k == "user-agent" {
                // consecutive User-agent lines open one group
                if !in_agents {
                    groups.push(Group { agents: Vec::new(), lines: Vec::new() });
                }
                in_agents = true;
                if let Some(g) = groups.last_mut() {
                    g.agents.push(agent_token(&v));
                }
            } else {
                in_agents = false;
                // lines before the first User-agent belong to no group
                if let Some(g) = groups.last_mut() {
                    g.lines.push((k, v));
                }
            }
        }
        Robots { groups }
    }

    /// Rules for a product token: all groups naming it (merged), else all `*` groups, else none.
    pub fn rules_for(&self, token: &str) -> Rules {
        let token = token.to_ascii_lowercase();
        let own: Vec<&Group> = self.groups.iter().filter(|g| g.agents.contains(&token)).collect();
        let (group, chosen) = if !own.is_empty() {
            (GroupChoice::Own, own)
        } else {
            let star: Vec<&Group> = self.groups.iter().filter(|g| g.agents.iter().any(|a| a == "*")).collect();
            if star.is_empty() {
                (GroupChoice::None, star)
            } else {
                (GroupChoice::Star, star)
            }
        };
        let mut rules = Vec::new();
        let mut crawl_delay: Option<f64> = None;
        for g in chosen {
            for (k, v) in &g.lines {
                match k.as_str() {
                    // an empty Disallow/Allow value is no rule (RFC 9309 §2.2.2)
                    "allow" | "disallow" if !v.is_empty() => {
                        let v = canon(v);
                        if let Some(re) = pattern(&v) {
                            rules.push(Rule { allow: k == "allow", pattern: v, re });
                        }
                    }
                    "crawl-delay" => {
                        if let Ok(d) = v.parse::<f64>() {
                            if d.is_finite() && d >= 0.0 {
                                crawl_delay = Some(crawl_delay.map_or(d, |x| x.max(d)));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Rules { rules, crawl_delay, group }
    }
}

impl Rules {
    pub fn allow_all() -> Rules {
        Rules { rules: Vec::new(), crawl_delay: None, group: GroupChoice::None }
    }

    /// Longest matching pattern decides; at equal length Allow wins; no match allows.
    pub fn decide(&self, path: &str) -> Verdict {
        let path = canon(path);
        // RFC 9309 §2.2.2: /robots.txt is implicitly allowed
        if path == "/robots.txt" {
            return Verdict { allowed: true, rule: None };
        }
        let mut best: Option<&Rule> = None;
        for r in &self.rules {
            if r.re.is_match(&path) {
                let better = match best {
                    None => true,
                    Some(b) => r.pattern.len() > b.pattern.len() || (r.pattern.len() == b.pattern.len() && r.allow && !b.allow),
                };
                if better {
                    best = Some(r);
                }
            }
        }
        match best {
            None => Verdict { allowed: true, rule: None },
            Some(r) => Verdict { allowed: r.allow, rule: Some((r.allow, r.pattern.clone())) },
        }
    }

    pub fn allowed(&self, path: &str) -> bool {
        self.decide(path).allowed
    }
}

/// RFC 9309 §2.2.2: `%XX` of an unreserved character is decoded, other `%xx` are upper-cased,
/// bytes outside ASCII become `%XX`.
fn canon(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = String::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c == b'%' && i + 2 < b.len() &&b[i + 1].is_ascii_hexdigit() && b[i + 2].is_ascii_hexdigit() {
            let v = u8::from_str_radix(&s[i + 1..i + 3], 16).unwrap_or(0);
            if v.is_ascii_alphanumeric() || b"-._~".contains(&v) {
                out.push(v as char);
            } else {
                out.push('%');
                out.push_str(&s[i + 1..i + 3].to_ascii_uppercase());
            }
            i += 3;
        } else if c.is_ascii() {
            out.push(c as char);
            i += 1;
        } else {
            out.push_str(&format!("%{c:02X}"));
            i += 1;
        }
    }
    out
}

/// A robots pattern as an anchored regex: `*` is any sequence, a final `$` anchors the end.
fn pattern(p: &str) -> Option<Regex> {
    let anchored = p.ends_with('$');
    let body = if anchored { &p[..p.len() - 1] } else { p };
    let mut re = String::from("^");
    for (i, part) in body.split('*').enumerate() {
        if i > 0 {
            re.push_str(".*");
        }
        re.push_str(&regex::escape(part));
    }
    if anchored {
        re.push('$');
    }
    Regex::new(&re).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rules(txt: &str) -> Rules {
        Robots::parse(txt).rules_for("MovaBot")
    }

    #[test]
    fn star_group_applies_without_own_group() {
        let r = rules("User-agent: *\nDisallow: /private\n");
        assert_eq!(r.group, GroupChoice::Star);
        assert!(!r.allowed("/private/x"));
        assert!(r.allowed("/public"));
    }

    #[test]
    fn own_group_replaces_star_and_ignores_version() {
        let t = "User-agent: *\nDisallow: /\n\nUser-agent: MovaBot/2.1\nDisallow: /tmp\n";
        let r = rules(t);
        assert_eq!(r.group, GroupChoice::Own);
        assert!(r.allowed("/page"), "own group does not inherit the * Disallow: /");
        assert!(!r.allowed("/tmp/a"));
        // negative control: another robot falls back to *
        assert!(!Robots::parse(t).rules_for("OtherBot").allowed("/page"));
    }

    #[test]
    fn grouped_user_agents_and_merged_groups() {
        let t = "User-agent: a\nUser-agent: movabot\nDisallow: /x\n\nUser-agent: MOVABOT\nDisallow: /y\n";
        let r = rules(t);
        assert!(!r.allowed("/x") && !r.allowed("/y") && r.allowed("/z"));
    }

    #[test]
    fn longest_match_and_allow_tie() {
        let r = rules("User-agent: *\nDisallow: /a\nAllow: /a/b\nDisallow: /a/b/c\n");
        assert!(!r.allowed("/a/x"));
        assert!(r.allowed("/a/b/x"));
        assert!(!r.allowed("/a/b/c/d"));
        let v = r.decide("/a/b/x");
        assert_eq!(v.rule, Some((true, "/a/b".to_string())));
        let tie = rules("User-agent: *\nDisallow: /p\nAllow: /p\n");
        assert!(tie.allowed("/p"), "equal length: Allow wins");
    }

    #[test]
    fn wildcards_and_end_anchor() {
        let r = rules("User-agent: *\nDisallow: /*.pdf$\nDisallow: /s*/q\n");
        assert!(!r.allowed("/doc/a.pdf"));
        assert!(r.allowed("/doc/a.pdf?x=1"), "$ anchors the end");
        assert!(!r.allowed("/search/q"));
        assert!(r.allowed("/q"));
    }

    #[test]
    fn crawl_delay_and_empty_disallow() {
        let r = rules("User-agent: *\nCrawl-delay: 5\nDisallow:\n");
        assert_eq!(r.crawl_delay, Some(5.0));
        assert!(r.allowed("/anything"));
    }

    #[test]
    fn robots_txt_always_allowed_and_percent_forms() {
        let r = rules("User-agent: *\nDisallow: /\n");
        assert!(r.allowed("/robots.txt"));
        assert!(!r.allowed("/"));
        let p = rules("User-agent: *\nDisallow: /%7Euser\n");
        assert!(!p.allowed("/~user/x"), "%7E and ~ are one form");
    }

    #[test]
    fn no_groups_allow_all() {
        let r = rules("# nothing\nSitemap: https://x/s.xml\n");
        assert_eq!(r.group, GroupChoice::None);
        assert!(r.allowed("/"));
    }
}
