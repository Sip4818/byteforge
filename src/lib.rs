
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
}
