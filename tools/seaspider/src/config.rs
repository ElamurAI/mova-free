//! Configuration: the abstract robot MovaBot and every bound of the downloader.
//!
//! One JSON file. The robot identity (product token, version, contact page) is configuration,
//! not code: the public example carries a placeholder contact page, a concrete deployment supplies
//! its own. Personal data never enters the identity: the contact must be an http(s) page URL with
//! no user information, no `@` and no `mailto:`. The per-IP interval may be raised, never lowered
//! below 2 seconds.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// The floor of the per-IP interval: one request per IP every 2 seconds at most.
pub const MIN_IP_INTERVAL_MS: u64 = 2000;

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub robot: Robot,
    /// Store directory; relative paths are resolved against the configuration file's directory.
    pub store: String,
    #[serde(default)]
    pub politeness: Politeness,
    #[serde(default)]
    pub robots: RobotsCfg,
    #[serde(default)]
    pub fetch: FetchCfg,
    #[serde(default)]
    pub storage: StorageCfg,
    #[serde(default)]
    pub resume: ResumeCfg,
    #[serde(default)]
    pub follow: FollowCfg,
    #[serde(default)]
    pub network: NetworkCfg,
    #[serde(default)]
    pub service: ServiceCfg,
}

/// The served animal: its host-wide singleton lock and who may ask it.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct ServiceCfg {
    /// The singleton lock taken by `serve`, `fetch` and `robots` (absolute). Unset: the host-wide
    /// default `default_lock()`, shared by every Sea Spider of this user on the host.
    pub lock: Option<PathBuf>,
    /// Caller names admitted on the socket (`from` of the request; a request without one is the
    /// Octopus's). Default: the Octopus, Crab and the Whales (`whale`; `shark`, the Whales' caller
    /// name before the Whale rename, is admitted by default for one release).
    pub callers: Vec<String>,
}

/// Who may ask Sea Spider by default.
pub const DEFAULT_CALLERS: [&str; 4] = ["octopus", "crab", "whale", "shark"];
/// The caller a request without `from` is taken to come from (the `seaspider ask` CLI default).
pub const IMPLICIT_CALLER: &str = "octopus";

impl Default for ServiceCfg {
    fn default() -> Self {
        ServiceCfg { lock: None, callers: DEFAULT_CALLERS.iter().map(|s| s.to_string()).collect() }
    }
}

/// The host-wide default singleton lock: `$XDG_STATE_HOME/mova/seaspider.lock`, else
/// `$HOME/.local/state/mova/seaspider.lock` (outside temp directories, which cleaners may empty
/// under a held lock), else `<temp dir>/mova-seaspider-<uid>.lock`.
pub fn default_lock() -> PathBuf {
    let abs = |v: Option<std::ffi::OsString>| v.map(PathBuf::from).filter(|p| p.is_absolute());
    if let Some(d) = abs(std::env::var_os("XDG_STATE_HOME")) {
        return d.join("mova").join("seaspider.lock");
    }
    if let Some(h) = abs(std::env::var_os("HOME")) {
        return h.join(".local/state/mova/seaspider.lock");
    }
    use std::os::unix::fs::MetadataExt;
    let uid = std::fs::metadata("/proc/self").map(|m| m.uid()).unwrap_or(0);
    std::env::temp_dir().join(format!("mova-seaspider-{uid}.lock"))
}

/// The abstract robot: what robots.txt groups name and what the User-Agent header says.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Robot {
    /// RFC 9309 product token matched against robots.txt User-agent lines.
    #[serde(default = "default_token")]
    pub token: String,
    /// Version in the User-Agent; defaults to the crate version.
    #[serde(default = "default_version")]
    pub version: String,
    /// A public page that explains the robot (no e-mail, no personal names).
    pub contact_url: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct Politeness {
    /// Minimum gap per resolved IP, measured from the end of one request to the start of the next.
    pub ip_interval_ms: u64,
    /// Crawl-delay above this is not honoured by waiting: the URL is recorded as deferred.
    pub max_crawl_delay_secs: u64,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct RobotsCfg {
    /// How long a fetched robots.txt is trusted (RFC 9309 §2.4: at most 24 hours).
    pub ttl_secs: u64,
    /// How long an unreachable robots.txt (5xx, 429, network) keeps the origin closed.
    pub unreachable_retry_secs: u64,
    /// Redirects followed when fetching robots.txt (RFC 9309 §2.3.1.2: at least five).
    pub max_redirects: u32,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct FetchCfg {
    pub connect_timeout_ms: u64,
    /// Whole request including the body.
    pub timeout_ms: u64,
    /// Decoded body bytes kept; the rest is cut and the record marked truncated.
    pub max_body_bytes: u64,
    pub max_redirects: u32,
    pub accept: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct StorageCfg {
    /// A segment rolls over once it has reached this size.
    pub segment_max_bytes: u64,
    pub zstd_level: i32,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct ResumeCfg {
    /// None: a URL with a completed record is never fetched again; Some(s): refetch after s seconds.
    pub refetch_after_secs: Option<u64>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct FollowCfg {
    /// Off by default: only the queue is fetched.
    pub enabled: bool,
    /// Link depth from a queue URL (queue URLs have depth 0).
    pub max_depth: u32,
    /// Pages added by following, per run.
    pub max_pages: u64,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct NetworkCfg {
    /// Loopback, private, link-local and other non-public addresses are refused unless true.
    pub allow_private_addresses: bool,
    /// Host -> IP overrides (like curl --resolve); for tests and pinned deployments.
    pub resolve: BTreeMap<String, IpAddr>,
    pub dns_ttl_secs: u64,
}

fn default_token() -> String {
    "MovaBot".into()
}

fn default_version() -> String {
    env!("CARGO_PKG_VERSION").into()
}

impl Default for Politeness {
    fn default() -> Self {
        Politeness { ip_interval_ms: MIN_IP_INTERVAL_MS, max_crawl_delay_secs: 60 }
    }
}

impl Default for RobotsCfg {
    fn default() -> Self {
        RobotsCfg { ttl_secs: 86400, unreachable_retry_secs: 3600, max_redirects: 5 }
    }
}

impl Default for FetchCfg {
    fn default() -> Self {
        FetchCfg {
            connect_timeout_ms: 10_000,
            timeout_ms: 30_000,
            max_body_bytes: 10 << 20,
            max_redirects: 5,
            accept: "text/html,application/xhtml+xml;q=0.9,text/plain;q=0.8,*/*;q=0.5".into(),
        }
    }
}

impl Default for StorageCfg {
    fn default() -> Self {
        StorageCfg { segment_max_bytes: 256 << 20, zstd_level: 9 }
    }
}

impl Default for FollowCfg {
    fn default() -> Self {
        FollowCfg { enabled: false, max_depth: 1, max_pages: 1000 }
    }
}

impl Default for NetworkCfg {
    fn default() -> Self {
        NetworkCfg { allow_private_addresses: false, resolve: BTreeMap::new(), dns_ttl_secs: 3600 }
    }
}

impl Robot {
    /// `MovaBot/<version> (+<contact URL>)`.
    pub fn user_agent(&self) -> String {
        format!("{}/{} (+{})", self.token, self.version, self.contact_url)
    }

    fn validate(&self) -> Result<(), String> {
        let t = &self.token;
        if t.is_empty() || t.len() > 64 || !t.bytes().all(|b| b.is_ascii_alphabetic() || b == b'-' || b == b'_') {
            return Err(format!("robot.token must be 1..64 ASCII letters, '-' or '_' (RFC 9309 product token): {t:?}"));
        }
        let v = &self.version;
        if v.is_empty() || v.len() > 32 || !v.bytes().all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b)) {
            return Err(format!("robot.version must be 1..32 of [A-Za-z0-9._-]: {v:?}"));
        }
        let c = &self.contact_url;
        let u = reqwest::Url::parse(c).map_err(|e| format!("robot.contact_url is not a URL: {e}"))?;
        if !matches!(u.scheme(), "http" | "https") || u.host_str().is_none() {
            return Err("robot.contact_url must be an http(s) page URL".into());
        }
        if c.contains('@') || !u.username().is_empty() || u.password().is_some() || c.to_ascii_lowercase().contains("mailto") {
            return Err("robot.contact_url must not carry e-mail addresses or user information".into());
        }
        if c.len() > 200 || c.chars().any(|ch| ch.is_whitespace() || ch.is_control() || ch == '(' || ch == ')') {
            return Err("robot.contact_url must be at most 200 characters without spaces or parentheses".into());
        }
        Ok(())
    }
}

impl Config {
    /// Load, validate and resolve the store path against the configuration file's directory.
    pub fn load(path: &Path) -> Result<(Config, PathBuf), String> {
        let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        if bytes.len() > 1 << 20 {
            return Err("configuration larger than 1 MiB".into());
        }
        let cfg: Config = serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display()))?;
        cfg.validate()?;
        let store = PathBuf::from(&cfg.store);
        let store = if store.is_absolute() { store } else { path.parent().unwrap_or(Path::new(".")).join(store) };
        Ok((cfg, store))
    }

    pub fn validate(&self) -> Result<(), String> {
        self.robot.validate()?;
        if self.store.is_empty() {
            return Err("store must be a directory path".into());
        }
        let p = &self.politeness;
        if p.ip_interval_ms < MIN_IP_INTERVAL_MS {
            return Err(format!("politeness.ip_interval_ms must be at least {MIN_IP_INTERVAL_MS} (got {})", p.ip_interval_ms));
        }
        if p.max_crawl_delay_secs > 86400 {
            return Err("politeness.max_crawl_delay_secs must be at most 86400".into());
        }
        let r = &self.robots;
        if r.ttl_secs == 0 || r.ttl_secs > 86400 {
            return Err("robots.ttl_secs must be in 1..=86400 (RFC 9309 §2.4)".into());
        }
        if r.unreachable_retry_secs < 60 {
            return Err("robots.unreachable_retry_secs must be at least 60".into());
        }
        if r.max_redirects < 5 || r.max_redirects > 20 {
            return Err("robots.max_redirects must be in 5..=20".into());
        }
        let f = &self.fetch;
        if f.connect_timeout_ms == 0 || f.timeout_ms == 0 || f.timeout_ms > 300_000 || f.connect_timeout_ms > 300_000 {
            return Err("fetch timeouts must be in 1..=300000 ms".into());
        }
        if f.max_body_bytes == 0 || f.max_body_bytes > 1 << 30 {
            return Err("fetch.max_body_bytes must be in 1..=1 GiB".into());
        }
        if f.max_redirects > 20 {
            return Err("fetch.max_redirects must be at most 20".into());
        }
        if f.accept.is_empty() || f.accept.chars().any(|c| c.is_control()) {
            return Err("fetch.accept must be a non-empty header value".into());
        }
        let s = &self.storage;
        if s.segment_max_bytes < 1 << 20 || s.segment_max_bytes > 4 << 30 {
            return Err("storage.segment_max_bytes must be in 1 MiB..=4 GiB".into());
        }
        if !(1..=19).contains(&s.zstd_level) {
            return Err("storage.zstd_level must be in 1..=19".into());
        }
        if self.follow.max_depth > 10 {
            return Err("follow.max_depth must be at most 10".into());
        }
        if self.network.dns_ttl_secs == 0 || self.network.dns_ttl_secs > 86400 {
            return Err("network.dns_ttl_secs must be in 1..=86400".into());
        }
        let sv = &self.service;
        if sv.lock.as_ref().is_some_and(|l| !l.is_absolute()) {
            return Err("service.lock must be an absolute path".into());
        }
        if sv.callers.is_empty() || sv.callers.len() > 16 || !sv.callers.iter().all(|c| mova_octopus::socket::caller_name(c)) {
            return Err("service.callers must be 1..16 caller names (lowercase ASCII letters, digits, '-')".into());
        }
        Ok(())
    }

    /// The singleton lock this configuration uses.
    pub fn lock_path(&self) -> PathBuf {
        self.service.lock.clone().unwrap_or_else(default_lock)
    }

    /// Whether `caller` may ask this Sea Spider.
    pub fn admits(&self, caller: &str) -> bool {
        self.service.callers.iter().any(|c| c == caller)
    }

    pub fn ip_interval(&self) -> Duration {
        Duration::from_millis(self.politeness.ip_interval_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(extra: &str) -> Result<Config, String> {
        let txt = format!(r#"{{"robot": {{"contact_url": "https://example.invalid/movabot"}}, "store": "s"{extra}}}"#);
        let c: Config = serde_json::from_str(&txt).map_err(|e| e.to_string())?;
        c.validate().map(|_| c)
    }

    #[test]
    fn defaults_and_user_agent() {
        let c = cfg("").unwrap();
        assert_eq!(c.robot.token, "MovaBot");
        assert_eq!(c.robot.user_agent(), format!("MovaBot/{} (+https://example.invalid/movabot)", env!("CARGO_PKG_VERSION")));
        assert_eq!(c.politeness.ip_interval_ms, 2000);
        assert!(!c.follow.enabled, "following links is off by default");
    }

    #[test]
    fn ip_interval_never_below_two_seconds() {
        assert!(cfg(r#", "politeness": {"ip_interval_ms": 1999}"#).is_err());
        assert!(cfg(r#", "politeness": {"ip_interval_ms": 2500}"#).is_ok());
    }

    #[test]
    fn contact_refuses_personal_data() {
        for bad in ["mailto:x@y.z", "https://user@site.example/", "https://site.example/?c=a@b.c", "ftp://site.example/"] {
            let t = format!(r#"{{"robot": {{"contact_url": "{bad}"}}, "store": "s"}}"#);
            let c: Config = serde_json::from_str(&t).unwrap();
            assert!(c.validate().is_err(), "{bad}");
        }
        let t = r#"{"robot": {"token": "Mova Bot", "contact_url": "https://site.example/"}, "store": "s"}"#;
        assert!(serde_json::from_str::<Config>(t).unwrap().validate().is_err(), "token with a space");
    }

    #[test]
    fn callers_allowlist_and_lock() {
        let c = cfg("").unwrap();
        for who in ["octopus", "crab", "whale", "shark"] {
            assert!(c.admits(who), "{who} may ask by default");
        }
        assert!(!c.admits("fish") && !c.admits("eel"), "others may not");
        assert_eq!(c.lock_path(), default_lock());
        assert!(c.lock_path().is_absolute());
        let c = cfg(r#", "service": {"callers": ["crab"], "lock": "/run/x/seaspider.lock"}"#).unwrap();
        assert!(c.admits("crab") && !c.admits("whale") && !c.admits("octopus"));
        assert_eq!(c.lock_path(), PathBuf::from("/run/x/seaspider.lock"));
        assert!(cfg(r#", "service": {"callers": []}"#).is_err());
        assert!(cfg(r#", "service": {"callers": ["Crab"]}"#).is_err());
        assert!(cfg(r#", "service": {"lock": "rel.lock"}"#).is_err());
    }

    #[test]
    fn unknown_fields_refused() {
        assert!(cfg(r#", "politness": {}"#).is_err());
    }
}
