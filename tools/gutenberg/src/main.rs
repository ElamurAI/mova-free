//! Project Gutenberg books for the corpus: select by group from `pg_catalog.csv`, download slowly.
//!
//!   gutenberg select <pg_catalog.csv> <out.jsonl> <group>... [--max-died YEAR] [--skip <dir>]... [--not-pd]
//!   gutenberg fetch <list.jsonl> <out-root> [--pause SECONDS]
//!
//! Groups (matched on Subjects and Bookshelves, case-insensitive): `titanic`, `shipwreck` (marine accidents,
//! lifeboats, rescue, named liners), `disaster` (earthquakes, floods, fires, mine and railroad accidents, explosions,
//! epidemics — war books left out), `detective` (detective and mystery stories), `tales` (fairy tales, folklore, fables,
//! legends, mythology), `erotic` (erotic literature, sexuality & erotica), `children`, `fiction`. A book goes
//! to the first group it matches, in the order given. Only English texts whose every dated person died in
//! `--max-died` or earlier (default 1955: public domain in the EU and Ukraine in 2026, life + 70); books with
//! an undated person are skipped. `--not-pd` selects the opposite: the books that are not (or not provably)
//! public domain in the EU/UA — for analysis logs only (gap journal, unknown words, illogic, absurdity), never
//! as corpus text, trees or QA answer texts; they go to a separate root. `--skip <dir>` skips ids already present in a folder (`pg<id>.txt` files or `…-pg-<id>` folders).
//! `fetch` writes `<out-root>/<group>/pg<id>.txt` from the PGLAF mirror (aleph.pglaf.org/cache/epub, the mirror
//! Project Gutenberg asks bulk downloads to use), skips existing files,
//! pauses between requests (default 2 s), logs books without a plain text to `missing.txt` and stops
//! after 10 failures in a row (fail-fast).

use std::collections::BTreeSet;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde_json::{Value, json};

/// RFC 4180 CSV: quoted fields, doubled quotes, newlines inside quotes.
fn csv_rows(text: &str) -> Vec<Vec<String>> {
    let (mut rows, mut row, mut field) = (Vec::new(), Vec::new(), String::new());
    let mut q = false;
    let mut it = text.chars().peekable();
    while let Some(c) = it.next() {
        match (q, c) {
            (true, '"') if it.peek() == Some(&'"') => {
                field.push('"');
                it.next();
            }
            (true, '"') => q = false,
            (true, c) => field.push(c),
            (false, '"') => q = true,
            (false, ',') => row.push(std::mem::take(&mut field)),
            (false, '\n') => {
                row.push(std::mem::take(&mut field));
                rows.push(std::mem::take(&mut row));
            }
            (false, '\r') => {}
            (false, c) => field.push(c),
        }
    }
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    rows
}

const GROUPS: [(&str, &[&str]); 10] = [
    // the world of time: clocks, calendars, the measurement and the idea of time
    ("time", &["clocks and watches", "clock and watch", "time measurements", "time --", "calendar", "chronology", "sundials", "horology", "space and time", "relativity (physics)"]),
    // the Titanic subcorpus (corpus 0.4): the Titanic itself, accidents at sea, other disasters of 1812–1932, and
    // the detective stories that read testimonies the same way
    ("titanic", &["titanic (steamship)", "titanic"]),
    ("shipwreck", &["shipwrecks", "marine accidents", "steamboat disasters", "survival after airplane accidents, shipwrecks", "lifeboats", "lifesaving", "rescue work", "ship fires", "lusitania (steamship)", "empress of ireland", "general slocum", "eastland"]),
    ("disaster", &["disasters", "earthquakes", "floods", "fires --", "great fire", "mine accidents", "mine explosions", "railroad accidents", "explosions", "epidemics", "volcanic eruptions", "hurricanes", "tornadoes", "famines", "avalanches"]),
    ("detective", &["detective and mystery stories", "mystery fiction", "crime -- fiction", "detective"]),
    ("legal", &["law --", "legislation", "statutes", "constitutional law", "treaties", "constitutions", "legal documents", "law -- united states", "contracts"]),
    ("tales", &["fairy tales", "folklore", "fables", "legends", "mythology", "fairy tales, etc"]),
    ("erotic", &["erotic", "sexuality & erotica"]),
    ("children", &["children's", "juvenile"]),
    ("fiction", &["fiction"]),
];

/// Death years of all persons in the Authors field ("Name, 1824-1905; Other, 1854-1916 [Editor]").
/// `None` if some person has no death year (undated or still living / unknown).
fn death_years(authors: &str) -> Option<Vec<u32>> {
    let mut out = Vec::new();
    for p in authors.split(';').map(str::trim).filter(|p| !p.is_empty()) {
        // the last "-YYYY" in the entry; BCE dates count as dead
        if p.contains("BCE") {
            out.push(0);
            continue;
        }
        let d = p.rsplit_once('-').map(|(_, r)| r.chars().skip_while(|c| !c.is_ascii_digit()).take_while(char::is_ascii_digit).collect::<String>());
        match d.and_then(|d| d.parse::<u32>().ok()) {
            Some(y) if y > 0 => out.push(y),
            _ => return None,
        }
    }
    (!out.is_empty()).then_some(out)
}

fn select(catalog: &Path, out: &Path, groups: &[String], max_died: u32, skip: &[String], not_pd: bool) -> Result<()> {
    for g in groups {
        if !GROUPS.iter().any(|x| x.0 == g) {
            bail!("unknown group {g}; known: {}", GROUPS.iter().map(|x| x.0).collect::<Vec<_>>().join(", "));
        }
    }
    let mut have: BTreeSet<String> = BTreeSet::new();
    for d in skip {
        for e in std::fs::read_dir(d).with_context(|| d.clone())? {
            let n = e?.file_name().to_string_lossy().to_string();
            // pg<id>.txt or a folder named …-pg-<id> / …-pg<id>-…
            if let Some(k) = n.rfind("pg") {
                let id: String = n[k + 2..].trim_start_matches('-').chars().take_while(char::is_ascii_digit).collect();
                if !id.is_empty() {
                    have.insert(id);
                }
            }
        }
    }
    let rows = csv_rows(&std::fs::read_to_string(catalog)?);
    let head = rows.first().context("empty catalog")?;
    let col = |n: &str| head.iter().position(|h| h == n).with_context(|| format!("no column {n}"));
    let (ci, ct, cty, cl, ca, cs, cb) = (col("Text#")?, col("Title")?, col("Type")?, col("Language")?, col("Authors")?, col("Subjects")?, col("Bookshelves")?);
    let mut f = std::io::BufWriter::new(std::fs::File::create(out)?);
    let mut counts: Vec<(String, usize)> = groups.iter().map(|g| (g.clone(), 0)).collect();
    let (mut not_pd_n, mut skipped) = (0usize, 0usize);
    for r in rows.iter().skip(1).filter(|r| r.len() == head.len()) {
        if r[cty] != "Text" || r[cl] != "en" {
            continue;
        }
        let hay = format!("{} ; {}", r[cs], r[cb]).to_lowercase();
        let Some(gi) = groups.iter().position(|g| GROUPS.iter().find(|x| x.0 == g).unwrap().1.iter().any(|k| hay.contains(k))) else { continue };
        // war is described less: a disaster book about war, battles or the military is left out (accidents at sea
        // and the Titanic stay whatever else they touch)
        if groups[gi] == "disaster" && ["world war", "battles", "military", "war of", "-- campaigns", "naval operations"].iter().any(|w| hay.contains(w)) {
            continue;
        }
        let died = match death_years(&r[ca]) {
            Some(ys) if ys.iter().all(|&y| y <= max_died) => Some(ys.into_iter().max().unwrap_or(0)),
            _ => None,
        };
        if died.is_some() == not_pd {
            not_pd_n += 1;
            continue;
        }
        if have.contains(&r[ci]) {
            skipped += 1;
            continue;
        }
        counts[gi].1 += 1;
        writeln!(f, "{}", json!({"id": r[ci], "group": groups[gi], "title": r[ct], "authors": r[ca], "died": died, "public_domain_eu_ua": !not_pd, "subjects": r[cs]}))?;
    }
    f.flush()?;
    println!("{}  ({} the other side of the public-domain line: {not_pd_n}, already present: {skipped})", counts.iter().map(|(g, n)| format!("{g} {n}")).collect::<Vec<_>>().join(", "), if not_pd { "public domain (skipped)," } else { "not public domain in EU/UA or undated (skipped)," });
    Ok(())
}

fn fetch(list: &Path, root: &Path, pause: u64) -> Result<()> {
    let items: Vec<Value> = std::fs::read_to_string(list)?.lines().filter(|l| !l.trim().is_empty()).map(serde_json::from_str).collect::<Result<_, _>>()?;
    let todo: Vec<&Value> = items.iter().filter(|v| !root.join(v["group"].as_str().unwrap_or("x")).join(format!("pg{}.txt", v["id"].as_str().unwrap_or(""))).exists()).collect();
    eprintln!("{} books, {} to fetch (pause {pause}s)", items.len(), todo.len());
    let mut fails = 0usize;
    for (i, v) in todo.iter().enumerate() {
        let (id, group) = (v["id"].as_str().context("id")?, v["group"].as_str().context("group")?);
        let dir = root.join(group);
        std::fs::create_dir_all(&dir)?;
        let path = dir.join(format!("pg{id}.txt"));
        let tmp = dir.join(format!(".pg{id}.part"));
        let url = format!("https://aleph.pglaf.org/cache/epub/{id}/pg{id}.txt");
        // fallback: the classic files tree (/1/0/6/1061/1061-0.txt, then 1061.txt)
        let tree = if id.len() == 1 { format!("0/{id}") } else { format!("{}/{id}", id[..id.len() - 1].chars().map(String::from).collect::<Vec<_>>().join("/")) };
        let urls = [url.clone(), format!("https://aleph.pglaf.org/{tree}/{id}-0.txt"), format!("https://aleph.pglaf.org/{tree}/{id}.txt")];
        let mut ok = false;
        for u in &urls {
            if Command::new("curl").args(["-sfL", "--max-time", "120", "-A", "mova-corpus/0.1", "-o"]).arg(&tmp).arg(u).status()?.success() {
                ok = true;
                break;
            }
        }
        if !ok {
            let _ = std::fs::remove_file(&tmp);
            // some books have no plain-text file; log and go on, but stop on a run of failures
            eprintln!("[{}/{}] pg{id}: no text ({url})", i + 1, todo.len());
            std::fs::OpenOptions::new().create(true).append(true).open(root.join("missing.txt"))?.write_all(format!("{id}\t{group}\n").as_bytes())?;
            fails += 1;
            if fails >= 10 {
                bail!("10 failures in a row, stopping at pg{id}");
            }
        } else {
            fails = 0;
            std::fs::rename(&tmp, &path)?;
            if (i + 1) % 50 == 0 {
                eprintln!("[{}/{}] pg{id} {group}", i + 1, todo.len());
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(pause));
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
    match args.first().map(String::as_str) {
        Some("select") if args.len() >= 4 => {
            let skip: Vec<String> = args.windows(2).filter(|w| w[0] == "--skip").map(|w| w[1].clone()).collect();
            let mut pos = Vec::new();
            let mut i = 1;
            while i < args.len() {
                if args[i] == "--not-pd" { i += 1 } else if args[i].starts_with("--") { i += 2 } else { pos.push(args[i].clone()); i += 1 }
            }
            let max = opt("--max-died").map(|s| s.parse()).transpose()?.unwrap_or(1955);
            select(Path::new(&pos[0]), Path::new(&pos[1]), &pos[2..], max, &skip, args.iter().any(|a| a == "--not-pd"))
        }
        Some("fetch") if args.len() >= 3 => fetch(Path::new(&args[1]), Path::new(&args[2]), opt("--pause").map(|s| s.parse()).transpose()?.unwrap_or(2)),
        _ => bail!("usage: gutenberg select <pg_catalog.csv> <out.jsonl> <group>... [--max-died YEAR] [--skip <dir>]... [--not-pd] | gutenberg fetch <list.jsonl> <out-root> [--pause SECONDS]"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_and_death_years() {
        let r = csv_rows("a,b\n1,\"x, \"\"y\"\"\nz\"\n");
        assert_eq!(r[1], vec!["1".to_string(), "x, \"y\"\nz".to_string()]);
        assert_eq!(death_years("MacDonald, George, 1824-1905"), Some(vec![1905]));
        assert_eq!(death_years("Aesop, 621? BCE-565? BCE; Jacobs, Joseph, 1854-1916 [Editor]"), Some(vec![0, 1916]));
        assert_eq!(death_years("Bierce, Ambrose, 1842-1914?"), Some(vec![1914]));
        // negative control: living or undated persons are not public domain
        assert_eq!(death_years("Someone, 1950-"), None);
        assert_eq!(death_years("Anonymous"), None);
        assert_eq!(death_years(""), None);
    }
}
