use byteforge::tokenizer::BpeTokenizer;

fn main() {
    let corpus = r#"
    The quick brown fox jumps over the lazy dog.
    The quick brown fox jumps over the lazy dog.
    The quick brown fox jumps over the lazy dog.

    Machine learning allows computers to learn patterns from data.
    Machine learning models identify patterns and use them to make predictions.
    Data science combines statistics, programming, and domain knowledge.
    Data science helps people understand data and make better decisions.

    Rust is a systems programming language focused on performance and safety.
    Rust provides memory safety without requiring a garbage collector.
    Rust uses ownership, borrowing, and lifetimes to manage memory.
    A Rust programmer can build fast and reliable software.

    A tokenizer converts text into smaller units called tokens.
    A byte pair encoding tokenizer begins with individual bytes.
    It repeatedly finds the most frequent adjacent token pair.
    The tokenizer merges that pair and adds a new token to its vocabulary.
    The process continues until the desired vocabulary size is reached.

    The quick brown fox jumps over the lazy dog.
    Machine learning models learn from examples.
    Rust makes systems programming safe and efficient.
    A tokenizer learns frequent patterns in text.
    "#;

    let max_vocab_size = 350;
    let tokenizer = BpeTokenizer::train(corpus, max_vocab_size);

    let test_text = r#"
        Byteforge is a byte-level BPE tokenizer built in Rust!

        In 2026, machine learning is everywhere:
        - NLP processes human language.
        - LLMs generate text using tokens.
        - Rust provides speed, safety, and control.

        Let's test numbers: 1234567890, 3.14159, and 100%.
        Let's test punctuation: ! ? , . ; : ' " ( ) [ ] { }

        Unicode test: café, naïve, £50, €, 日本語, 🚀, 😊.

        Repeated patterns:
        token token tokenizer tokenization
        Rust Rust Rust
        machine learning machine learning

        The quick brown fox jumps over the lazy dog.
        End of test!
        "#;

    let tokens = tokenizer.encode(test_text);
    let decoded = tokenizer.decode(&tokens);

    println!("{:?}", tokens);
    println!("{}", decoded);

    assert_eq!(test_text, decoded);

    println!("Round-trip test passed!");
}