
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
            // TODO:handling non alpha
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
    fn test_pretokenization() {
        let corpus = "Hello world";
        // TODO: Update the v as per the implementation of handling non alpha in the function
        let v = Vec::from(["Hello", "world"]);
        assert_eq!(pretokenization(&corpus), v);
    }
}
