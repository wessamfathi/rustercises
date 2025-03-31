type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    words
}

fn add_words(this: &mut Document, word: String) {
    this.push(word);
}

fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}

fn main() {
    let words = vec!["hello".to_string()];
    let d = new_document(words);

    // convert by cloning
    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_words(&mut d2, "world".to_string());

    // modification of d2 doesn't affect d
    assert!(!get_words(&d).contains(&"world".into()));
}
