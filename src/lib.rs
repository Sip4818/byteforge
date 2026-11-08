
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
    let mut bytes_chunk: Vec<Vec<u8>> = Vec::new();
    for chunk in str_chunk{
        bytes_chunk.push(chunk.clone().into_bytes());
        
    }
    bytes_chunk
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
            "Hello", ",", " ", "world", "!", " ",
            "How", " ", "are", " ", "you", "?"
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
        assert_eq!(str_chunk_to_bytes(corpus), expected);
    }
}
