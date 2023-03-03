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

mod filter;
mod filter_map;
mod flat_map;
mod flatten;
mod into_iter;
mod map;

pub use {
    filter::Filter,
    filter_map::FilterMap,
    flat_map::FlatMap,
    flatten::Flatten,
    into_iter::{IntoIter, IntoIterator},
    map::Map,
};
