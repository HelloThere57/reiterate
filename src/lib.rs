#![allow(unused)]

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    fn count(mut self) -> usize
    where
        Self: Sized,
    {
        let mut count = 0;
        while let Some(_) = self.next() {
            count += 1;
        }
        count
    }
}

pub mod flatten;
pub mod into_iter;
pub mod map;

pub use {
    flatten::Flatten,
    into_iter::{IntoIter, IntoIterator},
    map::Map,
};
