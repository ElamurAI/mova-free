//! Empty input is legal at the public tokenized API boundary.
#[test]
fn decoder_accepts_no_words() {
    assert!(gparse::eisner::decode(&[vec![0.]]).is_empty());
}

#[test]
#[ignore = "requires GP_TEST_MODEL and CUDA"]
fn gpu_empty_sentences_preserve_positions() {
    let model = gparse::load(&std::env::var("GP_TEST_MODEL").expect("GP_TEST_MODEL")).unwrap();
    assert!(model.dev.is_cuda());
    let words = vec!["Cats".to_owned(), "sleep".to_owned(), ".".to_owned()];
    let reference = gparse::parse_forms(&model, &[words.clone()]).unwrap();
    let empty = gparse::parse_forms(&model, &[vec![], vec![]]).unwrap();
    assert_eq!(empty, vec![vec![], vec![]]);
    let mixed = gparse::parse_forms(&model, &[vec![], words.clone(), vec![], words]).unwrap();
    assert_eq!(mixed, vec![vec![], reference[0].clone(), vec![], reference[0].clone()]);
    let sentences = vec![gparse::data::Sent { comments: vec![], words: vec![], roles: vec![] }];
    let (trees, scores) = gparse::predict_scored(&model.net, &model.vocs, &model.dev, &sentences).unwrap();
    assert_eq!(trees, vec![vec![]]);
    assert_eq!(scores, vec![0.]);
}
