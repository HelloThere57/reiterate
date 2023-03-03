use std::ptr;

use crate::{Flatten, IntoIterator, Iterator, Map};

pub struct FlatMap<O, I, F>
where
    O: Iterator,
    I: IntoIterator,
    F: FnMut(O::Item) -> I,
{
    handler: Flatten<Map<O, F>, I>,
}
impl<O, I, F> FlatMap<O, I, F>
where
    O: Iterator,
    I: IntoIterator,
    F: FnMut(O::Item) -> I,
{
    pub fn new(outer: O, f: F) -> Self {
        let map = Map::new(outer, f);
        let flat_map = Flatten::new(map);

        Self { handler: flat_map }
    }
}

impl<O, I, F> Iterator for FlatMap<O, I, F>
where
    O: Iterator,
    I: IntoIterator,
    F: FnMut(O::Item) -> I,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.handler.next()
    }
}
#[cfg(test)]
mod tests {
    use super::FlatMap;
    use crate::{IntoIterator, Iterator};

    #[test]
    fn double() {
        let items: Vec<i32> = vec![0, 1, 2];
        let iter = IntoIterator::into_iter(items);
        let mut flat_map = FlatMap::new(iter, |n| [n, n]);
        assert_eq!(flat_map.next(), Some(0));
        assert_eq!(flat_map.next(), Some(0));
        assert_eq!(flat_map.next(), Some(1));
        assert_eq!(flat_map.next(), Some(1));
        assert_eq!(flat_map.next(), Some(2));
        assert_eq!(flat_map.next(), Some(2));
        assert_eq!(flat_map.next(), None);
    }

    #[test]
    fn grid_add() {
        let items: Vec<Vec<i32>> = vec![vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]];
        let iter = IntoIterator::into_iter(items);
        let mut flat_map = FlatMap::new(iter, |mut n| {
            n.push(3);
            n
        });
        assert_eq!(flat_map.next(), Some(0));
        assert_eq!(flat_map.next(), Some(1));
        assert_eq!(flat_map.next(), Some(2));
        assert_eq!(flat_map.next(), Some(3));
        assert_eq!(flat_map.next(), Some(0));
        assert_eq!(flat_map.next(), Some(1));
        assert_eq!(flat_map.next(), Some(2));
        assert_eq!(flat_map.next(), Some(3));
        assert_eq!(flat_map.next(), Some(0));
        assert_eq!(flat_map.next(), Some(1));
        assert_eq!(flat_map.next(), Some(2));
        assert_eq!(flat_map.next(), Some(3));
        assert_eq!(flat_map.next(), None);
    }
}
