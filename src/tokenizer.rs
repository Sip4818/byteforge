use crate:: {
    training,
    encoding,
    decoding
};

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

