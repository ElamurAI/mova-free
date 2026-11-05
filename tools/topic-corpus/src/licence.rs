//! Licence classes of corpus documents and the public-admission rule.
//!
//! Everything in a closed topic corpus is internal. A document may feed a *public* output only when
//! it is a library paper under CC BY, CC0 or public domain (no NC, ND or SA terms) whose text is not
//! local-only. Pages fetched by Sea Spider (`origin = seaspider`, kind `web`) and draft articles by an
//! Owl's consultant (`origin = sol`) are never admitted, whatever licence the site states: the
//! operator's rule of 2026-11-04 keeps Sea Spider's downloads and their trees out of every public
//! corpus and site.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Class {
    /// CC0 or public domain.
    PublicDomain,
    /// CC BY without NC, ND or SA.
    By,
    /// CC BY-SA (share-alike): quote and test only.
    ShareAlike,
    /// Any NC term.
    NonCommercial,
    /// ND without NC.
    NoDerivatives,
    /// arXiv's non-exclusive distribution licence, free-to-read copies, all rights reserved.
    Restricted,
    /// No licence stated.
    Unknown,
}

/// The class of a licence statement as the library or a site writes it («CC BY 4.0 (arXiv)»,
/// «CC BY-NC-SA 3.0 (ACL Anthology, до 2016)», «PD: …», «arXiv — non-exclusive …», a CC URL).
pub fn classify(s: &str) -> Class {
    let l = s.to_lowercase();
    let l = l.trim();
    if l.is_empty() {
        return Class::Unknown;
    }
    let cc = l.contains("cc by") || l.contains("cc-by") || l.contains("creativecommons.org/licenses/by") || l.starts_with("by-") || l == "by";
    if l.contains("cc0") || l.contains("publicdomain") || l.starts_with("pd") || l.contains("public domain") || l.contains("public-domain") {
        return Class::PublicDomain;
    }
    if cc || l.contains("creative commons") {
        let t = l.replace(['_', ' '], "-");
        if t.contains("-nc") || t.contains("noncommercial") || t.contains("non-commercial") {
            return Class::NonCommercial;
        }
        if t.contains("-nd") || t.contains("noderiv") || t.contains("no-deriv") {
            return Class::NoDerivatives;
        }
        if t.contains("-sa") || t.contains("sharealike") || t.contains("share-alike") {
            return Class::ShareAlike;
        }
        return Class::By;
    }
    if l.contains("non-exclusive") || l.contains("nonexclusive") || l.contains("all rights reserved") || l.contains("copyright") || l.contains("free to read") || l.contains("free-to-read") {
        return Class::Restricted;
    }
    Class::Unknown
}

/// Where a document came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    /// A text of the paper library.
    Library,
    /// A page fetched by Sea Spider.
    Seaspider,
    /// A draft article written by an Owl's consultant.
    Sol,
    /// A note we prepared from library metadata (an author without papers in the library).
    Notes,
}

/// Whether a document may feed a public output (with attribution). Library texts only, open
/// licence only, never local-only; Sea Spider pages and consultant drafts never.
pub fn public_admissible(origin: Origin, class: Class, local_only: bool) -> bool {
    origin == Origin::Library && !local_only && matches!(class, Class::By | Class::PublicDomain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_of_library_statements() {
        assert_eq!(classify("CC BY 4.0 (arXiv)"), Class::By);
        assert_eq!(classify("CC BY-NC-SA 3.0 (ACL Anthology, до 2016)"), Class::NonCommercial);
        assert_eq!(classify("CC BY-NC-ND (journal licence per DOAJ)"), Class::NonCommercial);
        assert_eq!(classify("CC BY-ND 4.0"), Class::NoDerivatives);
        assert_eq!(classify("CC BY-SA 4.0"), Class::ShareAlike);
        assert_eq!(classify("https://creativecommons.org/licenses/by/4.0/"), Class::By);
        assert_eq!(classify("cc-by"), Class::By);
        assert_eq!(classify("CC0 1.0"), Class::PublicDomain);
        assert_eq!(classify("PD: author † 1926"), Class::PublicDomain);
        assert_eq!(classify("arXiv — non-exclusive license to distribute"), Class::Restricted);
        assert_eq!(classify(""), Class::Unknown);
    }

    #[test]
    fn web_and_sol_are_never_public() {
        assert!(public_admissible(Origin::Library, Class::By, false));
        assert!(!public_admissible(Origin::Library, Class::By, true));
        assert!(!public_admissible(Origin::Library, Class::ShareAlike, false));
        assert!(!public_admissible(Origin::Library, Class::NonCommercial, false));
        // A page under CC BY stays internal: Sea Spider's downloads never go public.
        assert!(!public_admissible(Origin::Seaspider, Class::By, false));
        assert!(!public_admissible(Origin::Seaspider, Class::PublicDomain, false));
        assert!(!public_admissible(Origin::Sol, Class::By, false));
        assert!(!public_admissible(Origin::Notes, Class::PublicDomain, false));
    }
}
