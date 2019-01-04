use crate::reverse_index::ReverseIndex;
use std::marker::PhantomData;


trait Direction {}

pub struct Forwards;
pub struct Backwards;
impl Direction for Forwards {}
impl Direction for Backwards {}

#[derive(Debug, Clone)]
pub struct RiIter<'a, T, U> {
    starting_index: usize,
    reverse_index: &'a ReverseIndex<T>,
    offset: usize,
    direction: PhantomData<U>
}

impl <'a, T> Iterator for RiIter<'a, T, Forwards> {
    type Item = &'a T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let index = self.starting_index + self.offset;

        if index >= self.reverse_index.buffer.len() {
            return None
        } else {
            let reference: &T = &self.reverse_index[index];
            self.offset += 1;
            Some(reference)
        }
    }
}

impl <'a, T> Iterator for RiIter<'a, T, Backwards> {
    type Item = &'a T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let index = self.starting_index.checked_sub(self.offset)?;
        let reference: &T = &self.reverse_index[index];
        self.offset += 1;
        Some(reference)
    }
}


#[derive(Clone, Debug)]
pub struct RiIndex {
    pub (crate) index: usize,
}

impl  RiIndex {

    pub (crate) fn new(index: usize) -> Self {
        RiIndex {
            index,
        }
    }

    pub fn current<'a, T>(&self, reverse_index: &'a ReverseIndex<T>) -> &'a T {
        &reverse_index[self.index]
    }

    pub fn forwards<'a, T>(&self, reverse_index: &'a ReverseIndex<T>) -> RiIter<'a, T, Forwards> {
        RiIter {
            starting_index: self.index,
            reverse_index,
            offset: 1,
            direction: PhantomData
        }
    }

    pub fn backwards<'a, T>(&self, reverse_index: &'a ReverseIndex<T>) -> RiIter<'a, T, Backwards> {
        RiIter {
            starting_index: self.index,
            reverse_index,
            offset: 1,
            direction: PhantomData
        }
    }
}


