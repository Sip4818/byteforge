use std::collections::HashMap;
fn main() {

    let mut vocab: mut Vec<Vec32> = Vec::new()
    for i in 0..256 {
        vocab.push(i)
    }
    let mut count: u32 = 255






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

    let bytes : Vec<u8> = str_to_byte("Hello World");

    fn get_next_Id(counter: mut u32) -> mut u32{
        counter += 1
        counter
    }

    println!("{:?}", get_freq_pair(&bytes));

    



}
