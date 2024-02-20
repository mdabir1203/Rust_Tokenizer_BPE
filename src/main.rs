use tokenizer_project::BasicTokenizer;

fn main() {
    let tokenizer = BasicTokenizer::new(1000, false, false);
    let text_sample = "Rusting it out bruv !+!";
    let tokens = tokenizer.tokenize(text_sample);
    println!("Tokens: {:?}", tokens);


    let texts = vec![
        text_sample,
        "Rusting it out bruv !+!",
    ];
    let vocab = tokenizer.build_vocabulary(texts);
    println!("Vocabulary: {:?}", vocab);
}

