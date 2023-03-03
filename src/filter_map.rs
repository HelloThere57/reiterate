use crate::{Filter, Iterator, Map};

pub struct FilterMap<O, F, N>
where
    O: Iterator,
    F: FnMut(<O as Iterator>::Item) -> Option<N>,
{
    map: Map<O, F>,
}

impl<O, F, N> FilterMap<O, F, N>
where
    O: Iterator,
    F: FnMut(O::Item) -> Option<N>,
{
    pub fn new(outer: O, f: F) -> Self {
        FilterMap {
            map: Map::new(outer, f),
        }
    }
}

impl<O, F, N> Iterator for FilterMap<O, F, N>
where
    O: Iterator,
    F: FnMut(O::Item) -> Option<N>,
{
    type Item = N;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let n = self.map.next()?;
            if n.is_some() {
                break n;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{FilterMap, IntoIterator, Iterator};

    #[test]
    fn get() {
        let items = vec![0, 1, 2, 3, 4];
        let indices = vec![3, 5, 7, 2, 8, 999999, 0];
        let iter = IntoIterator::into_iter(indices);
        let mut filter_map = FilterMap::new(iter, |i| items.get(i));
        assert_eq!(filter_map.next(), Some(&3));
        assert_eq!(filter_map.next(), Some(&2));
        assert_eq!(filter_map.next(), Some(&0));
        assert_eq!(filter_map.next(), None);
    }

    #[test]
    fn checked_math() {
        let numbers: Vec<u32> = vec![0, 8, 2, 5, 4, 13];
        let iter = IntoIterator::into_iter(numbers);
        let mut filter_map = FilterMap::new(iter, |n| n.checked_sub(5));

        assert_eq!(filter_map.next(), Some(3));
        assert_eq!(filter_map.next(), Some(0));
        assert_eq!(filter_map.next(), Some(8));
        assert_eq!(filter_map.next(), None);
    }
}
