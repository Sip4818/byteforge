use crate::{decoding, encoding, training};

pub struct BpeTokenizer {
    pub vocab: Vec<Vec<u8>>,
    pub merges: Vec<((u32, u32), u32)>,
}

impl BpeTokenizer {
    pub fn train(corpus: &str, max_vocab_size: usize) -> Self {
        training::train(corpus, max_vocab_size)
    }

    pub fn encode(&self, text: &str) -> Vec<u32> {
        encoding::encode(text, &self.merges)
    }
    pub fn decode(&self, tokens: &Vec<u32>) -> String {
        decoding::decode(tokens, &self.vocab)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let corpus = "Hello world! Rust is great. Hello Rust!";

        let tokenizer = BpeTokenizer::train(corpus, 300);

        let test_text = "Hello Rust! Testing 123, café 🚀";

        let encoded = tokenizer.encode(test_text);
        let decoded = tokenizer.decode(&encoded);

        assert_eq!(test_text, decoded);
    }
}
