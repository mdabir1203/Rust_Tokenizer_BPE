use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use std::fmt::Display;

pub struct BasicTokenizer {
    pub vocab_size: usize,
    pub verbose: bool,
    pub include_punctuation: bool,
}

impl BasicTokenizer {
    pub fn new(vocab_size: usize, verbose: bool, include_punctuation: bool) -> Self {
        BasicTokenizer {
            vocab_size,
            verbose,
            include_punctuation
        }
    }

    pub fn tokenize(&self, text: &dyn Display) -> Vec<String> {
        let tokens: Vec<String> = if self.include_punctuation {
            // Using unicode segmentation -> handling punctuation + whitespace
            UnicodeSegmentation::unicode_words_with_whitespace(text)
            .map(|s| s.to_string())
            .collect()
        } else {
            text.unicode_words().map(|s| s.to_string()).collect()
        };

        if self.verbose {
            println!("Tokenized text: {:?}", tokens);
        }

        tokens
    }

    // Building vocab from texts : 
    // Improve / modify on project needs 
    pub fn build_vocabulary(&self, texts: Vec<&dyn Display>) -> HashMap<String, usize> {
        let mut vocab: HashMap<String, usize> = HashMap::new();
        
        for text in texts {
            let tokens = self.tokenize(text);
            for token in tokens {
                *vocab.entry(token).or_insert(0) += 1;
            }
        }
    if self.vocab_size > 0 {
        let mut sorted_vocab: Vec<_> = vocab.into_iter().collect();
        sorted_vocab.sort_by(|a, b| b.1.cmp(&a.1));
        vocab = sorted_vocab.into_iter().take(self.vocab_size).collect();
    }

    if self.verbose {
        println!("Vocabulary: {:?}", vocab);
    }

    vocab
 }
}