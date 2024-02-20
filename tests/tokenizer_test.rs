use tokenizer_project::BasicTokenizer;

#[test]
fn test_tokenization() {
    let tokenizer = BasicTokenizer::new(0, false, true);
    let text = "Testing, one, two.";
    let tokens = tokenizer.tokenize(text);
    assert_eq!(tokens, vec!["Testing", ",", "one", ",", "two", "."]);
}