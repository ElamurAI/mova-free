//! robots.txt and host pacing — from the very first request (project rule).
//!
//! The first request to each host is its `/robots.txt` (RFC 9309):
//! - the `User-agent` group that names our agent (`MovaLibrary`), otherwise the `*` groups;
//! - `Allow`/`Disallow`: the longest match wins, on a tie `Allow`; `*` is anything, `$` is the end;
//! - `Crawl-delay` is the pause between requests to the host (not less than the base one).
//!
//! A 2xx response gives rules; 4xx — no file, no restrictions; 5xx or a network failure — the host is closed
//! until the end of the run (RFC: "complete disallow").

use std::collections::HashMap;
use std::time::{Duration, Instant};

use anyhow::Result;
use regex::Regex;

use crate::http::{Polite, UA, utc_now};

pub struct Robots {
    /// (allow, pattern length, pattern)
    rules: Vec<(bool, usize, Regex)>,
    pub crawl_delay: Option<f64>,
}

impl Robots {
    pub fn allow_all() -> Robots {
        Robots { rules: Vec::new(), crawl_delay: None }
    }

    pub fn deny_all() -> Robots {
        Robots { rules: vec![(false, 1, Regex::new("^/").expect("regex"))], crawl_delay: None }
    }

    pub fn parse(txt: &str, agent: &str) -> Robots {
        let agent = agent.to_ascii_lowercase();
        struct Group {
            agents: Vec<String>,
            lines: Vec<(String, String)>,
        }
        let mut groups: Vec<Group> = Vec::new();
        let mut in_agents = false;
        for raw in txt.lines() {
            let line = raw.split('#').next().unwrap_or("").trim();
            let Some((k, v)) = line.split_once(':') else { continue };
            let (k, v) = (k.trim().to_ascii_lowercase(), v.trim().to_string());
            if k == "user-agent" {
                if !in_agents {
                    groups.push(Group { agents: Vec::new(), lines: Vec::new() });
                }
                in_agents = true;
                if let Some(g) = groups.last_mut() {
                    g.agents.push(v.to_ascii_lowercase());
                }
            } else {
                in_agents = false;
                if let Some(g) = groups.last_mut() {
                    g.lines.push((k, v));
                }
            }
        }
        let ours: Vec<&Group> =
            groups.iter().filter(|g| g.agents.iter().any(|a| a != "*" && !a.is_empty() && agent.contains(a.as_str()))).collect();
        let chosen: Vec<&Group> = if ours.is_empty() { groups.iter().filter(|g| g.agents.iter().any(|a| a == "*")).collect() } else { ours };
        let mut rules = Vec::new();
        let mut crawl_delay: Option<f64> = None;
        for g in chosen {
            for (k, v) in &g.lines {
                match k.as_str() {
                    "allow" | "disallow" if !v.is_empty() => {
                        if let Some(re) = pattern(v) {
                            rules.push((k == "allow", v.len(), re));
                        }
                    }
                    "crawl-delay" => {
                        if let Ok(d) = v.parse::<f64>() {
                            crawl_delay = Some(crawl_delay.map_or(d, |x| x.max(d)));
                        }
                    }
                    _ => {}
                }
            }
        }
        Robots { rules, crawl_delay }
    }

    pub fn allowed(&self, path: &str) -> bool {
        let mut best: Option<(usize, bool)> = None;
        for (allow, len, re) in &self.rules {
            if re.is_match(path) {
                let better = match best {
                    None => true,
                    Some((l, a)) => *len > l || (*len == l && *allow && !a),
                };
                if better {
                    best = Some((*len, *allow));
                }
            }
        }
        best.is_none_or(|(_, a)| a)
    }
}

/// A robots.txt pattern → a regular expression anchored at the start of the path.
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

/// The host belongs to GitHub — by name or by address (GitHub Pages under a custom domain:
/// `complexanalysis.org` → 185.199.108.153). Project rule: we do not contact GitHub, so
/// DNS is checked before the first HTTP request. Ranges come from GitHub's public list (meta):
/// 185.199.108.0/22, 140.82.112.0/20, 143.55.64.0/20, 192.30.252.0/22, 2606:50c0::/32, 2a0a:a440::/29.
pub fn github_host(host: &str) -> bool {
    let h = host.trim_end_matches('.').to_ascii_lowercase();
    let by_name = ["github.com", "github.io", "githubusercontent.com", "githubassets.com", "github.dev"]
        .iter()
        .any(|d| h == *d || h.ends_with(&format!(".{d}")));
    if by_name {
        return true;
    }
    use std::net::{IpAddr, ToSocketAddrs};
    let Ok(addrs) = (h.as_str(), 443).to_socket_addrs() else { return false };
    addrs.into_iter().any(|a| match a.ip() {
        IpAddr::V4(v) => {
            let n = u32::from(v);
            let net = |base: [u8; 4], bits: u32| (n >> (32 - bits)) == (u32::from_be_bytes(base) >> (32 - bits));
            net([185, 199, 108, 0], 22) || net([140, 82, 112, 0], 20) || net([143, 55, 64, 0], 20) || net([192, 30, 252, 0], 22)
        }
        IpAddr::V6(v) => {
            let s = v.segments();
            s[0] == 0x2606 && s[1] == 0x50c0 || (s[0] == 0x2a0a && (s[1] & 0xfff8) == 0xa440)
        }
    })
}

/// (scheme, host, path with query): `https://h/a/b?x=1` → (`https`, `h`, `/a/b?x=1`).
pub fn split_url(url: &str) -> (String, String, String) {
    let (scheme, rest) = url.split_once("://").unwrap_or(("https", url));
    let (host, path) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    let path = path.split('#').next().unwrap_or("/");
    (scheme.to_ascii_lowercase(), host.to_ascii_lowercase(), path.to_string())
}

/// Host gate: robots.txt of each host (cached per run) and the pause between requests to the host.
pub struct Gate {
    base: Duration,
    robots: HashMap<String, Robots>,
    last: HashMap<String, Instant>,
    github: std::collections::HashSet<String>,
}

impl Gate {
    pub fn new(base: Duration) -> Gate {
        Gate { base, robots: HashMap::new(), last: HashMap::new(), github: std::collections::HashSet::new() }
    }

    fn delay(&self, host: &str) -> Duration {
        let cd = self.robots.get(host).and_then(|r| r.crawl_delay).map(Duration::from_secs_f64);
        cd.map_or(self.base, |c| self.base.max(c))
    }

    fn pace(&self, host: &str) {
        if let Some(t) = self.last.get(host) {
            let (el, d) = (t.elapsed(), self.delay(host));
            if el < d {
                std::thread::sleep(d - el);
            }
        }
    }

    /// Whether `url` may be fetched. The first request to a host is robots.txt. `Ok(Err(reason))` — not allowed.
    /// After `Ok(Ok(()))` the caller makes the request right away (the host pause has already been observed).
    pub fn check(&mut self, http: &mut Polite, url: &str) -> Result<std::result::Result<(), String>> {
        let (scheme, host, path) = split_url(url);
        if !self.robots.contains_key(&host) && github_host(&host) {
            println!("{} GITHUB {host}: GitHub host (name or GitHub Pages address) — not contacting", utc_now());
            self.robots.insert(host.clone(), Robots::deny_all());
            self.github.insert(host.clone());
        }
        if self.github.contains(&host) {
            return Ok(Err(format!("{host} — GitHub (project rule: not contacting)")));
        }
        if !self.robots.contains_key(&host) {
            self.pace(&host);
            let robots_url = format!("{scheme}://{host}/robots.txt");
            let (r, how) = match http.text(&robots_url) {
                Ok(Ok(body)) => {
                    let r = Robots::parse(&body, UA);
                    let how = format!("{} rules, Crawl-delay {:?}", r.rules.len(), r.crawl_delay);
                    (r, how)
                }
                Ok(Err(st)) if (400..500).contains(&st.code) => (Robots::allow_all(), format!("HTTP {} — no restrictions", st.code)),
                Ok(Err(st)) => (Robots::deny_all(), format!("HTTP {} — host closed until the end of the run", st.code)),
                Err(e) => (Robots::deny_all(), format!("failure ({e:#}) — host closed until the end of the run")),
            };
            println!("{} ROBOTS {host}: {how}", utc_now());
            self.last.insert(host.clone(), Instant::now());
            self.robots.insert(host.clone(), r);
        }
        if !self.robots[&host].allowed(&path) {
            return Ok(Err(format!("robots.txt {host} disallows {path}")));
        }
        self.pace(&host);
        self.last.insert(host, Instant::now());
        Ok(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robots_rules() {
        let txt = "User-agent: Googlebot\nDisallow: /\n\nUser-agent: *\nDisallow: /cgi-bin/\nDisallow: /*.cgi$\nAllow: /cgi-bin/ok\nCrawl-delay: 120\n";
        let r = Robots::parse(txt, UA);
        assert_eq!(r.crawl_delay, Some(120.0));
        assert!(r.allowed("/opentextbooks/subjects/physics.json"));
        assert!(!r.allowed("/cgi-bin/x"));
        assert!(r.allowed("/cgi-bin/ok/y"));
        assert!(!r.allowed("/a/run.cgi"));
        assert!(r.allowed("/a/run.cgi?x=1"));
        // negative control: "Disallow: /" for everyone closes everything
        let all = Robots::parse("User-agent: *\nDisallow: /\n", UA);
        assert!(!all.allowed("/src/2508.00001v1"));
        // our own group overrides `*`
        let own = Robots::parse("User-agent: *\nDisallow: /\n\nUser-agent: MovaLibrary\nDisallow: /private\n", UA);
        assert!(own.allowed("/books/a.pdf") && !own.allowed("/private/x"));
        // an empty Disallow allows everything; so does HTML instead of robots.txt
        assert!(Robots::parse("User-agent: *\nDisallow:\n", UA).allowed("/x"));
        assert!(Robots::parse("<html><body>404</body></html>", UA).allowed("/x"));
        assert_eq!(split_url("https://H.org/a/b?x=1#f"), ("https".into(), "h.org".into(), "/a/b?x=1".into()));
        // GitHub by name; the address check is `github_host` in a real run (tests do not touch DNS)
        assert!(github_host("github.com") && github_host("rjournal.github.io") && github_host("raw.githubusercontent.com"));
        assert!(!github_host("notgithub.com.example.invalid"));
    }
}
