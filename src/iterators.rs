
use crate::{IntoIter, Iter, IterMut, Map};
use std::marker::PhantomData;

impl<'a, V: Clone + 'a> Iterator for Iter<'a, V> {
    type Item = (usize, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_def() {
            let node = unsafe { &*self.head.add(self.current.get()) };
            let mut next = node.get_next();
            std::mem::swap(&mut self.current, &mut next);
            Some((next.get(), node.get().unwrap()))
        } else {
            None
        }
    }
}

impl<'a, V: Clone + 'a> Iterator for IterMut<'a, V> {
    type Item = (usize, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_def() {
            let node = unsafe { &mut *self.head.add(self.current.get()) };
            let mut next = node.get_next();
            std::mem::swap(&mut self.current, &mut next);
            Some((next.get(), node.get_mut().unwrap()))
        } else {
            None
        }
    }
}

impl<'a, V: Clone + 'a> Iterator for IntoIter<'a, V> {
    type Item = (usize, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_def() {
            let node = unsafe { &*self.head.add(self.current.get()) };
            let mut next = node.get_next();
            std::mem::swap(&mut self.current, &mut next);
            Some((next.get(), node.get().unwrap().clone()))
        } else {
            None
        }
    }
}

impl<'a, V: Clone> IntoIterator for &'a Map<V> {
    type Item = (usize, V);
    type IntoIter = IntoIter<'a, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            current: self.first_used,
            head: self.head,
            _marker: PhantomData,
        }
    }
}

impl<V: Clone> Map<V> {
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> Iter<'_, V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't iter() non-initialized Map");
        Iter {
            current: self.first_used,
            head: self.head,
            _marker: PhantomData,
        }
    }
    #[inline]
    #[must_use]
    #[allow(clippy::iter_without_into_iter)]
    pub const fn iter_mut(&mut self) -> IterMut<'_, V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't iter_mut() non-initialized Map");
        IterMut {
            current: self.first_used,
            head: self.head,
            _marker: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub const fn into_iter(&self) -> IntoIter<'_, V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't into_iter() non-initialized Map");
        IntoIter {
            current: self.first_used,
            head: self.head,
            _marker: PhantomData,
        }
    }
}

#[test]
fn insert_and_iterate() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "one");
    m.insert(1, "two");
    m.insert(2, "three");
    let mut sum = 0;
    let mut count = 0;
    for (k, _v) in m.iter() {
        sum += k;
        count += 1;
    }
    assert_eq!(3, count);
    assert_eq!(3, sum);
}

#[cfg(test)]
#[path = "iterators_tests.rs"]
mod tests;
