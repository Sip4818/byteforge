
    use std::collections::HashMap;
    pub fn pretokenization(corpus: &str) -> Vec<String>{
        let mut chunks: Vec<String> = Vec::new();
        let mut curr_chunk: String = String::new();
        for ch in corpus.chars(){

            if ch.is_alphabetic(){
                curr_chunk.push(ch);

            } else{
                if curr_chunk.len() != 0{
                    chunks.push(curr_chunk.clone());
                    curr_chunk.clear();
                }
                chunks.push(ch.to_string().clone())
            }
        
        }
        if !curr_chunk.is_empty(){
            chunks.push(curr_chunk.clone());
            curr_chunk.clear()
        }
        return chunks;
    }

    pub fn str_chunk_to_bytes(str_chunk: Vec<String>) -> Vec<Vec<u8>>{
        let mut bytes_chunks: Vec<Vec<u8>> = Vec::new();
        for chunk in str_chunk{
            bytes_chunks.push(chunk.clone().into_bytes());
            
        }
        bytes_chunks
    }
    pub fn create_pairs_from_chunk(chunk: Vec<u8>) -> Vec<(u8, u8)>{
        let n = chunk.len();

        let mut pairs: Vec<(u8, u8)> = Vec::new();
            if n == 0{
            return pairs;
        }
        for i in 0..n-1{
            pairs.push((chunk[i], chunk[i + 1]));
        }
        pairs
        
    }


    pub fn count_pairs(bytes_chunks: Vec<Vec<u8>>) -> HashMap<(u8, u8), usize>{
        let mut pair_frequency : HashMap<(u8, u8), usize> = HashMap::new();
        for chunk in bytes_chunks{
            let pairs = create_pairs_from_chunk(chunk);
            for pair in pairs{
                *pair_frequency.entry(pair).or_insert(0) += 1;
            }
        }
        pair_frequency


    }
    pub fn get_most_frequent_pair(pair_frequency: HashMap<(u8, u8), usize>) -> Option<((u8,u8), usize)>{
        let mut most_freq_pair: Option<((u8, u8), usize)> = None;
        let mut max_freq: usize = 0;

        for (pair, freq) in &pair_frequency {
            if *freq > max_freq{
                max_freq = *freq;
                most_freq_pair = Some((*pair, *freq));
            }  
        }
        return most_freq_pair 

    }




    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_pretokenization_with_string() {
            let corpus = "Hello, world! How are you?";

            let expected = Vec::from([
                "Hello", ",", " ", "world", "!", " ",
                "How", " ", "are", " ", "you", "?"
            ]);

            assert_eq!(pretokenization(corpus), expected);
        }

        #[test]
        fn test_pretokenization_with_empty_string() {
            let corpus = "";

            let expected: Vec<String> = Vec::new();

            assert_eq!(pretokenization(corpus), expected);
        }
        #[test]
        fn test_str_chunk_to_bytes(){
            let corpus_chunk = Vec::from([
                "Hello".to_string(), ",".to_string(), " ".to_string(), "world".to_string(), "!".to_string(), " ".to_string(),
                "How".to_string(), " ".to_string(), "are".to_string(), " ".to_string(), "you".to_string(), "?".to_string()
            ]);

            let expected = vec![
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
            assert_eq!(str_chunk_to_bytes(corpus_chunk), expected);
        }
        #[test]
        fn test_create_pair_from_chunk(){
            let byte_chunk = vec![72, 101, 108, 108, 111];
            let expected = vec![(72, 101), (101, 108), (108, 108), (108, 111)];
            assert_eq!(create_pairs_from_chunk(byte_chunk), expected);

        }
        #[test]
        fn test_create_pairs_from_chunk_single_byte() {
            let byte_chunk = vec![72];
            let expected: Vec<(u8, u8)> = vec![];

            assert_eq!(create_pairs_from_chunk(byte_chunk), expected);
        }
        #[test]
        fn test_create_pairs_from_chunk_empty() {
            let byte_chunk: Vec<u8> = vec![];
            let expected: Vec<(u8, u8)> = vec![];

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
            assert_eq!(count_pairs(byte_chunks), expected)
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
        let pair_frequency: HashMap<(u8, u8), usize> = HashMap::new();

        assert_eq!(get_most_frequent_pair(pair_frequency), None);
    }
    }
