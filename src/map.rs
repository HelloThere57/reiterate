pub struct Map<O, F, B>
where
    O: crate::Iterator,
    F: FnMut(O::Item) -> B,
{
    outer: O,
    f: F,
}
impl<O, F, B> Map<O, F, B>
where
    O: crate::Iterator,
    F: FnMut(O::Item) -> B,
{
    pub fn new(outer: O, f: F) -> Self {
        Map { outer, f }
    }
}

impl<O, F, B> crate::Iterator for Map<O, F, B>
where
    O: crate::Iterator,
    F: FnMut(O::Item) -> B,
{
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        self.outer.next().map(|x| (self.f)(x))
    }
}

#[cfg(test)]
mod tests {
    use crate::Iterator;
    #[test]
    fn squares() {
        let items = vec![0i32, 1, 2, 3, 4, 5];
        let iter = crate::IntoIterator::into_iter(items);
        let mut map = crate::Map::new(iter, |n| n * n);
        assert_eq!(map.next(), Some(0));
        assert_eq!(map.next(), Some(1));
        assert_eq!(map.next(), Some(4));
        assert_eq!(map.next(), Some(9));
        assert_eq!(map.next(), Some(16));
        assert_eq!(map.next(), Some(25));
        assert_eq!(map.next(), None);
    }
    #[test]
    fn strings() {
        let items = vec!["a", "b", "c", "d"];
        let iter = crate::IntoIterator::into_iter(items);
        let mut map = crate::Map::new(iter, String::from);
        assert_eq!(map.next(), Some(String::from("a")));
        assert_eq!(map.next(), Some(String::from("b")));
        assert_eq!(map.next(), Some(String::from("c")));
        assert_eq!(map.next(), Some(String::from("d")));
        assert_eq!(map.next(), None);
    }
}
