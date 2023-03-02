use std::ptr;

use crate::{Flatten, Iterator, Map};

pub struct FlatMap<O, I, F>
where
    O: crate::Iterator,
    I: crate::IntoIterator,
    F: FnMut(O::Item) -> I,
{
    handler: Flatten<Map<O, F>, I>,
}
impl<O, I, F> FlatMap<O, I, F>
where
    O: crate::Iterator,
    I: crate::IntoIterator,
    F: FnMut(O::Item) -> I,
{
    pub fn new(outer: O, f: F) -> Self {
        let map = Map::new(outer, f);
        let flat_map = Flatten::new(map);

        Self { handler: flat_map }
    }
}

impl<O, I, F> crate::Iterator for FlatMap<O, I, F>
where
    O: crate::Iterator,
    I: crate::IntoIterator,
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

    fn double() {
        let items: Vec<i32> = vec![0, 1, 2];
        let iter = crate::IntoIterator::into_iter(items);
        let flat_map = FlatMap::new(iter, |_| [""]);
    }
}
