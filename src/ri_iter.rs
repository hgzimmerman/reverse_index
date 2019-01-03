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

impl <'a, T> RiIter<'a, T, Forwards> {
    pub (crate) fn new(reverse_index: &'a ReverseIndex<T>, index: usize) -> Self {
        RiIter {
            starting_index: index,
            offset: 0,
            reverse_index,
            direction: PhantomData
        }
    }

    pub fn into_backwards(&self) -> RiIter<'a, T, Backwards> {
         RiIter {
            starting_index: self.starting_index,
            offset: 0,
            reverse_index: self.reverse_index,
            direction: PhantomData
        }
    }
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
pub struct RiIndex<'a, T> {
    index: usize,
    reverse_index: &'a ReverseIndex<T>,
}

impl <'a,T> RiIndex<'a, T> {

    pub (crate) fn new(index: usize, reverse_index: &'a ReverseIndex<T>) -> Self {
        RiIndex {
            index,
            reverse_index
        }
    }

    pub fn current(&self) -> &T {
        &self.reverse_index[self.index]
    }
    pub fn forwards(&self) -> RiIter<'a, T, Forwards> {
        RiIter {
            starting_index: self.index,
            reverse_index: self.reverse_index,
            offset: 1,
            direction: PhantomData
        }
    }

    pub fn backwards(&self) -> RiIter<'a, T, Backwards> {
        RiIter {
            starting_index: self.index,
            reverse_index: self.reverse_index,
            offset: 1,
            direction: PhantomData
        }
    }
}


