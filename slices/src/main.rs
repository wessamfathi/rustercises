fn main() {
    let my_string = String::from("hello world");

    // first_word worka on slices
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    // also on references
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // also on slices of literals
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // also on literals
    let word = first_word(&my_string_literal);

    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
