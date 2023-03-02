use std::ptr;

pub trait IntoIterator {
    type Item;
    type IntoIter: crate::Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}

pub struct IntoIter<T> {
    pub(super) current: *mut T,
    pub(super) end: *mut T,
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
        if len == 0 {
            return Self::IntoIter {
                current: ptr::null_mut(),
                end: ptr::null_mut(),
            };
        }
        let first = self.leak().first_mut().unwrap() as *mut T;
        let end = unsafe { first.add(len) };

        Self::IntoIter {
            current: first,
            end,
        }
    }
}
impl<I> IntoIterator for I
where
    I: crate::Iterator,
{
    type Item = I::Item;
    type IntoIter = I;

    #[inline]
    fn into_iter(self) -> I {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::Iterator;
    #[test]
    fn into_iter() {
        let items = vec![0, 1, 2, 3, 4];
        let mut into_iter = crate::IntoIterator::into_iter(items);
        assert_eq!(into_iter.next(), Some(0));
        assert_eq!(into_iter.next(), Some(1));
        assert_eq!(into_iter.next(), Some(2));
        assert_eq!(into_iter.next(), Some(3));
        assert_eq!(into_iter.next(), Some(4));
        assert_eq!(into_iter.next(), None);
    }
}
