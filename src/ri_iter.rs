use crate::reverse_index::ReverseIndex;

pub struct RiIter<'a, T> {
    pub (crate) index: usize,
    pub (crate) reverse_index: &'a ReverseIndex<T>
}

impl <'a, T> Iterator for RiIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.index > self.reverse_index.buffer.len() {
            return None
        } else {
            let reference: &T = &self.reverse_index[self.index];
            self.index += 1;
            Some(reference)
        }
    }
}

impl <'a, T> DoubleEndedIterator for RiIter<'a, T> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.index > self.reverse_index.buffer.len() {
            return None
        } else {
            let reference: &T = &self.reverse_index[self.index];
            self.index -= 1;
            Some(reference)
        }
    }
}