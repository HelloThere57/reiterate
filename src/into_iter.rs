use std::ptr;

pub trait IntoIterator {
    type Item;
    type IntoIter: crate::Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}

pub struct IntoIter<T> {
    pub(super) current: *const T,
    pub(super) end: *const T,
}

impl<T> crate::Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }
        unsafe {
            if (self.current as usize != self.end as usize) {
                let a = self.current.read();
                self.current = self.current.add(1);
                Some(a)
            } else {
                None
            }
        }
    }
}

impl<T> crate::IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = crate::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        let first = self.leak().first().map_or(ptr::null(), |r| r as *const T);
        let end = unsafe { first.add(len) };
        Self::IntoIter {
            current: first,
            end,
        }
    }
}
impl<T, const N: usize> crate::IntoIterator for [T; N] {
    type Item = T;
    type IntoIter = crate::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        let first = match self.first() {
            None => ptr::null(),
            Some(r) => r as *const T,
        };
        let end = unsafe { first.add(len) };
        Self::IntoIter {
            current: first,
            end,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Iterator;
    #[test]
    fn vec() {
        let items = vec![0, 1, 2, 3, 4];
        let mut into_iter = crate::IntoIterator::into_iter(items);
        assert_eq!(into_iter.next(), Some(0));
        assert_eq!(into_iter.next(), Some(1));
        assert_eq!(into_iter.next(), Some(2));
        assert_eq!(into_iter.next(), Some(3));
        assert_eq!(into_iter.next(), Some(4));
        assert_eq!(into_iter.next(), None);
    }
    #[test]
    fn array() {
        let items = [0, 1, 2, 3, 4];
        let mut into_iter = crate::IntoIterator::into_iter(items);
        assert_eq!(into_iter.next(), Some(0));
        assert_eq!(into_iter.next(), Some(1));
        assert_eq!(into_iter.next(), Some(2));
        assert_eq!(into_iter.next(), Some(3));
        assert_eq!(into_iter.next(), Some(4));
        assert_eq!(into_iter.next(), None);
    }
}
