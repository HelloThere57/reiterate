use crate::{IntoIterator, Iterator};

pub struct Flatten<O, I>
where
    O: Iterator<Item = I>,
    I: IntoIterator,
{
    outer: O,
    inner: Option<I::IntoIter>,
}

impl<O, I> Flatten<O, I>
where
    O: Iterator<Item = I>,
    I: IntoIterator,
{
    pub fn new(mut outer: O) -> Self {
        let inner = outer.next().map(IntoIterator::into_iter);
        Flatten { outer, inner }
    }
}

impl<O, I> Iterator for Flatten<O, I>
where
    O: Iterator<Item = I>,
    I: IntoIterator,
{
    type Item = <<O as Iterator>::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.as_mut().and_then(|i| i.next()) {
                Some(item) => break Some(item),
                None => {
                    match self.outer.next() {
                        Some(iterator) => self.inner = Some(iterator.into_iter()),
                        None => break None,
                    };
                    continue;
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Flatten, IntoIterator, Iterator};

    #[test]
    fn empty() {
        let empty: Vec<Vec<()>> = vec![vec![], vec![], vec![]];
        let iter = IntoIterator::into_iter(empty);
        let mut flat = Flatten::new(iter);
        assert_eq!(flat.count(), 0);
    }
    #[test]
    fn many() {
        let items = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
        let iter = IntoIterator::into_iter(items);
        let mut flat = Flatten::new(iter);
        assert_eq!(flat.next(), Some('a'));
        assert_eq!(flat.next(), Some('b'));
        assert_eq!(flat.next(), Some('c'));
        assert_eq!(flat.next(), Some('d'));
        assert_eq!(flat.next(), Some('e'));
        assert_eq!(flat.next(), Some('f'));
        assert_eq!(flat.next(), None);
    }
}
