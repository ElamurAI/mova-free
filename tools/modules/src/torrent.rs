//! BitTorrent v1 metainfo (BEP 3) for publishing a module folder or one release file: bencoding, SHA-1 piece hashes
//! over the files in path order, the info hash and a magnet link. Optional trackers (`announce`, `announce-list`,
//! BEP 12) and HTTPS web seeds (`url-list`, BEP 19) so a client can fetch the files from our server when no peer is
//! online; a magnet link may carry `xl` (length), `ws` (web seed) and `xs` (the .torrent over HTTPS).
//!
//! The reading side is what `mova fetch` needs: parse a magnet link, read a single-file metainfo, check its info hash
//! and verify a downloaded file piece by piece. The peer wire protocol, trackers, DHT and metadata exchange (BEP 9)
//! are not implemented here; a BitTorrent client does that.

use anyhow::{Context, Result, bail, ensure};
use sha1::{Digest, Sha1};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::Path;

/// A bencoded value; dictionaries keep their keys in byte order, as BEP 3 requires.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Int(i64),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Dict(BTreeMap<Vec<u8>, Value>),
}

impl Value {
    pub fn str(s: &str) -> Value {
        Value::Bytes(s.as_bytes().to_vec())
    }
    pub fn encode(&self, out: &mut Vec<u8>) {
        match self {
            Value::Int(i) => out.extend_from_slice(format!("i{i}e").as_bytes()),
            Value::Bytes(b) => {
                out.extend_from_slice(format!("{}:", b.len()).as_bytes());
                out.extend_from_slice(b);
            }
            Value::List(l) => {
                out.push(b'l');
                l.iter().for_each(|v| v.encode(out));
                out.push(b'e');
            }
            Value::Dict(d) => {
                out.push(b'd');
                for (k, v) in d {
                    Value::Bytes(k.clone()).encode(out);
                    v.encode(out);
                }
                out.push(b'e');
            }
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();
        self.encode(&mut v);
        v
    }
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Dict(d) => d.get(key.as_bytes()),
            _ => None,
        }
    }
}

/// Decode one bencoded value; returns it and the bytes it used. Dictionary keys must be strictly increasing.
pub fn decode(b: &[u8]) -> Result<(Value, usize)> {
    fn int(b: &[u8], end: u8) -> Result<(i64, usize)> {
        let i = b.iter().position(|&c| c == end).context("unterminated integer")?;
        let s = std::str::from_utf8(&b[..i])?;
        ensure!(!s.is_empty() && s != "-0" && !(s.len() > 1 && (s.starts_with('0') || s.starts_with("-0"))), "non-canonical integer «{s}»");
        Ok((s.parse()?, i + 1))
    }
    match b.first() {
        Some(b'i') => {
            let (n, used) = int(&b[1..], b'e')?;
            Ok((Value::Int(n), 1 + used))
        }
        Some(b'l') => {
            let (mut items, mut at) = (Vec::new(), 1);
            while b.get(at) != Some(&b'e') {
                let (v, used) = decode(b.get(at..).context("truncated list")?)?;
                items.push(v);
                at += used;
            }
            Ok((Value::List(items), at + 1))
        }
        Some(b'd') => {
            let (mut d, mut at) = (BTreeMap::new(), 1);
            let mut last: Option<Vec<u8>> = None;
            while b.get(at) != Some(&b'e') {
                let (k, used) = decode(b.get(at..).context("truncated dictionary")?)?;
                let Value::Bytes(k) = k else { bail!("a dictionary key is a string") };
                ensure!(last.as_ref().is_none_or(|l| *l < k), "dictionary keys out of order");
                at += used;
                let (v, used) = decode(b.get(at..).context("truncated dictionary")?)?;
                at += used;
                last = Some(k.clone());
                d.insert(k, v);
            }
            Ok((Value::Dict(d), at + 1))
        }
        Some(c) if c.is_ascii_digit() => {
            let (n, used) = int(b, b':')?;
            let n = usize::try_from(n)?;
            let s = b.get(used..used + n).context("truncated string")?;
            Ok((Value::Bytes(s.to_vec()), used + n))
        }
        _ => bail!("not bencode"),
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// Percent-encode for a magnet link's query values.
fn pct(s: &str) -> String {
    s.bytes().map(|c| if c.is_ascii_alphanumeric() || b"-._~".contains(&c) { (c as char).to_string() } else { format!("%{c:02X}") }).collect()
}

/// The piece length: a power of two from 256 KiB to 16 MiB, keeping at most about 2 000 pieces.
pub fn piece_length(total: u64) -> u64 {
    let mut p: u64 = 256 * 1024;
    while p < 16 * 1024 * 1024 && total / p > 2000 {
        p *= 2;
    }
    p
}

pub struct Torrent {
    pub bytes: Vec<u8>,
    pub info_hash: String,
    pub magnet: String,
    pub files: usize,
    pub total: u64,
    pub pieces: usize,
    pub piece_length: u64,
}

/// A .torrent of a module folder: it must verify against its module.json; the torrent holds module.json and every
/// listed file, under the folder name `<name>-<version>`, in path order. Deterministic: no creation date or client
/// name, so the same folder and options give the same bytes and info hash.
pub fn make(dir: &Path, trackers: &[String], web_seeds: &[String]) -> Result<Torrent> {
    let mbytes = std::fs::read(dir.join("module.json")).with_context(|| format!("reading {}/module.json", dir.display()))?;
    let m = crate::Manifest::parse(&mbytes)?;
    crate::verify_dir(dir, &m)?;
    for u in trackers.iter().chain(web_seeds) {
        ensure!(u.starts_with("https://") || u.starts_with("udp://") || u.starts_with("http://"), "a tracker or web seed is a URL: {u}");
    }
    for u in web_seeds {
        // BEP 19, multi-file: a client fetches <web seed><torrent name>/<path>, so the seed is the folder that holds
        // the `<name>-<version>/` folder
        ensure!(u.starts_with("https://") && u.ends_with('/'), "a web seed is an https:// folder URL ending in / that holds <name>-<version>/: {u}");
    }
    let mut paths: Vec<String> = m.files.iter().map(|f| f.path.clone()).collect();
    paths.push("module.json".into());
    paths.sort();
    let mut data: Vec<(String, Vec<u8>)> = Vec::new();
    for p in &paths {
        let b = if p == "module.json" { mbytes.clone() } else { crate::read_verified(dir, m.file(p).expect("a listed file"))? };
        data.push((p.clone(), b));
    }
    let total: u64 = data.iter().map(|d| d.1.len() as u64).sum();
    let plen = piece_length(total);
    let (mut pieces, mut buf) = (Vec::new(), Vec::with_capacity(plen as usize));
    for (_, b) in &data {
        let mut rest: &[u8] = b;
        while !rest.is_empty() {
            let take = (plen as usize - buf.len()).min(rest.len());
            buf.extend_from_slice(&rest[..take]);
            rest = &rest[take..];
            if buf.len() == plen as usize {
                pieces.extend_from_slice(Sha1::digest(&buf).as_slice());
                buf.clear();
            }
        }
    }
    if !buf.is_empty() {
        pieces.extend_from_slice(Sha1::digest(&buf).as_slice());
    }
    let name = format!("{}-{}", m.name, m.version);
    let files = Value::List(
        data.iter()
            .map(|(p, b)| {
                let mut f = BTreeMap::new();
                f.insert(b"length".to_vec(), Value::Int(b.len() as i64));
                f.insert(b"path".to_vec(), Value::List(p.split('/').map(Value::str).collect()));
                Value::Dict(f)
            })
            .collect(),
    );
    let mut info = BTreeMap::new();
    info.insert(b"files".to_vec(), files);
    info.insert(b"name".to_vec(), Value::str(&name));
    info.insert(b"piece length".to_vec(), Value::Int(plen as i64));
    info.insert(b"pieces".to_vec(), Value::Bytes(pieces.clone()));
    let info = Value::Dict(info);
    let info_hash = hex(Sha1::digest(info.to_bytes()).as_slice());
    let mut top = BTreeMap::new();
    top.insert(b"info".to_vec(), info);
    top.insert(b"comment".to_vec(), Value::str(&format!("Mova module {}@{}; licence {} ({}); attribution: {}", m.name, m.version, m.licence, m.licence_class.as_str(), m.attribution.line())));
    if let Some(t) = trackers.first() {
        top.insert(b"announce".to_vec(), Value::str(t));
        top.insert(b"announce-list".to_vec(), Value::List(trackers.iter().map(|t| Value::List(vec![Value::str(t)])).collect()));
    }
    if !web_seeds.is_empty() {
        top.insert(b"url-list".to_vec(), Value::List(web_seeds.iter().map(|u| Value::str(u)).collect()));
    }
    let magnet = Magnet { btih: info_hash.clone(), dn: name.clone(), xl: None, tr: trackers.to_vec(), ws: web_seeds.to_vec(), xs: Vec::new() }.link();
    Ok(Torrent { bytes: Value::Dict(top).to_bytes(), info_hash, magnet, files: data.len(), total, pieces: pieces.len() / 20, piece_length: plen })
}

/// SHA-1 piece hashes of a byte stream read in pieces of `plen` bytes (the last piece may be shorter).
pub fn hash_pieces(mut r: impl Read, plen: u64) -> Result<(Vec<u8>, u64)> {
    let (mut pieces, mut total) = (Vec::new(), 0u64);
    let mut buf = vec![0u8; plen as usize];
    loop {
        let mut got = 0;
        while got < buf.len() {
            let n = r.read(&mut buf[got..])?;
            if n == 0 {
                break;
            }
            got += n;
        }
        if got == 0 {
            break;
        }
        total += got as u64;
        pieces.extend_from_slice(Sha1::digest(&buf[..got]).as_slice());
        if got < buf.len() {
            break;
        }
    }
    Ok((pieces, total))
}

/// A .torrent of one file (a release archive), streamed from disk: info {length, name, piece length, pieces}. A web
/// seed of a single-file torrent is the file's own `https://` URL (BEP 19); `xs` URLs (where the .torrent itself is
/// served) go into the magnet link only. Deterministic: no creation date or client name.
pub fn single_file(path: &Path, name: &str, trackers: &[String], web_seeds: &[String], xs: &[String], comment: &str) -> Result<Torrent> {
    ensure!(!name.is_empty() && !name.contains('/'), "a torrent name is one file name: {name}");
    for u in trackers {
        ensure!(u.starts_with("https://") || u.starts_with("udp://") || u.starts_with("http://"), "a tracker is a URL: {u}");
    }
    for u in web_seeds.iter().chain(xs) {
        ensure!(u.starts_with("https://") && !u.ends_with('/'), "a web seed or .torrent source of one file is its https:// URL: {u}");
    }
    let total = std::fs::metadata(path).with_context(|| format!("reading {}", path.display()))?.len();
    let plen = piece_length(total);
    let (pieces, read) = hash_pieces(std::io::BufReader::new(std::fs::File::open(path)?), plen)?;
    ensure!(read == total, "{} changed while it was hashed", path.display());
    let mut info = BTreeMap::new();
    info.insert(b"length".to_vec(), Value::Int(total as i64));
    info.insert(b"name".to_vec(), Value::str(name));
    info.insert(b"piece length".to_vec(), Value::Int(plen as i64));
    info.insert(b"pieces".to_vec(), Value::Bytes(pieces.clone()));
    let info = Value::Dict(info);
    let info_hash = hex(Sha1::digest(info.to_bytes()).as_slice());
    let mut top = BTreeMap::new();
    top.insert(b"info".to_vec(), info);
    if !comment.is_empty() {
        top.insert(b"comment".to_vec(), Value::str(comment));
    }
    if let Some(t) = trackers.first() {
        top.insert(b"announce".to_vec(), Value::str(t));
        top.insert(b"announce-list".to_vec(), Value::List(trackers.iter().map(|t| Value::List(vec![Value::str(t)])).collect()));
    }
    if !web_seeds.is_empty() {
        top.insert(b"url-list".to_vec(), Value::List(web_seeds.iter().map(|u| Value::str(u)).collect()));
    }
    let magnet = Magnet { btih: info_hash.clone(), dn: name.to_string(), xl: Some(total), tr: trackers.to_vec(), ws: web_seeds.to_vec(), xs: xs.to_vec() }.link();
    Ok(Torrent { bytes: Value::Dict(top).to_bytes(), info_hash, magnet, files: 1, total, pieces: pieces.len() / 20, piece_length: plen })
}

/// A magnet link (BEP 9 form): `xt=urn:btih:<40 hex>`, `dn`, `xl`, trackers `tr`, web seeds `ws` (BEP 19) and
/// exact sources `xs` (here: the .torrent over HTTPS).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Magnet {
    pub btih: String,
    pub dn: String,
    pub xl: Option<u64>,
    pub tr: Vec<String>,
    pub ws: Vec<String>,
    pub xs: Vec<String>,
}

fn unpct(s: &str) -> Result<String> {
    let b = s.as_bytes();
    let (mut out, mut i) = (Vec::new(), 0);
    while i < b.len() {
        match b[i] {
            b'%' => {
                let h = s.get(i + 1..i + 3).context("a truncated %-escape")?;
                out.push(u8::from_str_radix(h, 16).with_context(|| format!("a bad %-escape %{h}"))?);
                i += 3;
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    Ok(String::from_utf8(out)?)
}

impl Magnet {
    /// The link, parameters in a fixed order: xt, dn, tr…, ws…, xs…, xl.
    pub fn link(&self) -> String {
        let mut m = format!("magnet:?xt=urn:btih:{}&dn={}", self.btih, pct(&self.dn));
        for t in &self.tr {
            m.push_str(&format!("&tr={}", pct(t)));
        }
        for w in &self.ws {
            m.push_str(&format!("&ws={}", pct(w)));
        }
        for x in &self.xs {
            m.push_str(&format!("&xs={}", pct(x)));
        }
        if let Some(l) = self.xl {
            m.push_str(&format!("&xl={l}"));
        }
        m
    }

    /// Parse a magnet link with a v1 info hash (40 hex digits; 32-character base32 is decoded too). Unknown
    /// parameters are ignored.
    pub fn parse(s: &str) -> Result<Magnet> {
        let q = s.strip_prefix("magnet:?").context("a magnet link starts with magnet:?")?;
        let mut m = Magnet::default();
        for kv in q.split('&').filter(|p| !p.is_empty()) {
            let (k, v) = kv.split_once('=').with_context(|| format!("a magnet parameter without a value: {kv}"))?;
            let v = unpct(v)?;
            match k.split('.').next().unwrap_or(k) {
                "xt" => {
                    let Some(h) = v.strip_prefix("urn:btih:") else { continue };
                    m.btih = match h.len() {
                        40 if h.chars().all(|c| c.is_ascii_hexdigit()) => h.to_ascii_lowercase(),
                        32 => hex(&base32(h)?),
                        _ => bail!("not a v1 info hash: {h}"),
                    };
                }
                "dn" => m.dn = v,
                "xl" => m.xl = Some(v.parse().with_context(|| format!("xl is a length: {v}"))?),
                "tr" => m.tr.push(v),
                "ws" => m.ws.push(v),
                "xs" | "as" => m.xs.push(v),
                _ => {}
            }
        }
        ensure!(!m.btih.is_empty(), "the magnet link has no urn:btih: info hash");
        Ok(m)
    }
}

fn base32(s: &str) -> Result<Vec<u8>> {
    let (mut bits, mut acc, mut out) = (0u32, 0u64, Vec::new());
    for c in s.bytes() {
        let v = match c.to_ascii_uppercase() {
            c @ b'A'..=b'Z' => c - b'A',
            c @ b'2'..=b'7' => c - b'2' + 26,
            _ => bail!("not base32: {s}"),
        } as u64;
        acc = (acc << 5) | v;
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
        }
    }
    ensure!(out.len() == 20, "a base32 info hash is 20 bytes");
    Ok(out)
}

/// What `mova fetch` reads from a single-file .torrent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metainfo {
    pub info_hash: String,
    pub name: String,
    pub length: u64,
    pub piece_length: u64,
    pub pieces: Vec<u8>,
    pub url_list: Vec<String>,
    pub trackers: Vec<String>,
}

/// Read a single-file v1 .torrent: the info hash is the SHA-1 of the info dictionary's own bytes as they stand in the
/// file; the piece count must match the length. Multi-file torrents are refused (release files are single archives).
pub fn read_metainfo(bytes: &[u8]) -> Result<Metainfo> {
    let (top, used) = decode(bytes)?;
    ensure!(used == bytes.len(), "trailing bytes after the torrent");
    let info = top.get("info").context("no info dictionary")?;
    let info_hash = hex(Sha1::digest(info.to_bytes()).as_slice());
    let bytes_of = |v: Option<&Value>| match v {
        Some(Value::Bytes(b)) => Some(b.clone()),
        _ => None,
    };
    let int_of = |v: Option<&Value>| match v {
        Some(Value::Int(i)) if *i >= 0 => Some(*i as u64),
        _ => None,
    };
    ensure!(info.get("files").is_none(), "a multi-file torrent; mova fetch reads single-file release torrents");
    let name = String::from_utf8(bytes_of(info.get("name")).context("no name")?)?;
    let length = int_of(info.get("length")).context("no length")?;
    let piece_length = int_of(info.get("piece length")).filter(|p| *p > 0).context("no piece length")?;
    let pieces = bytes_of(info.get("pieces")).context("no pieces")?;
    ensure!(pieces.len() % 20 == 0 && (pieces.len() / 20) as u64 == length.div_ceil(piece_length), "the piece count does not match the length");
    let url_list = match top.get("url-list") {
        Some(Value::Bytes(b)) => vec![String::from_utf8(b.clone())?],
        Some(Value::List(l)) => l.iter().filter_map(|v| bytes_of(Some(v))).map(String::from_utf8).collect::<std::result::Result<_, _>>()?,
        _ => Vec::new(),
    };
    let mut trackers: Vec<String> = Vec::new();
    if let Some(Value::List(tiers)) = top.get("announce-list") {
        for t in tiers {
            if let Value::List(l) = t {
                trackers.extend(l.iter().filter_map(|v| bytes_of(Some(v))).filter_map(|b| String::from_utf8(b).ok()));
            }
        }
    } else if let Some(a) = bytes_of(top.get("announce")) {
        trackers.push(String::from_utf8(a)?);
    }
    Ok(Metainfo { info_hash, name, length, piece_length, pieces, url_list, trackers })
}

/// Check a downloaded file against a torrent's length and every piece's SHA-1; names the first bad piece.
pub fn verify_file(path: &Path, m: &Metainfo) -> Result<()> {
    let (pieces, total) = hash_pieces(std::io::BufReader::new(std::fs::File::open(path).with_context(|| format!("reading {}", path.display()))?), m.piece_length)?;
    ensure!(total == m.length, "{}: {total} bytes, the torrent says {}", path.display(), m.length);
    if let Some(i) = pieces.chunks(20).zip(m.pieces.chunks(20)).position(|(a, b)| a != b) {
        bail!("{}: piece {i} does not match the torrent's SHA-1", path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bencode_round_trip_and_canonical_form() {
        let mut d = BTreeMap::new();
        d.insert(b"zz".to_vec(), Value::Int(-3));
        d.insert(b"a".to_vec(), Value::List(vec![Value::str("spam"), Value::Int(0)]));
        let v = Value::Dict(d);
        assert_eq!(v.to_bytes(), b"d1:al4:spami0ee2:zzi-3ee");
        assert_eq!(decode(&v.to_bytes()).unwrap(), (v.clone(), 24));
        // negative controls: keys out of order, a leading zero, a truncated string
        assert!(decode(b"d2:zzi1e1:ai2ee").is_err());
        assert!(decode(b"i03e").is_err());
        assert!(decode(b"5:abc").is_err());
    }

    #[test]
    fn a_module_torrent_is_deterministic_and_hashes_its_pieces() {
        let d = crate::tests::tmp("torrent");
        let m = crate::tests::make_module(&d, "t", "1.0", "cc-by-sa", "CC-BY-SA-4.0");
        let a = make(&m, &["https://tracker.example.org/announce".into()], &["https://example.org/modules/".into()]).unwrap();
        let b = make(&m, &["https://tracker.example.org/announce".into()], &["https://example.org/modules/".into()]).unwrap();
        assert_eq!(a.bytes, b.bytes);
        assert!(a.magnet.starts_with(&format!("magnet:?xt=urn:btih:{}&dn=t-1.0&tr=https%3A%2F%2F", a.info_hash)));
        let (top, used) = decode(&a.bytes).unwrap();
        assert_eq!(used, a.bytes.len());
        let info = top.get("info").unwrap();
        assert_eq!(hex(Sha1::digest(info.to_bytes()).as_slice()), a.info_hash);
        assert_eq!(info.get("name"), Some(&Value::str("t-1.0")));
        // one piece over module.json and the layer, in path order (layers/… < module.json)
        let mut all = crate::tests::LAYER.as_bytes().to_vec();
        all.extend(std::fs::read(m.join("module.json")).unwrap());
        assert_eq!(info.get("pieces"), Some(&Value::Bytes(Sha1::digest(&all).to_vec())));
        // negative controls: a changed file is refused; a plain-http web seed is refused
        assert!(make(&m, &[], &["http://example.org/".into()]).is_err());
        assert!(make(&m, &[], &["https://example.org/t-1.0".into()]).is_err());
        std::fs::write(m.join("layers/senses.tsv"), "changed").unwrap();
        assert!(make(&m, &[], &[]).is_err());
    }

    #[test]
    fn piece_length_grows_with_size() {
        assert_eq!(piece_length(10), 256 * 1024);
        assert_eq!(piece_length(2_000 * 256 * 1024), 256 * 1024);
        assert_eq!(piece_length(4_000 * 256 * 1024 + 1), 512 * 1024);
        assert_eq!(piece_length(8_000 * 256 * 1024 + 1), 1024 * 1024);
        assert_eq!(piece_length(u64::MAX), 16 * 1024 * 1024);
    }

    #[test]
    fn a_single_file_torrent_reads_back_and_verifies_its_pieces() {
        let d = crate::tests::tmp("single");
        let f = d.join("a.tar.gz");
        // three pieces of 256 KiB and a short fourth one
        let data: Vec<u8> = (0..(3 * 256 * 1024 + 1000)).map(|i| (i * 7 % 251) as u8).collect();
        std::fs::write(&f, &data).unwrap();
        let (tr, ws, xs) = (vec!["udp://tracker.example.org:1337/announce".to_string()], vec!["https://example.org/0.5/a.tar.gz".to_string()], vec!["https://example.org/0.5/a.tar.gz.torrent".to_string()]);
        let t = single_file(&f, "a.tar.gz", &tr, &ws, &xs, "test").unwrap();
        assert_eq!(t.bytes, single_file(&f, "a.tar.gz", &tr, &ws, &xs, "test").unwrap().bytes, "deterministic");
        assert_eq!(t.pieces, 4);
        let m = read_metainfo(&t.bytes).unwrap();
        assert_eq!((m.info_hash.as_str(), m.name.as_str(), m.length, m.url_list.clone(), m.trackers.clone()), (t.info_hash.as_str(), "a.tar.gz", data.len() as u64, ws.clone(), tr.clone()));
        verify_file(&f, &m).unwrap();
        // the magnet link carries everything a client needs without the .torrent, and parses back
        let g = Magnet::parse(&t.magnet).unwrap();
        assert_eq!(g, Magnet { btih: t.info_hash.clone(), dn: "a.tar.gz".into(), xl: Some(data.len() as u64), tr, ws, xs });
        assert_eq!(g.link(), t.magnet);
        // negative controls: one changed byte in the third piece; a cut file; a tampered info dictionary
        let mut bad = data.clone();
        bad[2 * 256 * 1024 + 5] ^= 1;
        std::fs::write(&f, &bad).unwrap();
        let e = verify_file(&f, &m).unwrap_err().to_string();
        assert!(e.contains("piece 2"), "{e}");
        std::fs::write(&f, &data[..data.len() - 1]).unwrap();
        assert!(verify_file(&f, &m).is_err());
        let mut tb = t.bytes.clone();
        let at = tb.windows(8).position(|w| w == b"a.tar.gz").unwrap();
        tb[at] = b'b';
        assert_ne!(read_metainfo(&tb).unwrap().info_hash, t.info_hash);
        // a folder web seed or plain-http seed is refused for one file
        assert!(single_file(&f, "a.tar.gz", &[], &["https://example.org/0.5/".into()], &[], "").is_err());
        assert!(single_file(&f, "a.tar.gz", &[], &["http://example.org/a.tar.gz".into()], &[], "").is_err());
    }

    #[test]
    fn magnet_links_parse_hex_and_base32_and_refuse_garbage() {
        let hexh = "c12fe1c06bba254a9dc9f519b335aa7c1367a88a";
        let m = Magnet::parse(&format!("magnet:?xt=urn:btih:{}&dn=x%20y&tr=udp%3A%2F%2Ft%3A1", hexh.to_uppercase())).unwrap();
        assert_eq!((m.btih.as_str(), m.dn.as_str(), m.tr[0].as_str()), (hexh, "x y", "udp://t:1"));
        let b32 = Magnet::parse("magnet:?xt=urn:btih:YEX6DQDLXISUVHOJ6UM3GNNKPQJWPKEK").unwrap();
        assert_eq!(b32.btih, hexh);
        assert!(Magnet::parse("magnet:?dn=x").is_err());
        assert!(Magnet::parse("magnet:?xt=urn:btih:1234").is_err());
        assert!(Magnet::parse("http://x").is_err());
        assert!(Magnet::parse("magnet:?xt=urn:btih:YEX6DQDLXISUVHOJ6UM3GNNKPQJWPKEK&xl=many").is_err());
    }
}
