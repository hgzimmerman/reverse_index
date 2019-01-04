use std::collections::BTreeMap;
use std::ops::Index;

/// A data-structure that sacrifices memory size for speed,
/// creating a large map of substrings to their constituent larger string.
#[derive(Debug, Clone)]
pub struct ReverseIndex<T> {
    pub map: BTreeMap<String, Vec<usize>>, // the usize is an index
    pub buffer: Box<[T]>,
}

/// A function that determines how an element of the buffer will be indexed in the map.
pub trait IndexFn<T>: Fn(&mut ReverseIndex<T>, usize) -> () {}
impl <F, T: AsRef<str>> IndexFn<T> for F where F: Fn(&mut ReverseIndex<T>, usize) -> () {}


impl <T> Index<usize> for ReverseIndex<T> {
    type Output = T;

    fn index(&self, index: usize) -> &<Self as Index<usize>>::Output {
        self.buffer.index(index)
    }
}

impl <T> ReverseIndex<T>
where T: AsRef<str> + PartialEq + Ord
{
    pub fn from_buffer(buffer: Vec<T>, index_fn: impl IndexFn<T> ) -> ReverseIndex<T> {
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

    pub fn from_iter(iter: impl Iterator<Item=T>,  index_fn: impl IndexFn<T>) -> ReverseIndex<T> {
        Self::from_buffer(iter.collect(), index_fn)
    }



    /// This will cause the backing buffer to no longer be sorted, and offers no protection against
    /// duplication, but as a benefit, it doesn't have to reindex, which makes this fast-ish.
    pub fn add_word(self, string: T, index_fn: impl IndexFn<T> ) -> Self {
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
    pub fn get(&self, search: &str) -> Vec<&T> {
        if let Some(indicies)  = self.map.get(search) {
         indicies
            .iter()
            .map(|index| {
                self.buffer.get(*index).unwrap()
            })
            .collect::<Vec<&T>>()

        } else {
            Vec::with_capacity(0)
        }
    }


    pub fn eject_buffer(self) -> Vec<T> {
        Vec::from(self.buffer)
    }

    /// Adds another vector of words to the RI.
    /// It performs sorting, and deduplication of the new buffer.
    /// Then it reindexes.
    pub fn concatonate_dedup_and_reindex(self, mut buffer: Vec<T>, index_fn: impl IndexFn<T>) -> Self {
        let mut old_buffer = self.eject_buffer();
        old_buffer.append(&mut buffer);
        old_buffer.sort();
        old_buffer.dedup();
        Self::from_buffer(old_buffer, index_fn)
    }
}

