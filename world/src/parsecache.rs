//! Lossless parse reuse for fast school probes. Full probes refresh every tree.
//! The old state::parse_text_cached reader drops PTB tags; it cannot be used here.
use anyhow::{Context, Result, ensure};
use en::annotate::Word;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
};

pub(crate) fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub(crate) fn file_hash(path: &Path) -> Result<String> {
    let mut f = fs::File::open(path).with_context(|| format!("fingerprint {}", path.display()))?;
    let mut h = Sha256::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(format!("{:x}", h.finalize()))
}
pub(crate) fn data_root() -> PathBuf {
    std::env::var_os("MOVA_DATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join("mova-data")
        })
}
/// Only parser dependencies. Question/answer contents are cache keys, never labels.
fn identity() -> Result<String> {
    let mut parts = BTreeMap::new();
    parts.insert("schema".to_string(), "school-parse-1".to_string());
    parts.insert("executable".into(), file_hash(&std::env::current_exe()?)?);
    parts.insert("en_model".into(), file_hash(&crate::tree::model_path())?);
    parts.insert("mode".into(), format!("{:?}", crate::exec::mode()));
    if crate::exec::mode() == crate::exec::Mode::Gpu {
        let dir = std::env::var_os("MOVA_GPARSE_MODEL")
            .map(PathBuf::from)
            .unwrap_or_else(|| data_root().join("en/models/gparse"));
        for name in [
            "cfg.txt",
            "model.safetensors",
            "words.txt",
            "xpos.txt",
            "upos.txt",
            "rels.txt",
        ] {
            parts.insert(format!("gpu/{name}"), file_hash(&dir.join(name))?);
        }
        parts.insert(
            "driver".into(),
            hash(&fs::read("/proc/driver/nvidia/version").unwrap_or_default()),
        );
    }
    for name in [
        "GP_CPU",
        "GP_BATCH",
        "GP_FUSED_SOFTMAX",
        "GP_GELU",
        "CUDA_VISIBLE_DEVICES",
        "WORLD_NO_REPAIR",
        "WORLD_REPAIR",
        "WORLD_DOMAIN",
        "INDUCE_WITHOUT",
        "PRAG_OFF",
        "PRAG_VN",
    ] {
        if let Some(value) = std::env::var_os(name) {
            parts.insert(format!("env/{name}"), value.to_string_lossy().into_owned());
        }
    }
    // A hot rule/matrix file may change without a new executable.
    for name in ["WORLD_RULES", "GP_LEX"] {
        if let Some(path) = std::env::var_os(name) {
            let p = PathBuf::from(path);
            parts.insert(format!("file/{name}"), optional_hash(&p)?);
        }
    }
    if let Some(path) = std::env::var_os("GLOBAL_HOT") {
        for name in [
            "absurdity-roles.tsv",
            "absurdity-roles-tale.tsv",
            "idioms.tsv",
        ] {
            parts.insert(
                format!("hot/{name}"),
                optional_hash(&PathBuf::from(&path).join(name))?,
            );
        }
    }
    Ok(hash(&serde_json::to_vec(&parts)?))
}
fn optional_hash(path: &Path) -> Result<String> {
    match fs::metadata(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok("absent".into()),
        Err(e) => Err(e.into()),
        Ok(_) => file_hash(path),
    }
}

#[derive(Serialize, Deserialize)]
struct Envelope {
    schema: u32,
    identity: String,
    checksum: String,
    trees: BTreeMap<String, Vec<Word>>,
}
pub(crate) struct Cache {
    path: PathBuf,
    identity: String,
    trees: BTreeMap<String, Vec<Word>>,
    refresh: bool,
    pub hits: usize,
    pub misses: usize,
    pub repaired: bool,
}
impl Cache {
    pub fn open(root: &Path, refresh: bool) -> Result<Self> {
        let id = identity()?;
        Self::open_key(root, id, refresh)
    }
    fn open_key(root: &Path, id: String, refresh: bool) -> Result<Self> {
        fs::create_dir_all(root)?;
        let path = root.join(format!("{id}.json"));
        let mut repaired = false;
        let trees = if refresh {
            BTreeMap::new()
        } else {
            match fs::read(&path) {
                Ok(bytes) => match decode(&bytes, &id) {
                    Ok(trees) => trees,
                    Err(e) => {
                        eprintln!("school parse cache: discard invalid entry: {e}");
                        repaired = true;
                        BTreeMap::new()
                    }
                },
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => BTreeMap::new(),
                Err(e) => return Err(e.into()),
            }
        };
        Ok(Self {
            path,
            identity: id,
            trees,
            refresh,
            hits: 0,
            misses: 0,
            repaired,
        })
    }
    pub fn signature(&self) -> &str {
        &self.identity
    }
    pub fn parse(&mut self, texts: &[String]) -> Result<Vec<Vec<Word>>> {
        self.parse_with(texts, crate::exec::parse_many)
    }
    fn parse_with(
        &mut self,
        texts: &[String],
        parser: impl FnOnce(&[String]) -> Result<Vec<Vec<Word>>>,
    ) -> Result<Vec<Vec<Word>>> {
        let missing: Vec<String> = texts
            .iter()
            .filter(|t| self.refresh || !self.trees.contains_key(*t))
            .cloned()
            .collect();
        self.hits += texts.len() - missing.len();
        self.misses += missing.len();
        if !missing.is_empty() {
            let parsed = parser(&missing)?;
            ensure!(
                parsed.len() == missing.len(),
                "cached parser returned a partial batch"
            );
            for (text, words) in missing.into_iter().zip(parsed) {
                self.trees.insert(text, words);
            }
            // Recheck the inputs before publishing parses under their identity.
            // Unit tests use synthetic keys and call write() directly.
        }
        texts
            .iter()
            .map(|t| self.trees.get(t).cloned().context("missing parse"))
            .collect()
    }
    pub fn finish(&self) -> Result<()> {
        ensure!(
            identity()? == self.identity,
            "parser dependencies changed during the benchmark"
        );
        if self.misses > 0 {
            self.write()?;
        }
        Ok(())
    }
    fn write(&self) -> Result<()> {
        let entry = Envelope {
            schema: 1,
            identity: self.identity.clone(),
            checksum: hash(&serde_json::to_vec(&self.trees)?),
            trees: self.trees.clone(),
        };
        crate::store::write_atomic(&self.path, &serde_json::to_vec(&entry)?)
    }
}
fn decode(bytes: &[u8], id: &str) -> Result<BTreeMap<String, Vec<Word>>> {
    let e: Envelope = serde_json::from_slice(bytes)?;
    ensure!(e.schema == 1 && e.identity == id, "cache identity mismatch");
    ensure!(
        e.checksum == hash(&serde_json::to_vec(&e.trees)?),
        "cache checksum mismatch"
    );
    Ok(e.trees)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn scratch() -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "school-cache-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&p).unwrap();
        p
    }
    #[test]
    fn cache_preserves_tags_features_empty_trees_and_refreshes() {
        let root = scratch();
        let word = Word {
            form: "cat".into(),
            lemma: "cat".into(),
            upos: en::gram::UPos::NOUN,
            tag: en::gram::Tag::NN,
            feats: en::gram::Feats(1u128 << 100),
            head: 0,
            rel: en::gram::Rel::Root,
        };
        let texts = vec!["The cat.".to_string(), "".into()];
        let parsed = vec![vec![word], vec![]];
        let mut cache = Cache::open_key(&root, "a".into(), false).unwrap();
        assert_eq!(
            cache.parse_with(&texts, |_| Ok(parsed.clone())).unwrap(),
            parsed
        );
        cache.write().unwrap();
        let mut cached = Cache::open_key(&root, "a".into(), false).unwrap();
        assert_eq!(
            cached.parse_with(&texts, |_| panic!("cache miss")).unwrap(),
            parsed
        );
        assert_eq!((cached.hits, cached.misses), (2, 0));
        let mut fresh = Cache::open_key(&root, "a".into(), true).unwrap();
        fresh
            .parse_with(&texts, |xs| {
                assert_eq!(xs, &texts);
                Ok(parsed.clone())
            })
            .unwrap();
        assert_eq!((fresh.hits, fresh.misses), (0, 2));
        let other = Cache::open_key(&root, "b".into(), false).unwrap();
        assert!(
            other.trees.is_empty(),
            "different model/code identity reused trees"
        );
        let mut broken: Envelope = serde_json::from_slice(&fs::read(&cache.path).unwrap()).unwrap();
        broken.trees.get_mut(&texts[0]).unwrap()[0].head = 999;
        fs::write(&cache.path, serde_json::to_vec(&broken).unwrap()).unwrap();
        let repaired = Cache::open_key(&root, "a".into(), false).unwrap();
        assert!(repaired.repaired && repaired.trees.is_empty());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn partial_parser_output_never_becomes_a_cache() {
        let root = scratch();
        let mut cache = Cache::open_key(&root, "partial".into(), false).unwrap();
        assert!(
            cache
                .parse_with(&["one".into(), "two".into()], |_| Ok(vec![vec![]]))
                .is_err()
        );
        assert!(!cache.path.exists() && cache.trees.is_empty());
        fs::remove_dir_all(root).unwrap();
    }
}
#[cfg(test)]
mod transition_tests {
    use super::*;
    fn words(marker: usize) -> Vec<Word> {
        vec![Word {
            form: "cat".into(),
            lemma: "cat".into(),
            upos: en::gram::UPos::NOUN,
            tag: en::gram::Tag::NN,
            feats: en::gram::Feats(1u128 << 100),
            head: marker,
            rel: en::gram::Rel::Root,
        }]
    }
    struct Scratch(PathBuf);
    impl Scratch {
        fn new() -> Self {
            static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let serial = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "parsecache-transitions-{}-{serial}",
                std::process::id()
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
        fn cache(&self, refresh: bool) -> Cache {
            Cache::open_key(&self.0, "synthetic".into(), refresh).unwrap()
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }
    #[test]
    fn bounded_batch_relation_and_complement() {
        let root = Scratch::new();
        let mut cases = 0;
        let mut memberships = 0;
        for n in 0..=3 {
            for code in 0..(1usize << n) {
                for mask in 0..4 {
                    for refresh in [false, true] {
                        let xs: Vec<String> = (0..n)
                            .map(|i| if code & (1 << i) == 0 { "a" } else { "b" }.into())
                            .collect();
                        let mut cache = root.cache(refresh);
                        for (i, key) in ["a", "b"].iter().enumerate() {
                            if mask & (1 << i) != 0 {
                                cache.trees.insert((*key).into(), words(10 + i));
                            }
                        }
                        // Membership belongs to the initial map, not to an incremental insertion map.
                        let before = cache.trees.clone();
                        let wanted: Vec<_> = xs
                            .iter()
                            .filter(|x| refresh || !before.contains_key(*x))
                            .cloned()
                            .collect();
                        let mut expected = before;
                        for (i, key) in wanted.iter().enumerate() {
                            expected.insert(key.clone(), words(i));
                        }
                        let mut calls = 0;
                        let actual = cache
                            .parse_with(&xs, |batch| {
                                calls += 1;
                                assert_eq!(batch, &wanted);
                                Ok((0..batch.len()).map(words).collect())
                            })
                            .unwrap();
                        assert_eq!(
                            actual,
                            xs.iter().map(|x| expected[x].clone()).collect::<Vec<_>>()
                        );
                        assert_eq!(cache.trees, expected);
                        assert_eq!((cache.hits, cache.misses), (n - wanted.len(), wanted.len()));
                        assert_eq!(calls, usize::from(!wanted.is_empty()));
                        // Entire U projects onto (parser-call bit, hit delta, miss delta).
                        for call in 0..=1 {
                            for hits in 0..=n {
                                for misses in 0..=n {
                                    let allowed = call == usize::from(!wanted.is_empty())
                                        && hits == n - wanted.len()
                                        && misses == wanted.len();
                                    let observed =
                                        (call, hits, misses) == (calls, cache.hits, cache.misses);
                                    assert_eq!(!allowed, !observed);
                                    memberships += 1;
                                }
                            }
                        }
                        cases += 1;
                    }
                }
            }
        }
        assert_eq!(cases, 120);
        println!("CHECK batch cases={cases} complement_memberships={memberships}");
    }
    #[test]
    fn duplicate_preimages_and_extensional_parser_premise() {
        let root = Scratch::new();
        let xs = vec!["a".into(), "a".into()];
        let mut observations = Vec::new();
        for old_a in 0..2 {
            for old_b in 0..2 {
                for first in 0..2 {
                    for last in 0..2 {
                        let mut cache = root.cache(true);
                        cache.trees.insert("a".into(), words(old_a));
                        cache.trees.insert("b".into(), words(old_b));
                        let output = cache
                            .parse_with(&xs, |_| Ok(vec![words(first), words(last)]))
                            .unwrap();
                        assert_eq!(cache.trees["b"], words(old_b));
                        observations.push(((old_a, old_b, first, last), output));
                    }
                }
            }
        }
        for target in 0..=2 {
            let actual: Vec<_> = observations
                .iter()
                .filter(|(_, output)| *output == vec![words(target), words(target)])
                .map(|(input, _)| *input)
                .collect();
            let expected: Vec<_> = (0..2)
                .flat_map(|a| {
                    (0..2).flat_map(move |b| (0..2).map(move |first| (a, b, first, target)))
                })
                .filter(|(_, _, _, last)| *last < 2)
                .collect();
            assert_eq!(actual, expected);
            assert_eq!(actual.len(), if target < 2 { 8 } else { 0 });
        }
        // Fixed f(text), independent of occurrence, batch and history, plus an
        // initially compatible cache is sufficient for direct-parser equivalence.
        let f = |text: &str| words(usize::from(text == "b"));
        let mixed = vec!["b".into(), "a".into(), "b".into()];
        for refresh in [false, true] {
            let mut cache = root.cache(refresh);
            cache.trees.insert("a".into(), f("a"));
            let expected: Vec<_> = mixed.iter().map(|x: &String| f(x)).collect();
            let mut output = cache
                .parse_with(&mixed, |batch| Ok(batch.iter().map(|x| f(x)).collect()))
                .unwrap();
            assert_eq!(output, expected);
            output[0][0].head = 99;
            assert_eq!(cache.trees["b"], f("b"));
        }
        let mut cache = root.cache(false);
        let direct = vec![words(0), words(1)];
        let output = cache.parse_with(&xs, |_| Ok(direct.clone())).unwrap();
        assert_ne!(
            output, direct,
            "occurrence-dependent parses violate the premise"
        );
        assert_eq!(output, vec![words(1), words(1)]);
        println!(
            "CHECK preimages antecedents=16 fibers=8,8,0 extensional_cases=2 counterexample=1"
        );
    }
    #[test]
    fn two_step_attempt_ledgers_and_failure_frames() {
        let root = Scratch::new();
        let xs = vec!["a".into(), "b".into(), "b".into()];
        let mut histories = 0;
        // 0=Err, 1=short, 2=long, 3=complete. The two outcomes vary independently.
        for refresh in [false, true] {
            for first in 0..4 {
                for second in 0..4 {
                    let mut cache = root.cache(refresh);
                    cache.trees.insert("a".into(), words(9));
                    cache.write().unwrap();
                    let disk = fs::read(&cache.path).unwrap();
                    let mut h = 0;
                    let mut m = 0;
                    let mut committed_occurrences = 0;
                    let mut committed_keys = std::collections::BTreeSet::new();
                    for (step, outcome) in [first, second].into_iter().enumerate() {
                        let before = cache.trees.clone();
                        let wanted: Vec<_> = xs
                            .iter()
                            .filter(|x| refresh || !before.contains_key(*x))
                            .cloned()
                            .collect();
                        h += xs.len() - wanted.len();
                        m += wanted.len();
                        let mut calls = 0;
                        let result = cache.parse_with(&xs, |batch| {
                            calls += 1;
                            assert_eq!(batch, &wanted);
                            match outcome {
                                0 => anyhow::bail!("injected parser failure"),
                                1 => Ok(vec![words(step); batch.len() - 1]),
                                2 => Ok(vec![words(step); batch.len() + 1]),
                                _ => Ok(vec![words(step); batch.len()]),
                            }
                        });
                        let success = wanted.is_empty() || outcome == 3;
                        assert_eq!(result.is_ok(), success);
                        assert_eq!(calls, usize::from(!wanted.is_empty()));
                        assert_eq!((cache.hits, cache.misses), (h, m));
                        if success {
                            committed_occurrences += wanted.len();
                            let mut expected = before;
                            for text in wanted {
                                committed_keys.insert(text.clone());
                                expected.insert(text, words(step));
                            }
                            assert_eq!(cache.trees, expected);
                        } else {
                            assert_eq!(cache.trees, before);
                        }
                        assert!(committed_occurrences <= m);
                        assert!(committed_keys.len() <= committed_occurrences);
                        assert_eq!(fs::read(&cache.path).unwrap(), disk);
                        assert!(!cache.repaired);
                        assert_eq!(cache.signature(), "synthetic");
                    }
                    histories += 1;
                    fs::remove_file(&cache.path).unwrap();
                }
            }
        }
        assert_eq!(histories, 32);
        let cache = root.cache(false);
        fs::write(&cache.path, b"invalid").unwrap();
        assert!(root.cache(false).repaired);
        let mut cache = root.cache(true);
        assert!(!cache.repaired && cache.trees.is_empty());
        for _ in 0..2 {
            cache
                .parse_with(&["a".into()], |_| Ok(vec![words(0)]))
                .unwrap();
        }
        assert_eq!((cache.hits, cache.misses), (0, 2));
        println!(
            "CHECK histories=32 steps=64 outcomes=error,short,long,complete refresh_repeated=2"
        );
    }
    #[test]
    fn envelope_admission_roundtrip_and_identity_fibers() {
        let root = Scratch::new();
        let mut cache = root.cache(false);
        cache.trees.insert("a".into(), words(4));
        cache.trees.insert("".into(), vec![]);
        cache.write().unwrap();
        let bytes = fs::read(&cache.path).unwrap();
        assert_eq!(decode(&bytes, "synthetic").unwrap(), cache.trees);
        for mask in 0..8 {
            let mut entry: Envelope = serde_json::from_slice(&bytes).unwrap();
            if mask & 1 == 0 {
                entry.schema = 2;
            }
            if mask & 2 == 0 {
                entry.identity = "other".into();
            }
            if mask & 4 == 0 {
                entry.checksum = "wrong".into();
            }
            let result = decode(&serde_json::to_vec(&entry).unwrap(), "synthetic");
            assert_eq!(result.is_ok(), mask == 7);
            if let Err(error) = result {
                assert_eq!(
                    error.to_string(),
                    if mask & 3 != 3 {
                        "cache identity mismatch"
                    } else {
                        "cache checksum mismatch"
                    }
                );
            }
        }
        // Accepted map observations cannot reconstruct raw JSON uniquely.
        let entry: Envelope = serde_json::from_slice(&bytes).unwrap();
        let pretty = serde_json::to_vec_pretty(&entry).unwrap();
        assert_ne!(bytes, pretty);
        assert_eq!(decode(&pretty, "synthetic").unwrap(), cache.trees);
        let raw = String::from_utf8(bytes.clone()).unwrap();
        let extra = format!("{} , \"unknown\": true }}", raw.strip_suffix('}').unwrap());
        assert_eq!(decode(extra.as_bytes(), "synthetic").unwrap(), cache.trees);
        assert!(decode(&bytes, "other").is_err());
        let mut edited: Envelope = serde_json::from_slice(&bytes).unwrap();
        edited.trees.get_mut("a").unwrap()[0].head = 999;
        assert!(decode(&serde_json::to_vec(&edited).unwrap(), "synthetic").is_err());
        edited.checksum = hash(&serde_json::to_vec(&edited.trees).unwrap());
        assert_eq!(
            decode(&serde_json::to_vec(&edited).unwrap(), "synthetic").unwrap(),
            edited.trees
        );
        println!("CHECK envelopes guards=8 accepted=1 representations=3 checksum_recomputed=1");
    }
}
