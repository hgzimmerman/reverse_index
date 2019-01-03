use crate::reverse_index::ReverseIndex;
use apply::Apply;
use std::collections::BTreeMap;

use crate::ri_iter::RiIndex;


/// The DocumentReverseIndex will ingest a document and segment it into individual words.
/// These words are used as keys that can be used to match on the whole document.
/// This structure is useful in finding a document given a listing of words that appear in the
/// document. 
pub struct DocumentReverseIndex<T> (ReverseIndex<T>);

impl <T> DocumentReverseIndex<T>
    where T: AsRef<str> + PartialEq + Ord
{
    /// String_index MUST be in the range of the buffer.
    /// Gets the string in the buffer at the specified index,
    /// splits the string into words delimited by whitespace,
    /// inserts these words as keys into the reverse index, pointing to the index.
    fn split_string_and_index(ri: &mut ReverseIndex<T>, string_index: usize) {
        let string: &str = &ri.buffer[string_index].as_ref(); // get the string from the buffer.

        for word in string.split_whitespace() {
            ri.map.entry(word.to_string())
                .and_modify(|v| v.push(string_index))
                .or_insert_with(|| {
                    let mut vec: Vec<usize> = if word.len() == 1 {
                        Vec::with_capacity(20)
                    } else {
                        Vec::with_capacity(7)
                    };
                    vec.push(string_index);
                    vec
                }); 
        }
    }

    /// Constructs the reverse index from a buffer of documents.
    /// This offers no deduplication functionality.
    pub fn from_buffer(buffer: Vec<T> ) -> Self {
        ReverseIndex::<T>::from_buffer(buffer, DocumentReverseIndex::split_string_and_index).apply(DocumentReverseIndex)
    }

    pub fn from_iter(iter: impl Iterator<Item=T>) -> Self {
        ReverseIndex::<T>::from_iter(iter, DocumentReverseIndex::split_string_and_index).apply(DocumentReverseIndex)
    }

    /// Adds a document to the reverse index.
    pub fn add_document(self, string: T) -> Self {
        self.0.add_word(string, DocumentReverseIndex::split_string_and_index).apply(DocumentReverseIndex)
    }

    /// Gets the indicies that can be used to look up the full documents.
    fn get_raw_indicies(&self, search: &str) -> impl Iterator<Item=usize> {
        let search_words: Vec<&str> = search.split_whitespace().collect();

        // Get the documents where the indices appear the most.
        let indicies_to_counts = search_words
            .iter()
            .filter_map(|word| {
                self.0.map.get(*word) // Get matching indices
            })
            .fold(BTreeMap::<usize,usize>::new(), |mut map, indices| { // BufferIndex -> Count.
                indices
                    .iter()
                    .for_each(|index| {
                        map.entry(*index)
                            .and_modify(|count| *count += 1)
                            .or_insert_with(|| 1);
                    });
                map
            });
        indicies_to_counts.into_iter().map(|(index, _count)| index)
    }

    /// Given a search string, this will split the search string by whitespace 
    /// and search the reverse index for it.
    /// It will select n documents ordered by how many of the search terms appeared in each
    /// document.
    ///
    /// # Arguments
    /// * `search` - The search string used to find documents.
    /// * `number_of_documents` - The upper bound on the number of documents to return.
    pub fn get(&self, search: &str, number_of_documents: usize) -> Vec<&T> {
        self.get_raw_indicies(search)
            .filter_map(|index| {
                self.0.buffer.get(index)
            })
            .take(number_of_documents)
            .collect()
    }

    /// Gets a list of iterators that can be used get and search around to adjacently ordered documents.
    pub fn get_indices(&self, search: &str, number_of_documents: usize) -> Vec<RiIndex<T>> {
        self.get_raw_indicies(search)
            .map(|index| {
                RiIndex::new(index, &self.0)
            })
            .take(number_of_documents)
            .collect()
    }

    /// Adds a buffer of other documents, sorts them, uses the structured buffer, and then reruns
    /// the indexing process.
    pub fn concatonate_dedup_and_reindex(self, buffer: Vec<T>) -> Self {
        self.0
            .concatonate_dedup_and_reindex(buffer, DocumentReverseIndex::split_string_and_index)
            .apply(DocumentReverseIndex)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Document {
        pub name: String,
        pub content: String
    }

    impl AsRef<str> for Document {
        fn as_ref(&self) -> &str {
            &self.content
        }
    }

    #[test]
    fn get() {
        let documents: Vec<String> = vec![
            "the quick brown fox jumps over the lazy dog",
            "lorem ipusm dolor sit",
            "brown jumps"
        ]
            .into_iter()
            .map(String::from)
            .collect();

        let ri = DocumentReverseIndex::from_buffer(documents);
        let found = ri.get("the", 10);
        assert_eq!(found.len(), 1);
    }


    #[test]
    fn get_ordered() {
        let documents: Vec<String> = vec![
            "the quick brown fox jumps over the lazy dog",
            "lorem ipusm dolor sit",
            "brown jumps"
        ]
            .into_iter()
            .map(String::from)
            .collect();

        let ri = DocumentReverseIndex::from_buffer(documents);
        let found = ri.get("brown fox jumps", 10);
        assert_eq!(found.len(), 2);
        // The first returned element should be the one that matches more words
        assert_eq!(found[0], "the quick brown fox jumps over the lazy dog");
    }

    // TODO it would be nice to do a pass that will reorder based on how well the query sequentially
// matches the document.
    #[test]
    fn get_equal_query_quality() {
        let documents: Vec<String> = vec![
            "the quick brown fox jumps over the lazy dog",
            "lorem ipusm dolor sit",
            "brown jumps"
        ]
            .into_iter()
            .map(String::from)
            .collect();

        let ri = DocumentReverseIndex::from_buffer(documents);
        let found = ri.get("brown jumps", 10);
        assert_eq!(found.len(), 2);
        // The first returned element should be the one that matches more words
        assert_eq!(found[0], "the quick brown fox jumps over the lazy dog");
    }


    #[test]
    fn get_iters_forward() {
        let documents: Vec<String> = vec![
            "the quick brown fox jumps over the lazy dog",
            "lorem ipsum dolor sit",
            "brown jumps"
        ]
            .into_iter()
            .map(String::from)
            .collect();

        let ri = DocumentReverseIndex::from_buffer(documents);
        let found = ri.get_iters("the", 10);
        assert_eq!(found.len(), 1);
        let mut forwards = found[0].forwards();
        assert_eq!(forwards.next().unwrap(), &String::from("lorem ipsum dolor sit"));
        assert_eq!(forwards.next().unwrap(), &String::from("brown jumps"));
        assert!(forwards.next().is_none())
    }

    #[test]
    fn get_iters_backward() {
        let documents: Vec<String> = vec![
            "the quick brown fox jumps over the lazy dog",
            "lorem ipsum dolor sit",
            "brown jumps"
        ]
            .into_iter()
            .map(String::from)
            .collect();

        let ri = DocumentReverseIndex::from_buffer(documents);
        let found = ri.get_iters("the", 10);
        assert_eq!(found.len(), 1);
        let ri_index = &found[0];
        let mut backwards = ri_index.backwards();
        assert!(backwards.next().is_none())
    }
}
