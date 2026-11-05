//! Politeness: one request per resolved IP every interval (at least 2 s), hosts serialised with
//! their own delay, and the DNS that makes the IP limit real.
//!
//! Adapted from the family Spider (`spider/src/net.rs`): every IP has its own "next free moment",
//! so hosts on one IP (shared hosting) share the pause, and a host with several IPs goes to the IP
//! that frees first. Not carried over: the Spider's CDN exception (an IP shared by many domains is
//! not throttled there; here it always is), address-pool harvesting and health bans.
//!
//! The IP the scheduler reserved is the IP the connection goes to: the reqwest resolver returns
//! only the pinned address, and the client keeps no idle connections.

use reqwest::dns::{Addrs, Name, Resolve, Resolving};
use std::collections::BTreeMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Next free moments per IP and per host.
pub struct Politeness {
    ip_interval: Duration,
    ip_next: BTreeMap<IpAddr, Instant>,
    host_next: BTreeMap<String, Instant>,
}

impl Politeness {
    pub fn new(ip_interval: Duration) -> Politeness {
        Politeness { ip_interval, ip_next: BTreeMap::new(), host_next: BTreeMap::new() }
    }

    pub fn ip_interval(&self) -> Duration {
        self.ip_interval
    }

    /// Earliest moment a request to `host` could start (no reservation).
    pub fn earliest(&self, host: &str, ips: &[IpAddr], now: Instant) -> Instant {
        let h = self.host_next.get(host).copied().unwrap_or(now).max(now);
        ips.iter().map(|ip| self.ip_next.get(ip).copied().unwrap_or(now).max(h)).min().unwrap_or(h)
    }

    /// Reserve the IP that frees first (ties: the lowest address) and the start moment
    /// `max(now, host free, IP free)`. Both are booked at once, so nothing else starts on that IP
    /// or host before `start + interval` / `start + host_delay`.
    pub fn reserve(&mut self, host: &str, ips: &[IpAddr], host_delay: Duration, now: Instant) -> (IpAddr, Instant) {
        assert!(!ips.is_empty(), "reserve needs at least one IP");
        let h = self.host_next.get(host).copied().unwrap_or(now).max(now);
        let mut sorted: Vec<IpAddr> = ips.to_vec();
        sorted.sort();
        sorted.dedup();
        let mut best: Option<(IpAddr, Instant)> = None;
        for ip in sorted {
            let free = self.ip_next.get(&ip).copied().unwrap_or(now).max(h);
            if best.is_none_or(|(_, b)| free < b) {
                best = Some((ip, free));
            }
        }
        let (ip, start) = best.expect("non-empty");
        self.ip_next.insert(ip, start + self.ip_interval);
        self.host_next.insert(host.to_string(), start + host_delay.max(self.ip_interval));
        (ip, start)
    }

    /// The request ended: the next one to this IP/host waits the interval from the end as well.
    pub fn finish(&mut self, host: &str, ip: IpAddr, end: Instant, host_delay: Duration) {
        let e = self.ip_next.entry(ip).or_insert(end);
        *e = (*e).max(end + self.ip_interval);
        let h = self.host_next.entry(host.to_string()).or_insert(end);
        *h = (*h).max(end + host_delay.max(self.ip_interval));
    }
}

/// True for addresses on the public internet; loopback, private, link-local, shared, multicast,
/// documentation and unspecified ranges are not.
pub fn is_public(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v) => {
            let o = v.octets();
            !(v.is_loopback()
                || v.is_private()
                || v.is_link_local()
                || v.is_unspecified()
                || v.is_broadcast()
                || v.is_multicast()
                || v.is_documentation()
                || o[0] == 0
                || (o[0] == 100 && (64..128).contains(&o[1]))
                || (o[0] == 192 && o[1] == 0 && o[2] == 0)
                || (o[0] == 198 && (18..20).contains(&o[1]))
                || o[0] >= 240)
        }
        IpAddr::V6(v) => is_public_v6(v),
    }
}

/// IPv6 rule: only global unicast (`2000::/3`) is public, minus its special-purpose blocks. Forms that embed
/// an IPv4 address the network may route to (IPv4-mapped `::ffff:0:0/96`, NAT64 well-known `64:ff9b::/96`,
/// 6to4 `2002::/16`) are public only when the embedded IPv4 is. Everything else outside `2000::/3` is refused:
/// unspecified, loopback, IPv4-compatible `::a.b.c.d`, IPv4-translated `::ffff:0:a.b.c.d`, local-use NAT64
/// `64:ff9b:1::/48`, discard `100::/64`, SRv6 `5f00::/16`, unique-local `fc00::/7`, link-local `fe80::/10`,
/// site-local `fec0::/10` and multicast `ff00::/8`. Inside `2000::/3` the IETF protocol block `2001::/23`
/// (Teredo `2001::/32`, benchmarking, ORCHID), documentation `2001:db8::/32` and `3fff::/20` are refused.
fn is_public_v6(v: std::net::Ipv6Addr) -> bool {
    let s = v.segments();
    let embedded = |hi: u16, lo: u16| std::net::Ipv4Addr::new((hi >> 8) as u8, hi as u8, (lo >> 8) as u8, lo as u8);
    if let Some(v4) = v.to_ipv4_mapped() {
        return is_public(IpAddr::V4(v4));
    }
    // NAT64 well-known prefix 64:ff9b::/96 (RFC 6052): the last 32 bits are the IPv4 address
    if s[..6] == [0x64, 0xff9b, 0, 0, 0, 0] {
        return is_public(IpAddr::V4(embedded(s[6], s[7])));
    }
    // 6to4 2002::/16 (RFC 3056): bits 16..48 are the IPv4 address
    if s[0] == 0x2002 {
        return is_public(IpAddr::V4(embedded(s[1], s[2])));
    }
    if (s[0] & 0xe000) != 0x2000 {
        return false;
    }
    !((s[0] == 0x2001 && s[1] < 0x0200) || (s[0] == 0x2001 && s[1] == 0x0db8) || (s[0] == 0x3fff && s[1] < 0x1000))
}

/// DNS with a cache, overrides and the public-address rule; pins the reserved IP for reqwest.
pub struct Dns {
    ttl: Duration,
    allow_private: bool,
    overrides: BTreeMap<String, IpAddr>,
    cache: BTreeMap<String, (Instant, Vec<IpAddr>)>,
    pins: Arc<Mutex<BTreeMap<String, IpAddr>>>,
}

impl Dns {
    pub fn new(ttl: Duration, allow_private: bool, overrides: BTreeMap<String, IpAddr>) -> Dns {
        let overrides = overrides.into_iter().map(|(k, v)| (k.to_ascii_lowercase(), v)).collect();
        Dns { ttl, allow_private, overrides, cache: BTreeMap::new(), pins: Arc::new(Mutex::new(BTreeMap::new())) }
    }

    /// Sorted, deduplicated addresses of a host; an IP literal is its own address.
    pub async fn resolve(&mut self, host: &str) -> Result<Vec<IpAddr>, String> {
        let host = host.trim_start_matches('[').trim_end_matches(']').to_ascii_lowercase();
        let all: Vec<IpAddr> = if let Ok(ip) = host.parse::<IpAddr>() {
            vec![ip]
        } else if let Some(ip) = self.overrides.get(&host) {
            vec![*ip]
        } else if let Some((at, ips)) = self.cache.get(&host).filter(|(at, _)| at.elapsed() < self.ttl) {
            let _ = at;
            ips.clone()
        } else {
            let mut got: Vec<IpAddr> = tokio::net::lookup_host((host.as_str(), 0)).await.map_err(|e| format!("dns error: {e}"))?.map(|s| s.ip()).collect();
            got.sort();
            got.dedup();
            if got.is_empty() {
                return Err("dns error: no addresses".into());
            }
            self.cache.insert(host.clone(), (Instant::now(), got.clone()));
            got
        };
        if self.allow_private {
            return Ok(all);
        }
        let public: Vec<IpAddr> = all.into_iter().filter(|ip| is_public(*ip)).collect();
        if public.is_empty() {
            return Err(format!("{host} resolves only to non-public addresses (network.allow_private_addresses is false)"));
        }
        Ok(public)
    }

    /// The next connection to `host` goes to `ip`.
    pub fn pin(&self, host: &str, ip: IpAddr) {
        self.pins.lock().unwrap().insert(host.to_ascii_lowercase(), ip);
    }

    pub fn resolver(&self) -> PinResolver {
        PinResolver(self.pins.clone())
    }
}

/// reqwest resolver: only the pinned address, never a fresh lookup.
pub struct PinResolver(Arc<Mutex<BTreeMap<String, IpAddr>>>);

impl Resolve for PinResolver {
    fn resolve(&self, name: Name) -> Resolving {
        let ip = self.0.lock().unwrap().get(&name.as_str().to_ascii_lowercase()).copied();
        let host = name.as_str().to_string();
        Box::pin(async move {
            let ip = ip.ok_or_else(|| -> Box<dyn std::error::Error + Send + Sync> { format!("no pinned address for {host}").into() })?;
            let addrs: Addrs = Box::new(std::iter::once(SocketAddr::new(ip, 0)));
            Ok(addrs)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ip(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    #[test]
    fn same_ip_waits_the_interval_across_hosts() {
        let mut p = Politeness::new(Duration::from_secs(2));
        let t = Instant::now();
        let one = [ip("10.0.0.9")];
        let (_, a) = p.reserve("a.example", &one, Duration::ZERO, t);
        let (_, b) = p.reserve("b.example", &one, Duration::ZERO, t);
        assert_eq!(a, t);
        assert!(b >= t + Duration::from_secs(2), "hosts on one IP share the pause");
    }

    #[test]
    fn host_delay_and_ip_spread() {
        let mut p = Politeness::new(Duration::from_secs(2));
        let t = Instant::now();
        let two = [ip("10.0.0.2"), ip("10.0.0.1")];
        let (x, s1) = p.reserve("h", &two, Duration::from_secs(5), t);
        assert_eq!((x, s1), (ip("10.0.0.1"), t), "ties go to the lowest address");
        // another host on a free IP is not held back by host h
        let (z, s0) = p.reserve("other", &two, Duration::ZERO, t);
        assert_eq!((z, s0), (ip("10.0.0.2"), t), "the free IP serves another host at once");
        let (y, s2) = p.reserve("h", &two, Duration::from_secs(5), t);
        assert_eq!(y, ip("10.0.0.1"));
        assert_eq!(s2, t + Duration::from_secs(5), "the host delay serialises the host even when an IP is free earlier");
        // the IP is booked from that start
        let (_, s3) = p.reserve("third", &[ip("10.0.0.1")], Duration::ZERO, t);
        assert!(s3 >= t + Duration::from_secs(7));
    }

    #[test]
    fn finish_extends_from_the_end() {
        let mut p = Politeness::new(Duration::from_secs(2));
        let t = Instant::now();
        let one = [ip("10.0.0.3")];
        let (_, s) = p.reserve("h", &one, Duration::ZERO, t);
        p.finish("h", one[0], s + Duration::from_secs(7), Duration::ZERO);
        assert!(p.earliest("g", &one, t) >= t + Duration::from_secs(9));
    }

    #[test]
    fn public_ranges() {
        for s in ["127.0.0.1", "10.1.2.3", "192.168.0.1", "172.16.0.1", "169.254.1.1", "100.64.0.1", "::1", "fd00::1", "fe80::1", "::ffff:127.0.0.1", "0.0.0.0"] {
            assert!(!is_public(ip(s)), "{s}");
        }
        for s in ["1.1.1.1", "93.184.216.34", "2606:4700::1111"] {
            assert!(is_public(ip(s)), "{s}");
        }
    }

    /// IPv6 forms that reach a non-public host: embedded private IPv4 (NAT64, 6to4, IPv4-compatible,
    /// IPv4-translated), Teredo and the other special-purpose blocks. Red before the `2000::/3` rule.
    #[test]
    fn ipv6_special_and_embedded_ranges_are_not_public() {
        for s in [
            "::",
            "::1",
            "::127.0.0.1",
            "::10.0.0.1",
            "::ffff:10.0.0.1",
            "::ffff:0:10.0.0.1",
            "64:ff9b::10.0.0.1",
            "64:ff9b::127.0.0.1",
            "64:ff9b:1::1.1.1.1",
            "2002:0a00:0001::1",
            "2002:7f00:0001::1",
            "2002:c0a8:0101::1",
            "2001:0:4136:e378:8000:63bf:3fff:fdd2",
            "2001:2::1",
            "2001:10::1",
            "2001:db8::1",
            "3fff::1",
            "100::1",
            "5f00::1",
            "fc00::1",
            "fd12:3456::1",
            "fe80::1",
            "fec0::1",
            "ff02::1",
            "ff0e::1",
        ] {
            assert!(!is_public(ip(s)), "{s}");
        }
        for s in ["2606:4700::1111", "2a00:1450:4001:80b::200e", "64:ff9b::1.1.1.1", "2002:0101:0101::1", "::ffff:1.1.1.1"] {
            assert!(is_public(ip(s)), "{s}");
        }
    }
}
