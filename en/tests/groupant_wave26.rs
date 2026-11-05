//! Original finite composition fixtures: lossless representation, raw tree gate,
//! and Kahn-style functional graph decomposition retain distinct contracts.
use en::{conllu::Doc, convert::tree_gate};
use mova_algorithms::functional_graph_decomposition::decompose;

fn text(heads: &[usize]) -> String {
    let mut out = String::from("# sent_id = composition\n");
    for (i, &h) in heads.iter().enumerate() {
        out += &format!(
            "{}\tw{}\t_\tNOUN\tNN\t_\t{h}\t{}\t_\t_\n",
            i + 1,
            i + 1,
            if h == 0 { "root" } else { "dep" }
        );
    }
    out
}

fn successors(heads: &[usize]) -> Vec<usize> {
    let mut out = vec![0];
    out.extend_from_slice(heads);
    out
}

// Independent per-token walk; uses neither component IDs nor parser tokens.
fn first_nonterminating(heads: &[usize]) -> Option<usize> {
    (1..=heads.len()).find(|&start| {
        let mut v = start;
        for _ in 0..heads.len() {
            if v == 0 {
                return false;
            }
            v = heads[v - 1];
        }
        v != 0
    })
}

fn check(heads: &[usize]) {
    let raw = text(heads);
    let doc = Doc::parse(&raw).unwrap();
    assert_eq!(doc.text(), raw);
    let sentence = &doc.sents[0];
    assert_eq!(
        sentence
            .sent
            .tokens
            .iter()
            .map(|t| t.head)
            .collect::<Vec<_>>(),
        heads
    );
    let graph = decompose(&successors(heads)).unwrap();
    assert_eq!(graph.cycles[graph.cycle_id[0]], vec![0]);
    let failing = (1..=heads.len()).find(|&v| graph.cycle_id[v] != graph.cycle_id[0]);
    assert_eq!(failing, first_nonterminating(heads));
    let expected = failing.map_or(Ok(()), |v| {
        Err(format!("tree gate: cycle through word {v}"))
    });
    assert_eq!(tree_gate(sentence), expected, "heads={heads:?}");
    for v in 1..=heads.len() {
        if graph.cycle_id[v] == graph.cycle_id[0] {
            assert_eq!(graph.entry[v], 0);
            let mut x = v;
            for _ in 0..graph.distance[v] {
                x = successors(heads)[x];
            }
            assert_eq!(x, 0);
        }
    }
}

#[test]
fn exhaustive_head_relation() {
    let mut cases = 0;
    for n in 1..=5usize {
        for mut code in 0..(n + 1).pow(n as u32) {
            let mut heads = vec![0; n];
            for h in &mut heads {
                *h = code % (n + 1);
                code /= n + 1;
            }
            if heads.iter().filter(|&&h| h == 0).count() != 1 {
                continue;
            }
            check(&heads);
            cases += 1;
        }
    }
    assert_eq!(cases, 3413);
}

#[test]
fn coordinates_first_failure_and_threshold() {
    assert_eq!(successors(&[3, 0, 4, 3]), vec![0, 3, 0, 4, 3]);
    check(&[3, 0, 4, 3]); // token 1 is a tail into a cycle not containing it.
    check(&[1, 0, 3]); // self-loop and multiple nonroot components.
    check(&[]);
    for n in [64, 65, 66] {
        let mut heads: Vec<_> = (0..n).collect();
        check(&heads);
        heads[1] = n;
        heads[n - 1] = n - 1;
        check(&heads);
    }
}

#[test]
fn raw_head_preflight_and_error_precedence() {
    for bad in ["_", "-1", "not-a-number", "184467440737095516160"] {
        let raw = format!("1\tx\t_\tNOUN\tNN\t_\t{bad}\troot\t_\t_\n");
        let doc = Doc::parse(&raw).unwrap();
        assert_eq!(doc.text(), raw);
        assert_eq!(doc.sents[0].sent.tokens[0].head, 0);
        assert_eq!(
            tree_gate(&doc.sents[0]),
            Err(format!("tree gate: word 1: head '{bad}' is not a number"))
        );
    }
    let cases = [
        (
            "1\tx\t_\tNOUN\tNN\t_\t2\tdep\t_\t_\n",
            "tree gate: word 1: head 2 outside the sentence (1 words)",
        ),
        (
            "1\tx\t_\tNOUN\tNN\t_\t0\tdep\t_\t_\n",
            "tree gate: word 1: head 0, relation 'dep' — root only on the root",
        ),
        (
            "1\tx\t_\tNOUN\tNN\t_\t1\troot\t_\t_\n",
            "tree gate: word 1: head 1, relation 'root' — root only on the root",
        ),
    ];
    for (raw, error) in cases {
        let doc = Doc::parse(raw).unwrap();
        assert_eq!(tree_gate(&doc.sents[0]), Err(error.into()));
    }
    for (heads, roots) in [(vec![1], 0), (vec![0, 0], 2)] {
        let doc = Doc::parse(&text(&heads)).unwrap();
        assert_eq!(
            tree_gate(&doc.sents[0]),
            Err(format!("tree gate: roots {roots}, expected one"))
        );
    }
    // A later raw error precedes an earlier cyclic head during preflight.
    let raw = text(&[1, 0]).replace("\t0\troot", "\t_\troot");
    let doc = Doc::parse(&raw).unwrap();
    assert_eq!(
        tree_gate(&doc.sents[0]),
        Err("tree gate: word 2: head '_' is not a number".into())
    );
}

#[test]
fn representation_domain_is_separate() {
    assert!(Doc::parse("").unwrap().sents.is_empty());
    let error = Doc::parse("1\tx\n").unwrap_err();
    assert_eq!(error.to_string(), "line 1: 2 columns, expected 10");
    let error = Doc::parse("2\tx\t_\tNOUN\tNN\t_\t0\troot\t_\t_\n").unwrap_err();
    assert_eq!(
        format!("{error:#}"),
        "sentence at line 1: : word with ID '2' out of place (expected 1)"
    );
    let error = Doc::parse("1\tx\t_\tNOUN\tNN\t_\t0\tunknown\t_\t_\n").unwrap_err();
    assert_eq!(
        format!("{error:#}"),
        "sentence at line 1: : unknown UD relation 'unknown'"
    );
}

#[test]
fn multiword_and_empty_rows_preserve_raw_coordinates() {
    let raw = "# sent_id = mwt\n1-2\tcan't\t_\t_\t_\t_\tbad\t_\t_\tSpaceAfter=No\n1\tcan\t_\tAUX\tMD\t_\t0\troot\t_\t_\n2\tnot\t_\tPART\tRB\t_\t1\tdep\t_\t_\n2.1\tghost\t_\t_\t_\t_\tbad\t_\t_\t_\n";
    let doc = Doc::parse(raw).unwrap();
    assert_eq!(doc.text(), raw);
    let s = &doc.sents[0];
    assert_eq!(s.rows.len(), 4);
    assert_eq!(s.words, vec![1, 2]);
    assert_eq!(s.sent.tokens.len(), 2);
    assert_eq!(
        s.sent.tokens.iter().map(|t| t.head).collect::<Vec<_>>(),
        vec![0, 1]
    );
    assert_eq!(tree_gate(s), Ok(()));
    let graph = decompose(&successors(&[0, 1])).unwrap();
    assert_eq!(graph.distance, vec![0, 1, 2]);
}
