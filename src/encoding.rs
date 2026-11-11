use crate::preprocessing:: {
    pretokenization,
    chunks_to_byte_ids
};

pub fn merge_chunks(byte_chunks : Vec<Vec<u32>>) -> Vec<u32> {
    let mut big_chunk:Vec<u32> = Vec::new();
    for chunk in byte_chunks {
        big_chunk.extend(chunk.clone());

    }
    big_chunk

}

pub fn encode(text: &str, merge_rules: &[((u32, u32), u32)]) -> Vec<u32> {

    let str_chunks: Vec<String> = pretokenization(text);
    let mut byte_chunks: Vec<Vec<u32>> = chunks_to_byte_ids(str_chunks);

    for rule in merge_rules {

        for chunk in &mut byte_chunks {
            if chunk.len() <= 1 {
                continue;

            }
            let mut i = 0;
            while i + 1 < chunk.len(){
                if chunk[i] == rule.0.0 && chunk[i + 1] == rule.0.1{
                    chunk.remove(i);
                    chunk.remove(i);
                    chunk.insert(i, rule.1);
                }
                else{
                    i += 1;
            }
        
            }
        }
    }
    return merge_chunks(byte_chunks)

}

#[cfg(test)]
mod tests {
    use super:: *;

    #[test]
    fn test_merge_chunks() {
        let input = vec![
            vec![72, 101, 108],
            vec![108, 111],
            vec![32],
            vec![87, 111, 114, 108, 100],
        ];

        let expected = vec![
            72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100
        ];

        assert_eq!(merge_chunks(input), expected);
    }
    #[test]
    fn test_encode() {
        let text = "Hello";

        let merge_rules = vec![
            ((108, 108), 256),
            ((101, 256), 257),
            ((72, 257), 258),
        ];

        let expected = vec![258, 111];

        assert_eq!(encode(text, &merge_rules), expected);
    }

    
}