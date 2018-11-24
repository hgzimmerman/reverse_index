use std::collections::BTreeMap;
use apply::Apply;

pub mod word_reverse_index;
pub mod document_reverse_index;

/// A datastructure that sacrifices size for speed,
/// creating a large map of substrings to their constituent larger string.
#[derive(Debug, Clone)]
pub(crate) struct ReverseIndex {
    pub map: BTreeMap<String, Vec<usize>>, // the usize is an index
    pub buffer: Box<[String]>,
}

pub(crate) trait IndexFn: Fn(&mut ReverseIndex, usize) -> () {}
impl <F> IndexFn for F where F: Fn(&mut ReverseIndex, usize) -> () {}



impl ReverseIndex {
    pub fn from_buffer(buffer: Vec<String>, index_fn: impl IndexFn ) -> ReverseIndex {
        let map: BTreeMap<String, Vec<usize>> = BTreeMap::new();

        let mut ri = ReverseIndex {
            map,
            buffer: buffer.into_boxed_slice(),
        };

        for index in 0..ri.buffer.len() {
            index_fn(&mut ri,index);
        }
        ri
    }



    /// This will cause the backing buffer to no longer be sorted, and offers no protection against
    /// duplication, but as a benefit, it doesn't have to reindex, which makes this fast-ish.
    pub fn add_word(self, string: String, index_fn: impl IndexFn ) -> Self {
        let mut ri = ReverseIndex {
            map: self.map,
            buffer: {
                let mut b = Vec::from(self.buffer);
                b.push(string);
                b.into_boxed_slice()
            }
        };
        let length = ri.buffer.len();

        index_fn(&mut ri, length - 1);
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
    pub fn concatonate_dedup_and_reindex(self, mut buffer: Vec<String>, index_fn: impl IndexFn) -> Self {
        let mut old_buffer = self.eject_buffer();
        old_buffer.append(&mut buffer);
        old_buffer.sort();
        old_buffer.dedup();
        Self::from_buffer(old_buffer, index_fn)
    }
}


