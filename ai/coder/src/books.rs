//! Exercise extracts from the library's open books (`data/raw/codelearn-book-*`): pdftotext (poppler, as in
//!) → pages → exercise labels → excerpts with the PDF page number. Licences checked against the text of the
//! books themselves: Erickson — CC BY 4.0; ODS — CC BY; SICP (2nd ed., mitpress edition) — CC BY-SA 4.0.

use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::util::home;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Excerpt {
    pub book: String,
    pub label: String,
    pub page: usize,
    pub text: String,
}

pub struct Book {
    pub key: &'static str,
    pub slug: &'static str,
    pub title: &'static str,
    pub license: &'static str,
}

pub const BOOKS: &[Book] = &[
    Book { key: "erickson", slug: "codelearn-book-2019-erickson-algorithms", title: "Jeff Erickson, Algorithms (2019)", license: "CC BY 4.0" },
    Book { key: "ods", slug: "codelearn-book-2013-morin-open-data-structures-python-edition", title: "Pat Morin, Open Data Structures (pseudocode edition 0.1Gβ)", license: "CC BY" },
    Book { key: "sicp", slug: "codelearn-book-1996-abelson-structure-interpretation-computer-programs-2nd", title: "Abelson, Sussman & Sussman, Structure and Interpretation of Computer Programs, 2nd ed.", license: "CC BY-SA 4.0" },
];

fn pdftotext(slug: &str) -> Result<String> {
    let bin = home().join("bin/poppler/usr/bin");
    let pdf: PathBuf = home().join("raw").join(slug).join(format!("{slug}.pdf"));
    let out = Command::new(bin.join("pdftotext"))
        .env("LD_LIBRARY_PATH", bin.join("../lib/x86_64-linux-gnu"))
        .args(["-q", "-enc", "UTF-8"])
        .arg(&pdf)
        .arg("-")
        .output()
        .context("pdftotext")?;
    if !out.status.success() {
        bail!("pdftotext {}", pdf.display());
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Exercise label at the start of a line: SICP "Exercise 1.11:", ODS "Exercise 2.1.", Erickson — "N." after "Exercises".
fn label_of(key: &str, line: &str, in_ex: bool, chapter: usize) -> Option<String> {
    let l = line.trim_start();
    match key {
        "sicp" | "ods" => {
            let rest = l.strip_prefix("Exercise ")?;
            let num: String = rest.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
            let num = num.trim_end_matches('.');
            if num.contains('.') && num.len() >= 3 { Some(format!("Exercise {num}")) } else { None }
        }
        _ => {
            if !in_ex {
                return None;
            }
            let num: String = l.chars().take_while(|c| c.is_ascii_digit()).collect();
            if !num.is_empty() && l[num.len()..].starts_with(". ") && num.len() <= 2 {
                Some(format!("Chapter {chapter}, Exercise {num}"))
            } else {
                None
            }
        }
    }
}

pub fn excerpts(book: &Book, max_chars: usize) -> Result<Vec<Excerpt>> {
    let text = pdftotext(book.slug)?;
    let mut out: Vec<Excerpt> = Vec::new();
    let mut cur: Option<Excerpt> = None;
    let mut in_ex = false;
    let mut chapter = 0usize;
    let mut sections = 0usize;
    for (p, page) in text.split('\u{c}').enumerate() {
        for line in page.lines() {
            if book.key == "erickson" && line.trim() == "Exercises" {
                in_ex = true;
                chapter = sections;
                sections += 1;
                if let Some(e) = cur.take() {
                    out.push(e);
                }
                continue;
            }
            if let Some(label) = label_of(book.key, line, in_ex, chapter) {
                if let Some(e) = cur.take() {
                    out.push(e);
                }
                cur = Some(Excerpt { book: book.key.into(), label, page: p + 1, text: line.trim().to_string() });
                continue;
            }
            if let Some(e) = cur.as_mut() {
                if e.text.len() < max_chars && !line.trim().is_empty() {
                    e.text.push(' ');
                    e.text.push_str(line.trim());
                }
            }
        }
    }
    if let Some(e) = cur.take() {
        out.push(e);
    }
    for e in &mut out {
        if e.text.len() > max_chars {
            let cut = e.text.char_indices().take_while(|(i, _)| *i < max_chars).last().map(|(i, _)| i).unwrap_or(0);
            e.text.truncate(cut);
            e.text.push_str(" …");
        }
    }
    Ok(out)
}

/// Selection for the prompt: chapters where the exercises are computational (Erickson 1–3 — recursion, backtracking, DP; ODS 1–2, 11;
/// SICP 1–2), up to `n` items, evenly spread.
pub fn pick(book: &Book, all: &[Excerpt], n: usize) -> Vec<Excerpt> {
    let ok: Vec<&Excerpt> = all
        .iter()
        .filter(|e| {
            let chap: usize = e
                .label
                .trim_start_matches("Chapter ")
                .trim_start_matches("Exercise ")
                .split(|c: char| !c.is_ascii_digit())
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(99);
            let chapters: &[usize] = match book.key {
                "erickson" => &[1, 2, 3],
                "ods" => &[1, 2, 11],
                _ => &[1, 2],
            };
            chapters.contains(&chap) && e.text.len() > 60
        })
        .collect();
    if ok.len() <= n {
        return ok.into_iter().cloned().collect();
    }
    let step = ok.len() as f64 / n as f64;
    (0..n).map(|k| ok[(k as f64 * step) as usize].clone()).collect()
}
