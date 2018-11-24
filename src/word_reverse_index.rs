use crate::reverse_index::ReverseIndex;
use apply::Apply;

/// The WordReverseIndex will ingest a set of words and will segment those words into 
/// substrings of  [0..1] through [0..n] where n is the length of the word.
/// This creates a reverse index where given an substring (starting at the beginning)
/// of an indexed word, a list of full words can be retrieved.
/// This structure is useful in constructing a basic completion engine, commonly found in shells.
pub struct WordReverseIndex (ReverseIndex<String>);

impl WordReverseIndex {
    /// String_index MUST be in the range of the buffer.
    /// Gets the string in the buffer at the specified index,
    /// splits the string into substrings in the order [0..1] through [0..n],
    /// inserts these substrings as keys into the reverse index, pointing to the index.
    fn split_string_and_index(ri: &mut ReverseIndex<String>, string_index: usize) {
        let string = &ri.buffer[string_index];
        let len = string.len();
        for i in 1..=len {
            let key: String = string.chars().take(i).collect();
            let key_len = key.len();
            ri.map.entry(key)
                .and_modify(|v| v.push(string_index))
                .or_insert_with(|| {
                    let mut vec: Vec<usize> = if key_len == 1 {
                        Vec::with_capacity(20)
                    } else {
                        Vec::with_capacity(2)
                    };
                    vec.push(string_index);
                    vec
                }); 
        }
    }

    pub fn from_buffer(buffer: Vec<String> ) -> Self {
        ReverseIndex::from_buffer(buffer, WordReverseIndex::split_string_and_index).apply(WordReverseIndex)
    }

    pub fn add_word(self, string: String) -> Self {
        self.0
            .add_word(string, WordReverseIndex::split_string_and_index)
            .apply(WordReverseIndex)
    }

    pub fn get(&self, search: &str) -> Vec<&String> {
        self.0.get(search)
    }

    pub fn concatonate_dedup_and_reindex(self, buffer: Vec<String>) -> Self {
        self.0
            .concatonate_dedup_and_reindex(buffer, WordReverseIndex::split_string_and_index)
            .apply(WordReverseIndex)
    }
}

#[test]
fn ri_substring() {
    let words: Vec<String> = vec![
        "app",
        "apple",
        "banana",
        "yeet",
        "hello",
        "hell"
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let ri = WordReverseIndex::from_buffer(words);
    let found = ri.get("app");
    assert_eq!(found.len(), 2);
}

#[test]
fn ri_fullstring() {
    let words: Vec<String> = vec![
        "app",
        "apple",
        "banana",
        "yeet",
        "hello",
        "hell"
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let ri = WordReverseIndex::from_buffer(words);
    let found = ri.get("apple");
    assert_eq!(found.len(), 1);
}

#[test]
fn ri_add_word() {
    let words: Vec<String> = vec![
        "app",
        "apple"
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let ri = WordReverseIndex::from_buffer(words);
    let ri = ri.add_word("yeet".to_string());

    let found = ri.get("ye");
    assert_eq!(found.len(), 1);
    assert_eq!(**found.get(0).unwrap(), "yeet".to_string());
}
