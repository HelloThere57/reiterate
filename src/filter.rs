pub struct Filter<O, F>
where
    O: crate::Iterator,
    F: Fn(&O::Item) -> bool,
{
    outer: O,
    f: F,
}

impl<O, F> Filter<O, F>
where
    O: crate::Iterator,
    F: Fn(&O::Item) -> bool,
{
    pub fn new(outer: O, f: F) -> Self {
        Filter { outer, f }
    }
}

impl<O, F> crate::Iterator for Filter<O, F>
where
    O: crate::Iterator,
    F: Fn(&O::Item) -> bool,
{
    type Item = O::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let r = self.outer.next()?;
            if (self.f)(&r) {
                break Some(r);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Filter, Iterator};

    #[test]
    fn all() {
        let items: Vec<i32> = vec![0, 1, 2, 3];
        let iter = crate::IntoIterator::into_iter(items);
        let mut filtered = Filter::new(iter, |_| true);

        assert_eq!(filtered.next(), Some(0));
        assert_eq!(filtered.next(), Some(1));
        assert_eq!(filtered.next(), Some(2));
        assert_eq!(filtered.next(), Some(3));
        assert_eq!(filtered.next(), None);
    }
    #[test]
    fn none() {
        let items: Vec<i32> = vec![0, 1, 2, 3];
        let iter = crate::IntoIterator::into_iter(items);
        let mut filtered = Filter::new(iter, |_| false);

        assert_eq!(filtered.next(), None);
    }
    #[test]
    fn even() {
        let items: Vec<i32> = vec![0, 1, 2, 3];
        let iter = crate::IntoIterator::into_iter(items);
        let mut filtered = Filter::new(iter, |n| n % 2 == 0);

        assert_eq!(filtered.next(), Some(0));
        assert_eq!(filtered.next(), Some(2));
        assert_eq!(filtered.next(), None);
    }
    #[test]
    fn strings() {
        let items: Vec<&str> = vec!["ab", "cd", "e", "f"];
        let iter = crate::IntoIterator::into_iter(items);
        let mut filtered = Filter::new(iter, |s| s.len() == 2);

        assert_eq!(filtered.next(), Some("ab"));
        assert_eq!(filtered.next(), Some("cd"));
        assert_eq!(filtered.next(), None);
    }
}
