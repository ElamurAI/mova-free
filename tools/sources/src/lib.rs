//! sources as a library: ACL Anthology BibTeX parsing and paper manifest rows — shared by
//! `sources bib`/`bib-ids` and `tools/treebanks` (matching treebank citations to ACL Anthology).

pub mod bib;
#[cfg(test)]
mod wave33_tests {
    #[test]
    fn public_bib_namespace_compiles_and_projects() {
        let entries: Vec<crate::bib::Entry> =
            crate::bib::parse("@misc{k,title={A {B}},year={2016}}");
        assert_eq!(entries[0].get("title"), "A B");
        assert_eq!(crate::bib::row(&entries[0], "s", "t", 1)["kind"], "paper");
    }
}
