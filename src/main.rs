use byteforge::pretokenization;
use byteforge::str_chunk_to_bytes;
fn main() {
    println!("{:?}",pretokenization("Hello world"));
    println!("{:?}",str_chunk_to_bytes(pretokenization("Hello world")));
}
