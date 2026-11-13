



pub fn decode(merged_chunk: &Vec<u32>, vocab: &Vec<Vec<u8>>) -> String {
    let mut original_byte_tokens :Vec<u8> = Vec::new();

    for bt in merged_chunk {


        original_byte_tokens.extend(vocab[*bt as usize].clone());
    }
    let s = String::from_utf8(original_byte_tokens).expect("Invalid UTF-8");

    return s


}

#[cfg(test)]
mod test {
    use crate::train::{
    initialize_vocab,
    update_vocabulary,
    };
    use super::*;
    #[test]
    fn test_decode() {
        let vocab = initialize_vocab();

        let mut vocab = update_vocabulary(vocab, (108, 108)); // 256 = ll
        vocab = update_vocabulary(vocab, (101, 256));        // 257 = ell

        let byte_chunk = vec![72, 101, 257, 111]; // Heello

        let expected = String::from("Heello");

        assert_eq!(decode(&byte_chunk, &vocab), expected);
    }


}