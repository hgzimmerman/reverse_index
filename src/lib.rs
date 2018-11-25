mod reverse_index;
pub mod word_reverse_index;
pub mod document_reverse_index;

pub use self::word_reverse_index::WordReverseIndex;
pub use self::document_reverse_index::{
    Document,
    DocumentReverseIndex
};

