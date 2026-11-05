//! `mova oewn`: install Open English WordNet 2025 (JSON, CC BY 4.0, McCrae et al.,
//! <https://github.com/globalwordnet/english-wordnet>) into `$MOVA_DATA/raw/en-oewn/json`, where the school's world
//! model reads it (`world::wordnet`).
//!
//!   mova oewn install [--url URL] [--zip FILE]   # download (or take FILE), check, unpack, check, publish
//!   mova oewn check [DIR]                        # the inventory and load checks of an installed folder
//!   mova oewn inventory ZIP_SHA256 DIR           # print the inventory of an unpacked folder (for a new edition)
//!
//! The zip must have the SHA-256 of the inventory's header (`ZIP_SHA256`). It is unpacked into a staging folder
//! beside `json/`; the staging folder must hold exactly the inventory's files (names, sizes, SHA-256, nothing else)
//! and must load: every JSON file parses, there are entries, and every sense of every entry names a synset that is
//! there (the reader of `world::wordnet` skips unresolved senses silently, so the check is stricter than the reader).
//! Only then is it renamed to `json/` in one step; an earlier different `json/` is kept as `json.before-oewn-2025`.
//! An installed `json/` is reused only when it passes the same two checks, so an interrupted or damaged unpacking
//! is never taken for an installation.

use anyhow::{Context, Result, bail, ensure};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The SHA-256 of `english-wordnet-2025-json.zip` of the OEWN 2025 GitHub release.
pub const ZIP_SHA256: &str = "7d749f6e2c39e6970e4997839dcf6e42fd281f3c2fae0171d2192bae8cfa4b51";
pub const ZIP_NAME: &str = "english-wordnet-2025-json.zip";
pub const DEFAULT_URL: &str = "https://github.com/globalwordnet/english-wordnet/releases/download/2025-edition/english-wordnet-2025-json.zip";
/// The files of that zip: a header `# zip-sha256 <hex>`, then `path<TAB>bytes<TAB>sha256` per file.
pub const INVENTORY_2025: &str = include_str!("../oewn-2025-json.tsv");

/// The expected unpacked files, bound to the SHA-256 of the zip they come from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Inventory {
    pub zip_sha256: String,
    pub files: BTreeMap<String, (u64, String)>,
}

impl Inventory {
    pub fn parse(s: &str) -> Result<Inventory> {
        let mut lines = s.lines();
        let zip_sha256 = lines.next().and_then(|l| l.strip_prefix("# zip-sha256 ")).context("the inventory starts with `# zip-sha256 <hex>`")?.trim().to_string();
        ensure!(zip_sha256.len() == 64 && zip_sha256.bytes().all(|c| c.is_ascii_hexdigit()), "malformed zip SHA-256 in the inventory");
        let mut files = BTreeMap::new();
        for l in lines.filter(|l| !l.is_empty()) {
            let c: Vec<&str> = l.split('\t').collect();
            ensure!(c.len() == 3 && crate::safe_rel(c[0]) && !c[0].contains('/') && c[2].len() == 64, "malformed inventory line: {l}");
            ensure!(files.insert(c[0].to_string(), (c[1].parse::<u64>().context("inventory bytes")?, c[2].to_string())).is_none(), "{} twice in the inventory", c[0]);
        }
        ensure!(!files.is_empty(), "an empty inventory");
        Ok(Inventory { zip_sha256, files })
    }

    /// The shipped inventory of OEWN 2025; its header must name `ZIP_SHA256`.
    pub fn oewn_2025() -> Result<Inventory> {
        let i = Inventory::parse(INVENTORY_2025)?;
        ensure!(i.zip_sha256 == ZIP_SHA256, "the shipped OEWN inventory is of zip {}, not {ZIP_SHA256}", i.zip_sha256);
        Ok(i)
    }

    /// The inventory of an unpacked folder (flat, regular files only).
    pub fn of_dir(zip_sha256: &str, dir: &Path) -> Result<Inventory> {
        let mut files = BTreeMap::new();
        for e in std::fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
            let p = e?.path();
            let m = std::fs::symlink_metadata(&p)?;
            ensure!(m.is_file(), "{} is not a regular file", p.display());
            files.insert(p.file_name().context("a file name")?.to_string_lossy().into_owned(), (m.len(), crate::fetch::sha256_file(&p)?));
        }
        Ok(Inventory { zip_sha256: zip_sha256.to_string(), files })
    }

    pub fn render(&self) -> String {
        let mut s = format!("# zip-sha256 {}\n", self.zip_sha256);
        for (p, (b, h)) in &self.files {
            s.push_str(&format!("{p}\t{b}\t{h}\n"));
        }
        s
    }

    /// `dir` holds exactly these files: no missing, no extra, no other kinds of entries, sizes and SHA-256 equal.
    pub fn check_dir(&self, dir: &Path) -> Result<()> {
        ensure!(dir.is_dir(), "{} is not a folder", dir.display());
        let mut seen = BTreeMap::new();
        for e in std::fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
            let p = e?.path();
            let m = std::fs::symlink_metadata(&p)?;
            let name = p.file_name().context("a file name")?.to_string_lossy().into_owned();
            ensure!(m.is_file(), "{name} in {} is not a regular file", dir.display());
            ensure!(self.files.contains_key(&name), "{name} in {} is not in the inventory", dir.display());
            seen.insert(name, (m.len(), p));
        }
        for (name, (bytes, sha)) in &self.files {
            let (got, p) = seen.get(name).with_context(|| format!("{name} is missing from {}", dir.display()))?;
            ensure!(got == bytes, "{name} in {} has {got} bytes, the inventory says {bytes}", dir.display());
            ensure!(&crate::fetch::sha256_file(p)? == sha, "{name} in {} does not match its SHA-256", dir.display());
        }
        Ok(())
    }
}

/// What the load check found.
#[derive(Debug, PartialEq, Eq)]
pub struct Loaded {
    pub lemmas: usize,
    pub senses: usize,
    pub synsets: usize,
}

/// Read the folder as `world::wordnet` does (entries from `entries-*.json`, synsets from the other JSON files but
/// `frames.json`) and require that it loads: every file parses as a JSON object, there are entries, and every
/// sense names a synset that is there.
pub fn check_load(dir: &Path) -> Result<Loaded> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)?.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "json")).collect();
    files.sort();
    let mut synsets = std::collections::BTreeSet::new();
    let mut wanted: Vec<(String, String)> = Vec::new();
    let mut lemmas = 0;
    for f in &files {
        let name = f.file_name().context("a file name")?.to_string_lossy().into_owned();
        let v: serde_json::Value = serde_json::from_slice(&std::fs::read(f)?).with_context(|| format!("{name} is not JSON"))?;
        let obj = v.as_object().with_context(|| format!("{name} is not a JSON object"))?;
        if name.starts_with("entries-") {
            for (lemma, by_pos) in obj {
                lemmas += 1;
                for e in by_pos.as_object().into_iter().flat_map(|m| m.values()) {
                    for s in e["sense"].as_array().into_iter().flatten() {
                        wanted.push((lemma.clone(), s["synset"].as_str().with_context(|| format!("a sense of {lemma} in {name} has no synset"))?.to_string()));
                    }
                }
            }
        } else if name != "frames.json" {
            synsets.extend(obj.keys().cloned());
        }
    }
    ensure!(!wanted.is_empty(), "no WordNet entries with senses in {}", dir.display());
    if let Some((lemma, id)) = wanted.iter().find(|(_, id)| !synsets.contains(id)) {
        bail!("the sense of {lemma} names synset {id}, which is not in {}", dir.display());
    }
    Ok(Loaded { lemmas, senses: wanted.len(), synsets: synsets.len() })
}

/// Install into `root` (`$MOVA_DATA/raw/en-oewn`): reuse a `json/` that passes both checks; otherwise take the zip
/// (`zip` given, or `root/ZIP_NAME` if it has the right SHA-256, or downloaded from `url`), unpack into staging,
/// check, publish. Returns report lines.
pub fn install(inv: &Inventory, root: &Path, url: &str, zip: Option<&Path>) -> Result<Vec<String>> {
    let json = root.join("json");
    let mut rep = Vec::new();
    if json.exists() {
        match inv.check_dir(&json).and_then(|()| check_load(&json)) {
            Ok(l) => {
                rep.push(format!("{} is present ({} files checked; {} lemmas, {} senses, {} synsets load)", json.display(), inv.files.len(), l.lemmas, l.senses, l.synsets));
                return Ok(rep);
            }
            Err(e) => rep.push(format!("{} fails its checks ({e:#}); unpacking again", json.display())),
        }
    }
    std::fs::create_dir_all(root)?;
    let kept = root.join(ZIP_NAME);
    let zip = match zip {
        Some(z) => z.to_path_buf(),
        None if kept.is_file() && crate::fetch::sha256_file(&kept)? == inv.zip_sha256 => kept,
        None => {
            let part = root.join(format!("{ZIP_NAME}.part"));
            rep.push(format!("downloading {url}"));
            crate::fetch::download_to(url, &part)?;
            let got = crate::fetch::sha256_file(&part)?;
            ensure!(got == inv.zip_sha256, "checksum mismatch for {url}: expected {}, got {got}; the download is {}", inv.zip_sha256, part.display());
            std::fs::rename(&part, &kept)?;
            kept
        }
    };
    let got = crate::fetch::sha256_file(&zip)?;
    ensure!(got == inv.zip_sha256, "{} has SHA-256 {got}, the inventory is of {}", zip.display(), inv.zip_sha256);
    let staging = root.join(format!(".json-staging-{}", std::process::id()));
    ensure!(!staging.exists(), "{} is in the way", staging.display());
    std::fs::create_dir_all(&staging)?;
    let checked = (|| -> Result<Loaded> {
        let st = std::process::Command::new("unzip").arg("-q").arg("-o").arg(&zip).arg("-d").arg(&staging).status().context("running unzip (needed for Open English WordNet)")?;
        ensure!(st.success(), "unzip could not unpack {}", zip.display());
        inv.check_dir(&staging)?;
        check_load(&staging)
    })();
    let l = match checked {
        Ok(l) => l,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&staging);
            return Err(e.context("Open English WordNet refused after unpacking; nothing installed"));
        }
    };
    if json.exists() {
        rep.push(format!("kept the earlier {} as {}", json.display(), crate::fetch::set_aside(&json, "oewn-2025")?.display()));
    }
    std::fs::rename(&staging, &json)?;
    rep.push(format!("installed {} ({} files; {} lemmas, {} senses, {} synsets load)", json.display(), inv.files.len(), l.lemmas, l.senses, l.synsets));
    Ok(rep)
}

const USAGE: &str = "usage: mova oewn install [--url URL] [--zip FILE] | mova oewn check [DIR] | mova oewn inventory ZIP_SHA256 DIR";

/// `mova oewn …` (args after `oewn`).
pub fn main_args(args: &[String]) -> Result<()> {
    let root = crate::data_dir().join("raw/en-oewn");
    match args.first().map(String::as_str) {
        Some("install") => {
            let (mut url, mut zip) = (DEFAULT_URL.to_string(), None);
            let mut i = 1;
            while i < args.len() {
                let v = args.get(i + 1).cloned().with_context(|| format!("{} needs a value", args[i]))?;
                match args[i].as_str() {
                    "--url" => url = v,
                    "--zip" => zip = Some(PathBuf::from(v)),
                    a => bail!("unknown option {a}\n{USAGE}"),
                }
                i += 2;
            }
            for l in install(&Inventory::oewn_2025()?, &root, &url, zip.as_deref())? {
                println!("{l}");
            }
        }
        Some("check") => {
            let dir = args.get(1).map(PathBuf::from).unwrap_or_else(|| root.join("json"));
            Inventory::oewn_2025()?.check_dir(&dir)?;
            let l = check_load(&dir)?;
            println!("{}: inventory ok; {} lemmas, {} senses, {} synsets load", dir.display(), l.lemmas, l.senses, l.synsets);
        }
        Some("inventory") if args.len() == 3 => print!("{}", Inventory::of_dir(&args[1], Path::new(&args[2]))?.render()),
        _ => bail!("{USAGE}"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// CRC-32 (IEEE), for the fixture zip.
    fn crc32(b: &[u8]) -> u32 {
        let mut c = !0u32;
        for &x in b {
            c ^= x as u32;
            for _ in 0..8 {
                c = if c & 1 == 1 { (c >> 1) ^ 0xEDB8_8320 } else { c >> 1 };
            }
        }
        !c
    }

    /// A stored (uncompressed) zip of the given files.
    fn zip(files: &[(&str, &str)]) -> Vec<u8> {
        let (mut out, mut central) = (Vec::new(), Vec::new());
        for (name, body) in files {
            let (n, b, crc) = (name.as_bytes(), body.as_bytes(), crc32(body.as_bytes()));
            let off = out.len() as u32;
            let common = |v: &mut Vec<u8>| {
                v.extend_from_slice(&0u16.to_le_bytes()); // flags
                v.extend_from_slice(&0u16.to_le_bytes()); // stored
                v.extend_from_slice(&0u32.to_le_bytes()); // time, date
                v.extend_from_slice(&crc.to_le_bytes());
                v.extend_from_slice(&(b.len() as u32).to_le_bytes());
                v.extend_from_slice(&(b.len() as u32).to_le_bytes());
                v.extend_from_slice(&(n.len() as u16).to_le_bytes());
                v.extend_from_slice(&0u16.to_le_bytes()); // extra
            };
            out.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
            out.extend_from_slice(&20u16.to_le_bytes());
            common(&mut out);
            out.extend_from_slice(n);
            out.extend_from_slice(b);
            central.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
            central.extend_from_slice(&20u16.to_le_bytes());
            central.extend_from_slice(&20u16.to_le_bytes());
            common(&mut central);
            central.extend_from_slice(&[0u8; 6]); // comment length, disk, internal attributes
            central.extend_from_slice(&0o100644u32.wrapping_shl(16).to_le_bytes());
            central.extend_from_slice(&off.to_le_bytes());
            central.extend_from_slice(n);
        }
        let (cd_off, cd_len) = (out.len() as u32, central.len() as u32);
        out.extend_from_slice(&central);
        out.extend_from_slice(&0x0605_4b50u32.to_le_bytes());
        out.extend_from_slice(&[0u8; 4]);
        out.extend_from_slice(&(files.len() as u16).to_le_bytes());
        out.extend_from_slice(&(files.len() as u16).to_le_bytes());
        out.extend_from_slice(&cd_len.to_le_bytes());
        out.extend_from_slice(&cd_off.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out
    }

    const GOOD: [(&str, &str); 4] = [
        ("entries-a.json", r#"{"cat":{"n":{"sense":[{"synset":"1-n"}]}}}"#),
        ("entries-d.json", r#"{"dog":{"n":{"sense":[{"synset":"2-n"}]}}}"#),
        ("noun.animal.json", r#"{"1-n":{"partOfSpeech":"n","definition":["a cat"],"members":["cat"],"hypernym":["3-n"]},"2-n":{"partOfSpeech":"n","definition":["a dog"],"members":["dog"],"hypernym":["3-n"]}}"#),
        ("noun.Tops.json", r#"{"3-n":{"partOfSpeech":"n","definition":["an animal"],"members":["animal"]}}"#),
    ];

    /// The zip at a `file://` URL and the inventory of its unpacked files.
    fn fixture(d: &Path, files: &[(&str, &str)]) -> (String, Inventory) {
        std::fs::create_dir_all(d).unwrap();
        let z = d.join("src.zip");
        std::fs::write(&z, zip(files)).unwrap();
        let sum = crate::fetch::sha256_file(&z).unwrap();
        let files = files.iter().map(|(n, b)| (n.to_string(), (b.len() as u64, crate::sha256_hex(b.as_bytes())))).collect();
        (format!("file://{}", z.display()), Inventory { zip_sha256: sum, files })
    }

    #[test]
    fn the_shipped_inventory_is_bound_to_the_zip_sum() {
        let i = Inventory::oewn_2025().unwrap();
        assert_eq!(i.files.len(), 73);
        assert!(i.files.contains_key("entries-a.json") && i.files.contains_key("frames.json"));
        assert_eq!(Inventory::parse(&i.render()).unwrap(), i);
        // negative control: an inventory of another zip is refused as the OEWN 2025 inventory
        let other = INVENTORY_2025.replacen(ZIP_SHA256, &"0".repeat(64), 1);
        assert_ne!(Inventory::parse(&other).unwrap().zip_sha256, ZIP_SHA256);
        assert!(Inventory::parse("entries-a.json\t1\tx\n").is_err());
    }

    #[test]
    fn an_interrupted_or_damaged_extraction_is_never_taken_for_an_installation() {
        let d = crate::tests::tmp("oewn");
        let (url, inv) = fixture(&d.join("srv"), &GOOD);
        let root = d.join("data/raw/en-oewn");
        let json = root.join("json");
        // an interrupted earlier unpacking straight into json/: the old sentinel entries-a.json is there, the rest not
        std::fs::create_dir_all(&json).unwrap();
        std::fs::write(json.join("entries-a.json"), GOOD[0].1).unwrap();
        assert!(inv.check_dir(&json).is_err(), "negative control: the sentinel alone must fail the inventory");
        // and a staging folder left by a killed run
        std::fs::create_dir_all(root.join(".json-staging-1")).unwrap();
        std::fs::write(root.join(".json-staging-1/entries-a.json"), "{").unwrap();
        let rep = install(&inv, &root, &url, None).unwrap();
        assert!(rep.iter().any(|l| l.contains("fails its checks")), "{rep:?}");
        inv.check_dir(&json).unwrap();
        assert_eq!(check_load(&json).unwrap(), Loaded { lemmas: 2, senses: 2, synsets: 3 });
        assert!(root.join("json.before-oewn-2025/entries-a.json").is_file(), "the partial folder is kept aside");
        assert!(!root.join(format!(".json-staging-{}", std::process::id())).exists());
        // a complete installation is reused, without the zip
        let rep = install(&inv, &root, "https://invalid.example/none.zip", None).unwrap();
        assert!(rep.iter().any(|l| l.contains("is present")), "{rep:?}");
        // a truncated file, a missing file, an extra file: unpacked again from the kept zip
        type Damage = fn(&Path);
        let damages: [(&str, Damage); 4] = [
            ("truncated", |j| std::fs::write(j.join("noun.Tops.json"), "{").unwrap()),
            ("same size, other bytes", |j| std::fs::write(j.join("entries-d.json"), GOOD[1].1.replace("dog", "cow")).unwrap()),
            ("missing", |j| std::fs::remove_file(j.join("noun.animal.json")).unwrap()),
            ("extra", |j| std::fs::write(j.join("verb.extra.json"), "{}").unwrap()),
        ];
        for (why, damage) in damages {
            damage(&json);
            assert!(inv.check_dir(&json).is_err(), "{why}: the check must go red");
            let rep = install(&inv, &root, "https://invalid.example/none.zip", None).unwrap();
            assert!(rep.iter().any(|l| l.contains("fails its checks")), "{why}: {rep:?}");
            inv.check_dir(&json).unwrap();
        }
    }

    #[test]
    fn a_wrong_zip_or_one_that_does_not_load_installs_nothing() {
        let d = crate::tests::tmp("oewn-bad");
        // the served zip differs from the inventory's
        let (url, mut inv) = fixture(&d.join("srv"), &GOOD);
        inv.zip_sha256 = "0".repeat(64);
        let root = d.join("a/raw/en-oewn");
        assert!(install(&inv, &root, &url, None).is_err());
        assert!(!root.join("json").exists());
        // the zip matches its inventory, but an entry names a synset that is not there: the load check refuses it
        let mut broken = GOOD;
        broken[1] = ("entries-d.json", r#"{"dog":{"n":{"sense":[{"synset":"9-n"}]}}}"#);
        let (url, inv) = fixture(&d.join("srv-broken"), &broken);
        let root = d.join("b/raw/en-oewn");
        let e = install(&inv, &root, &url, None).unwrap_err();
        assert!(format!("{e:#}").contains("9-n"), "{e:#}");
        assert!(!root.join("json").exists());
        assert_eq!(std::fs::read_dir(&root).unwrap().filter_map(|e| e.ok()).filter(|e| e.file_name().to_string_lossy().starts_with(".json-staging")).count(), 0);
        // an archive with an unlisted file is refused too
        let mut more = GOOD.to_vec();
        more.push(("extra.json", "{}"));
        let (url, mut inv) = fixture(&d.join("srv-more"), &more);
        inv.files.remove("extra.json");
        let root = d.join("c/raw/en-oewn");
        assert!(install(&inv, &root, &url, None).is_err());
        assert!(!root.join("json").exists());
    }
}
