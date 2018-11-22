use std::collections::BTreeMap;
use apply::Apply;

/// A datastructure that sacrifices size for speed, creating a large map of substrings to their
/// constituent larger string.
#[derive(Debug, Clone)]
pub struct ReverseIndex {
    map: BTreeMap<String, Vec<usize>>, // the usize is an index
    buffer: Box<[String]> // not growable
}


impl ReverseIndex {
    pub fn from_buffer(buffer: Vec<String>) -> ReverseIndex {
        let map: BTreeMap<String, Vec<usize>> = BTreeMap::new();

        let mut ri = ReverseIndex {
            map,
            buffer: buffer.into_boxed_slice()
        };

        for index in 0..ri.buffer.len() {
            ri.split_string_and_index(index);
        }
        ri
    }

    /// String_index MUST be in the range of the buffer.
    /// Gets the string in the buffer at the specified index,
    /// splits the string into substrings in the order [0..1] through [0..n],
    /// inserts these substrings as keys into the reverse index, pointing to the index.
    fn split_string_and_index(&mut self, string_index: usize) {
        let string = &self.buffer[string_index];
        let len = string.len();
        for i in 1..=len {
            let key: String = string.chars().take(i).collect();
            let key_len = key.len();
            self.map.entry(key)
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

    // TODO this needs tests
    /// This will cause the backing buffer to no longer be sorted, and offers no protection against
    /// duplication, but as a benefit, it doesn't have to reindex, which makes this fast-ish.
    pub fn add_word(self, string: String) -> Self {
        let mut ri = ReverseIndex {
            map: self.map,
            buffer: {
                let mut b = Vec::from(self.buffer);
                b.push(string);
                b.into_boxed_slice()
            }
        };
        let length = ri.buffer.len();

        ri.split_string_and_index(length - 1);
        ri
    }
    
    /// Given a string of a substring of a string, return all matching strings from the reverse
    /// index.
    pub fn get(&self, search: &str) -> Option<Vec<String>> {
        let indicies: &Vec<usize>  = self.map.get(search)?;
        indicies
            .iter()
            .map(|index| {
                self.buffer.get(*index).unwrap().clone()
            })
            .collect::<Vec<String>>()
            .apply(Some)
    }
    pub fn eject_buffer(self) -> Vec<String> {
        Vec::from(self.buffer)
    }

    /// Adds another vector of words to the ri.
    /// It performs sorting, and deduplication of the new buffer.
    /// Then it reindexes.
    pub fn concatonate_dedup_and_reindex(self, mut buffer: Vec<String>) -> Self{
        let mut old_buffer = self.eject_buffer();
        old_buffer.append(&mut buffer);
        old_buffer.sort();
        old_buffer.dedup();
        Self::from_buffer(old_buffer)
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

    let ri = ReverseIndex::from_buffer(words);
    let found = ri.get("app").unwrap();
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

    let ri = ReverseIndex::from_buffer(words);
    let found = ri.get("apple").unwrap();
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

    let ri = ReverseIndex::from_buffer(words);
    let ri = ri.add_word("yeet".to_string());

    let found = ri.get("ye").unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found.get(0).unwrap(), &"yeet".to_string());
}
