use std::collections::HashMap;
use crate::preprocessing:: {
    pretokenization,
    chunks_to_byte_ids
};



pub fn create_pairs_from_chunk(chunk: Vec<u32>) -> Vec<(u32, u32)>{
    let n = chunk.len();

    let mut pairs: Vec<(u32, u32)> = Vec::new();
        if n == 0 {
        return pairs;
    }
    for i in 0..n-1 {
        pairs.push((chunk[i], chunk[i + 1]));
    }
    pairs
    
}


pub fn count_pairs(bytes_chunks: &Vec<Vec<u32>>) -> HashMap<(u32, u32), usize>{
    let mut pair_frequency : HashMap<(u32, u32), usize> = HashMap::new();
    for chunk in bytes_chunks {
        let pairs = create_pairs_from_chunk(chunk.clone());
        for pair in pairs{
            *pair_frequency.entry(pair).or_insert(0) += 1;
        }
    }
    pair_frequency


}
pub fn get_most_frequent_pair(pair_frequency: HashMap<(u32, u32), usize>) -> Option<((u32,u32), usize)>{
    let mut most_freq_pair: Option<((u32, u32), usize)> = None;
    let mut max_freq: usize = 0;

    for (pair, freq) in &pair_frequency {
        if *freq > max_freq{
            max_freq = *freq;
            most_freq_pair = Some((*pair, *freq));
        }  
    }
    return most_freq_pair 

}

pub fn initialize_vocab() -> Vec<Vec<u8>> {
    let mut vocab: Vec<Vec<u8>> = Vec::new();
    for i in 0..=255 {
        vocab.push(vec![i]);
    }
    vocab
}

pub fn update_vocabulary(mut vocab: Vec<Vec<u8>>, new_pair: (u32,u32))->Vec<Vec<u8>>{
    let mut new_token_combo = vocab[new_pair.0 as usize].clone();

    new_token_combo.extend(vocab[new_pair.1 as usize].clone());        
    vocab.push(new_token_combo);

    vocab
    
    

}

pub struct BpeTokenizer {
    pub vocab: Vec<Vec<u8>>,
    pub merges: Vec<((u32, u32), u32)>,
}
pub fn train_bpe(corpus: &str, max_vocab_size: usize) -> BpeTokenizer{
    let chunked_corpus: Vec<String> = pretokenization(corpus);
    let mut bytes_chunks: Vec<Vec<u32>> = chunks_to_byte_ids(chunked_corpus);
    let mut vocab = initialize_vocab();
    let mut merges = Vec::new();
    while vocab.len() < max_vocab_size {
        let pair_frequencies = count_pairs(&bytes_chunks);
        let Some((pair, _frequency)) = get_most_frequent_pair(pair_frequencies) else {
            break;
        };
        let new_token_id = vocab.len() as u32;
        vocab = update_vocabulary(vocab, pair);
        merges.push((pair, new_token_id ));
        bytes_chunks = merge_pair(pair, bytes_chunks, new_token_id);



    }
    BpeTokenizer {
        vocab,
        merges
    }
}




pub fn merge_pair(freq_pair: (u32, u32), mut bytes_chunks: Vec<Vec<u32>>, new_token_id: u32) -> Vec<Vec<u32>>{
    for chunk in &mut bytes_chunks{
        let mut i = 0;
        while i + 1 < chunk.len(){
            if chunk[i] == freq_pair.0 && chunk[i + 1] == freq_pair.1{
                chunk.remove(i);
                chunk.remove(i);
                chunk.insert(i, new_token_id);
            }
            else{
                i += 1
            }
            
        }
    }
    bytes_chunks
}





#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_create_pair_from_chunk(){
        let byte_chunk = vec![72, 101, 108, 108, 111];
        let expected = vec![(72, 101), (101, 108), (108, 108), (108, 111)];
        assert_eq!(create_pairs_from_chunk(byte_chunk), expected);

    }
    #[test]
    fn test_create_pairs_from_chunk_single_byte() {
        let byte_chunk = vec![72];
        let expected: Vec<(u32, u32)> = vec![];

        assert_eq!(create_pairs_from_chunk(byte_chunk), expected);
    }
    #[test]
    fn test_create_pairs_from_chunk_empty() {
        let byte_chunk: Vec<u32> = vec![];
        let expected: Vec<(u32, u32)> = vec![];

        assert_eq!(create_pairs_from_chunk(byte_chunk), expected);
    }
    #[test]
    fn test_count_pairs(){
        let byte_chunks = vec![
            vec![72, 101, 108, 108, 111],
            vec![44],
            vec![32],
            vec![119, 111, 114, 108, 100],
            vec![33],
            vec![32],
            vec![72, 111, 119],
            vec![32],
            vec![97, 114, 101],
            vec![32],
            vec![121, 111, 117],
            vec![63],
        ];
        let expected = HashMap::from([
            ((72, 101), 1),  // He
            ((101, 108), 1), // el
            ((108, 108), 1), // ll
            ((108, 111), 1), // lo
            ((119, 111), 1), // wo
            ((111, 114), 1), // or
            ((114, 108), 1), // rl
            ((108, 100), 1), // ld
            ((72, 111), 1),  // Ho
            ((111, 119), 1), // ow
            ((97, 114), 1),  // ar
            ((114, 101), 1), // re
            ((121, 111), 1), // yo
            ((111, 117), 1), // ou
        ]);
        assert_eq!(count_pairs(&byte_chunks), expected)
    }
#[test]
fn test_count_most_frequent_pair() {
    let pair_frequency = HashMap::from([
        ((72, 101), 5),
        ((108, 108), 12),
        ((101, 108), 8),
        ((119, 111), 3),
    ]);

    let expected = Some(((108, 108), 12));

    assert_eq!(
        get_most_frequent_pair(pair_frequency),
        expected
    );
}
#[test]
fn test_count_most_frequent_pair_empty() {
    let pair_frequency: HashMap<(u32, u32), usize> = HashMap::new();

    assert_eq!(get_most_frequent_pair(pair_frequency), None);
}
#[test]
fn test_merge_pair() {
    let new_token_id = 256;

    let byte_chunks = vec![
        vec![72, 101, 108, 108, 111],
        vec![44],
        vec![32],
        vec![119, 111, 114, 108, 100],
        vec![33],
        vec![32],
        vec![72, 111, 119],
        vec![32],
        vec![97, 114, 101],
        vec![32],
        vec![121, 111, 117],
        vec![63],
    ];

    let freq_pair = (108, 108);

    let expected = vec![
        vec![72, 101, 256, 111],
        vec![44],
        vec![32],
        vec![119, 111, 114, 108, 100],
        vec![33],
        vec![32],
        vec![72, 111, 119],
        vec![32],
        vec![97, 114, 101],
        vec![32],
        vec![121, 111, 117],
        vec![63],
    ];

    assert_eq!(
        merge_pair(freq_pair, byte_chunks, new_token_id),
        expected
    );
}
#[test]
fn test_initialize_vocab() {

    let expected:Vec<Vec<u8>> = (0..=255)
    .map(|b| vec![b])
    .collect();


    assert_eq!(initialize_vocab(), expected)
}
#[test]
fn test_update_vocabulary() {
    let vocab = initialize_vocab();

    let new_pair = (108, 108);

    let updated_vocab = update_vocabulary(vocab, new_pair);

    assert_eq!(updated_vocab.len(), 257);
    assert_eq!(updated_vocab[256], vec![108, 108]);
}
#[test]
fn test_update_vocabulary_with_merged_token() {
    let mut vocab = initialize_vocab();

    // First merge: (108, 108) -> 256
    vocab = update_vocabulary(vocab, (108, 108));

    // Second merge: (101, 256) -> 257
    vocab = update_vocabulary(vocab, (101, 256));

    assert_eq!(vocab.len(), 258);
    assert_eq!(vocab[256], vec![108, 108]);
    assert_eq!(vocab[257], vec![101, 108, 108]);
}



#[test]
fn test_train_bpe() {
    let corpus = "Hello Hello";
    let max_vocab_size = 260;

    let tokenizer = train_bpe(corpus, max_vocab_size);

    // Vocabulary should never exceed target size
    assert!(tokenizer.vocab.len() <= max_vocab_size);

    // Initial vocabulary must contain all 256 byte tokens
    assert_eq!(tokenizer.vocab[0], vec![0]);
    assert_eq!(tokenizer.vocab[255], vec![255]);

    // Every merge should create exactly one new token
    assert_eq!(tokenizer.vocab.len(), 256 + tokenizer.merges.len());

    // Every merge rule should point to the corresponding vocabulary entry
    for (i, (pair, new_token_id)) in tokenizer.merges.iter().enumerate() {
        assert_eq!(*new_token_id as usize, 256 + i);

        let mut expected = tokenizer.vocab[pair.0 as usize].clone();
        expected.extend(tokenizer.vocab[pair.1 as usize].clone());

        assert_eq!(tokenizer.vocab[*new_token_id as usize], expected);
    }
}
#[test]
fn test_train_bpe_empty_corpus() {
    let tokenizer = train_bpe("", 300);

    assert_eq!(tokenizer.vocab.len(), 256);
    assert!(tokenizer.merges.is_empty());
}

#[test]
fn test_train_bpe_target_256() {
    let tokenizer = train_bpe("Hello", 256);

    assert_eq!(tokenizer.vocab.len(), 256);
    assert!(tokenizer.merges.is_empty());
}
}