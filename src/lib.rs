use std::collections::HashMap;

pub fn pretokenization(corpus: &str) -> Vec<String>{
    let mut chunks: Vec<String> = Vec::new();
    let mut curr_chunk: String = String::new();
    for ch in corpus.chars(){

        if ch.is_alphabetic(){
            curr_chunk.push(ch);

        } else{
            if curr_chunk.len() !=0{
                chunks.push(curr_chunk.clone());
                chunks.clear();
            }
            //handling non alpha

        }
    }
    return chunks;
}

fn str_to_byte(text: &str) -> Vec<u8> {
    let mut str_bytes : Vec<u8> = Vec::new();
    for ch_byte in text.bytes() {
        str_bytes.push(ch_byte);
    }
    str_bytes
}

fn get_freq_pair(bytes: &[u8]) -> HashMap<(u8, u8), u32> {
    let mut freq_map = HashMap::new();
    for pair in bytes.windows(2) {
        freq_map.entry((pair[0], pair[1]))
                .and_modify(|count| *count += 1)
                .or_insert(1);
    }
    freq_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretokenization() {
        let corpus = "Hello world";
        assert_eq!(pretokenization(&corpus), vec!(["Hello"," ", "world"]));
    }
}
